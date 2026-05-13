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

Development automation. Not published. Provides `cargo xtask check`, `test`, `build`, and `add`.
