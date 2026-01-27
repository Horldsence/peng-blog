-- Initial schema for peng-blog
-- Create post table
CREATE TABLE IF NOT EXISTS post (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    published_at TEXT,
    created_at TEXT NOT NULL
);

-- Create index for published posts query optimization
CREATE INDEX IF NOT EXISTS idx_post_published_at ON post(published_at);

-- Create index for created_at for ordering
CREATE INDEX IF NOT EXISTS idx_post_created_at ON post(created_at);

-- Enable WAL mode for better concurrent performance
PRAGMA journal_mode = WAL;

-- Optimize for typical blog workload (read-heavy)
PRAGMA synchronous = NORMAL;
PRAGMA cache_size = -64000;
PRAGMA temp_store = memory;
