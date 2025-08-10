-- This file should undo anything in `up.sql`
-- Revert to the old table structure
DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id INT4 NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    age INT4 NOT NULL
);
