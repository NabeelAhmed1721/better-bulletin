use std::fmt;

use ego_tree::{NodeId, NodeRef, Tree};

pub const BASE_URL: &str = "https://bulletins.psu.edu";

#[derive(Debug)]
pub struct UndergraduateProgram {
    pub title: String,
    pub link: String,
    // pub description: String,
    pub program_type: UndergraduateProgramType,
    pub image: String,
    pub keywords: Vec<String>,
    pub college: Option<College>, // some programs rely on others, hence, don't actually have a college
                                  // pub field: Option<String>,
}

type CampusListFull = CampusList<21>; // 21 campuses
type RawProgramType = String; // TODO: better name?

#[allow(dead_code)]
#[derive(Debug)]
pub enum UndergraduateProgramType {
    // degrees available at 21 campuses
    BaccalaureateDegree(RawProgramType, CampusListFull),
    AssociateDegree(RawProgramType, CampusListFull),
    // can be completed anywhere courses are offered
    Minor(RawProgramType),
    Certificate(RawProgramType),
    // can be completed only at UniversityPark
    ROTC(RawProgramType),
}

impl fmt::Display for UndergraduateProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_buffer = String::new();

        string_buffer += &format!("{}", self.title);
        if let Some(ref college) = self.college {
            string_buffer += &format!(" [{}]", college);
        }
        string_buffer += &format!("\n -> {}", self.link);
        string_buffer += &format!("\n -> {}", self.image);

        // TODO: clean up
        // if let Some(ref program_location) = self.field {
        //     string_buffer += &format!(" [{}]", program_location);
        // }

        match self.program_type {
            UndergraduateProgramType::BaccalaureateDegree(ref degree_type, ref campus) => {
                string_buffer += &format!("\n\t -> [{}] @ {}", degree_type, campus);
            }
            UndergraduateProgramType::AssociateDegree(ref degree_type, ref campus) => {
                string_buffer += &format!("\n\t -> [{}] @ {}", degree_type, campus)
            }
            UndergraduateProgramType::Minor(_) => {
                string_buffer += &format!("\n\t -> Minor");
            }
            UndergraduateProgramType::Certificate(_) => {
                string_buffer += &format!("\n\t -> Certificate");
            }
            UndergraduateProgramType::ROTC(_) => {
                string_buffer += &format!("\n\t -> ROTC");
            }
        }
        write!(f, "{}", string_buffer)
    }
}

#[derive(Debug)]
pub enum College {
    AgriculturalSciences,
    ArtsAndArchitecture,
    Communications, // Donald P. Bellisario College of Communications
    EarthAndMineralSciences,
    Science, // Eberly College of Science
    Education,
    Engineering,
    HealthAndHumanDevelopment,
    InformationSciencesAndTechnology,
    Intercollege,
    LiberalArts,
    Nursing,
    Abington, // The Abington College
    Altoona,  // The Altoona College
    Berks,    // The Berks College
    Behrend,  // The Behrend College
    Capital,  // The Capital College
    Business, // Smeal College of Business
    UniversityCollege,
}

impl From<&str> for College {
    fn from(value: &str) -> Self {
        match value {
            "Agricultural Sciences" => College::AgriculturalSciences,
            "Arts and Architecture" => College::ArtsAndArchitecture,
            "Donald P. Bellisario College of Communications" => College::Communications,
            "Earth and Mineral Sciences" => College::EarthAndMineralSciences,
            "Eberly College of Science" => College::Science,
            "Education" => College::Education,
            "Engineering" => College::Engineering,
            "Health and Human Development" => College::HealthAndHumanDevelopment,
            "Information Sciences and Technology" => College::InformationSciencesAndTechnology,
            "Intercollege" => College::Intercollege,
            "Liberal Arts" => College::LiberalArts,
            "Nursing" => College::Nursing,
            "Penn State Abington, The Abington College" => College::Abington,
            "Penn State Altoona, The Altoona College" => College::Altoona,
            "Penn State Berks, The Berks College" => College::Berks,
            "Penn State Erie, The Behrend College" => College::Behrend,
            "Penn State Harrisburg, The Capital College" => College::Capital,
            "Smeal College of Business" => College::Business,
            "University College" => College::UniversityCollege,
            _ => panic!("Invalid College: {}. (Check noisy data)", value),
        }
    }
}

