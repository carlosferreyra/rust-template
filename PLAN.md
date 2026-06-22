# Rust Template Simplification Plan

## Goal

Make the generated workspace minimal and let its architecture grow explicitly.
Every new project starts with only the public library crate:

```text
crates/<project-name>/
crates/xtask/
```

The template will no longer ask whether to include a CLI or generate `-core` and
`-types` crates automatically. Users add those crates later through one
scaffolding interface:

```sh
cargo xtask add core
cargo xtask add types
cargo xtask add cli
```

`cargo xtask add cli` is the reserved-name form for creating the project's CLI
using the same high-level split as uv: `<project-name>-cli` owns the Clap command
model, while `<project-name>` owns the executable entrypoint and dispatch. All
other names create ordinary library crates.

## Phase 1: Reduce the initial scaffold

- Remove the `include_cli` cargo-generate placeholder and its pre-generation
  conditional.
- Stop generating `<project-name>-core`, `<project-name>-types`, and
  `<project-name>-cli` during initial project generation.
- Keep `<project-name>` as the public library crate and `xtask` as the local
  automation crate.
- Remove unused initial workspace dependencies and example code that assumes the
  core/types dependency chain or an existing CLI.
- Make `cargo xtask build` build and test the workspace without attempting to run
  a binary that may not exist.
- Update the post-generation instructions and template documentation to describe
  the minimal starting point and explicit crate additions.

### Acceptance criteria

- A newly generated project contains only `<project-name>` and `xtask` workspace
  members.
- No `include_cli` prompt or non-interactive variable remains.
- The initial workspace passes `cargo metadata`, `cargo xtask check`,
  `cargo xtask test`, and `cargo xtask build`.
- No generated source or documentation claims that core, types, or CLI crates
  already exist.

## Phase 2: Make `cargo xtask add` the scaffolding interface

Keep the command compact and use the crate suffix as its interface:

```text
cargo xtask add <name>
```

### Library behavior

- Any valid non-reserved name creates `crates/<project-name>-<name>` as a library
  crate.
- `cargo xtask add core` and `cargo xtask add types` use the same library path;
  they are explicit names, not automatically coupled architectural layers.
- New library crates inherit applicable workspace package fields and workspace
  lints.
- The command adds the new crate to `[workspace.dependencies]` with both `path`
  and version so other workspace crates can opt into it using
  `workspace = true`.
- The command does not automatically add dependencies between crates. The user
  decides the dependency direction explicitly.

### Reserved CLI behavior

- Reserve the exact name `cli` for the project CLI scaffold.
- `cargo xtask add cli` creates `crates/<project-name>-cli` as a library crate. It
  owns the Clap-derived parser, top-level arguments, subcommands, and CLI-only
  validation. It does not contain `main` and does not depend on the
  `<project-name>` crate.
- Register `<project-name>-cli` in `[workspace.dependencies]` with its local path
  and workspace version.
- Add `<project-name>-cli` as a dependency of the existing `<project-name>` crate.
  This dependency direction follows uv: the entrypoint package imports its CLI
  model, never the reverse.
- Add `src/bin/<project-name>.rs` to the existing `<project-name>` package. Keep
  this file deliberately thin: collect `args_os`, call the public entrypoint in
  `<project-name>::main`, and return its `ExitCode`.
- Add a public `<project-name>::main(args: impl Iterator<Item = OsString>) ->
ExitCode` entrypoint in the existing crate. It parses `<project-name>-cli`,
  initializes logging, dispatches commands, renders user-facing errors, and maps
  outcomes to process exit codes.
- Keep reusable domain behavior outside `<project-name>-cli`. Clap types describe
  the command interface; command execution remains in `<project-name>` initially
  and may later delegate to explicit crates such as `core`.
- Set `default-run = "<project-name>"` on the `<project-name>` package. Do not add
  an explicit `[[bin]]` entry for the conventional `src/bin/<project-name>.rs`
  target unless Cargo configuration requires additional target metadata.
- Keep `<project-name>-cli` unpublished as an internal interface. The existing
  `<project-name>` package remains publishable and carries the executable target,
  matching the entrypoint-package role of uv without copying uv's project-specific
  companion binaries.
- Include a minimal `--version` path and one placeholder subcommand so parsing,
  dispatch, help, and exit behavior can be tested end to end.
- Re-running the command fails without modifying files and reports that the CLI
  scaffold already exists, whether detected through the crate directory,
  workspace dependency, package dependency, or binary entrypoint.
- Do not add a second `--bin` interface in this phase; the reserved name avoids two
  ways to create the same project CLI.

### Validation and failure behavior

- Validate `<name>` before writing anything. Accept lowercase ASCII crate suffixes
  composed of letters, digits, and hyphens, beginning with a letter.
- Reject `xtask`, `cli-*`, empty names, path separators, underscores, uppercase
  letters, and names that would collide with an existing workspace package or
  directory.
- Validate all intended manifest and README changes before creating files.
- Report errors to stderr and return a non-zero exit code.
- Avoid partial scaffolds: if validation fails, the workspace remains unchanged.

### Acceptance criteria

