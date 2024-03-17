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

-- check if it exists, if not create it.
CREATE TABLE IF NOT EXISTS UndergraduateProgramType (
  id INTEGER PRIMARY KEY,
  type VARCHAR(255) UNIQUE NOT NULL
);

-- check if it exists, if not create it.
CREATE TABLE IF NOT EXISTS College (
  id INTEGER PRIMARY KEY,
  name VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS Keywords (
  id INTEGER PRIMARY KEY,
  keyword VARCHAR(255),
  program_id INT NOT NULL,

  FOREIGN KEY (program_id) REFERENCES UndergraduateProgram(id)
);

CREATE TABLE IF NOT EXISTS Campus (
  id INTEGER PRIMARY KEY,
  name VARCHAR(255),
  program_id INT NOT NULL,

  FOREIGN KEY (program_id) REFERENCES UndergraduateProgram(id)
);
