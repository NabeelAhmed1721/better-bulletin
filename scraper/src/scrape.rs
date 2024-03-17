use crate::{
    bulletin::{
        AttributeList, CampusList, College, CourseIdentifier, CourseRequirementTree,
        FullAttributeList, UndergraduateCourse, UndergraduateCourseDeviantFlags,
        UndergraduateCourseFlags, UndergraduateCourseRequirements, UndergraduateProgram,
        UndergraduateProgramType, BASE_URL,
    },
    utility::{ReplaceMany, TrimAll},
};
use scraper::{ElementRef, Selector};

pub trait Scrappable<T> {
    fn extract_list(html: &str) -> Vec<T>;
}

pub struct ScrapeUndergraduatePrograms;

#[derive(Debug)]
struct ProgramDetails<'a> {
    program_title: &'a str,
    program_type: UndergraduateProgramType,
    program_college: Option<College>,
    program_keywords: Vec<&'a str>,
}

type ProgramTitleDetails<'a> = (&'a str, &'a str);

impl ScrapeUndergraduatePrograms {
    fn extract_program_details<'a>(element: &'a ElementRef) -> ProgramDetails<'a> {
        let raw_program_title = element
            .select(&Selector::parse("span.title").unwrap())
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();

        // extract title details
        let program_keywords = Self::extract_keywords(element);
        let (program_title, raw_program_type) =
            Self::extract_program_title_details(raw_program_title);
        let program_type = Self::extract_program_type(element, raw_program_type);

        let program_college = match program_type {
            UndergraduateProgramType::ROTC(_) => None, // ROTC doesn't have college
            _ => match raw_program_title {
                // TODO: separate noise-filtering logic into own function
                // check against certain noisy data
                "Multidisciplinary Studies, B.A. (University College)" => {
                    Some(College::UniversityCollege)
                }
                "Science, B.S. (Behrend)" => Some(College::Behrend),
                _ => Some(College::from(*program_keywords.get(1).unwrap())),
            },
        };

        ProgramDetails {
            program_title,
            program_type,
            program_college,
            program_keywords,
        }
    }

    fn extract_keywords<'a>(element: &'a ElementRef) -> Vec<&'a str> {
        let selector = Selector::parse(".item-container > .keyword").unwrap();
        let raw_keyword_list = element.select(&selector);

        let mut keywords = Vec::new();

        for element in raw_keyword_list {
            keywords.push(element.text().next().unwrap());
        }

        keywords
    }

    /// Accepts a program title as a String and returns a tuple of three elements:
    /// (program_title, program_type, program_location)
    fn extract_program_title_details<'a>(raw_title: &'a str) -> ProgramTitleDetails<'a> {
        // exceptions ("noise" in data)
        if let Some(cleaned_result) = Self::manage_noisy_titles(&raw_title) {
            return cleaned_result;
        }

        let comma_idx = match raw_title.rfind(", ") {
            Some(idx) => idx,
            // all titles have a comma, and those without are manged in `manage_noisy_titles`.
            None => panic!("Cannot find comma in title: {}", raw_title),
        };

        let (program_title, program_details) = raw_title.split_at(comma_idx);
        // Some((program_title, program_details)) => {
        let program_title = program_title.trim();
        let program_details = program_details.trim_start_matches(",").trim();

        match program_details.split_once("(") {
            // title + type + field
            Some((program_type, _)) => {
                let program_type = program_type.trim();
                // let program_field = program_field.trim_end_matches(')');

                return (program_title.into(), program_type.into());
            }
            // title + type
            None => (program_title.into(), program_details.into()),
        }
    }

    fn manage_noisy_titles(raw_title: &str) -> Option<ProgramTitleDetails> {
        match raw_title {
            "Science, B.S./Business Administration, M.B.A." => {
                Some(("Science, B.S./Business Administration, M.B.A.", "B.S."))
            }
            "Air Force ROTC" => Some(("Air Force ROTC", "ROTC")),
            "Army ROTC" => Some(("Army ROTC", "ROTC")),
            "Naval Science/Naval Reserve Officer Training Corps (NROTC)" => {
                Some(("Naval Science/Naval ROTC", "ROTC"))
            }
            "Bachelor of Philosophy Degree" => Some(("Philosophy", "B.Phil.")),
            "Global and International Studies Major" => {
                // B.S, B.A, B.F.A, etc. (matches students' first major)
                return Some(("Global and International Studies", "B."));
            }
            _ => None,
        }
    }

    fn extract_program_type(
        element: &ElementRef,
        raw_program_type: &str,
    ) -> UndergraduateProgramType {
        if let Some(up_campus_types) = match raw_program_type {
            "Minor" => Some(UndergraduateProgramType::Minor(raw_program_type.into())),
            "Certificate" => Some(UndergraduateProgramType::Certificate(
                raw_program_type.into(),
            )),
            "ROTC" => Some(UndergraduateProgramType::ROTC(raw_program_type.into())),
            _ => None,
        } {
            return up_campus_types;
        }

        // campus list element
        let raw_campus_list = element
            .select(&Selector::parse(".context-overlay > p.list").unwrap())
            .next(); // list is represented as a single string

        if let Some(element) = raw_campus_list {
            if let Some(child) = element.children().next_back() {
                let campus_list: Vec<&str> = child
                    .value()
                    .as_text()
                    .unwrap()
                    .trim()
                    .split(", ")
                    .collect();

                let campus_list = CampusList::from(&campus_list);

                if raw_program_type.starts_with("B.") {
                    return UndergraduateProgramType::BaccalaureateDegree(
                        raw_program_type.into(),
                        campus_list,
                    );
                } else if raw_program_type.starts_with("A.") {
                    return UndergraduateProgramType::AssociateDegree(
                        raw_program_type.into(),
                        campus_list,
                    );
                } else {
                    panic!("Unknown program type: {:?}", raw_program_type);
                }
            }
        }

        // shouldn't hit this because specific-problematic items should be cleaned by
        // `extract_program_title_details` via `manage_noisy_titles`
        panic!("Unknown program type: {:?}", raw_program_type);
    }
}

