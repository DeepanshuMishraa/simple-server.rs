-- Drop the existing users table and recreate with SERIAL id
DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    age INT4 NOT NULL
);
