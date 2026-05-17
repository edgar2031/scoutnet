//! Voyage AI embedder — returns a 1536-dim vector for each input.

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

/// Voyage AI embeddings HTTP endpoint.
pub const VOYAGE_URL: &str = "https://api.voyageai.com/v1/embeddings";

/// Embedding model identifier.
pub const MODEL: &str = "voyage-3-large";

/// Expected embedding dimensionality.
pub const EXPECTED_DIM: usize = 1536;

#[derive(Deserialize)]
struct VoyageResponse {
    data: Vec<EmbeddingItem>,
}

#[derive(Deserialize)]
struct EmbeddingItem {
    embedding: Vec<f32>,
}

/// Parses a raw Voyage API JSON response string into the embedding vector.
///
/// # Errors
///
/// * JSON parse error
/// * empty `data` array
/// * dimension mismatch (not exactly [`EXPECTED_DIM`] floats)
pub fn parse_voyage_response(json: &str) -> Result<Vec<f32>> {
    let resp: VoyageResponse = serde_json::from_str(json)
        .context("failed to parse Voyage response JSON")?;
    let item = resp.data.into_iter().next()
        .ok_or_else(|| anyhow!("empty data array in Voyage response"))?;
    if item.embedding.len() != EXPECTED_DIM {
        return Err(anyhow!(
            "expected {EXPECTED_DIM} dimensions, got {}",
            item.embedding.len()
        ));
    }
    Ok(item.embedding)
}

/// Formats a Rust `&[f32]` slice as a pgvector literal: `[0.1,0.2,...]`.
pub fn to_pgvector_literal(v: &[f32]) -> String {
    let parts: Vec<String> = v.iter().map(ToString::to_string).collect();
    format!("[{}]", parts.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn voyage_1536_dim_parses() {
        let embedding: Vec<f32> = (0..1536).map(|i| i as f32 * 0.001).collect();
        let response = serde_json::json!({
            "data": [{ "embedding": embedding, "index": 0, "object": "embedding" }],
            "model": "voyage-3-large",
        });
        let v = parse_voyage_response(&response.to_string()).unwrap();
        assert_eq!(v.len(), 1536);
        assert!((v[1535] - 1.535).abs() < 1e-3);
    }

    #[test]
    fn malformed_response_errors() {
        assert!(parse_voyage_response(r#"{"error":"unauthorized"}"#).is_err());
    }

    #[test]
    fn wrong_dim_errors() {
        let response = serde_json::json!({
            "data": [{ "embedding": vec![0.1_f32; 100], "index": 0, "object": "embedding" }],
        });
        assert!(parse_voyage_response(&response.to_string()).is_err());
    }

    #[test]
    fn pgvector_literal_format() {
        assert_eq!(to_pgvector_literal(&[0.1, 0.2, 0.3]), "[0.1,0.2,0.3]");
    }
}
