-- Add migration script here
CREATE TABLE IF NOT EXISTS author (
  id  bigint,
  first_name VARCHAR(128) NOT NULL,
  last_name VARCHAR(128) NOT NULL,
  PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS book (
  id bigint,
  title VARCHAR(256) NOT NULL,
  year_published int NOT NULL,
  author_id bigint NOT NULL,
  PRIMARY KEY(id),
  CONSTRAINT fk_author
    FOREIGN KEY(author_id)
      REFERENCES author(id)
);
