-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  email TEXT NOT NULL,
  dob TEXT NOT NULL
)
