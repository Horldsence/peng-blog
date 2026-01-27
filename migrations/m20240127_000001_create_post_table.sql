-- Migration: Create post table
-- Version: 20240127_000001

-- User table for authentication and authorization
CREATE TABLE IF NOT EXISTS user (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    permissions INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_user_username ON user(username);

-- Post table with ownership tracking
CREATE TABLE IF NOT EXISTS post (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    published_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_post_published_at ON post(published_at);
CREATE INDEX IF NOT EXISTS idx_post_created_at ON post(created_at);
CREATE INDEX IF NOT EXISTS idx_post_user_id ON post(user_id);
