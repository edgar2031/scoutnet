-- Monthly budget cap per user credential
ALTER TABLE ai_credentials
    ADD COLUMN IF NOT EXISTS monthly_cap_usd DOUBLE PRECISION NOT NULL DEFAULT 100.0;

-- AI token usage log for cost tracking and budget enforcement
CREATE TABLE ai_usage (
    id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id       UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    feature       TEXT NOT NULL,
    input_tokens  INTEGER NOT NULL DEFAULT 0,
    output_tokens INTEGER NOT NULL DEFAULT 0,
    cost_usd      DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ai_usage_user_month ON ai_usage (user_id, created_at);
