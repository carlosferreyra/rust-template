# rust-template

A minimal `cargo-generate` workspace whose architecture and repository tooling
grow through `cargo xtask`.

## Generate a project

```sh
cargo generate --git https://github.com/carlosferreyra/rust-template --allow-commands
```

The initial project contains one dependency-free public crate, `xtask`, and only
the files needed to build them. CI, release automation, contribution guides,
agent instructions, extra crates, and a CLI are opt-in.

## Grow the workspace

```sh
cargo xtask scaffold crate core
cargo xtask scaffold crate worker --bin --private
cargo xtask scaffold cli
cargo xtask scaffold ci
cargo xtask scaffold ci --preset full
cargo xtask scaffold docs
cargo xtask scaffold agents --claude
```

Scaffolds plan all changes before writing, preserve user-owned files, support
`--dry-run`, and are idempotent. `core` and `types` remain ordinary names rather
than mandatory architecture layers.

## Develop

```sh
cargo xtask check
cargo xtask test
cargo xtask test parser
cargo xtask build
cargo xtask ci
```

The built-in path uses Cargo and Rustup only. Optional commands report missing
tools instead of installing software implicitly.

## Optional tools

```sh
cargo xtask doctor
cargo xtask tools sync test
cargo xtask tools sync coverage
cargo xtask tools sync ci
cargo xtask tools sync release
```

Tools are pinned in one registry and installed under `.xtask/tools`, never into
the user's global Cargo environment.

## Release

```sh
cargo xtask tools sync release
cargo xtask release init
cargo xtask release plan
cargo xtask release prepare
cargo xtask release prepare minor --execute
```

`release init` always writes the cargo-release/changelog files. When the
workspace has a publishable binary, it also delegates distribution setup to
`dist init --yes`; library-only workspaces skip cargo-dist. `release plan`
delegates to `dist plan` and therefore requires a publishable binary. `release
prepare` delegates to `cargo-release` with `--no-publish`; an explicit
`--execute` is required to create release metadata.

Generated projects are snapshots and do not update automatically when this
template changes.
