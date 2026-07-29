# Remaining Template Work

The minimal generated surface, Clap-based `xtask`, capability scaffolds,
project-local tool groups, and release wrappers are implemented.

## Verification follow-ups

- Add `cargo xtask release init` to automated CI using the pinned dist version;
  the isolated local verification already covers its generated files.
- Add Windows smoke coverage for project-local executable resolution.
- Verify all pinned optional-tool versions during scheduled template CI.

## Portability follow-up

- Replace network-dependent license fetching in `hooks/post-generate.rhai` with
  a compact offline license strategy that does not add every license body to
  each generated project.

## Atomicity follow-up

- Stage every file in a scaffold plan before committing cross-file writes, then
  roll back completed renames if a later rename fails.
