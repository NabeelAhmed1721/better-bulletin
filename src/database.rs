use ego_tree::NodeId;
use libsql_client::{args, de, Statement};

use crate::bulletin::{self, CourseIdentifier, CourseRequirementTree, UndergraduateCourse, UndergraduateProgram};

pub trait Synchronizable<T> {
    fn sync(url: &str, items: &[T]);
}

pub struct SyncUndergraduatePrograms;

#[derive(serde::Deserialize)]
struct IdRecord {
    id: i64,
}

impl Synchronizable<UndergraduateProgram> for SyncUndergraduatePrograms {
    fn sync(url: &str, items: &[UndergraduateProgram]) {
        let db = match libsql_client::local::Client::new(url) {
            Ok(db) => db,
            Err(e) => panic!("{}", e),
        };

        // create tables if they don't already exist
        match db.batch([
            Statement::new(
                r#"
                    CREATE TABLE IF NOT EXISTS UndergraduateProgram (
                        id INTEGER PRIMARY KEY,
                        title VARCHAR(255) NOT NULL,
                        link VARCHAR(255) NOT NULL UNIQUE,
                        image VARCHAR(255),
                        type_id INT NOT NULL,
                        college_id INT, -- can be null
                    
                        FOREIGN KEY (type_id) REFERENCES UndergraduateProgramType(id),
                        FOREIGN KEY (college_id) REFERENCES College(id)
                    );
                "#,
            ),
            // check if it exists, if not create it.
            Statement::new(
                r#"
                    CREATE TABLE IF NOT EXISTS UndergraduateProgramType (
                        id INTEGER PRIMARY KEY,
                        type VARCHAR(255) UNIQUE NOT NULL
                    );
                "#,
            ),
            // check if it exists, if not create it.
            Statement::new(
                r#"
                    CREATE TABLE IF NOT EXISTS College (
                        id INTEGER PRIMARY KEY,
                        name VARCHAR(255) UNIQUE NOT NULL
                    );
                "#,
            ),
            Statement::new(
                r#"
                    CREATE TABLE IF NOT EXISTS Keywords (
                        id INTEGER PRIMARY KEY,
                        keyword VARCHAR(255),
                        program_id INT NOT NULL,
                    
                        FOREIGN KEY (program_id) REFERENCES UndergraduateProgram(id)
                    );
                "#,
            ),
            Statement::new(
                r#"
                    CREATE TABLE IF NOT EXISTS Campus (
                        id INTEGER PRIMARY KEY,
                        name VARCHAR(255),
                        program_id INT NOT NULL,
                    
                        FOREIGN KEY (program_id) REFERENCES UndergraduateProgram(id)
                    );
                "#,
            ),
        ]) {
            Ok(res) => res,
            Err(e) => panic!("{}", e),
        };

        for item in items {
            let (raw_type, campus_list): (&str, Option<Vec<&str>>) = match &item.program_type {
                bulletin::UndergraduateProgramType::BaccalaureateDegree(raw_type, campus_list) => {
                    (&raw_type, Some(campus_list.disassemble()))
                }
                bulletin::UndergraduateProgramType::AssociateDegree(raw_type, campus_list) => {
                    (&raw_type, Some(campus_list.disassemble()))
                }
                bulletin::UndergraduateProgramType::Certificate(raw_type) => (&raw_type, None),
                bulletin::UndergraduateProgramType::Minor(raw_type) => (&raw_type, None),
                bulletin::UndergraduateProgramType::ROTC(raw_type) => (&raw_type, None),
            };

            let undergraduate_program_type_id: i64 = {
                let res = match db.batch([
                    Statement::with_args(
                        "INSERT OR IGNORE INTO UndergraduateProgramType (type) VALUES (?)",
                        args!(raw_type),
                    ),
                    Statement::with_args(
                        "SELECT id FROM UndergraduateProgramType WHERE type = ?",
                        args!(raw_type),
                    ),
                ]) {
                    Ok(res) => res,
                    Err(e) => panic!("{}", e),
                };

                res.last()
                    .unwrap()
                    .rows
                    .iter()
                    .map(de::from_row)
                    .collect::<Result<Vec<IdRecord>, _>>()
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .id
            };

            let college_id: Option<i64> = match &item.college {
                Some(college) => {
                    let college = &college.to_string();
                    let res = match db.batch([
                        Statement::with_args(
                            "INSERT OR IGNORE INTO College (name) VALUES (?)",
                            args!(college),
                        ),
                        Statement::with_args(
                            "SELECT id FROM College WHERE name = ?",
                            args!(college),
                        ),
                    ]) {
                        Ok(res) => res,
                        Err(e) => panic!("{}", e),
                    };

                    Some(
                        res.last()
                            .unwrap()
                            .rows
                            .iter()
                            .map(de::from_row)
                            .collect::<Result<Vec<IdRecord>, _>>()
                            .unwrap()
                            .get(0)
                            .unwrap()
                            .id,
                    )
                }
                None => None,
            };

            let program_res = match db.execute(Statement::with_args(
                "INSERT OR IGNORE INTO UndergraduateProgram (title, link, image, type_id, college_id) VALUES (?, ?, ?, ?, ?)",
                args!(
                    *item.title,
                    *item.link,
                    *item.image,
                    undergraduate_program_type_id,
                    college_id,
                ),
            )) {
                Ok(res) => res,
                Err(e) => panic!("{}", e),
            };

            let program_id = match program_res.last_insert_rowid {
                Some(id) => {
                    if id == 0 {
                        continue;
                    } else {
                        id
                    }
                }
                None => {
                    panic!("program_id couldn't be found.")
                }
            };

            let mut batch_statements = Vec::new();

            if let Some(campus_list) = campus_list {
                for campus in campus_list {
                    batch_statements.push(Statement::with_args(
                        "INSERT INTO Campus (name, program_id) VALUES (?, ?)",
                        args!(campus, program_id),
                    ));
                }
            }

            match db.batch(batch_statements) {
                Ok(res) => res,
                Err(e) => panic!("{}", e),
            };

            let mut batch_statements = Vec::new();

            for keyword in &item.keywords {
                batch_statements.push(Statement::with_args(
                    "INSERT INTO Keywords (keyword, program_id) VALUES (?, ?)",
                    args!(keyword, program_id),
                ));
            }

            match db.batch(batch_statements) {
                Ok(res) => res,
                Err(e) => panic!("{}", e),
            };
        }

        println!("Sync complete.")
    }
}