impl Scrappable<UndergraduateProgram> for ScrapeUndergraduatePrograms {
    fn extract_list(html: &str) -> Vec<UndergraduateProgram> {
        // println!("Parsing...");
        let document = scraper::Html::parse_document(html);

        // println!("Done parsing.");

        let mut programs: Vec<UndergraduateProgram> = Vec::new();

        for element in document.select(&Selector::parse("ul.isotope .item").unwrap()) {
            /* selections */

            let raw_link = element
                .select(&Selector::parse("a").unwrap())
                .next()
                .unwrap()
                .attr("href")
                .unwrap();

            let raw_image = element
                .select(&Selector::parse(".item-container > .image").unwrap())
                .next()
                .unwrap()
                .attr("style")
                .unwrap()
                .split_once("url(")
                .unwrap()
                .1
                .split_once(")")
                .unwrap()
                .0;

            /* parsing */

            let ProgramDetails {
                program_title,

                program_type, // includes campus list
                program_college,
                program_keywords,
            } = Self::extract_program_details(&element);

            // remove first char '/' from raw_link
            let program_link = format!("{}/{}", BASE_URL, &raw_link[1..]);
            // "-med" is a lower resolution of image
            let program_image = format!("{}/{}", BASE_URL, &raw_image[1..].replace("-med", ""));

            /* deserialization */
            let program = UndergraduateProgram {
                title: program_title.into(),
                link: program_link,
                program_type,
                image: program_image,
                college: program_college,
                keywords: program_keywords.into_iter().map(String::from).collect(),
                // field: program_field.map(String::from),
            };

            programs.push(program);
        }

        programs
    }
}

pub struct ScrapeUndergraduateCourseGroups;

pub struct CourseGroupIdentifier {
    pub code: String,
    pub title: String,
    pub raw_link: String, // TODO: URL struct?
}

