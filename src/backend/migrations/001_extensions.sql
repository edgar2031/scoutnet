-- Enable pgvector extension for HNSW cosine similarity indexes.
CREATE EXTENSION IF NOT EXISTS vector;
-- Enable UUID generation functions.
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
