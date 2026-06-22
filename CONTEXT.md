# Template context

## Domain language

- **Public crate:** `crates/<project-name>`, the only project crate generated at
  initialization. It is publishable and may later carry the executable.
- **Added crate:** a library created explicitly by `cargo xtask add <suffix>`.
  Its name implies no dependency direction.
- **CLI model:** the unpublished `<project-name>-cli` library created by the
  reserved `cargo xtask add cli` command. It owns Clap types and validation.
- **Entrypoint:** the thin binary and dispatch function in the public crate. It
  depends on the CLI model, following the uv repository's separation.

## Invariants

- New projects contain only the public crate and `xtask`.
- `core` and `types` are ordinary, optional suffixes.
- Dependencies between added crates are always explicit.
- Publishable crates share one workspace version and release in lockstep.
- `xtask` and the CLI model are never published.
- Trusted publishing expects `.github/workflows/release.yml` and the GitHub
  `release` environment.
- Privileged crates.io setup is explicit and opt-in.
- Generated projects are snapshots and do not automatically track this template.
