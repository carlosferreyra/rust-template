# Rust template refactor plan

Status: implemented on `codex/refactor`; this remains the decision record and
acceptance checklist.

## Baseline

This plan compares:

- `carlosferreyra/rust-template` at `579655c57e4a41e64e5dc78fc7cfaab40f9ffb53`.
- `astral-sh/uv` at `9f928602938ac5cf1cd6b294a725833c16f5720e`
  (`0.12.9`, inspected on 2026-09-01).

Both revisions were checked against their current GitHub `main` branches before
writing this plan. The local template worktree was clean. The existing
`codex/refactor` branch must be preserved.

## Executive recommendation

Keep the template minimal and keep the `cargo xtask` interface, but refactor it
around four rules:

1. Reserve `crates/` for product modules named with the project prefix and a
   concrete ownership noun: `<project>-resolver`, `<project>-workspace`,
   `<project>-client`, and so on.
2. Do not pre-generate speculative crates. Start with one product crate and
   extract a crate only when an internal module has earned a real seam.
3. Make the generated `README.md` the first-run architecture guide. The current
   detailed guidance is in `TEMPLATE.md`, which generated projects do not
   receive.
4. Deepen the workspace and scaffold modules inside `xtask`, while keeping their
   external command interface small and stable.

The important correction is that uv does **not** use vague abstract names. Its
crates name concrete responsibilities such as `uv-resolver`, `uv-installer`,
`uv-workspace`, `uv-client`, and `uv-cache`. It has no `uv-core`, `uv-common`, or
`uv-utils`. The reusable abstraction is the naming discipline, dependency
direction, and locality—not uv's current count of 70 workspace packages.

## Interpretation and scope

This plan interprets “replicate uv's repository structure” as:

- use a project prefix consistently;
- name crates after stable domain or operational responsibilities;
- keep the entry/composition crate allowed to have high fan-out;
- separate command parsing, domain behavior, adapters, development tooling, and
  reusable test support only when they are genuinely independent modules;
- centralize workspace package metadata and shared dependency versions;
- document the crate graph and its growth rules.

It does **not** mean generating uv's crate list, copying Python-package-manager
names into unrelated projects, or creating a generic layered architecture.

## What uv actually demonstrates

| uv evidence | Reusable lesson | Do not copy literally |
| --- | --- | --- |
| Root `Cargo.toml` uses `members = ["crates/*"]` and central `[workspace.dependencies]` entries for internal crates. | Give internal packages one canonical path/version declaration and use `{ workspace = true }` at call sites. | Do not centralize a dependency used only by repository tooling merely for visual uniformity. |
| `crates/uv` is the shipped entry/composition package and depends on most product crates. | A composition root may have high fan-out; that is where assembly belongs. | Do not force a public library to absorb CLI dependencies just to resemble uv. uv's Rust interface is explicitly not a stable public interface. |
| `crates/uv-cli` owns the command model. | Keep parsing and command vocabulary separate from implementation behavior. | Do not create one crate per subcommand. |
| `crates/uv-dispatch` centralizes cross-module orchestration and documents that it avoids cycles among build, resolver, and installer modules. | Introduce an orchestration module only when coordination is substantial and improves locality. | Do not generate `<project>-dispatch` in every new project. |
| `crates/uv-types` contains fundamental shared types and traits used to avoid cycles. | A shared-contract module can be legitimate when multiple modules need the same interface. | Do not use `types` as a miscellaneous data-structure drawer. |
| `crates/uv-dev` is private and exposed through `.cargo/config.toml` as `cargo dev`. | Repository automation should be private and available behind a short Cargo alias. | The template can retain the conventional `cargo xtask`; it need not rename the package to `<project>-dev`. |
| `crates/uv-test` contains reusable integration-test infrastructure, while end-to-end suites live under `crates/uv/tests/<area>`. | Extract test support only after fixtures and harness behavior are reused. Test the shipped interface at the composition package. | Do not create a test-support crate for one helper. |
| `crates/README.md` explains the ownership of important crates. | Keep a human-readable crate index, not just a directory listing. | Do not require users to reverse-engineer ownership from Cargo manifests. |
| `CONTRIBUTING.md` documents targeted test, formatting, lint, generated-file, and development commands. | The repository should provide one discoverable operational path. | Do not copy uv's Python, release, or platform commands into a generic Rust template. |