impl From<&College> for &str {
    fn from(value: &College) -> Self {
        match value {
            College::AgriculturalSciences => "Agricultural Sciences",
            College::ArtsAndArchitecture => "Arts and Architecture",
            College::Communications => "Donald P. Bellisario College of Communications",
            College::EarthAndMineralSciences => "Earth and Mineral Sciences",
            College::Science => "Eberly College of Science",
            College::Education => "Education",
            College::Engineering => "Engineering",
            College::HealthAndHumanDevelopment => "Health and Human Development",
            College::InformationSciencesAndTechnology => "Information Sciences and Technology",
            College::Intercollege => "Intercollege",
            College::LiberalArts => "Liberal Arts",
            College::Nursing => "Nursing",
            College::Abington => "Penn State Abington, The Abington College",
            College::Altoona => "Penn State Altoona, The Altoona College",
            College::Berks => "Penn State Berks, The Berks College",
            College::Behrend => "Penn State Erie, The Behrend College",
            College::Capital => "Penn State Harrisburg, The Capital College",
            College::Business => "Smeal College of Business",
            College::UniversityCollege => "University College",
        }
    }
}

impl fmt::Display for College {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <&Self as Into<&str>>::into(self))
    }
}

#[derive(Debug)]
pub enum Campus {
    Abington,
    Altoona,
    Beaver,
    Berks,
    Brandywine,
    DuBois,
    Erie, // aka Behrend
    Fayette,
    GreaterAllegheny,
    Harrisburg, // aka Capital
    Hazleton,
    LehighValley,
    MontAlto,
    NewKensington,
    Schuylkill,
    Scranton,
    Shenango,
    UniversityPark, // aka Main Campus
    WilkesBarre,
    WorldCampus,
    York,
}

impl From<&str> for Campus {
    fn from(value: &str) -> Self {
        match value {
            "Abington" => Campus::Abington,
            "Altoona" => Campus::Altoona,
            "Beaver" => Campus::Beaver,
            "Berks" => Campus::Berks,
            "Brandywine" => Campus::Brandywine,
            "DuBois" => Campus::DuBois,
            "Erie" => Campus::Erie,
            "Fayette" => Campus::Fayette,
            "Greater Allegheny" => Campus::GreaterAllegheny,
            "Harrisburg" => Campus::Harrisburg,
            "Hazleton" => Campus::Hazleton,
            "Lehigh Valley" => Campus::LehighValley,
            "Mont Alto" => Campus::MontAlto,
            "New Kensington" => Campus::NewKensington,
            "Schuylkill" => Campus::Schuylkill,
            "Scranton" => Campus::Scranton,
            "Shenango" => Campus::Shenango,
            "University Park" => Campus::UniversityPark,
            "Wilkes-Barre" => Campus::WilkesBarre,
            "World Campus" => Campus::WorldCampus,
            "York" => Campus::York,
            _ => panic!("Invalid Campus: {}. (Check noisy data)", value),
        }
    }
}

impl From<usize> for Campus {
    fn from(value: usize) -> Self {
        match value {
            0 => Campus::Abington,
            1 => Campus::Altoona,
            2 => Campus::Beaver,
            3 => Campus::Berks,
            4 => Campus::Brandywine,
            5 => Campus::DuBois,
            6 => Campus::Erie,
            7 => Campus::Fayette,
            8 => Campus::GreaterAllegheny,
            9 => Campus::Harrisburg,
            10 => Campus::Hazleton,
            11 => Campus::LehighValley,
            12 => Campus::MontAlto,
            13 => Campus::NewKensington,
            14 => Campus::Schuylkill,
            15 => Campus::Scranton,
            16 => Campus::Shenango,
            17 => Campus::UniversityPark,
            18 => Campus::WilkesBarre,
            19 => Campus::WorldCampus,
            20 => Campus::York,
            _ => panic!("Invalid Campus: {}. (Check noisy data)", value),
        }
    }
}

