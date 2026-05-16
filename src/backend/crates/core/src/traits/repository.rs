//! Base repository trait — every DB accessor implements this.

use crate::AppError;

/// Marker trait for all repository types.
///
/// Repositories own all SQL queries for a single domain entity.
/// Business logic lives in services — repositories only query.
pub trait Repository: Send + Sync {}

/// Generic CRUD operations for a domain entity.
///
/// Implement this for any entity that has standard create/read/update/delete.
pub trait Crud<Entity, Id>: Repository {
    /// Insert a new entity and return it with generated fields (id, created_at).
    ///
    /// # Errors
    ///
    /// * [`AppError::Conflict`] — unique constraint violated
    /// * [`AppError::Database`] — other database error
    fn create(
        &self,
        entity: Entity,
    ) -> impl std::future::Future<Output = Result<Entity, AppError>> + Send;

    /// Fetch a single entity by primary key.
    ///
    /// # Errors
    ///
    /// * [`AppError::NotFound`] — no row with this id
    /// * [`AppError::Database`] — database error
    fn find_by_id(
        &self,
        id: Id,
    ) -> impl std::future::Future<Output = Result<Entity, AppError>> + Send;

    /// Persist changes to an existing entity.
    ///
    /// # Errors
    ///
    /// * [`AppError::NotFound`] — entity does not exist
    /// * [`AppError::Database`] — database error
    fn update(
        &self,
        entity: Entity,
    ) -> impl std::future::Future<Output = Result<Entity, AppError>> + Send;

    /// Remove an entity by primary key.
    ///
    /// # Errors
    ///
    /// * [`AppError::Database`] — database error
    fn delete(
        &self,
        id: Id,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;
}