At the inspected revision, uv has 70 workspace packages, of which 64 are
library-only and two are private. That scale is evidence of years of domain
pressure, not a desirable generated starting point.

## Current template friction

1. The generated `README.md` is only a title, description, and a pointer to
   `cargo xtask --help`. The useful guidance in `TEMPLATE.md` is excluded by
   `cargo-generate.toml`.
2. The examples `core` and `worker` imply generic layers rather than ownership.
3. `cargo xtask scaffold crate <suffix>` produces the correct
   `<project>-<suffix>` shape, but its help and generated files do not explain
   how to choose a meaningful suffix.
4. `cargo xtask scaffold cli` adds Clap and `<project>-cli` as normal
   dependencies of the public library package. Library consumers therefore pay
   for an interface they did not request.
5. `[workspace.metadata.xtask]` models exactly one `public-crate`. That name
   assumes a library product and is too narrow for an uv-like application
   composition root.
6. `Workspace::has_publishable_binary` discovers packages by scanning
   `crates/*`, rather than using Cargo's workspace model.
7. `scaffold.rs` is 674 lines and combines five scaffold families, TOML edits,
   file planning, crate indexing, and large embedded templates. Its command
   interface is small, but its implementation has poor locality.
8. The `Plan` implementation is a useful deep module—dry-run, idempotence,
   generated-file ownership, and preflight checks are concentrated there—but it
   is buried inside `scaffold.rs` and has no focused tests.
9. `xtask` does not inherit workspace lints.
10. The only end-to-end verification is one Ubuntu shell script; there are no
    focused tests for workspace discovery, scaffold planning, rendered README
    guidance, or CLI dependency isolation.

## Target generated layout

Keep the default product surface minimal:

```text
<project>/
├── Cargo.toml
├── README.md
├── .cargo/config.toml
├── crates/
│   └── <project>/
│       ├── Cargo.toml
│       └── src/lib.rs
└── tools/
    └── xtask/
        ├── Cargo.toml
        ├── assets/scaffold/
        └── src/
```

Recommended workspace membership:

```toml
[workspace]
members = ["crates/*", "tools/xtask"]
resolver = "3"

[workspace.metadata.xtask]
crate-prefix = "{{project-name}}"
primary-crate = "{{project-name}}"
```

`cargo xtask` remains unchanged through:

```toml
[alias]
xtask = "run --package xtask --"
```

Moving `xtask` from `crates/xtask` to `tools/xtask` keeps `crates/` semantically
clean without pretending generic repository automation is a product-domain
crate. This is intentionally an adaptation of uv's `uv-dev` pattern, not a
literal copy.

An illustrative mature application may later look like this:

```text
crates/
├── atlas/                 # shipped entry/composition package
├── atlas-cli/             # command grammar and parsing
├── atlas-workspace/       # workspace discovery and model
├── atlas-resolver/        # resolution behavior
├── atlas-client/          # remote protocol adapter
├── atlas-distribution/    # distribution behavior
└── atlas-distribution-types/ # shared values only if the graph requires them
```

The template must label this example as illustrative. None of these extra
crates should be generated by default.

## Crate naming and extraction policy

Use “semantic name” in the generated documentation instead of “abstract name.”
It is less ambiguous and matches what uv actually does.

### Naming rules

- Every product package is either `<project>` or `<project>-<semantic-name>`.
- Prefer a stable noun from the project's domain: `workspace`, `resolver`,
  `distribution`, `configuration`, `client`, `installer`, `cache`, or a
  project-specific equivalent.
- A command name is not automatically a crate. `add`, `remove`, `sync`, and
  similar commands normally remain source modules behind the CLI interface.
- A Cargo feature is not automatically a crate. Extract only if it has its own
  invariants, dependency weight, platform constraint, reuse, or test surface.
- Avoid `core`, `common`, `shared`, `utils`, `helpers`, and `misc`; these names
  hide ownership.