- `cargo xtask add core` and `cargo xtask add types` create independent library
  crates and register their workspace dependencies.
- `cargo xtask add cli` creates the CLI-model crate, wires it into the existing
  entrypoint package, and builds the `<project-name>` binary successfully.
- A generated crate is immediately included by `members = ["crates/*"]` and
  passes formatting and Clippy checks.
- Invalid or duplicate requests leave the repository unchanged.

## Phase 3: Test the template and crate generator

Add generation smoke tests around the minimal template and focused tests around
`xtask add`.

- Generate a project non-interactively and assert that no unresolved Liquid
  placeholders remain.
- Assert that only the public library and xtask crates exist initially.
- Run the initial workspace check, test, and build tasks.
- In separate disposable copies, run `cargo xtask add core`, `types`, a generic
  library name, and `cli`.
- Verify the resulting directories, package targets, inherited metadata,
  workspace dependency entries, and build results.
- For the CLI case, verify the dependency direction
  `<project-name> -> <project-name>-cli`, assert that the reverse dependency is
  absent, and run the binary's help, version, placeholder-command, and invalid-
  command paths.
- Test invalid names and duplicate additions, including a before/after working
  tree comparison proving that failures are non-mutating.
- Run these smoke tests in CI on Linux. Continue using the normal workspace test
  matrix for generated Rust portability.

### Acceptance criteria

- CI detects broken template substitution, initial generation, library
  scaffolding, CLI scaffolding, and manifest registration.
- Tests cover both successful additions and non-mutating failures.
- Smoke tests do not publish crates, create repositories, or modify the template
  working tree.

## Phase 4: Align documentation and maintenance policy

- Rewrite the workspace-shape documentation around incremental growth rather than
  a mandatory façade/core/types architecture.
- Document `<project-name>` as the stable public entry point; additional crates
  have no implied role beyond the name chosen by the user.
- Describe `core` and `types` as optional conventions, not required modules.
- Document `cli` as the one reserved `xtask add` name and explain the uv-inspired
  split: `<project-name>-cli` owns Clap definitions, while `<project-name>` owns
  startup, dispatch, errors, and the executable target.
- Update README generation and trusted-publishing documentation so they discover
  actual publishable workspace members instead of assuming a fixed crate set.
- Add a concise `CONTEXT.md` recording the minimal-first rule, naming convention,
  reserved CLI behavior, explicit dependency policy, and lockstep publishing
  invariant.

### Generated README onboarding block

Add a visible, beginner-oriented section to the generated root `README.md`. Keep
it in normal Markdown so it is visible immediately on GitHub and in an editor;
do not hide the instructions inside an HTML comment.

- Wrap the section in explicit markers such as:

  ```markdown
  <!-- BEGIN: DELETE AFTER PROJECT SETUP -->
  > [!IMPORTANT]
  > **Template setup guide — delete this entire section after completing it.**
  ...
  <!-- END: DELETE AFTER PROJECT SETUP -->
  ```

- Start with a short explanation of the generated layout: the project begins with
  `crates/<project-name>` and `crates/xtask`, and grows only when the user asks
  `xtask` to add a crate.
- Provide a copy-paste first-run path:
  `cargo xtask check`, `cargo xtask test`, and `cargo xtask build`, with one-line
  explanations of what each command verifies.
- Show how to create the first ordinary library crate with
  `cargo xtask add <name>`, using `core` as an example while explaining that the
  name does not imply or create dependencies.
- Show how to create the CLI with `cargo xtask add cli`, explain the generated
  `<project-name>-cli`/`<project-name>` split, and include commands for running
  help, version, and the placeholder subcommand.
- Explain how to connect a newly added crate: add its `workspace = true`
  dependency to the consuming crate and import it from Rust. Include one compact
  manifest snippet and one compact Rust snippet.
- List the small set of files beginners commonly update first: root and package
  descriptions, crate documentation, `crates/README.md`, workspace dependencies,
  tests, and changelog.
- Call out that `cli` is a reserved `cargo xtask add` name, not an ordinary crate
  suffix. Explain that it atomically creates `<project-name>-cli`, adds the Clap
  dependency seam, and adds the executable entrypoint to `<project-name>`; users
  must choose another suffix if they want an unrelated library containing
  “cli” behavior.
- Explain that the generated project is a snapshot, not a live link to the
  template. Later template changes are not applied automatically; users should
  review the template changelog/diff and copy desired maintenance changes into
  their project deliberately.
- End with a setup checklist: replace placeholder documentation, add the intended
  crates, run checks, initialize release tooling only if needed, then delete the
  entire marked section.

### Optional crates.io publishing walkthrough

Include a clearly optional “Publish to crates.io” subsection inside the removable
onboarding block. Keep it procedural enough for a first-time publisher, while
linking to dedicated maintenance documentation for deeper release details.

- State explicitly that publishing requires a crates.io account, verified email,
  and ownership of every crate name being published. Direct the user to sign in
  through crates.io before creating a token.
- Explain that crate names are global and first-come, first-served. Have the user
  check availability for `<project-name>` and every publishable workspace member
  before relying on those names.