impl From<Campus> for &str {
    fn from(value: Campus) -> Self {
        match value {
            Campus::Abington => "Abington",
            Campus::Altoona => "Altoona",
            Campus::Beaver => "Beaver",
            Campus::Berks => "Berks",
            Campus::Brandywine => "Brandywine",
            Campus::DuBois => "DuBois",
            Campus::Erie => "Erie",
            Campus::Fayette => "Fayette",
            Campus::GreaterAllegheny => "Greater Allegheny",
            Campus::Harrisburg => "Harrisburg",
            Campus::Hazleton => "Hazleton",
            Campus::LehighValley => "Lehigh Valley",
            Campus::MontAlto => "Mont Alto",
            Campus::NewKensington => "New Kensington",
            Campus::Schuylkill => "Schuylkill",
            Campus::Scranton => "Scranton",
            Campus::Shenango => "Shenango",
            Campus::UniversityPark => "University Park",
            Campus::WilkesBarre => "Wilkes-Barre",
            Campus::WorldCampus => "World Campus",
            Campus::York => "York",
        }
    }
}

// TODO: implement this, add `from` with From trait
// pub trait EnumList {
//     fn new() -> Self;
//     fn from(list: &[&str]) -> Self;
//     fn contains(&self, item: &str) -> bool;
//     fn add(&mut self, item: &str);
//     fn remove(&mut self, item: &str);
//     fn disassemble<'a>(&self) -> Vec<&'a str>;
// }

#[derive(Debug)]
pub struct CampusList<const T: usize> {
    list: [bool; T],
}

#[allow(dead_code)]
impl<const T: usize> CampusList<T> {
    pub fn new() -> Self {
        Self { list: [false; T] }
    }

    pub fn from(list: &[&str]) -> Self {
        let mut campus_list = Self { list: [false; T] };

        for campus in list {
            // TODO: USE CAMPUS::TRY_FROM INSTEAD
            // ignore list
            match *campus {
                "Hershey Med Ctr" | "Nurses at Hershey" => continue,
                _ => (),
            }

            campus_list.list[Campus::from(*campus) as usize] = true;
        }

        campus_list
    }

    pub fn contains(&self, item: &str) -> bool {
        self.list[Campus::from(item) as usize]
    }

    pub fn add(&mut self, item: &str) {
        self.list[Campus::from(item) as usize] = true;
    }

    pub fn remove(&mut self, item: &str) {
        self.list[Campus::from(item) as usize] = false;
    }

    pub fn disassemble<'a>(&self) -> Vec<&'a str> {
        let mut result: Vec<&str> = Vec::new();

        for (idx, campus) in self.list.iter().enumerate() {
            if *campus {
                result.push(Campus::from(idx).into());
            }
        }

        result
    }
}

impl<const T: usize> fmt::Display for CampusList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.disassemble().join(", "))
    }
}

#[derive(Debug, Clone)]
pub struct CourseIdentifier {
    pub code: String,         // MATH
    pub number: u16,          // 140
    pub suffix: Option<char>, // H (more info: https://advising.psu.edu/abbreviations-acronyms-and-codes)
}

impl TryFrom<&str> for CourseIdentifier {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (code, number_suffix) = match value.split_once(|s| match s {
            // remove any non-breaking-spaces
            ' ' | '\u{A0}' => true,
            _ => false,
        }) {
            Some((code, number_suffix)) => (code, number_suffix),
            None => return Err("Couldn't find a space in the course identifier."),
        };

        let last_number_suffix_char = match number_suffix.chars().last() {
            Some(last_number_suffix_char) => last_number_suffix_char,
            None => return Err("Number suffix is empty."),
        };

        let (number, suffix): (&str, Option<char>) = if last_number_suffix_char.is_alphabetic() {
            let (number, suffix) = number_suffix.split_at(number_suffix.len() - 1);

            let suffix = match suffix.chars().next() {
                Some(suffix) => Some(suffix),
                None => return Err("Suffix is empty."),
            };

            (number, suffix)
        } else {
            (number_suffix, None)
        };

        let number = match number.parse::<u16>() {
            Ok(number) => number,
            Err(_) => return Err("Couldn't parse number."),
        };

        Ok(Self {
            code: code.into(),
            number: number,
            suffix,
        })
    }
}

impl fmt::Display for CourseIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(suffix) = self.suffix {
            write!(f, "{} {}{}", self.code, self.number, suffix)
        } else {
            write!(f, "{} {}", self.code, self.number)
        }
    }
}

pub type FullAttributeList = AttributeList<22>;
// TODO: abstract into struct