- Use `types` only for a coherent shared vocabulary or traits that prevent a
  real dependency cycle. Prefer domain-qualified names such as
  `distribution-types` over an unbounded `types` crate.
- Use `dispatch` only for real orchestration across multiple domain modules.
- Use `client`, `git`, `fs`, `unix`, or `windows` for concrete adapters or
  platform modules only when the dependency or platform isolation is valuable.
- Keep repository automation private. Keep reusable test support private unless
  publishing it is an explicit product decision.

### Extraction test

Start behavior as an internal source module. Extract it into a crate when most
of the following are true:

1. It owns a coherent set of invariants and vocabulary.
2. Callers can use a smaller interface than the implementation they receive.
3. The deletion test shows that removing the module would spread its complexity
   across multiple callers.
4. It creates useful locality for changes, bugs, and tests.
5. Its dependency weight, platform requirements, compile profile, or reuse is
   meaningfully different from its caller.
6. Tests can exercise the same seam used by callers.

Do not introduce a trait merely to make a future adapter possible. One adapter
is a hypothetical seam; two adapters, or an immediate testing/runtime need, make
the seam real.

## Dependency direction

The README should teach constraints, not prescribe a universal layer stack:

```text
                  <project>
            entry / composition root
             /        |         \
            v         v          v
   <project>-cli   domain modules   concrete adapters
                       |
                       v
          small value/contract modules

tools/xtask and optional <project>-test may depend inward.
Production crates never depend on tools/xtask or test support.
```

Rules:

- The entry/composition crate may depend on many modules; assembly is its job.
- The CLI module owns parsing and command vocabulary, not product behavior.
- Domain modules do not depend on the entry/composition crate or CLI module.
- Put an interface at a seam where behavior actually varies. Prefer defining
  the interface near the consumer.
- Concrete adapters may depend on domain values and consumer-owned interfaces;
  do not make domain behavior depend on a concrete adapter when multiple
  adapters exist.
- A shared contract crate is a last-mile cycle breaker, not the default home of
  every public type.
- Cross-crate dependencies use `{ workspace = true }` in member manifests.
- When one member depends on another, root `[workspace.dependencies]` owns the
  canonical path and, for publishable crates, version. Standalone binary,
  benchmark, and development packages need no unused dependency entry.
- External versions belong at the root when they are shared policy; a dependency
  used by one crate may stay local to make its ownership obvious.
- Dependencies used only by `xtask` stay in `tools/xtask/Cargo.toml`. This
  mirrors uv-dev's deliberate distinction between product-shared and
  development-only dependencies.

## Refactor phases

### Phase 0 — Record the architecture contract

Files:

- `CONTEXT.md`
- new `docs/adr/0001-workspace-crate-structure.md`

Changes:

- Replace “public crate” as the only modeled concept with:
  - **primary crate**: the package that owns the shipped artifact or main
    interface;
  - **crate prefix**: the package-name prefix for product crates;
  - **semantic crate name**: a stable domain or operational noun;
  - **composition crate**: the module that assembles the shipped application;
  - **development command**: repository automation exposed through
    `cargo xtask`.
- Preserve the existing scaffold, capability, tool-group, and operational-command
  terms where still accurate.
- Record the decision to keep generation minimal and to teach growth in the
  rendered README instead of pre-generating a crate taxonomy.
- Record why `tools/xtask` is outside `crates/`.

Verification:

- Every target layout and scaffold rule in later phases uses these terms.
- No ADR or invariant claims that all projects are libraries or all projects
  are applications.

### Phase 1 — Add characterization tests before moving code

Files:

- `scripts/test-template.sh`
- new focused tests under the xtask package
- optionally a small fixture directory under `tools/xtask/tests/fixtures/`

Add coverage for:

- the exact minimal rendered file tree;
- `cargo xtask` from the workspace root and a descendant directory;
- dry-run, idempotence, invalid names, and user-owned-file refusal;
- workspace path/version registration for a semantic crate such as `resolver`;
- private and binary crate targets;
- release detection from Cargo metadata;
- the rendered README containing the required architecture sections;
- CLI scaffolding not changing the public library's dependency set in the
  library-safe mode;