pub struct SyncUndergraduateCourses;

impl Synchronizable<UndergraduateCourse> for SyncUndergraduateCourses {
    fn sync(url: &str, items: &[UndergraduateCourse]) {
        let db = match libsql_client::local::Client::new(url) {
            Ok(db) => db,
            Err(e) => panic!("{}", e),
        };

        // create tables if they don't already exist
        match db.batch([
            // TODO: add safety attributes (UNIQUE, NOT NULL, etc)
            Statement::new(
                r#"
                    CREATE TABLE IF NOT EXISTS UndergraduateCourse (
                        id INTEGER PRIMARY KEY,
                        code VARCHAR(5),
                        number INT,
                        suffix VARCHAR(1),
                        title VARCHAR(255),
                        description VARCHAR(65535),
                        credits REAL,
                        min_credits REAL,
                        GA bool,
                        GHW bool,
                        GH bool,
                        GN bool,
                        GQ bool,
                        GS bool,
                        GWS bool,
                        ITD bool,
                        LKD bool,
                        FYS bool,
                        IC bool,
                        US bool,
                        WCC bool,
                        BA bool,
                        BH bool,
                        BN bool,
                        BO bool,
                        BQ bool,
                        BS bool,
                        BF1 bool,
                        BF2 bool,
                        HNR bool,
                        is_prerequisite_concurrent_separate bool,
                        empty_crosslist bool,
                        unknown_requirement bool,

                        -- FOREIGN KEY (id) REFERENCES UndergraduateCourseCrossLists (course_id)
                        -- FOREIGN KEY (id) REFERENCES UndergraduateCoursePrerequisites (course_id)
                        -- FOREIGN KEY (id) REFERENCES UndergraduateCourseConcurrent (course_id)
                        -- FOREIGN KEY (id) REFERENCES UndergraduateCourseCorequisites (course_id)
                        -- FOREIGN KEY (id) REFERENCES UndergraduateCourseRecommended (course_id)
                        UNIQUE (code, number, suffix)
                    );
                "#,
            ),
            Statement::new(
                r#"
                    CREATE TABLE IF NOT EXISTS UndergraduateCourseCrossLists (
                        id INTEGER PRIMARY KEY,
                        course_id INT,
                        crossed_course_id INT,

                        FOREIGN KEY (crossed_course_id) REFERENCES UndergraduateCourse (id)
                    )
                "#,
            ),
            // TODO: merge all requirements into one table with type key to differentiate?
            Statement::new(
                r#"
                    CREATE TABLE IF NOT EXISTS UndergraduateCoursePrerequisites (
                        id INTEGER PRIMARY KEY,
                        logic VARCHAR(1),
                        course_id INT,
                        req_course_id INT,
                        parent INT,

                        FOREIGN KEY (req_course_id) REFERENCES UndergraduateCourse (id)
                        FOREIGN KEY (parent) REFERENCES UndergraduateCoursePrerequisites (id)

                    )
                "#,
            ),
            Statement::new(
                r#"
                    CREATE TABLE IF NOT EXISTS UndergraduateCourseConcurrent (
                        id INTEGER PRIMARY KEY,
                        logic VARCHAR(1),
                        course_id INT,
                        req_course_id INT,
                        parent INT,

                        FOREIGN KEY (req_course_id) REFERENCES UndergraduateCourse (id)
                        FOREIGN KEY (parent) REFERENCES UndergraduateCourseConcurrent (id)

                    )
                "#,
            ),
            Statement::new(
                r#"
                    CREATE TABLE IF NOT EXISTS UndergraduateCourseCorequisites (
                        id INTEGER PRIMARY KEY,
                        logic VARCHAR(1),
                        course_id INT,
                        req_course_id INT,
                        parent INT,

                        FOREIGN KEY (req_course_id) REFERENCES UndergraduateCourse (id)
                        FOREIGN KEY (parent) REFERENCES UndergraduateCourseCorequisites (id)

                    )
                "#,
            ),
            Statement::new(
                r#"
                    CREATE TABLE IF NOT EXISTS UndergraduateCourseRecommended (
                        id INTEGER PRIMARY KEY,
                        logic VARCHAR(1),
                        course_id INT,
                        req_course_id INT,
                        parent INT,

                        FOREIGN KEY (req_course_id) REFERENCES UndergraduateCourse (id)
                        FOREIGN KEY (parent) REFERENCES UndergraduateCourseRecommended (id)

                    )
                "#,
            ),
        ]) {
            Ok(res) => res,
            Err(e) => panic!("{}", e),
        };

        // TODO: TOO MANY CLONES
        for item in items {
            // crosslist table
            // for crosslist in
            let CourseIdentifier {
                code,
                number,
                suffix,
            } = item.identifier.clone();

            // libSQL doesn't support char conversion?
            let suffix = suffix.map(|c| c.to_string());

            let _course_res = match db.execute(Statement::with_args(
                r#"INSERT OR IGNORE INTO UndergraduateCourse (
                    code,
                    number,
                    suffix,
                    title,
                    description,
                    credits,
                    min_credits,
                    GA,
                    GHW,
                    GH,
                    GN,
                    GQ,
                    GS,
                    GWS,
                    ITD,
                    LKD,
                    FYS,
                    IC,
                    US,
                    WCC,
                    BA,
                    BH,
                    BN,
                    BO,
                    BQ,
                    BS,
                    BF1,
                    BF2,
                    HNR,
                    is_prerequisite_concurrent_separate,
                    empty_crosslist,
                    unknown_requirement
                ) VALUES (
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?,
                    ?
                )"#,
                args!(
                    code,
                    number,
                    suffix,
                    item.title.clone(),
                    item.description.clone(),
                    item.credits,
                    item.min_credits,
                    if item.attribute_list.get(0) { 1 } else { 0 },
                    if item.attribute_list.get(1) { 1 } else { 0 },
                    if item.attribute_list.get(2) { 1 } else { 0 },
                    if item.attribute_list.get(3) { 1 } else { 0 },
                    if item.attribute_list.get(4) { 1 } else { 0 },
                    if item.attribute_list.get(5) { 1 } else { 0 },
                    if item.attribute_list.get(6) { 1 } else { 0 },
                    if item.attribute_list.get(7) { 1 } else { 0 },
                    if item.attribute_list.get(8) { 1 } else { 0 },
                    if item.attribute_list.get(9) { 1 } else { 0 },
                    if item.attribute_list.get(10) { 1 } else { 0 },
                    if item.attribute_list.get(11) { 1 } else { 0 },
                    if item.attribute_list.get(12) { 1 } else { 0 },
                    if item.attribute_list.get(13) { 1 } else { 0 },
                    if item.attribute_list.get(14) { 1 } else { 0 },
                    if item.attribute_list.get(15) { 1 } else { 0 },
                    if item.attribute_list.get(16) { 1 } else { 0 },
                    if item.attribute_list.get(17) { 1 } else { 0 },
                    if item.attribute_list.get(18) { 1 } else { 0 },
                    if item.attribute_list.get(19) { 1 } else { 0 },
                    if item.attribute_list.get(20) { 1 } else { 0 },
                    if item.attribute_list.get(21) { 1 } else { 0 },
                    if item.flags.is_prerequisite_concurrent_separate {
                        1
                    } else {
                        0
                    },
                    if item.flags.deviant.empty_crosslist {
                        1
                    } else {
                        0
                    },
                    if item.flags.deviant.unknown_requirement {
                        1
                    } else {
                        0
                    }
                ),
            )) {
                Ok(res) => res,
                Err(e) => panic!("{}", e),
            };
        }

