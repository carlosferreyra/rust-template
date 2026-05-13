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
cargo release patch --execute
```

This bumps the version, updates `CHANGELOG.md` via git-cliff, commits, tags, and pushes.
cargo-dist picks up the tag and publishes GitHub Release artifacts automatically.
