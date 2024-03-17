const express = require("express");
const app = express();
const sqlite3 = require("sqlite3");

const port = 4001;

// app.use(express);

const db = new sqlite3.Database("./bulletin.db", (err) => {
  if (err) {
    console.log("Error with database");
  } else {
    console.log("Connected to database");
  }
});

// app.get("/programs", (req, res) => {
//   db.all("SELECT * FROM UndergraduateProgram", (err, rows) => {
//     if (err) {
//       console.log("error");
//       res.status(500).json({ error: "Not working" });
//     } else {
//       res.json(rows);
//     }
//   });
// });

// app.get("/programs?title=Accounting&College_id=1", (req, res) => {
//   db.get(
//     "SELECT * FROM UndergraduateProgram WHERE title='Accounting' AND college_id=1;",
//     (err, row) => {
//       if (err) {
//         console.log("error");
//         res.status(500).json({ error: "Not working" });
//       } else {
//         res.json(row);
//       }
//     }
//   );
// });

/*
course info is returned
ex. http://localhost:4001/api/programs?title=Accounting&college_id=2
*/
app.get("/api/programs", (req, res) => {
  let query = "";

  let params = [];
  let values = [];

  if (req.query.id) {
    params.push("id=?");
    values.push(req.query.id);
  }

  if (req.query.title) {
    params.push("title=?");
    values.push(req.query.title);
  }

  if (req.query.type_id) {
    params.push("type_id=?");
    values.push(req.query.type_id);
  }

  if (req.query.college_id) {
    params.push("college_id=?");
    values.push(req.query.college_id);
  }

  query =
    "SELECT  UndergraduateProgram.id, title, link, image, type, name AS college FROM UndergraduateProgram" +
    " JOIN UndergraduateProgramType ON UndergraduateProgram.type_id = UndergraduateProgramType.id" +
    " JOIN College ON UndergraduateProgram.college_id = College.id";

  // wildcard implementation
  // if (params.length > 0) {
  //   query += " WHERE ";

  //   console.log(query);

  //   const holder = [];
  //   if (req.query.id) {
  //     holder.push("UndergraduateProgram.id LIKE %" + req.query.id + "%");
  //   }
  //   if (req.query.title) {
  //     holder.push("title LIKE %" + req.query.id + "%");
  //   }
  //   if (req.query.type_id) {
  //     holder.push("type LIKE %" + req.query.type_id + "%");
  //   }
  //   if (req.query.college_id) {
  //     holder.push("college LIKE %" + req.query.college_id + "%");
  //   }

  //   query += holder.join(" AND ");

  //   console.log(query);
  // }

  if (params.length > 0) {
    query += " WHERE " + params.join(" AND ");
  }

  db.all(query, values, (err, row) => {
    if (err) {
      console.log("error");
      res.status(500).json({ error: "Not working" });
    } else {
      res.json(row);
    }
  });
});

//campus info is returned
app.get("/api/campus", (req, res) => {
  let query = "";

  let params = [];
  let values = [];

  if (req.query.name) {
    params.push("name=?");
    values.push(req.query.name);
  }

  if (req.query.program_id) {
    params.push("program_id=?");
    values.push(req.query.program_id);
  }

  if (req.query.title) {
    params.push("title=?");
    values.push(req.query.title);
  }

  if (req.query.type) {
    params.push("type=?");
    values.push(req.query.type);
  }
  query =
    "SELECT name, program_id, UndergraduateProgram.title, type " +
    "FROM Campus" +
    " JOIN UndergraduateProgram ON Campus.program_id = UndergraduateProgram.id AND UndergraduateProgram.type_id" +
    " JOIN UndergraduateProgramType ON UndergraduateProgram.type_id = UndergraduateProgramType.id";

  if (params.length > 0) {
    query += " WHERE " + params.join(" AND ");
  }

  db.all(query, values, (err, row) => {
    if (err) {
      console.log("error");
      res.status(500).json({ error: "Not working" });
    } else {
      res.json(row);
    }
  });
});

//   if (
//     req.query.id &&
//     req.query.title &&
//     req.query.type_id &&
//     req.query.college_id
//   ) {
//     db.get(
//       "SELECT * FROM UndergraduateProgram WHERE id=? AND title=? AND type_id=? AND college_id=?",
//       [req.query.id, req.query.title, req.query.type_id, req.query.college_id],
//       (err, row) => {
//         if (err) {
//           console.log("error");
//           res.status(500).json({ error: "Not working" });
//         } else {
//           res.json(row);
//         }
//       }
//     );
//   } else {
//     db.all("SELECT * FROM UndergraduateProgram", (err, rows) => {
//       if (err) {
//         console.log("error");
//         res.status(500).json({ error: "Not working" });
//       } else {
//         res.json(rows);
//       }
//     });
//   }
// });

app.listen(port, () => {
  console.log(`its up giddyup at PORT ${port}`);
});