- Provide a metadata checklist for root `Cargo.toml` and each publishable package:
  choose a non-placeholder version, confirm description, license, repository,
  homepage, README, authors, keywords/categories, and ensure unintended internal
  crates use `publish = false`.
- State that `xtask` and the reserved `<project-name>-cli` crate are not published;
  the `<project-name>` package is the publishable crate and also carries the
  executable after CLI scaffolding.
- Require a real GitHub repository URL in `workspace.package.repository` before
  running the setup script because it derives the trusted-publisher owner and
  repository from that field.
- Explain token safety: create a short-lived crates.io token with the
  `publish-new` and `trusted-publishing` scopes required by
  `scripts/setup-crates-io-publish.py`; expose it only as
  `CARGO_REGISTRY_TOKEN` for the command, never commit it, print it, or place it
  in repository configuration.
- Show the supported first-publication sequence:

  1. Change the workspace version from `0.0.0` to the intended first real version.
  2. Run the full check, test, and package verification commands.
  3. Initialize and commit the cargo-dist `release.yml` workflow and configure
     the GitHub `release` environment expected by the setup script.
  4. Run `scripts/setup-crates-io-publish.py --dry-run` with the token to review
     every crate reservation and trusted-publisher change.
  5. Run the script without `--dry-run`; explain that it may publish `0.0.0`
     placeholders to reserve new names, registers the GitHub trusted publisher,
     enables trusted-publishing-only mode, and records configured crates in
     `.known-crates`.
  6. Commit `.known-crates` and the release configuration.
  7. Trigger the repository's documented release workflow for the first real
     version and verify the crate page and ownership on crates.io.

- Do not tell beginners to use `cargo publish` directly after enabling
  trusted-publishing-only mode. The tutorial must identify the GitHub release
  workflow as the publishing path and reserve token-based publication for the
  one-time setup script.
- Before documenting the sequence, reconcile `cargo xtask publish` with trusted
  publishing. Change it from a local `cargo release --execute` publication path
  into a release-preparation path that bumps the lockstep version, updates the
  changelog, commits, tags, and pushes; the trusted GitHub workflow must perform
  crates.io publication. Preserve a dry-run mode that shows the planned version,
  packages, tag, and workflow trigger without making changes.
- Include recovery guidance for the common first-run failures: name already
  claimed, missing package metadata, token missing required scopes, stale linked
  GitHub session, repository/workflow/environment mismatch, and an already
  published version that cannot be overwritten.
- Link to the current official crates.io publishing and trusted-publishing docs,
  and label those links as the source of truth when platform requirements differ
  from the generated tutorial.

### Acceptance criteria

- Documentation contains no fixed types → core → façade → CLI dependency graph.
- Every documented scaffolding command is exercised by a smoke test.
- Publishing utilities work from Cargo metadata and do not require core, types,
  or CLI crates to exist.
- A newly generated `README.md` visibly explains the first check, first library
  crate, first CLI, dependency wiring, and removal of the onboarding section.
- The README explicitly identifies `cli` as reserved scaffolding behavior and
  distinguishes `<project-name>-cli` from an ordinary added library.
- The onboarding commands are executed by smoke tests so copied instructions do
  not drift from the actual xtask interface.
- Removing everything between the start and end markers leaves a valid project
  README containing its title and project description.
- The optional publishing walkthrough identifies account and token requirements,
  metadata to customize, the placeholder/trusted-publisher setup, the first real
  release path, and credential-safety rules.
- A release integration test or non-publishing dry run verifies package discovery,
  release ordering, and workflow configuration without contacting crates.io.

## Phase 5: Complete the remaining audit improvements

After the scaffold simplification is stable:

- Align `cargo xtask check` with CI by checking all targets and denying warnings.
- Make the canonical test task cover nextest and doctests.
- Add CI verification for the declared MSRV and correct unsupported CI claims in
  `TEMPLATE.md`.
- Replace network-dependent license fetching with bundled license templates so
  generation works offline and cross-platform.
- Add `cargo xtask doctor` to report missing optional development tools.
- Remove the Node/Prettier README-generation dependency if deterministic output is
  preserved by tests.
- Pin the development Rust toolchain separately from the tested MSRV.

## Implementation order

1. Reduce the initial scaffold.
2. Extend `cargo xtask add`, including the reserved `cli` behavior.
3. Add smoke and failure-atomicity tests.
4. Update architecture and publishing documentation.
5. Apply the remaining CI, portability, and tooling improvements.

Each phase should be reviewed independently. The first four phases form the
minimal-first scaffolding change; Phase 5 remains follow-up maintenance work.

## Constraints

- Keep exactly one initial project crate plus `xtask`.
- Use `cargo xtask add cli`, not a generation-time Boolean or a second CLI flag.
- Do not infer dependencies for `core`, `types`, or other library names.
- Preserve the `<project-name>-<suffix>` package naming convention.
- Keep the internal `<project-name>-cli` and xtask packages unpublished; keep the
  `<project-name>` entrypoint package publishable.
- Keep privileged crates.io publishing operations explicit and opt-in.
- Avoid unrelated refactors, formatting changes, or speculative crate roles.
