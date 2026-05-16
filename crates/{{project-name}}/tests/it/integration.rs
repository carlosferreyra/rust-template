//! Integration test for the façade crate.
//!
//! Lives in `tests/it/` so it consumes `{{project-name}}` as an external crate —
//! the same way a downstream user would. Uses `insta` for snapshot assertions;
//! run `cargo insta review` to update.

use {{crate_name}}::greet;

#[test]
fn greet_returns_expected_string() {
    let out = greet("world").expect("non-empty name should succeed");
    insta::assert_snapshot!(out, @"Hello, world!");
}

#[test]
fn greet_rejects_empty_name() {
    let err = greet("").expect_err("empty name should fail");
    insta::assert_snapshot!(err.to_string(), @"invalid input: name must not be empty");
}
