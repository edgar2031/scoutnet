//! Cosine similarity candidate query via pgvector.

use anyhow::Result;
use sqlx::{PgPool, Row};
use uuid::Uuid;

/// Cosine-distance threshold: profiles closer than this are considered candidates.
pub const COSINE_THRESHOLD: f64 = 0.35;

/// Max candidates returned by [`find_candidates`].
pub const MAX_CANDIDATES: i64 = 500;

/// A profile identified as a match candidate, with its cosine distance.
pub struct Candidate {
    /// User whose profile matched.
    pub user_id: Uuid,
    /// Cosine distance from query embedding (lower = closer).
    pub distance: f64,
}

/// Formats a `&[f32]` as a pgvector literal: `[0.1,0.2,...]`.
pub fn to_pgvector_literal(v: &[f32]) -> String {
    let parts: Vec<String> = v.iter().map(ToString::to_string).collect();
    format!("[{}]", parts.join(","))
}

/// Finds user profiles whose cosine distance to `embedding` is below
/// [`COSINE_THRESHOLD`], sorted ascending, up to [`MAX_CANDIDATES`].
///
/// # Errors
///
/// Returns the underlying sqlx error.
pub async fn find_candidates(pool: &PgPool, embedding: &[f32]) -> Result<Vec<Candidate>> {
    let vec_literal = to_pgvector_literal(embedding);

    let rows = sqlx::query(
        r#"
        SELECT user_id,
               (embedding <=> $1::vector)::float8 AS distance
        FROM user_profiles
        WHERE embedding IS NOT NULL
          AND (embedding <=> $1::vector) < $2
        ORDER BY distance ASC
        LIMIT $3
        "#,
    )
    .bind(&vec_literal)
    .bind(COSINE_THRESHOLD)
    .bind(MAX_CANDIDATES)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter()
        .map(|r| Candidate {
            user_id:  r.get::<Uuid, _>("user_id"),
            distance: r.get::<Option<f64>, _>("distance").unwrap_or(1.0),
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pgvector_literal_format() {
        assert_eq!(to_pgvector_literal(&[0.1, 0.2, 0.3]), "[0.1,0.2,0.3]");
    }

    #[test]
    fn constants() {
        assert!((COSINE_THRESHOLD - 0.35).abs() < f64::EPSILON);
        assert_eq!(MAX_CANDIDATES, 500);
    }
}