        let extract_and_sync_tree = |course_id: i64, table: &str, requirement_tree: &CourseRequirementTree| {

                // TODO: Shouldn't need to be 2 vectors. 
                let mut node_stack = Vec::<NodeId>::new();
                let mut node_id_stack = Vec::<i64>::new();

                for node in requirement_tree.tree.nodes() {
                    let node_val = node.value();
                    match node_val {
                        bulletin::CourseRequirementNode::AND => {
                            if node_id_stack.len() == 0 {
                                // insert root node
                                let res = match db.execute(Statement::with_args(
                                    format!("INSERT OR IGNORE INTO {} (logic, course_id, req_course_id, parent) VALUES (?, ?, ?, ?)", table),
                                    args!(
                                        "&", // and
                                        course_id, // only root points to course
                                        None::<i64>, // not "C" logic
                                        None::<i64>, // no parent
                                    ),
                                )) {
                                    Ok(res) => res,
                                    Err(e) => panic!("{}", e),
                                };

                                node_stack.push(node.id());
                                node_id_stack.push(
                                    res.last_insert_rowid.unwrap()
                                );
                            } else {
                                // insert root node
                                let res = match db.execute(Statement::with_args(
                                    format!("INSERT OR IGNORE INTO {} (logic, course_id, req_course_id, parent) VALUES (?, ?, ?, ?)", table),                                    args!(
                                        "&", // and
                                        None::<i64>, // not root
                                        None::<i64>, // not "C" logic
                                        *node_id_stack.last().unwrap(), // parent
                                    ),
                                )) {
                                    Ok(res) => res,
                                    Err(e) => panic!("{}", e),
                                };

                                node_stack.push(node.id());
                                node_id_stack.push(
                                    res.last_insert_rowid.unwrap()
                                );
                            }
                        }
                        bulletin::CourseRequirementNode::OR => {

                            if node_id_stack.len() == 0 {
                                // insert root node
                                let res = match db.execute(Statement::with_args(
                                    format!("INSERT OR IGNORE INTO {} (logic, course_id, req_course_id, parent) VALUES (?, ?, ?, ?)", table),                                    args!(
                                        "|", // or
                                        course_id, // only root points to course
                                        None::<i64>, // not "C" logic
                                        None::<i64>, // no parent
                                    ),
                                )) {
                                    Ok(res) => res,
                                    Err(e) => panic!("{}", e),
                                };

                                node_stack.push(node.id());
                                node_id_stack.push(
                                    res.last_insert_rowid.unwrap()
                                );
                            } else {
                                let res = match db.execute(Statement::with_args(
                                    format!("INSERT OR IGNORE INTO {} (logic, course_id, req_course_id, parent) VALUES (?, ?, ?, ?)", table),                                    args!(
                                        "|", // or
                                        None::<i64>, // not root
                                        None::<i64>, // not "C" logic
                                        *node_id_stack.last().unwrap(), // parent
                                    ),
                                )) {
                                    Ok(res) => res,
                                    Err(e) => panic!("{}", e),
                                };

                                node_stack.push(node.id());
                                node_id_stack.push(
                                    res.last_insert_rowid.unwrap(),
                                );
                            }
                        }
                        bulletin::CourseRequirementNode::COURSE(course) => {
                            // if current course isn't equal to the last 
                            // keep popping until match is found
                            while *node_stack.last().unwrap() != node.parent().unwrap().id() {
                                node_stack.pop().unwrap();
                                node_id_stack.pop().unwrap();
                            }

                            // find ref_course_id
                            let ref_course_id = {
                                let res = match db.execute(Statement::with_args(
                                    match course.suffix {
                                        Some(_) => "SELECT id FROM UndergraduateCourse WHERE code = ? AND number = ? AND suffix = ?",
                                        None => "SELECT id FROM UndergraduateCourse WHERE code = ? AND number = ? AND suffix IS NULL",
                                    }
                                    ,
                                    args!(
                                        course.code.clone(),
                                        course.number,
                                        course.suffix.map(|s| s.to_string())
                                    ),
                                    )) {
                                        Ok(res) => res,
                                        Err(e) => panic!("{}", e),
                                    };

                               match res.rows
                                        .iter() 
                                        .map(de::from_row)
                                        .collect::<Result<Vec<IdRecord>, _>>()
                                        .unwrap()
                                        .get(0) {  
                                        Some(id) => id,
                                        None => return, // skip courses that don't exist
                                    }
                                    .id
                            };
                           match db.execute(Statement::with_args(
                                format!("INSERT OR IGNORE INTO {} (logic, course_id, req_course_id, parent) VALUES (?, ?, ?, ?)", table),                                args!(
                                    "C", // course
                                    None::<i64>, // not root
                                    ref_course_id, // course id in db
                                    *node_id_stack.last().unwrap(), // parent
                                ),
                            )) {
                                Ok(res) => res,
                                Err(e) => panic!("{}", e),
                            };
                        }
                    }
            }
        };


