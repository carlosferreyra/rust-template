# rust-template

A minimal-first `cargo-generate` template for Rust workspaces, inspired by the
automation and executable separation in `astral-sh/uv`.

## Generate a project

```sh
cargo generate --git https://github.com/carlosferreyra/rust-template --allow-commands
```

The initial workspace contains only the public project library and `xtask`.
Architecture grows explicitly:

```sh
cargo xtask add core   # ordinary library crate
cargo xtask add types  # ordinary library crate
cargo xtask add cli    # reserved uv-style CLI scaffold
```

`core` and `types` are conventions, not mandatory layers. Ordinary additions do
not create dependencies. The reserved `cli` addition creates an unpublished
Clap-model library and makes the public crate the executable entrypoint:

```text
<project-name> -> <project-name>-cli
```

## Development

```sh
cargo xtask check
cargo xtask test
cargo xtask build
cargo xtask coverage
```

Checks cover formatting, all workspace targets, and warning-free Clippy. Tests
use nextest and also run doctests.

## Publishing

Publishable workspace crates share one version. Run
`scripts/setup-crates-io-publish.py` once to reserve names and configure the
`release.yml` trusted publisher. Thereafter `cargo xtask publish` prepares the
release without publishing locally; the trusted GitHub workflow publishes it.

See the generated README tutorial for the account, token, metadata, and first
release checklist.

## Maintainer documentation

- `CONTEXT.md` defines load-bearing template invariants.
- `PLAN.md` records the approved improvement roadmap and is not generated.
- Generated projects are snapshots; the template does not update them in place.
