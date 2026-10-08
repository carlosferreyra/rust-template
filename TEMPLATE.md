# rust-template

A minimal `cargo-generate` workspace whose architecture and repository tooling
grow through `cargo xtask`.

## Generate a project

```sh
cargo generate --git https://github.com/carlosferreyra/rust-template --allow-commands
```

The initial project contains one dependency-free primary crate and private
`tools/xtask` automation. CI, release automation, contribution guides, agent
instructions, extra crates, and a CLI are opt-in. The generated `README.md`
teaches the ownership and extraction rules for product crates.

Each generation updates stable Rust, resolves the current stable crates.io
releases of `cargo_metadata`, `clap`, and `toml_edit`, and fetches the latest
stable GitHub release tags for the bundled actions. Source manifests and action
references use placeholders; generated projects receive concrete versions.
The optional CLI inherits the same Clap version as `xtask`.

Generation requires network access, Cargo, Rustup, and curl. `--allow-commands`
allows the hook commands; omit it to approve each command interactively.
GitHub authentication is unnecessary, and SPDX license texts remain bundled.
The hook checks the generated workspace with the current stable compiler and
creates a fresh `Cargo.lock` before completing. Failed resolution or compilation
stops generation without falling back to older versions. Future major API
changes may require template source changes before generation succeeds again.
Rust tracks `stable`, with Clippy and rustfmt declared in the toolchain file;
the generated `rust-version` records the stable compiler used during generation.

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

Each sync updates Rust's `stable` toolchain, including Clippy and rustfmt, and
installs the latest stable releases of the selected tools under `.xtask/tools`.
Use `cargo xtask tools sync all` to update every group. Tool binaries are not
installed into the user's global Cargo environment; the stable Rust toolchain
is shared through Rustup. `--locked` uses each selected release's dependency
lockfile without pinning the tool itself. Existing working local or global
tools are accepted between syncs. Sync needs network access, and new releases
can change tool behavior. Sync does not rewrite dependency requirements or the
compiler baseline recorded when the project was generated.

The registry at `tools/xtask/assets/tooling.toml` contains generation placeholders
such as `actions/checkout@v{{checkout_version}}`. The hook resolves those into
release tags, which later CI scaffolds use. Generated YAML preserves GitHub's
own `${{ ... }}` expressions. CI installs tools without version pins, using the
install action's bundled tool manifests; those can trail upstream releases.
`tools sync` uses Cargo directly to resolve current releases. CI installs the
toolchain from `rust-toolchain.toml`; coverage sync adds `llvm-tools-preview`.

The template repository's own CI must run before placeholder substitution. Its
concrete action references are maintained directly in `.github/workflows/ci.yml`.
The template's `.github/dependabot.yml` checks GitHub Actions weekly and opens
update pull requests. Cargo updates are omitted because the template manifests
contain unresolved placeholders.

Full CI scaffolds include weekly Dependabot updates for Cargo dependencies and
GitHub Actions in generated projects, where the versions are concrete. Generated
projects keep their bundled scaffold snapshot, while tool versions refresh on
every sync. No weekly version-refresh commit is needed to keep new generations
current.

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
