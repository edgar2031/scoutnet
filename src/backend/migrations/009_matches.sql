CREATE TABLE matches (
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    lead_id    UUID NOT NULL REFERENCES leads(id) ON DELETE CASCADE,
    score      REAL NOT NULL,
    reason     TEXT NOT NULL,
    status     match_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, lead_id)
);

CREATE INDEX idx_matches_user_status ON matches (user_id, status, created_at DESC);
CREATE INDEX idx_matches_lead ON matches (lead_id);
