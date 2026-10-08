# Template context

This context describes the generated Rust workspace and the template's
minimal-first growth model.

## Domain language

- **Primary crate:** the package named by
  `workspace.metadata.xtask.primary-crate`; it owns the shipped artifact or
  main Rust interface.
- **Crate prefix:** the product-package prefix named by
  `workspace.metadata.xtask.crate-prefix`.
- **Semantic crate:** a product crate named for one stable domain or operational
  responsibility, such as `workspace`, `resolver`, or `client`.
- **Scaffold:** an explicit, idempotent repository mutation performed by
  `cargo xtask scaffold`.
- **Capability:** optional project surface such as a CLI, CI, documentation, or
  release automation.
- **Tool group:** latest stable external binaries installed project-locally by
  `cargo xtask tools sync`.
- **Operational command:** a repeatable command such as `check`, `test`, `ci`,
  or `release plan`; it never installs tools implicitly.
- **Development command:** repository automation reached through `cargo xtask`;
  it is not a product crate.

## Invariants

- Generated projects initially contain only the primary crate and `xtask`.
- The initial primary crate has no third-party dependencies.
- Product crates live under `crates/` and are named `<prefix>` or
  `<prefix>-<semantic-name>`.
- `tools/xtask` owns development commands and is not a product crate.
- Optional repository files appear only after their scaffold is requested.
- Scaffolds validate every intended change before writing.
- Scaffolds never overwrite unmarked user-owned files.
- Adding the CLI never overwrites the primary library source.
- Cross-crate dependencies use a root workspace declaration and a member
  `{ workspace = true }` reference.
- External dependency versions and action release tags resolve at generation
  time; source placeholders become concrete versions in generated projects.
- Generation requires network access and command permission, and checks the
  workspace with current stable Rust before completing.
- `xtask` resolves the workspace from any descendant directory.
- Optional tools resolve the latest stable release on sync and install under `.xtask/tools`.
- Release commands delegate to dist and cargo-release.
- Generated projects are snapshots and do not track template updates.
