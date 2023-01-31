-- Add migration script here
CREATE TABLE IF NOT EXISTS author (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT
);

CREATE TABLE IF NOT EXISTS book (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  title TEXT NOT NULL,
  year_published TEXT NOT NULL,
  author_id INTEGER NOT NULL,
  rented_to TEXT,
  FOREIGN KEY(author_id) REFERENCES author(id)
);

INSERT INTO author (name) VALUES ("John Doe");
