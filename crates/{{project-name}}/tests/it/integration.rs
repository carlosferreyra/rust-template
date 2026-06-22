//! Integration test for the public crate.
//!
//! Lives in `tests/it/` so it consumes `{{project-name}}` as an external crate —
//! the same way a downstream user would. Uses `insta` for snapshot assertions;
//! run `cargo insta review` to update.

#[test]
fn hello_returns_expected_string() {
    insta::assert_snapshot!({{crate_name}}::hello(), @"Hello from {{project-name}}!");
}
