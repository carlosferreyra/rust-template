# Remaining Template Work

The minimal-first scaffold and `cargo xtask add` workflow are implemented. Keep
this file focused only on work that is still incomplete or under-tested.

## Smoke-Test Coverage Gaps

- Extend `scripts/test-template.sh` to exercise `cargo xtask add types` and one
  generic library suffix, not only `core` and `cli`.
- For the CLI scaffold, assert the generated dependency direction:
  `<project-name> -> <project-name>-cli`, with no reverse dependency.
- Exercise the CLI help and invalid-command paths in the smoke test, in addition
  to version and the placeholder `hello` subcommand.
- Assert generated crate metadata and workspace dependency entries for added
  crates, including inherited workspace fields and `publish = false` for the CLI
  model.
- Add a smoke assertion that removing the generated README onboarding block
  leaves a valid README with the project title and description.

## Release And Publishing Verification

- Add a non-publishing release/setup verification path that checks package
  discovery, release ordering, and expected workflow/environment configuration
  without contacting crates.io.
- Keep `cargo xtask publish` aligned with trusted publishing: it should prepare
  changelog, commit, tag, and push metadata without locally publishing crates.

## Portability And Tooling Follow-Ups

- Replace network-dependent license fetching in `hooks/post-generate.rhai` with
  bundled license templates so generation works offline and without `gh`/`uv`.
- Add `cargo xtask doctor` to report missing optional development tools.
- Remove the Node/Prettier dependency from README generation if deterministic
  Markdown output can be preserved by tests.

## Atomicity Hardening

- Harden `cargo xtask add` so all intended manifest and README edits are fully
  validated before any filesystem writes occur.
- Expand non-mutating failure tests beyond duplicate CLI and invalid uppercase
  names to cover workspace dependency collisions, existing directories, reserved
  names, and partially present CLI scaffold markers.