        // once courses are added do crosslist and requirements
        for item in items {
            // get item record id
            let course_id = {
                let res = match db.execute(Statement::with_args(
                    match item.identifier.suffix {
                        Some(_) => "SELECT id FROM UndergraduateCourse WHERE code = ? AND number = ? AND suffix = ?",
                        None => "SELECT id FROM UndergraduateCourse WHERE code = ? AND number = ? AND suffix IS NULL",
                    },                        
                    args!(
                        item.identifier.code.clone(),
                        item.identifier.number,
                        item.identifier.suffix.map(|s| s.to_string())
                    ),
                )) {
                    Ok(res) => res,
                    Err(e) => panic!("{}", e),
                };

                res.rows
                    .iter()
                    .map(de::from_row)
                    .collect::<Result<Vec<IdRecord>, _>>()
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .id
            };

            // crosslist
            if let Some(crosslist) = &item.crosslist {
                let mut batch_statements = Vec::new();
                for course in crosslist {
                    let crossed_course_id = {
                        let res: libsql_client::ResultSet = match db.execute(Statement::with_args(
                            match course.suffix {
                                Some(_) => "SELECT id FROM UndergraduateCourse WHERE code = ? AND number = ? AND suffix = ?",
                                None => "SELECT id FROM UndergraduateCourse WHERE code = ? AND number = ? AND suffix IS NULL",
                            },
                            args!(
                                course.code.clone(),
                                course.number,
                                course.suffix.map(|s| s.to_string())
                            ),
                        )) {
                            Ok(res) => res,
                            Err(e) => panic!("{}", e),
                        };

                        match res.rows
                            .iter()
                            .map(de::from_row)
                            .collect::<Result<Vec<IdRecord>, _>>()
                            .unwrap()
                            .get(0)
                            {
                                Some(id) => id,
                                None => continue, // some courses don't exist??
                            }
                            .id
                    };

                    batch_statements.push(Statement::with_args(
                    "INSERT INTO UndergraduateCourseCrossLists (course_id, crossed_course_id) VALUES (?, ?)",
                    args!(
                        course_id,
                        crossed_course_id
                    ),
                ));
                }

                match db.batch(batch_statements) {
                    Ok(res) => res,
                    Err(e) => panic!("{}", e),
                };
            }

            // requirements
            match &item.requirements.prerequisites {
                Some(prerequisites) => {
                    extract_and_sync_tree(course_id, "UndergraduateCoursePrerequisites", prerequisites);
                }
                None => (),
            }
            match &item.requirements.concurrent {
                Some(concurrent) => {
                    extract_and_sync_tree(course_id, "UndergraduateCourseConcurrent", concurrent);
                }
                None => (),
            }
            match &item.requirements.corequisites {
                Some(corequisites) => {
                    extract_and_sync_tree(course_id, "UndergraduateCourseCorequisites", corequisites);
                }
                None => (),
            }
            match &item.requirements.recommended {
                Some(recommended) => {
                    extract_and_sync_tree(course_id, "UndergraduateCourseRecommended", recommended);
                }
                None => (),
            }
        }
    }
}
