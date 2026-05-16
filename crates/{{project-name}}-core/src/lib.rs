//! Core business logic for `{{project-name}}`.
//!
//! No I/O, no external service dependencies — keep this crate pure so it stays
//! easy to test, bench, and reason about.
//!
//! TODO: describe the core domain logic this crate owns.
//!
//! # Examples
//!
//! ```
//! use {{crate_name}}_core::greet;
//!
//! let msg = greet("world").unwrap();
//! assert_eq!(msg, "Hello, world!");
//! ```

use {{crate_name}}_types::{Error, Result};

/// Returns a greeting for `name`.
///
/// # Errors
///
/// Returns [`Error::Invalid`] if `name` is empty.
pub fn greet(name: &str) -> Result<String> {
    if name.is_empty() {
        return Err(Error::Invalid("name must not be empty".into()));
    }
    tracing::debug!(name, "building greeting");
    Ok(format!("Hello, {name}!"))
}
