-- Your SQL goes here
CREATE TABLE theorems (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL
);


INSERT INTO theorems VALUES (0, 'Some title', 'Some body');
