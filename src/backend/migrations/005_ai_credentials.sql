CREATE TABLE ai_credentials (
    id                UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id           UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider          ai_provider NOT NULL,
    -- AES-256-GCM encrypted API key fields (all four required together)
    ciphertext        BYTEA NOT NULL,
    nonce             BYTEA NOT NULL,
    encrypted_dek     BYTEA NOT NULL,
    dek_kms_key_id    TEXT NOT NULL,
    model_preference  TEXT,
    is_active         BOOLEAN NOT NULL DEFAULT TRUE,
    validated_at      TIMESTAMPTZ,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, provider)
);

CREATE INDEX idx_ai_credentials_user ON ai_credentials (user_id);
