//! Enricher worker — consumes raw messages, dedups, filters scams,
//! parses structured fields via LLM, computes embeddings, and persists to `leads`.

pub mod dedup;
pub mod embedder;
pub mod parser;
pub mod scam;
