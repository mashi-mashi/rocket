-- Add up migration script here
CREATE TABLE todo (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    done BOOLEAN NOT NULL DEFAULT 0,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);