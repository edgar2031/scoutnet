-- Subscription tiers stored as a Postgres native enum.
CREATE TYPE tier AS ENUM ('free', 'starter', 'pro', 'team');

-- Source of a scraped message.
CREATE TYPE source_type AS ENUM ('telegram', 'web');

-- Match lifecycle states.
CREATE TYPE match_status AS ENUM ('pending', 'ready', 'delivered', 'rejected', 'applied');

-- AI provider identifiers for BYOK credentials.
CREATE TYPE ai_provider AS ENUM ('openai', 'anthropic', 'google', 'mistral', 'custom');
