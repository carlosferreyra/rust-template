# Template context

## Domain language

- **Public crate:** the dependency-free crate named by
  `workspace.metadata.xtask.public-crate`.
- **Scaffold:** an explicit, idempotent repository mutation performed by
  `cargo xtask scaffold`.
- **Capability:** optional project surface such as a CLI, CI, documentation, or
  release automation.
- **Tool group:** pinned external binaries installed project-locally by
  `cargo xtask tools sync`.
- **Operational command:** a repeatable command such as `check`, `test`, `ci`,
  or `release plan`; it never installs tools implicitly.

## Invariants

- Generated projects initially contain only the public crate and `xtask`.
- The initial public crate has no third-party dependencies.
- Optional repository files appear only after their scaffold is requested.
- Scaffolds validate every intended change before writing.
- Scaffolds never overwrite unmarked user-owned files.
- Adding the CLI never overwrites the public library.
- Added path-dependency versions match `workspace.package.version`.
- `xtask` resolves the workspace from any descendant directory.
- Optional tools are pinned centrally and installed under `.xtask/tools`.
- Release commands delegate to dist and cargo-release.
- Generated projects are snapshots and do not track template updates.
