//! Public library for `{{project-name}}`.
//!
//! TODO: replace this paragraph with a one-sentence summary of what the crate
//! does, who it is for, and what problem it solves.
//!
//! # Overview
//!
//! TODO: describe the main concepts and how they relate to each other.
//!
//! # Examples
//!
//! ```
//! assert_eq!({{crate_name}}::hello(), "Hello from {{project-name}}!");
//! ```
//!
//! # Feature flags
//!
//! TODO: document any `[features]` this crate exposes, or remove this section.
//!
//! # Upgrading the crate-level docs
//!
//! Once your README has real content you can replace this entire doc comment
//! with a single line to keep rustdoc and the README in sync:
//!
//! ```rust,ignore
//! #![doc = include_str!("../README.md")]
//! ```

/// Returns a starter message.
#[must_use]
pub const fn hello() -> &'static str {
    "Hello from {{project-name}}!"
}
