# rust-template

A minimal `cargo-generate` workspace whose architecture and repository tooling
grow through `cargo xtask`.

## Generate a project

```sh
cargo generate --git https://github.com/carlosferreyra/rust-template
```

The initial project contains one dependency-free primary crate and private
`tools/xtask` automation. CI, release automation, contribution guides, agent
instructions, extra crates, and a CLI are opt-in. The generated `README.md`
teaches the ownership and extraction rules for product crates.

Generation uses bundled SPDX license texts and needs no external commands or
GitHub authentication. Once the template is available locally, generation works
offline. Rust tracks `stable`, with Clippy and rustfmt declared in the toolchain file.

## Grow the workspace

```sh
cargo xtask scaffold crate workspace
cargo xtask scaffold crate resolver --private
cargo xtask scaffold cli --entrypoint companion
cargo xtask scaffold ci
cargo xtask scaffold ci --preset full
cargo xtask scaffold docs
cargo xtask scaffold agents --claude
```

Scaffolds plan all changes before writing, preserve user-owned files, support
`--dry-run`, and are idempotent. Product crates use the project prefix plus a
semantic ownership noun; they are never pre-generated as a mandatory layer
stack.

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

The registry at `tools/xtask/assets/tooling.toml` also owns the action references
used by scaffolded workflows. Full CI renders its tool pins from the same
registry used by local tool installation. CI updates Rustup and installs the
toolchain from `rust-toolchain.toml`; coverage setup adds `llvm-tools-preview`.

Maintainers can refresh stable upstream releases and regenerate the template's
own workflow with Python 3.11 or newer:

```sh
python3 scripts/update-tooling.py --latest
python3 scripts/update-tooling.py --check
```

After editing the registry or its workflow template manually, run
`python3 scripts/update-tooling.py` to regenerate CI. The offline `--check` runs
in CI to catch drift. If Dependabot changes generated CI, update the registry
and regenerate the workflow as part of that change. Generated projects keep
their bundled snapshot until explicitly updated.

## Release

```sh
cargo xtask tools sync release
cargo xtask release init
cargo xtask release plan
cargo xtask release prepare
cargo xtask release prepare minor --execute
```

`release init` writes cargo-release configuration and creates a changelog if
none exists, preserving existing release history. When the
workspace has a publishable binary, it also delegates distribution setup to
`dist init --yes`; library-only workspaces skip cargo-dist. `release plan`
delegates to `dist plan` and therefore requires a publishable binary. `release
prepare` delegates to `cargo-release` with `--no-publish`; an explicit
`--execute` is required to create release metadata.

Generated projects are snapshots and do not update automatically when this
template changes.