- current platform path behavior, with a Windows CI smoke test added before
  changing executable resolution.

Verification:

- Tests pass before the structural move.
- Failures after each later phase identify behavior drift instead of merely
  reporting a final smoke-test failure.

### Phase 2 — Deepen workspace discovery

Files:

- root `Cargo.toml`
- `tools/xtask/Cargo.toml`
- `tools/xtask/src/workspace.rs`
- callers in scaffold and release modules

Changes:

- Replace `public-crate` metadata with `crate-prefix` and `primary-crate`.
- Use Cargo metadata as the source of truth for workspace members, target kinds,
  publishability, manifest paths, and the workspace root.
- Keep manifest mutation behind the workspace module rather than exposing raw
  root/name/version fields to every caller.
- Make publishable-binary detection work for explicit members and nonstandard
  target paths, not only immediate children of `crates/`.
- Add `[lints] workspace = true` to `xtask`.
- Keep one concrete implementation. Do not add a workspace trait until there is
  a second adapter or a demonstrated test seam that cannot use a fixture.

Verification:

- `cargo metadata --no-deps` reports the expected primary and tool packages.
- release and scaffold tests pass with wildcard and explicit workspace members.
- running `cargo xtask` from a descendant still resolves the same root.

### Phase 3 — Move xtask out of the product crate namespace

Files:

- move `crates/xtask/` to `tools/xtask/`
- root `Cargo.toml`
- `.cargo/config.toml`
- `cargo-generate.toml`
- `scripts/test-template.sh`

Changes:

- Set workspace members to `crates/*` plus `tools/xtask`.
- Preserve package name `xtask` and the exact `cargo xtask` command interface.
- Update cargo-generate include/exclude paths and all tests.
- Assert that every immediate child of generated `crates/` is the primary crate
  or begins with `<project>-`.

Verification:

- No command examples change for users.
- A fresh generated project builds and runs `cargo xtask --help`.
- `crates/` contains no repository-tool package.

### Phase 4 — Restore locality inside scaffolding

Suggested internal layout:

```text
tools/xtask/
├── assets/scaffold/
│   ├── agents/
│   ├── ci/
│   ├── docs/
│   └── release/
└── src/
    ├── scaffold/
    │   ├── mod.rs
    │   ├── plan.rs
    │   ├── crate.rs
    │   ├── cli.rs
    │   └── repository.rs
    ├── workspace.rs
    └── ...
```

Changes:

- Preserve `scaffold::run` as the small command-facing interface.
- Move `Plan` into `scaffold/plan.rs`; it remains the single deep module for
  preflight validation, ownership markers, dry-run reporting, and writes.
- Move crate/CLI rendering away from repository-capability rendering.
- Put large static scaffold bodies in `assets/scaffold/` and load them with
  `include_str!`; configure cargo-generate to copy but not Liquid-render bodies
  containing `${{ ... }}` or `{{version}}`.
- Keep renderers as internal implementation details. Do not manufacture traits
  for each scaffold family.
- Add focused tests for `Plan` conflicts, no-op reapplication, and dry-run.
- Address the already-recorded all-or-nothing write gap after the move: stage
  contents before replacement and document the remaining rename rollback
  behavior. Do not mix a filesystem-transaction redesign into the initial file
  move.

Verification:

- Every changed line belongs to either the move or one scaffold family.
- Generated output is byte-equivalent before intentional content changes.
- `Plan` tests cover the interface used by all scaffold families.

### Phase 5 — Make semantic crate scaffolding explicit

Files:

- `tools/xtask/src/cli.rs`
- `tools/xtask/src/scaffold/crate.rs`
- `TEMPLATE.md`
- generated `README.md`
- tests

Changes:

- Keep the simple command shape:

  ```sh
  cargo xtask scaffold crate <semantic-name> [--bin] [--private]
  ```

- Rename internal variables and help text from `suffix` to `semantic_name`.
- Keep package generation as `<crate-prefix>-<semantic-name>`.
- Replace `core` and `worker` examples with ownership examples such as
  `workspace`, `resolver`, or `client`, clearly marked illustrative.
