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