pub struct UndergraduateCourseRequirements {
    pub prerequisites: Option<CourseRequirementTree>,
    pub concurrent: Option<CourseRequirementTree>,
    pub corequisites: Option<CourseRequirementTree>,
    pub recommended: Option<CourseRequirementTree>,
}

pub struct UndergraduateCourseDeviantFlags {
    pub empty_crosslist: bool,
    pub unknown_requirement: bool, // TODO: just extra string that cannot be parsed at the moment. Ex. "FIFTH SEMESTER STANDING"
}

pub struct UndergraduateCourseFlags {
    // mostly false.
    // most courses have prerequisite requirements AND concurrent requirements
    // few courses have prerequisite requirements OR concurrent requirements (not both)
    pub is_prerequisite_concurrent_separate: bool,
    pub deviant: UndergraduateCourseDeviantFlags,
}

pub struct UndergraduateCourse {
    pub identifier: CourseIdentifier,
    pub title: String, // Calculus I
    pub description: Option<String>,
    pub credits: f32,
    pub min_credits: Option<f32>, // some courses have a credit range

    pub attribute_list: FullAttributeList,
    // TODO: crosslist or crosslists? db table is called crosslists
    pub crosslist: Option<Vec<CourseIdentifier>>,

    pub requirements: UndergraduateCourseRequirements,
    pub flags: UndergraduateCourseFlags,
}

enum Attribute {
    // General Requirements
    GA,  // Arts
    GHW, // Health and Wellness
    GH,  // Humanities
    GN,  // Natural Sciences
    GQ,  // Quantification
    GS,  // Social and Behavioral Sciences
    GWS, // Writing and Speaking

    ITD, // Inter-Domain
    LKD, // Linked

    FYS, // First-Year Seminar
    IC,  // International Cultures
    US,  // United States Cultures
    WCC, // Writing Across the Curriculum

    // B.A. Requirements
    BA,  // Bachelor of Arts: Arts
    BH,  // Bachelor of Arts: Humanities
    BN,  // Bachelor of Arts: Natural Sciences
    BO,  // Bachelor of Arts: Other Cultures
    BQ,  // Bachelor of Arts: Quantification
    BS,  // Bachelor of Arts: Social and Behavioral Sciences
    BF1, // Bachelor of Arts: Foreign/World Lang (12th Unit)
    BF2, // Bachelor of Arts: 2nd Foreign/World Language (All)

    // Extra
    HNR, // Honors
}

impl From<&str> for Attribute {
    fn from(value: &str) -> Self {
        match value {
            "General Education: Arts (GA)" => Attribute::GA,
            "General Education: Health and Wellness (GHW)" => Attribute::GHW,
            "General Education: Humanities (GH)" => Attribute::GH,
            "General Education: Natural Sciences (GN)" => Attribute::GN,
            "General Education: Quantification (GQ)" => Attribute::GQ,
            "General Education: Social and Behavioral Sciences (GS)" => Attribute::GS,
            "General Education: Writing/Speaking (GWS)" => Attribute::GWS,

            "General Education - Integrative: Interdomain" => Attribute::ITD,
            "General Education - Integrative: Linked" => Attribute::LKD,

            "First-Year Seminar" => Attribute::FYS,
            "International Cultures (IL)" => Attribute::IC,
            "United States Cultures (US)" => Attribute::US,
            "Writing Across the Curriculum" => Attribute::WCC,

            "Bachelor of Arts: Arts" => Attribute::BA,
            "Bachelor of Arts: Humanities" => Attribute::BH,
            "Bachelor of Arts: Natural Sciences" => Attribute::BN,
            "Bachelor of Arts: Other Cultures" => Attribute::BO,
            "Bachelor of Arts: Quantification" => Attribute::BQ,
            "Bachelor of Arts: Social and Behavioral Sciences" => Attribute::BS,
            "Bachelor of Arts: Foreign/World Lang (12th Unit)" => Attribute::BF1,
            "Bachelor of Arts: 2nd Foreign/World Language (All)" => Attribute::BF2,

            "Honors" => Attribute::HNR,

            _ => panic!("Invalid Attribute: {}. (Check noisy data)", value),
        }
    }
}