- Generate a meaningful crate-level doc comment and Cargo description. If a
  description is not supplied, use neutral wording and prompt the user in the
  README to replace it; do not infer domain behavior from the name.
- Do not add role enums such as `domain`, `adapter`, or `contract` unless they
  result in materially different generated files. Documentation is sufficient
  while all crates share the same skeleton.
- Register publishable library crates as root path-plus-version dependencies.
  Use path-only entries for private libraries, and do not create an unused root
  dependency entry for a standalone binary crate.
- Improve `crates/README.md` generation to include each package's Cargo
  description, targets, and private/publishable status, while preserving
  ownership markers for safe regeneration.

Verification:

- `scaffold crate resolver` creates `<project>-resolver`, registers the correct
  canonical workspace dependency for its publishability/target kind, and is
  idempotent.
- The scaffold does not create dependencies or interfaces that the user did not
  request.
- The crate index explains ownership rather than only listing links.

### Phase 6 — Fix CLI composition without forcing one product type

The current template is library-first, while uv is application-first. Preserve
that distinction explicitly.

Recommended command:

```sh
cargo xtask scaffold cli --entrypoint companion  # safe default for a public library
cargo xtask scaffold cli --entrypoint primary    # uv-like application composition
```

Behavior:

- Both modes create `<project>-cli` for the Clap command model.
- `companion` creates the binary target in the private CLI/application package,
  depends inward on the public `<project>` library, and does not add Clap or
  `<project>-cli` to the public library's normal dependencies.
- `primary` places the binary in the primary `<project>` package and makes that
  package the composition root, matching uv's `uv` plus `uv-cli` shape.
- In both modes, `src/bin/<project>.rs` stays a thin wrapper around a testable
  library entrypoint. Command implementation belongs in the appropriate domain
  module, not in the parser crate.
- Keep the existing guarantee that scaffolding a CLI never overwrites the
  public library source.

If adding a mode is considered too much interface, implement only `companion`
now because it preserves the current public-library contract, and document the
small manual move required for an uv-like application. Do not silently retain
the current dependency coupling.

Verification:

- A library consumer of `<project>` does not compile Clap after `companion`
  scaffolding.
- `cargo run --package <expected-package> -- --version` works in both modes.
- CLI parsing tests target `<project>-cli`; end-to-end behavior tests target the
  package that owns the binary.

### Phase 7 — Make the rendered README the architecture teacher

The generated README must contain, on day one:

1. **What was generated** — one product crate plus repository automation; no
   architecture has been assumed.
2. **Develop** — `cargo xtask check`, `test`, `build`, and `--help`.
3. **Grow like uv, not to uv's size** — explain project-prefixed semantic
   names and that uv reached 70 packages through domain pressure.
4. **Choose a crate name** — concrete good and bad examples.
5. **When to extract** — the extraction test, deletion test, depth, locality,
   and seam guidance from this plan.
6. **Dependency direction** — the small graph and rules above.
7. **CLI layouts** — library-safe companion vs uv-like primary composition.
8. **Testing** — unit tests beside behavior, integration tests at the shipped
   interface, shared test support only after repetition.
9. **Workspace dependency pattern** — root path/version plus member
   `{ workspace = true }` example.
10. **Scaffold recipes** — semantic crate, CLI, CI, docs, and agent commands.
11. **Generated files and ownership** — explain idempotence, markers, dry-run,
    and that generated projects are snapshots.
12. **Further reference** — link to uv's pinned repository revision and its
    `crates/README.md`, while stating that names must come from this project's
    own domain.

Minimum wording to retain:

> Start inside the primary crate. Extract a crate when it owns a coherent
> responsibility and gives callers a smaller interface than its implementation.
> Name it after that responsibility, not after a generic layer, command, or
> feature flag.

`TEMPLATE.md` remains documentation for template maintainers. It should link to
the rendered README contract and avoid duplicating instructions that can drift.

Verification:

- The smoke test generates a project and asserts all required README headings.
- No README command references template-only files excluded by cargo-generate.
- A reader can reproduce the illustrative uv-like layout using existing
  `cargo xtask scaffold` commands plus the documented dependency edits.

