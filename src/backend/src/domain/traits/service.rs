//! Base service trait — every business-logic unit implements this.

/// Marker trait for all service types.
///
/// Services coordinate one or more repositories and enforce business rules.
/// They never touch HTTP — that is the route handler's job.
pub trait Service: Send + Sync {}