impl From<usize> for Attribute {
    fn from(value: usize) -> Self {
        match value {
            0 => Attribute::GA,
            1 => Attribute::GHW,
            2 => Attribute::GH,
            3 => Attribute::GN,
            4 => Attribute::GQ,
            5 => Attribute::GS,
            6 => Attribute::GWS,

            7 => Attribute::ITD,
            8 => Attribute::LKD,

            9 => Attribute::FYS,
            10 => Attribute::IC,
            11 => Attribute::US,
            12 => Attribute::WCC,

            13 => Attribute::BA,
            14 => Attribute::BH,
            15 => Attribute::BN,
            16 => Attribute::BO,
            17 => Attribute::BQ,
            18 => Attribute::BS,
            19 => Attribute::BF1,
            20 => Attribute::BF2,

            21 => Attribute::HNR,

            _ => panic!("Invalid Attribute: {}. (Check noisy data)", value),
        }
    }
}

impl From<Attribute> for &str {
    fn from(value: Attribute) -> Self {
        match value {
            Attribute::GA => "General Education: Arts (GA)",
            Attribute::GHW => "General Education: Health and Wellness (GHW)",
            Attribute::GH => "General Education: Humanities (GH)",
            Attribute::GN => "General Education: Natural Sciences (GN)",
            Attribute::GQ => "General Education: Quantification (GQ)",
            Attribute::GS => "General Education: Social and Behavioral Sciences (GS)",
            Attribute::GWS => "General Education: Writing/Speaking (GWS)",

            Attribute::ITD => "General Education - Integrative: Interdomain",
            Attribute::LKD => "General Education - Integrative: Linked",

            Attribute::FYS => "First-Year Seminar",
            Attribute::IC => "International Cultures (IL)",
            Attribute::US => "United States Cultures (US)",
            Attribute::WCC => "Writing Across the Curriculum",

            Attribute::BA => "Bachelor of Arts: Arts",
            Attribute::BH => "Bachelor of Arts: Humanities",
            Attribute::BN => "Bachelor of Arts: Natural Sciences",
            Attribute::BO => "Bachelor of Arts: Other Cultures",
            Attribute::BQ => "Bachelor of Arts: Quantification",
            Attribute::BS => "Bachelor of Arts: Social and Behavioral Sciences",
            Attribute::BF1 => "Bachelor of Arts: Foreign/World Lang (12th Unit)",
            Attribute::BF2 => "Bachelor of Arts: 2nd Foreign/World Language (All)",

            Attribute::HNR => "Honors",
        }
    }
}

#[derive(Debug)]
pub struct AttributeList<const T: usize> {
    list: [bool; T],
}

#[allow(dead_code)]
impl<const T: usize> AttributeList<T> {
    pub fn new() -> Self {
        Self { list: [false; T] }
    }

    pub fn from(list: &[&str]) -> Self {
        let mut attr_list = Self { list: [false; T] };

        for attr in list {
            attr_list.list[Attribute::from(*attr) as usize] = true;
        }

        attr_list
    }

    pub fn get(&self, idx: usize) -> bool {
        self.list[idx]
    }

    pub fn contains(&self, item: &str) -> bool {
        self.list[Attribute::from(item) as usize]
    }

    pub fn add(&mut self, item: &str) {
        self.list[Attribute::from(item) as usize] = true;
    }

    pub fn remove(&mut self, item: &str) {
        self.list[Attribute::from(item) as usize] = false;
    }

    pub fn clear(&mut self) {
        self.list = [false; T];
    }

    pub fn is_empty(&self) -> bool {
        self.list.iter().all(|&x| !x)
    }

    pub fn disassemble<'a>(&self) -> Vec<&'a str> {
        let mut result: Vec<&str> = Vec::new();

        for (idx, attr) in self.list.iter().enumerate() {
            if *attr {
                result.push(Attribute::from(idx).into());
            }
        }

        result
    }
}

impl<const T: usize> fmt::Display for AttributeList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.disassemble().join(", "))
    }
}

#[derive(Debug, Clone)]
pub enum CourseRequirementNode {
    AND,
    OR,
    COURSE(CourseIdentifier),
}

pub struct CourseRequirementTree {
    pub tree: Tree<CourseRequirementNode>,
}

