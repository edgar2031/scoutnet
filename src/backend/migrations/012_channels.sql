-- Monitored channels/sources for parsers (Telegram channels, RSS feeds, etc.)
CREATE TABLE channels (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    source      source_type NOT NULL,
    url         TEXT NOT NULL UNIQUE,
    title       TEXT,
    is_active   BOOLEAN NOT NULL DEFAULT TRUE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_channels_source_active ON channels (source, is_active);
