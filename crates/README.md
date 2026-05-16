# Crates

## [{{project-name}}](./{{project-name}})

Public API entrypoint. The crate downstream consumers add to their `Cargo.toml`.

## [{{project-name}}-types](./{{project-name}}-types)

Shared type definitions. No crate that `{{project-name}}-types` depends on may itself depend
on `{{project-name}}-types` — this crate is the foundation of the dependency graph.

## [{{project-name}}-core](./{{project-name}}-core)

Core business logic. No I/O, no external service dependencies.
{% if include_cli %}

## [{{project-name}}-cli](./{{project-name}}-cli)

Command-line interface. A thin binary that delegates to `{{project-name}}`.
{% endif %}

## [xtask](./xtask)

Development automation. Not published. Provides:

| Command                           | What it does                                         |
| --------------------------------- | ---------------------------------------------------- |
| `cargo xtask check`               | fmt -> check -> clippy                               |
| `cargo xtask test [filter]`       | check -> full test suite (scoped if filter set)      |
| `cargo xtask build`               | test -> release build -> smoke-run the CLI           |
| `cargo xtask add <name>`          | Scaffold `crates/{{project-name}}-<name>/`           |
| `cargo xtask coverage`            | HTML coverage report via `cargo-llvm-cov`            |
| `cargo xtask publish [--execute]` | Workspace version bump + publish via `cargo-release` |
