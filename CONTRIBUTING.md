# Contributing to {{project-name}}

## Development workflow

```sh
cargo xtask check          # after every meaningful edit
cargo xtask test           # before committing
cargo xtask test <filter>  # scoped test run
cargo xtask build          # before pushing
```

## Adding a new crate

```sh
cargo xtask add <name>
```

Creates `crates/{{project-name}}-<name>/` with a `Cargo.toml` and `src/lib.rs`, inheriting all
workspace fields, and appends a stub section to `crates/README.md`.

The exact name `cli` is reserved. `cargo xtask add cli` creates the unpublished
`{{project-name}}-cli` Clap model and adds the executable entrypoint to the public
`{{project-name}}` crate.

## Commit conventions

Commits must follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>
```

Common types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `ci`.
Scope is the crate name without the `{{project-name}}-` prefix.

Examples:

```
feat(core): add initial processing logic
fix(types): correct field ordering in Foo struct
docs: update README with usage examples
chore(deps): bump xflags to 0.4
```

## Releasing

```sh
cargo xtask publish --execute --level patch
```

This prepares the version, changelog, commit, and tag without publishing locally.
The trusted GitHub release workflow publishes crates and release artifacts.