impl Scrappable<CourseGroupIdentifier> for ScrapeUndergraduateCourseGroups {
    fn extract_list(html: &str) -> Vec<CourseGroupIdentifier> {
        let document = scraper::Html::parse_document(&html);
        let selector = Selector::parse(".az_sitemap li").unwrap();
        let courses = document.select(&selector).skip(27); // skip #, A-Z tags

        let mut course_codes = Vec::new();

        for course in courses {
            let course = course
                .select(&Selector::parse("a").unwrap())
                .next()
                .unwrap();

            let (title, code) = course
                .text()
                .next()
                .unwrap()
                .rsplit_once(" (")
                .expect("Couldn't find course code.");

            let code = code.trim_end_matches(')');
            let raw_link = course.attr("href").unwrap();

            course_codes.push(CourseGroupIdentifier {
                code: code.into(),
                title: title.into(),
                raw_link: raw_link.into(),
            });
        }

        course_codes
    }
}

pub struct ScrapeUndergraduateCourses;

#[allow(dead_code)]
struct CourseExtraDetails {
    attribute_list: FullAttributeList,
    crosslist: Option<Vec<CourseIdentifier>>,
    requirements: UndergraduateCourseRequirements,
    flags: UndergraduateCourseFlags,
}

struct CourseRequirementTreeDetails {
    requirements: UndergraduateCourseRequirements,
    flags: UndergraduateCourseFlags,
}

impl ScrapeUndergraduateCourses {
    fn extract_course_credits(element: &ElementRef) -> (Option<f32>, f32) {
        let raw_credits = element
            .select(&Selector::parse(".course_credits").unwrap())
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .trim();

        match raw_credits.rfind("of") {
            Some(_) => {
                // ranged credit
                let (raw_min, raw_max) = raw_credits.split_once(" Credits/Maximum of ").unwrap();

                let max = raw_max.parse().unwrap();

                let min = match raw_min.split_once("-") {
                    Some((min, _)) => min.parse().unwrap(),
                    None => raw_min.parse().unwrap(),
                };

                match min == max {
                    true => (None, max),
                    false => (Some(min), max),
                }
            }
            None => {
                let raw_credits = raw_credits.replace(" Credits", "");

                match raw_credits.split_once("-") {
                    // ranged credit
                    Some((min, max)) => (Some(min.parse().unwrap()), max.parse().unwrap()),
                    None => {
                        // fixed credit
                        (None, raw_credits.parse().unwrap())
                    }
                }
            }
        }
    }

