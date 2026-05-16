CREATE TABLE leads (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    message_id  UUID NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    -- Voyage AI embedding: voyage-3-large = 1536 dimensions
    embedding   VECTOR(1536),
    skills      TEXT[],
    budget_min  INTEGER,
    budget_max  INTEGER,
    currency    TEXT,
    is_scam     BOOLEAN NOT NULL DEFAULT FALSE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- HNSW index for fast cosine similarity search (pgvector ≥ 0.5)
CREATE INDEX idx_leads_embedding ON leads
    USING hnsw (embedding vector_cosine_ops)
    WITH (m = 16, ef_construction = 64);
