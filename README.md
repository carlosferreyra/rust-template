# {{project-name}}

> {{project_description}}

<!-- BEGIN: DELETE AFTER PROJECT SETUP -->

> [!IMPORTANT]
> **Template setup guide — delete this entire marked section after setup.**

## Start here

This project begins deliberately small:

```text
crates/{{project-name}}/  # public library and, optionally, executable entrypoint
crates/xtask/             # development automation; never published
```

Verify the generated project:

```sh
cargo xtask check  # formatting, compilation, and Clippy
cargo xtask test   # checks plus the test suite
cargo xtask build  # tests plus release builds
```

## Add your first crate

Add an ordinary library with a suffix such as `core`:

```sh
cargo xtask add core
```

This creates `crates/{{project-name}}-core` and registers it under
`[workspace.dependencies]`. The name does not create a dependency. Add one only
to a crate that consumes it:

```toml
[dependencies]
{{project-name}}-core = { workspace = true }
```

```rust
use {{crate_name}}_core::hello;
```

## Add a command-line interface

`cli` is a reserved xtask name:

```sh
cargo xtask add cli
cargo run -- --help
cargo run -- --version
cargo run -- hello world
```

This follows uv's separation: `{{project-name}}-cli` owns the Clap argument and
subcommand model, while `{{project-name}}` owns startup, dispatch, errors, and
the executable. Use another suffix if you want an unrelated library rather than
this CLI scaffold.

## Customize the project

Replace placeholder descriptions and crate documentation, update
`crates/README.md`, add tests, and review the root workspace dependencies. This
generated repository is a snapshot: later template updates are not applied
automatically. Review the template changelog and copy changes you want.

## Optional: publish to crates.io

You need a crates.io account with a verified email, and every crate name must be
available or already owned by you. Crate names are global and first-come,
first-served.

1. Review `Cargo.toml`: use a real version instead of `0.0.0`, and confirm the
   description, license, repository, homepage, README, authors, keywords, and
   categories. Mark anything private with `publish = false`. `xtask` and the
   reserved `{{project-name}}-cli` crate are already private; `{{project-name}}`
   is the publishable entrypoint package.
2. Create the GitHub repository named in `workspace.package.repository`.
3. Initialize cargo-dist so `.github/workflows/release.yml` exists, and create
   the GitHub `release` environment expected by trusted publishing.
4. Create a short-lived crates.io token with `publish-new` and
   `trusted-publishing` scopes. Never commit or print it.
5. Preview setup, then apply it:

   ```sh
   CARGO_REGISTRY_TOKEN=... uv run scripts/setup-crates-io-publish.py --dry-run
   CARGO_REGISTRY_TOKEN=... uv run scripts/setup-crates-io-publish.py
   ```

   The script reserves new names with placeholder releases, registers the
   trusted GitHub publisher, enables trusted-publishing-only mode, and updates
   `.known-crates`. Commit `.known-crates`.
6. Run the documented release preparation command and let the trusted GitHub
   workflow publish the first real version. Do not run `cargo publish` after
   trusted-publishing-only mode is enabled.

Common failures mean the name is already claimed, metadata is incomplete, the
token lacks a scope, the linked GitHub session is stale, or the configured
repository/workflow/environment does not match. Published versions cannot be
overwritten. Treat the
[Cargo publishing guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
and the trusted-publishing instructions in your crates.io account as the source
of truth.

When the project is configured, replace the remaining TODO documentation and
delete everything between the `BEGIN` and `END` markers.

<!-- END: DELETE AFTER PROJECT SETUP -->
