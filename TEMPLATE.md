# rust-template

> A [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) template for new Rust
> workspace projects — uv-inspired structure, enterprise-grade CI, lockstep crates.io releases,
> and an `xtask` automation pyramid wired up out of the box.

## Usage

Interactive:

```sh
cargo generate --git https://github.com/carlosferreyra/rust-template --allow-commands
```

Non-interactive (every prompt has a `-d key=value` flag):

```sh
cargo generate --git https://github.com/carlosferreyra/rust-template --allow-commands \
  --name my-project \
  -d github_username=my-user \
  -d author_name="Your Name" \
  -d author_email=you@example.com \
  -d project_description="One-line description." \
  -d include_cli=true
```

| Prompt / flag         | Notes                                                  |
| --------------------- | ------------------------------------------------------ |
| `--name`              | Workspace name and prefix for all crates               |
| `github_username`     | Used in `repository`/`homepage` URLs                   |
| `author_name`         | `[workspace.package].authors`                          |
| `author_email`        | `[workspace.package].authors`                          |
| `project_description` | `[workspace.package].description` (optional)           |
| `include_cli`         | `true` → also generates `crates/{{project-name}}-cli/` |

---

## Prerequisites

```sh
cargo install cargo-generate    # the template runner
cargo install cargo-dist        # cross-platform binary releases
cargo install git-cliff         # changelog from conventional commits
cargo install cargo-release     # workspace version bump + publish

# Optional (used by xtask subcommands and local CI parity):
cargo install cargo-nextest cargo-llvm-cov cargo-deny cargo-msrv typos-cli
```

---

## What the post-generate hook does

1. Initializes git and commits the scaffold.
2. Prints next-step commands. **`cargo dist init -y`, `git-cliff --init`, and
   `gh repo create` are documented but not auto-run** — the user runs them once
   they're happy with the scaffold.

`cliff.toml`, `dist-workspace.toml`, and `release.yml` are generated from the
versions installed on your machine — never pinned to this template's age.

---

## Workspace shape

Library-first, modeled after the [uv repository](https://github.com/astral-sh/uv/tree/main/crates):

| Crate                    | Role                                                              |
| ------------------------ | ----------------------------------------------------------------- |
| `{{project-name}}`       | Public API façade — what downstream consumers add to `Cargo.toml` |
| `{{project-name}}-core`  | Core business logic; no I/O, no external service deps             |
| `{{project-name}}-types` | Shared types + the `Error` enum (foundation of the dep graph)     |
| `{{project-name}}-cli`   | CLI binary; thin layer over the façade _(optional)_               |
| `xtask`                  | Dev automation; not published _(unprefixed by convention)_        |

Dependency order:

```
types  →  core  →  façade  →  cli (optional)
```

### Publishing model — workspace lockstep

All library crates share a single version and publish together via `cargo release`.
`{{project-name}}-cli` is `publish = false` (binaries ship via `cargo-dist`); `xtask`
is `publish = false` (dev-only). There is no per-crate publish toggle — partial
releases would break the path/version coupling between crates.

---

## Development cycle (xtask pyramid)

```sh
cargo xtask check          # fmt → check → clippy
cargo xtask test [filter]  # check → test
cargo xtask build          # test → release build → smoke-run the CLI
cargo xtask add <name>     # scaffold crates/{{project-name}}-<name>/
cargo xtask coverage       # cargo-llvm-cov HTML report
cargo xtask publish        # workspace dry-run via cargo-release
cargo xtask publish --execute --level minor
```

Each of the first three is a strict superset of the previous — run `check`
after edits, `test` before committing, `build` before pushing.

---

## Quality gates (what CI enforces)

| Job      | Tool / file                  | Notes                                                |
| -------- | ---------------------------- | ---------------------------------------------------- |
| `format` | `cargo fmt --all --check`    | `rustfmt.toml` is workspace-scoped                   |
| `clippy` | `-D warnings`                | Pedantic + restrictions via `[workspace.lints]`      |
| `test`   | `cargo-nextest`              | Matrix: ubuntu/macos/windows; `.config/nextest.toml` |
| `msrv`   | `cargo-msrv verify`          | Reads `rust-version` from `[workspace.package]`      |
| `docs`   | `RUSTDOCFLAGS=-D warnings`   | Broken intra-doc links fail the build                |
| `deny`   | `cargo-deny check`           | Advisories, bans, licenses, sources — `deny.toml`    |
| `typos`  | `crate-ci/typos`             | `typos.toml`                                         |
| `semver` | `cargo-semver-checks` (PRs)  | Library crates only                                  |
| `audit`  | `rustsec/audit-check` (cron) | Separate `audit.yml` — daily + on lockfile change    |

Dependabot bumps cargo and actions weekly.

---

## Errors, observability, and dependencies

- **Errors:** library crates use `thiserror`-derived enums (see
  `crates/{{project-name}}-types/src/lib.rs`). The CLI converts to `anyhow` at the
  boundary with `.context(...)` so users get a chain on failure.
- **Tracing:** library code emits `tracing` events only; the binary owns the
  subscriber (see `crates/{{project-name}}-cli/src/logging.rs`). `RUST_LOG`
  controls the filter; output is JSON when stderr is not a TTY.
- **Shared deps:** `[workspace.dependencies]` in the root `Cargo.toml` declares
  every external dep with a major-only version range. Inter-workspace path
  deps live there too — `{ path = "...", version = "0.0.0" }` — so cargo-deny
  doesn't flag them as wildcards and `cargo release` flips them to crates.io
  versions on publish. Per-crate `Cargo.toml` files inherit with
  `dep.workspace = true`.

---

## Release flow (end-to-end)

```sh
cargo xtask publish --execute --level patch
# equivalent to:
cargo release patch --workspace --execute
```

1. Bumps the workspace version.
2. Updates `CHANGELOG.md` via git-cliff (`pre-release-hook` in `release.toml`).
3. Commits `chore(release): vX.Y.Z` and pushes a `vX.Y.Z` tag.
4. The tag triggers `.github/workflows/release.yml` (cargo-dist).
5. cargo-dist builds cross-platform binaries and attaches them to the release.
6. cargo-release publishes the library crates to crates.io in lockstep.

---

## Template variables (`cargo-generate.toml`)

| Variable              | Type   | Default    |
| --------------------- | ------ | ---------- |
| `project-name`        | string | (required) |
| `github_username`     | string | (required) |
| `author_name`         | string | (required) |
| `author_email`        | string | (required) |
| `project_description` | string | `""`       |
| `include_cli`         | bool   | `true`     |

---

## License

MIT © [Carlos Ferreyra](https://github.com/carlosferreyra)
