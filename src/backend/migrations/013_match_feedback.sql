-- Per-user quality signals (+1 good match / -1 bad match) for re-ranker training
CREATE TABLE match_feedback (
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id    UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    match_id   UUID NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    signal     SMALLINT NOT NULL CHECK (signal IN (-1, 1)),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, match_id)
);

CREATE INDEX idx_match_feedback_user ON match_feedback (user_id);
