//! Shared type definitions for `{{project-name}}`.
//!
//! This crate is the foundation of the dependency graph — nothing it depends on
//! may depend back on it. Library errors live here as `thiserror`-derived enums
//! so downstream crates can match on variants without a dependency on `anyhow`.
//!
//! TODO: document the key types this crate owns and any invariants callers must
//! uphold.
//!
//! # Error handling
//!
//! All public functions in this workspace return [`Result<T>`], which aliases
//! `std::result::Result<T, Error>`. Match on [`enum@Error`] variants to handle
//! specific failure modes; convert to `anyhow::Error` at binary boundaries
//! with `.context(...)` for user-facing messages.
//!
//! # Examples
//!
//! ```
//! use {{crate_name}}_types::{Error, Result};
//!
//! fn fallible(input: &str) -> Result<&str> {
//!     if input.is_empty() {
//!         return Err(Error::Invalid("input must not be empty".into()));
//!     }
//!     Ok(input)
//! }
//!
//! assert!(fallible("hello").is_ok());
//! assert!(fallible("").is_err());
//! ```

use thiserror::Error;

/// Top-level error type for `{{project-name}}`.
///
/// Library code returns `Result<T, Error>`. Binary code (the CLI) converts to
/// `anyhow::Error` at the boundary for ergonomic reporting.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// Placeholder variant. Replace with your real error cases.
    #[error("invalid input: {0}")]
    Invalid(String),
}

/// Convenient alias for crate-level results.
pub type Result<T> = std::result::Result<T, Error>;
