//! Top-level façade for `{{project-name}}`.
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
//! use {{crate_name}}::greet;
//!
//! let msg = greet("world").unwrap();
//! assert_eq!(msg, "Hello, world!");
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

pub use {{crate_name}}_core as core;
pub use {{crate_name}}_types as types;

pub use {{crate_name}}_core::greet;
pub use {{crate_name}}_types::{Error, Result};