impl CourseRequirementTree {
    fn construct(tokens: &[String]) -> Self {
        // root always starts with AND
        let mut tree = ego_tree::Tree::new(CourseRequirementNode::AND);
        let mut tree_depth_stack: Vec<NodeId> = Vec::new(); // LIFO

        // push root to stack
        tree_depth_stack.push(tree.root().id());

        // check mode & create tree
        for text in tokens {
            // last node in stack
            let mut parent = tree.get_mut(*tree_depth_stack.last().unwrap()).unwrap();

            // // some text here may be wrapped in `{}`
            // let text = text.trim_matches('{').trim_end_matches("}");

            match text.as_str() {
                "(" | "[" => {
                    // create new stack
                    // by default AND, however, once OR is encountered, node is changed to OR
                    let node = parent.append(CourseRequirementNode::AND);
                    // push ID
                    tree_depth_stack.push(node.id());
                }

                ")" | "]" => {
                    // pop stack
                    tree_depth_stack.pop();
                }

                "|" => {
                    // get closest node and change it to OR
                    let parent_value = parent.value();
                    *parent_value = CourseRequirementNode::OR;
                }

                "&" => {
                    // get closest node and change it to AND
                    let parent_value = parent.value();
                    *parent_value = CourseRequirementNode::AND;
                }

                // course
                _ => {
                    // push to closest node
                    parent.append(CourseRequirementNode::COURSE(
                        text.as_str().try_into().unwrap(),
                    ));
                }
            }
        }
        Self { tree }
    }
}

impl TryFrom<&str> for CourseRequirementTree {
    type Error = &'static str;
    /// All non-logical characters (!= OR, AND, (, ), [, ]) should be wrapped with curly braces
    /// E.g. ({MATH 140}AND { MATH 360 })
    fn try_from(requirements_string: &str) -> Result<Self, Self::Error> {
        // tokenize requirements_string
        let mut tokens = Vec::<String>::new();
        let mut token_buffer = String::new();
        let mut is_inside_escaped = false; // if current cursor is inside `{}`

        for char in requirements_string.chars() {
            if is_inside_escaped && char == '}' {
                // trim, push and flush buffer
                tokens.push(token_buffer.trim().to_string());
                token_buffer.clear();
                // set `is_inside_escape` flag to false
                is_inside_escaped = false;
            } else if is_inside_escaped {
                token_buffer.push(char);
            } else {
                match char {
                    '{' => {
                        // set `is_inside_escape` flag to true
                        is_inside_escaped = true;
                    }
                    '}' => {
                        return Err("Unmatched closing brace");
                    }
                    '|' | '&' | '(' | ')' | '[' | ']' => {
                        // push token
                        tokens.push(char.to_string());
                    }
                    // ignore everything else
                    _ => (),
                }
            }
        }

        Ok(Self::construct(&tokens))
    }
}

impl fmt::Display for CourseRequirementTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO:    fix bug:   "{MATH 140}|{MATH 141}&{PHYS 121}"
        //          expected:   MATH 140 OR (MATH 141 AND PHYS 121)
        //          result:     MATH 140 AND MATH 141 AND PHYS 121
        fn node_to_string(node: &NodeRef<CourseRequirementNode>) -> String {
            let mut buffer = Vec::<String>::new();

            // if node doesn't have children just return empty string
            if !node.has_children() {
                return String::new();
            }

            // go through each child
            for child in node.children() {
                // recursively call this function on types that have AND or OR types
                // else push anything else to current buffer
                match child.value() {
                    CourseRequirementNode::AND | CourseRequirementNode::OR => {
                        buffer.push(node_to_string(&child));
                    }
                    CourseRequirementNode::COURSE(course_identifier) => {
                        buffer.push(course_identifier.to_string());
                    }
                }
            }

            buffer.retain(|s| !s.trim().is_empty()); // remove empty strings

            let should_wrap = node.parent().is_none() || buffer.len() < 2;
            match node.value() {
                CourseRequirementNode::AND => {
                    match should_wrap {
                        true => format!("{}", buffer.join(" AND ")), // ignore root or if less than 2 courses are in array
                        false => format!("[{}]", buffer.join(" AND ")),
                    }
                }
                CourseRequirementNode::OR => match should_wrap {
                    true => format!("{}", buffer.join(" OR ")),
                    false => format!("({})", buffer.join(" OR ")),
                },
                CourseRequirementNode::COURSE(course_identifier) => {
                    panic!("Node value {} is not AND or OR", course_identifier)
                }
            }
        }

        // TODO: remove later?

        write!(f, "{}", node_to_string(&self.tree.root()))
    }
}
