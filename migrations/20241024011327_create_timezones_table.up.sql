-- Add up migration script here
CREATE TABLE timezones (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);