    // TODO: work on not relying on chunks from HTML. Create an algorithm to accept a single string
    // IDEA: instead of going char by char consecutively. Go over string multiple times through "filters"
    //       ex. replace all ; with |, replace all , with &. Parse all "possible" courses, collect deviants.
    fn create_text_chunks<'a>(element: &'a ElementRef) -> Vec<&'a str> {
        let mut buffer = Vec::<&str>::new();
        let mut cursor = Some(element.first_child().unwrap());
        while let Some(sibling) = cursor {
            buffer.push(match ElementRef::wrap(sibling) {
                Some(element) => element.text().next().unwrap(),
                None => sibling.value().as_text().unwrap(),
            });

            cursor = sibling.next_sibling();
        }
        buffer
    }

    /// Takes a vector of text chunks from HTML
    /// TODO: work on using single string only.
    fn create_requirement_trees(
        text_chunks: Vec<&str>,
        identifier: &CourseIdentifier,
    ) -> CourseRequirementTreeDetails {
        let mut prerequisites: Option<CourseRequirementTree> = None;
        let mut concurrent: Option<CourseRequirementTree> = None;
        let mut corequisites: Option<CourseRequirementTree> = None;
        let mut recommended: Option<CourseRequirementTree> = None;

        let mut flags = UndergraduateCourseFlags {
            // false by default
            is_prerequisite_concurrent_separate: false,
            deviant: UndergraduateCourseDeviantFlags {
                empty_crosslist: false,
                unknown_requirement: false,
            },
        };

        enum CourseRequirementMode {
            Prerequisite,
            Concurrent,
            Corequisite,
            Recommended,
        }

        let mut selected_mode: Option<CourseRequirementMode> = None;
        let mut requirement_text = String::new();

        // TODO: this is so horrible fix
        let mut switch_mode = |mode: Option<CourseRequirementMode>, text: String| -> String {
            if !text.is_empty() {
                // make sure there any no dangling ORs or ANDS in text
                let text = {
                    let mut text = text.as_str();
                    text = text.trim_start_matches("&");
                    text = text.trim_end_matches("&");
                    text = text.trim_start_matches("|");
                    text = text.trim_end_matches("|");
                    text
                };

                // flush current text
                if let Some(selected_mode) = &selected_mode {
                    // println!("BEFORE: {}", text);

                    let tree = CourseRequirementTree::try_from(text).ok();
                    let active_mode = match selected_mode {
                        CourseRequirementMode::Prerequisite => &mut prerequisites,
                        CourseRequirementMode::Concurrent => &mut concurrent,
                        CourseRequirementMode::Corequisite => &mut corequisites,
                        CourseRequirementMode::Recommended => &mut recommended,
                    };

                    // only assign if value isn't set before
                    // in a sense: write-once
                    if active_mode.is_none() {
                        *active_mode = tree;
                    }
                }
            }

            // print result
            // println!(
            //     "{}{}",
            //     "Mode\t->\t".yellow(),
            //     match mode {
            //         Some(ref mode) => match mode {
            //             CourseRequirementMode::Prerequisite => "Prerequisite",
            //             CourseRequirementMode::Concurrent => "Concurrent",
            //             CourseRequirementMode::Corequisite => "Corequisite",
            //             CourseRequirementMode::Recommended => "Recommended",
            //         },
            //         None => "None",
            //     }
            //     .yellow()
            // );

            selected_mode = mode;
            String::new()
        };

        for text_chunk in text_chunks {
            let text_chunk = text_chunk
                .to_ascii_uppercase()
                .replace_many(&[
                    // modify list
                    // even-though courses are not supposed to have these
                    // https://cim.psu.edu/user-guides/course-management/prerequisites-concurrents-corequisites/
                    // some course descriptions use ; for OR and , for AND (probably courses that haven't been updated in a while)
                    (";", " OR"),
                    (",", " AND"),
                    ("CONCURRENT COURSES", "CONCURRENT"),
                    ("RECOMMENDED PREPARATIONS", "RECOMMENDED PREPARATION"), // consistency purposes
                    // spelling mistakes
                    ("PRERQUISITE", "PREREQUISITE"),
                    ("PREREQUISTE", "PREREQUISITE"),
                    ("PREQUISITE", "PREREQUISITE"),
                    ("PREREQ ", "PREREQUISITE "),
                ])
                .trim_all();

            let mut requirement_text_buffer = String::new();

            // TODO: find a better way of doing this
            for (i, c) in text_chunk.chars().enumerate() {
                match c {
                    '(' | ')' | '[' | ']' => {
                        // println!("{}{}", "LOGIC\t->\t".blue(), c.to_string().blue());
                        requirement_text.push(c)
                    }
                    '.' | ':' => (), // ignore
                    _ => requirement_text_buffer.push(c),
                }

                match requirement_text_buffer.trim_all().as_str() {
                    "AND" => {
                        // println!("{}", "LOGIC\t->\tAND".blue());
                        requirement_text.push('&');
                        requirement_text_buffer.clear();
                    }
                    "OR" => {
                        // println!("{}", "LOGIC\t->\tOR".blue());
                        requirement_text.push('|');
                        requirement_text_buffer.clear();
                    }
                    // TODO: "AT AT LEAST ONE OF THE FOLLOWING" JUST MEANS TO MAKE A NEW '(' WITH "OR"
                    "C OR BETTER IN"
                    | "PRIOR TO"
                    | "ENFORCED"
                    | "AT ENROLLMENT"
                    | "PREPARATION" 
                    // Specific edge cases
                    | "PRIOR EXPOSURE TO R PROGRAMMING LANGUAGE" // GEOG 462 
                    => {
                        // if this is in buffer ignore it (for now...)
                        requirement_text_buffer.clear();
                    }
                    "PREREQUISITE" => {
                        requirement_text = switch_mode(
                            Some(CourseRequirementMode::Prerequisite),
                            requirement_text,
                        );
                        requirement_text_buffer.clear();
                    }
                    "CONCURRENT" => {
                        // check if previous char is `|`, if so,
                        // prerequisite and concurrent are separate
                        match requirement_text.chars().last() {
                            Some('|') => flags.is_prerequisite_concurrent_separate = true,
                            _ => (),
                        }
                        requirement_text =
                            switch_mode(Some(CourseRequirementMode::Concurrent), requirement_text);
                        requirement_text_buffer.clear();
                    }
                    "COREQUISITE" => {
                        requirement_text =
                            switch_mode(Some(CourseRequirementMode::Corequisite), requirement_text);
                        requirement_text_buffer.clear();
                    }
                    "RECOMMENDED" => {
                        requirement_text =
                            switch_mode(Some(CourseRequirementMode::Recommended), requirement_text);
                        requirement_text_buffer.clear();
                    }
                    // TODO: this is actually so bad. like for real work on making this better
                    // ... but it works ... so ¯\_(ಠ_ಠ)_/¯
                    // this messy if statements checks if current requirement_text_buffer is a course
                    // however, to prevent false positives for example:
                    // "MATH 140H" -> if the buffer isn't done filling up it'll mistake "MATH 1",
                    //                  "MATH 14" or "MATH 140" as a course (depending on how filled the buffer is),
                    //                  which is not true.
                    // to fix this, we check if the current buffer is a course AND if the buffer plus
                    // the next character isn't a course. If this is true, then it must mean we have
                    // exhausted what the buffer is supposed to hold in terms of a course
                    text if CourseIdentifier::try_from(text).is_ok()
                        && (i == &text_chunk.chars().count() - 1
                            || CourseIdentifier::try_from(
                                format!("{}{}", text, text_chunk.chars().nth(i + 1).unwrap())
                                    .as_str(),
                            )
                            .is_err()) =>
                    {
                        // TODO: remove all '\u{a0}'
                        let course_text = requirement_text_buffer.trim_all();
                        if
                        // don't add a course if its the same as the current one
                        course_text.replace('\u{a0}', " ") != identifier.to_string().as_str() && 
                            // if course is already in requirement_text
                            !requirement_text.contains(format!("{{{course_text}}}").as_str()) &&
                            // if it contains "ANY"
                            !course_text.contains("ANY")
                        {
                            // println!("ADDING\t->\t{}", course_text);
                            // wrap all non-logical text is `{}` to preserve spacing
                            requirement_text.push_str(format!("{{{course_text}}}").as_str());
                            requirement_text_buffer.clear();
                        }
                    }
                    _ => (),
                }
            }

            let requirement_text_buffer: &str = &requirement_text_buffer.trim_all();
            if !requirement_text_buffer.is_empty() {
                match requirement_text_buffer {
                    "S" => (), // ignore S, usually intended
                    _ => {
                        // println!(
                        //     "{}{}",
                        //     "DEVIANT\t->\t".red(),
                        //     requirement_text_buffer.red().bold()
                        // );
                        // flag course has deviants so further work can be done
                        flags.deviant.unknown_requirement = true;
                    }
                };
            }
        }

        // flush `requirement_text`
        switch_mode(None, requirement_text);

        CourseRequirementTreeDetails {
            requirements: UndergraduateCourseRequirements {
                prerequisites,
                concurrent,
                corequisites,
                recommended,
            },
            flags,
        }
    }

    fn parse_extra_details(
        element: &ElementRef,
        identifier: &CourseIdentifier,
    ) -> CourseExtraDetails {
        let selector: Selector = Selector::parse(".courseblockextra .noindent").unwrap();
        let raw_extra_detail_elements = element.select(&selector);
        let mut attribute_list: FullAttributeList = AttributeList::new();
        let mut crosslist: Option<Vec<CourseIdentifier>> = None;

        let mut requirements = UndergraduateCourseRequirements {
            prerequisites: None,
            concurrent: None,
            corequisites: None,
            recommended: None,
        };

        let mut flags = UndergraduateCourseFlags {
            is_prerequisite_concurrent_separate: false,
            deviant: UndergraduateCourseDeviantFlags {
                empty_crosslist: false,
                unknown_requirement: false,
            },
        };

        // TODO: add noise list (AFAM 410, AFAM 460, AFR 444, PHIL 453)

        // println!("------------");
        // println!("{}", identifier.to_string().magenta());

        if raw_extra_detail_elements.clone().count() == 0 {
            // println!("No attributes found.");
            // return; // attribute_list;
        } else {
            for raw_extra_detail_element in raw_extra_detail_elements {
                // println!("RESETTING...");

                // prerequisite or concurrent labels are mostly wrapped in <strong>
                if raw_extra_detail_element
                    .select(&Selector::parse("strong").unwrap())
                    .next()
                    .is_some()
                {
                    let text_chunks = Self::create_text_chunks(&raw_extra_detail_element);
                    let extra_details = Self::create_requirement_trees(text_chunks, identifier);
                    requirements = extra_details.requirements;
                    flags = extra_details.flags;
                } else {
                    // anything here is mostly text-based

                    let raw_detail: &str =
                        &raw_extra_detail_element.text().next().unwrap().trim_all();

                    if Self::is_noisy_extra_detail(raw_detail) {
                        continue;
                    };

                    // TODO: create function to check for cross-lists
                    if raw_detail.starts_with("Cross-listed with:") || raw_detail == "Cross-Listed"
                    {
                        let mut crosslist_buffer = Vec::new();
                        if raw_extra_detail_element.children().count() > 1 {
                            let mut cursor = raw_extra_detail_element.first_child();
                            while let Some(sibling) = cursor {
                                let text = match ElementRef::wrap(sibling) {
                                    Some(element) => element.text().next().unwrap(),
                                    None => sibling.value().as_text().unwrap(),
                                }
                                .to_ascii_uppercase()
                                .trim_all();

                                match CourseIdentifier::try_from(text.as_str()).ok() {
                                    Some(course) => crosslist_buffer.push(course),
                                    None => (), // ignore everything else
                                }

                                cursor = sibling.next_sibling();
                            }
                        } else {
                            // rarely, crosslist's will just be one single string separated by commas
                            for text in raw_detail
                                .replace("Cross-listed with:", "")
                                .trim()
                                .split(",")
                            {
                                let text = text.to_ascii_uppercase().trim_all();

                                match CourseIdentifier::try_from(text.as_str()).ok() {
                                    Some(course) => crosslist_buffer.push(course),
                                    None => (), // ignore everything else
                                }
                            }
                        }
                        // if by the end of the loop nothing was pushed to crosslist
                        // flag a deviant course
                        match crosslist_buffer.is_empty() {
                            true => flags.deviant.empty_crosslist = true,
                            false => crosslist = Some(crosslist_buffer),
                        }

                        continue;
                    }

                    // usually just attributes, but rarely course requirements
                    match raw_detail {
                        "Prerequisite"
                        | "Prerequisites"
                        | "Enforced Prerequisite at Enrollment"
                        | "Enforced Corequisite at Enrollment"
                        | "Recommended Preparation" => {
                            let text_chunks = Self::create_text_chunks(&raw_extra_detail_element);
                            let extra_details =
                                Self::create_requirement_trees(text_chunks, identifier);
                            requirements = extra_details.requirements;
                            flags = extra_details.flags;
                        }
                        _ => {
                            // some attributes are incorrectly spelt issues
                            let attribute = Self::fix_attribute_typos(raw_detail);

                            // add actual attributes
                            attribute_list.add(attribute);
                            //     println!(
                            //         "[{}] -> [{}]",
                            //         attribute.green(),
                            //         raw_attribute.children().count()
                            //     );
                        }
                    }
                }
            }
        }
        // // print tree
        // match requirements.prerequisites {
        //     Some(ref tree) => {
        //         println!("Prerequisites: {}", tree.to_string().green())
        //     }
        //     None => (),
        // }
        // match requirements.concurrent {
        //     Some(ref tree) => println!("Concurrent: {}", tree.to_string().green()),
        //     None => (),
        // }
        // match requirements.corequisites {
        //     Some(ref tree) => {
        //         println!("Corequisites: {}", tree.to_string().green())
        //     }
        //     None => (),
        // }
        // match requirements.recommended {
        //     Some(ref tree) => println!("Recommended: {}", tree.to_string().green()),
        //     None => (),
        // }

        // match crosslist {
        //     Some(ref list) => {
        //         println!(
        //             "Crosslist: {}",
        //             list.iter()
        //                 .map(|s| s.to_string())
        //                 .collect::<Vec<String>>()
        //                 .join(", ")
        //         )
        //     }
        //     None => (),
        // }

        // if !attribute_list.is_empty() {
        //     println!("Attributes: {}", attribute_list.to_string().green());
        // }
        // if flags.is_prerequisite_concurrent_separate {
        //     println!("Prerequisite & concurrent separate");
        // }
        // if flags.deviant.unknown_requirement {
        //     println!("Contains unknown requirement");
        // }
        // if flags.deviant.empty_crosslist {
        //     println!("Contains empty crosslist");
        // }

        CourseExtraDetails {
            attribute_list,
            crosslist,
            requirements,
            flags,
        }
    }

    fn fix_attribute_typos(attribute: &str) -> &str {
        match attribute {
            "General Education: Social and Behavioral Scien (GS)" => {
                "General Education: Social and Behavioral Sciences (GS)"
            }
            _ => attribute,
        }
    }

    /// All "noisy" attributes are skipped (for now..)
    fn is_noisy_extra_detail(attribute: &str) -> bool {
        // TODO: look into use later. not really useful right now
        if attribute.starts_with("GenEd Learning Objective:") {
            return true;
        }

        match attribute {
            // TODO: ACCTG 495 - "Full-Time Equivalent Course"
            "Full-Time Equivalent Course" => true,
            "Faculty approval of work experience proposal including employment agreement with an approved supervisor (e.g., registered architect or other approved professional)." => true,
            "." =>  true,
            _ => false,
        }
    }

    /// All "noisy" course titles are skipped for now
    fn is_noisy_course_title(title: &str) -> bool {
        match title {
            "EDAB TEMPH: Temporary Education Abroad Registration" => true,
            "EDAB TEMPI: Temporary Education Abroad Registration" => true,
            _ => false,
        }
    }
}

