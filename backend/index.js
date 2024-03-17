const express = require("express");
const cors = require('cors');
const sqlite3 = require("sqlite3");
const fs = require('fs');

const PORT = 3001;
const app = express();

// enable cors
app.use(cors());


// setup database
// pooled connection
const db = new sqlite3.Database("./bulletin.db", (err) => {
    if (err) {
        console.log("Error with database");
    } else {
        console.log("Connected to database");
    }
});

app.get("/api/stats", (req, res) => {
    // return database file creation date
    const creation_date = fs.statSync("./bulletin.db").birthtimeMs;

    db.get(`
        SELECT COUNT(*) FROM UndergraduateCourse;
    `, (err, row) => {
        if (err) {
            console.log("Error with database: ", err);
        } else {
            res.json({
                age: creation_date,
                record_count: row["COUNT(*)"]
            })
        }
    })
});

app.get("/api/course/crosslists/:id", (req, res) => {
    const { id } = req.params;

    db.all(`
        SELECT
            UndergraduateCourse.id,
            UndergraduateCourse.code,
            UndergraduateCourse.number,
            UndergraduateCourse.suffix,
            UndergraduateCourse.title
        FROM UndergraduateCourseCrossLists
        JOIN UndergraduateCourse ON UndergraduateCourseCrossLists.course_id = UndergraduateCourse.id
        JOIN UndergraduateCourse AS  CrossedUndergraduateCourse ON UndergraduateCourseCrossLists.crossed_course_id = CrossedUndergraduateCourse.id
        WHERE course_id = ${id};
    `, (err, crosslists) => {
            if (err) {
                console.log("Error with database: ", err);
            } else {
                res.json({ crosslists });
            }
        }
    )
})

app.get("/api/course/prerequisites/:id", (req, res) => {
    const { id } = req.params;
    // recursively go down requirement tree
    db.all(`
        WITH RECURSIVE tree AS (
            SELECT * 
            FROM UndergraduateCoursePrerequisites 
            WHERE course_id = ${id} 
        UNION ALL 
            SELECT node.* 
            FROM UndergraduateCoursePrerequisites node
            JOIN tree
                ON node.parent = tree.id
        ) SELECT * FROM tree;
    `, (err, rows) => {
        if (err) {
            console.log("Error with database: ", err);
        } else {
            res.json(rows);
        }
    })
})

app.get("/api/course/concurrent/:id", (req, res) => {
    const { id } = req.params;
    // recursively go down requirement tree
    db.all(`
        WITH RECURSIVE tree AS (
            SELECT * 
            FROM UndergraduateCourseConcurrent 
            WHERE course_id = ${id} 
        UNION ALL 
            SELECT node.* 
            FROM UndergraduateCourseConcurrent node
            JOIN tree
                ON node.parent = tree.id
        ) SELECT * FROM tree;
    `, (err, rows) => {
        if (err) {
            console.log("Error with database: ", err);
        } else {
            res.json(rows);
        }
    })
})

app.get("/api/course/corequisites/:id", (req, res) => {
    const { id } = req.params;
    // recursively go down requirement tree
    db.all(`
        WITH RECURSIVE tree AS (
            SELECT * 
            FROM UndergraduateCourseCorequisites 
            WHERE course_id = ${id} 
        UNION ALL 
            SELECT node.* 
            FROM UndergraduateCourseCorequisites node
            JOIN tree
                ON node.parent = tree.id
        ) SELECT * FROM tree;
    `, (err, rows) => {
        if (err) {
            console.log("Error with database: ", err);
        } else {
            res.json(rows);
        }
    })
})

app.get("/api/course/recommended/:id", (req, res) => {
    const { id } = req.params;
    // recursively go down requirement tree
    db.all(`
        WITH RECURSIVE tree AS (
            SELECT * 
            FROM UndergraduateCourseRecommended 
            WHERE course_id = ${id} 
        UNION ALL 
            SELECT node.* 
            FROM UndergraduateCourseRecommended node
            JOIN tree
                ON node.parent = tree.id
        ) SELECT * FROM tree;
    `, (err, rows) => {
        if (err) {
            console.log("Error with database: ", err);
        } else {
            res.json(rows);
        }
    })
})

app.get("/api/course/:id", (req, res) => {
    const { id } = req.params;
    db.get(`
        SELECT * FROM UndergraduateCourse WHERE id = ${id};
    `, (err, row) => {
        if (err) {
            console.log("Error with database: ", err);
        } else {
            res.json(row);
        }
    }); 
});

app.get("/api/search/courses", (req, res) => {
    const { query } = req.query;
    db.all(`
            SELECT * FROM UndergraduateCourse
            WHERE (
                CONCAT(code, ' ', number, suffix) LIKE '%${query}%' OR
                title LIKE '%${query}%' OR
                description LIKE '%${query}%'
            )
            LIMIT 25;
    `, (err, rows) => {
        if (err) {
            console.log("Error with database: ", err);
        } else {
            res.json(rows);
        }
    });
})


app.listen(PORT, () => {
    console.log(`Server running on PORT: ${PORT}`);
})