mod bulletin;
mod database;
mod scrape;
mod utility;

use bulletin::UndergraduateCourse;
use database::SyncUndergraduateCourses;
use scrape::{ScrapeUndergraduateCourseGroups, ScrapeUndergraduateCourses, Scrappable};

use crate::{
    database::{SyncUndergraduatePrograms, Synchronizable},
    scrape::ScrapeUndergraduatePrograms,
};

fn main() {
    let database_url: &str = "./bulletin.db";
    let programs_scrape_url: &str = &format!("{}/programs", bulletin::BASE_URL);

    let body = utility::retrieve_document(programs_scrape_url);
    let programs = ScrapeUndergraduatePrograms::extract_list(&body);

    // println!("Found {} programs", programs.len());
    SyncUndergraduatePrograms::sync(database_url, &programs);

    // undergrad course list
    let url = &format!(
        "{}/university-course-descriptions/undergraduate/",
        bulletin::BASE_URL
    );
    let html = utility::retrieve_document(url);
    let course_list = ScrapeUndergraduateCourseGroups::extract_list(&html);

    let mut courses = Vec::<UndergraduateCourse>::new();

    for course in &(course_list) {
        // for course in &(course_list[2..=2]) {
        let html =
            utility::retrieve_document(&(format!("{}{}", bulletin::BASE_URL, course.raw_link)));

        // println!("Extracting {}.", course.code);
        courses.append(&mut ScrapeUndergraduateCourses::extract_list(&html));
        // break;
    }

    println!("Saving to database...");
    SyncUndergraduateCourses::sync(database_url, &courses);
    println!("Saved to database.");
}

//
// -- SELECT COUNT(*) FROM UndergraduateProgram JOIN UndergraduateProgramType ON UndergraduateProgram.type_id = UndergraduateProgramType.id WHERE type = 'B.S.';
// SELECT * FROM Keywords JOIN UndergraduateProgram ON UndergraduateProgram.id = Keywords.program_id WHERE UndergraduateProgram.id = 2;
//