impl Scrappable<UndergraduateCourse> for ScrapeUndergraduateCourses {
    /// Scrapes a list of undergraduate courses from a program catalog page
    fn extract_list(html: &str) -> Vec<UndergraduateCourse> {
        let document = scraper::Html::parse_document(&html);
        let selector = Selector::parse(".sc_sccoursedescs .courseblock").unwrap();
        let course_elements = document.select(&selector);
        let mut courses = Vec::<UndergraduateCourse>::new();

        for course_element in course_elements {
            let raw_title = course_element
                .select(&Selector::parse(".course_codetitle").unwrap())
                .next()
                .unwrap()
                .text()
                .next()
                .unwrap();

            let description = match course_element
                .select(&Selector::parse(".courseblockdesc > p").unwrap())
                .next()
            {
                Some(desc) => Some(desc.text().next().unwrap().to_owned()),
                None => None,
            };

            // skip noisy courses
            if Self::is_noisy_course_title(raw_title) {
                continue;
            }

            let (raw_identifier, title) = raw_title.split_once(": ").unwrap();

            let title = title.trim_all();
            let identifier = CourseIdentifier::try_from(raw_identifier).unwrap();

            // if identifier.to_string() != "AMST 493" { continue; }

            let (min_credits, credits) = Self::extract_course_credits(&course_element);
            let CourseExtraDetails {
                attribute_list,
                crosslist,
                requirements,
                flags,
            } = Self::parse_extra_details(&course_element, &identifier);

            courses.push(UndergraduateCourse {
                identifier,
                title,
                description,
                credits,
                min_credits,
                attribute_list,
                crosslist,
                requirements,
                flags,
            });
        }

        courses
    }
}
