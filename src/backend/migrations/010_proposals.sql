CREATE TABLE proposals (
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    match_id   UUID NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    content    TEXT NOT NULL,
    version    INTEGER NOT NULL DEFAULT 1,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_proposals_match ON proposals (match_id);
CREATE INDEX idx_proposals_user  ON proposals (user_id, created_at DESC);
