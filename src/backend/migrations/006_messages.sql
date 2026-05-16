CREATE TABLE messages (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    source_id    TEXT NOT NULL,
    source_type  source_type NOT NULL,
    channel      TEXT NOT NULL,
    text         TEXT NOT NULL,
    posted_at    TIMESTAMPTZ NOT NULL,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (source_id, source_type)
);

CREATE INDEX idx_messages_channel   ON messages (channel);
CREATE INDEX idx_messages_posted_at ON messages (posted_at DESC);
