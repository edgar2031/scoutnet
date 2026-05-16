CREATE TABLE user_profiles (
    id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id       UUID NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    bio           TEXT,
    skills        TEXT[],
    -- Voyage AI embedding of the user's bio + skills for cosine matching
    embedding     VECTOR(1536),
    min_budget    INTEGER,
    channels      TEXT[],
    score_threshold REAL NOT NULL DEFAULT 0.70,
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- HNSW index on profile embeddings for reverse-lookup (matcher uses it)
CREATE INDEX idx_profiles_embedding ON user_profiles
    USING hnsw (embedding vector_cosine_ops)
    WITH (m = 16, ef_construction = 64);