### Phase 8 — CI and final verification

Required checks run against a freshly rendered fixture, because the template's
raw Cargo files intentionally contain cargo-generate placeholders:

```sh
bash scripts/test-template.sh

# The smoke harness runs these inside its generated project:
cargo fmt --all --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

Add proportionate CI coverage:

- keep the fast Ubuntu rendered-template smoke test;
- add Windows coverage for local executable/path resolution;
- add an MSRV job or pin the toolchain to the declared `rust-version`; a floating
  `stable` toolchain does not verify Rust 1.85 compatibility;
- keep optional-tool version checks scheduled rather than making every pull
  request install the full tool suite.

Final graph review:

- run `cargo metadata --no-deps` on the template and a generated fixture;
- inspect `cargo tree --workspace` for production dependencies on `xtask` or
  test support;
- confirm no path dependency bypasses `[workspace.dependencies]` without an
  explicit reason;
- confirm the entry/composition crate is the only intentionally high-fan-out
  product package;
- confirm the worktree diff contains no unrelated formatting or cleanup.

## Suggested implementation sequence

Use small reviewable commits:

1. `test: characterize rendered workspace and scaffold behavior`
2. `docs: record semantic crate structure decision`
3. `refactor: deepen xtask workspace discovery`
4. `refactor: move xtask under tools`
5. `refactor: split scaffold planning and rendering internals`
6. `fix: isolate cli composition from public library dependencies`
7. `docs: teach uv-inspired workspace growth in rendered readme`
8. `test: add windows and msrv template coverage`

Do not combine the xtask move, CLI behavior change, README rewrite, and atomic
write follow-up in one commit; each has a different verification surface.

## Non-goals

- Pre-generating `resolver`, `client`, `types`, `dispatch`, or any other
  speculative product crate.
- Reproducing uv's 70-package graph.
- Enforcing a universal domain/adapter/contract layer hierarchy.
- Creating a trait at every crate seam.
- Creating one crate per CLI command or Cargo feature.
- Moving every dependency into root `[workspace.dependencies]`.
- Replacing `xtask` with a new task runner.
- Redesigning release tooling beyond what is required by accurate workspace
  discovery.
- Cleaning unrelated existing code while moving modules.

## Acceptance criteria

- [ ] Fresh generation produces one product crate and one private xtask package.
- [ ] `crates/` contains only `<project>` and `<project>-*` product packages.
- [ ] `cargo xtask` works from the root and descendant directories.
- [ ] Workspace discovery and target detection use Cargo metadata.
- [ ] Every cross-crate dependency has one appropriate root path declaration;
      publishable crates include a version and member manifests use
      `{ workspace = true }`.
- [ ] Crate examples use semantic ownership nouns; generated guidance rejects
      generic catch-all and crate-per-command naming.
- [ ] The rendered README explains when **not** to extract a crate.
- [ ] No speculative domain crates are generated.
- [ ] The CLI parser is separate from behavior.
- [ ] Library-safe CLI scaffolding does not add Clap to public-library normal
      dependencies.
- [ ] The uv-like primary-entrypoint mode has a thin binary and a composition
      root.
- [ ] Production packages do not depend on xtask or reusable test support.
- [ ] `Plan` remains the single deep file-mutation module and has focused tests.
- [ ] Static scaffold assets are copied without accidental Liquid expansion.
- [ ] The crate index includes responsibility descriptions, not just names.
- [ ] Template smoke, lint, Windows path, and MSRV checks pass.
- [ ] The final diff is limited to files named by this plan.

## Decisions to confirm before implementation

The plan recommends defaults, but these two choices should be confirmed before
code changes because they affect generated repositories:

1. **xtask location:** recommended `tools/xtask` with the package and command
   still named `xtask`. Alternative: a project-prefixed `<project>-dev` package
   behind the `cargo xtask` alias, closer to uv but less conventional for a
   generic template.
2. **CLI entrypoint:** recommended library-safe `companion` default plus an
   explicit uv-like `primary` mode. Simpler alternative: ship only the
   library-safe mode and document the manual application conversion.

Everything else can be implemented without changing the template's minimal-first
contract.
