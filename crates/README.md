# Crates

## [{{project_name}}](./{{project-name}})

Public API entrypoint. The crate downstream consumers add to their `Cargo.toml`.

## [{{project_name}}-types](./{{project-name}}-types)

Shared type definitions. No crate that `{{project_name}}-types` depends on may itself depend
on `{{project_name}}-types` — this crate is the foundation of the dependency graph.

## [{{project_name}}-core](./{{project-name}}-core)

Core business logic. No I/O, no external service dependencies.
{% if include_cli %}

## [{{project_name}}-cli](./{{project-name}}-cli)

Command-line interface. A thin binary that delegates to `{{project_name}}`.
{% endif %}

## [xtask](./xtask)

Development automation. Not published. Provides `cargo xtask check`, `test`, `build`, and `add`.
