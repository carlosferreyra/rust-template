#!/usr/bin/env bash
set -euo pipefail

template_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
destination=$(mktemp -d)
trap 'rm -rf "$destination"' EXIT

generate_project() {
  local name=$1
  cargo generate \
    --path "$template_root" \
    --name "$name" \
    --destination "$destination" \
    --allow-commands \
    --define github_username=smoke-user \
    --define author_name="Smoke Test" \
    --define author_email=smoke@example.com \
    --define project_description="Generated smoke test" \
    --define license=MIT
}

generate_project template-fixture

project="$destination/template-fixture"
cd "$project"

test -d crates/template-fixture
test -d tools/xtask
test ! -e crates/xtask
test ! -e crates/template-fixture-core
test ! -e crates/template-fixture-cli
test ! -e .github
test ! -e .config
test ! -e docs
test ! -e scripts
test ! -e CONTEXT.md
test ! -e CONTRIBUTING.md
test ! -e crates/README.md
test ! -e deny.toml
test ! -e typos.toml
test -f .cargo/config.toml
grep -Eq '^xtask = "run --package xtask --"$' .cargo/config.toml
grep -Fq '## Grow the workspace' README.md
grep -Fq 'semantic crate names' README.md
placeholder_matches=$(find . -type f \
  ! -path './Cargo.lock' \
  ! -path './tools/xtask/src/scaffold/*' \
  ! -path './tools/xtask/assets/*' \
  -exec grep -EH '\{\{[^}]+\}\}' {} + | grep -Ev '\$\{\{' || true)
if [ -n "$placeholder_matches" ]; then
  printf '%s\n' "$placeholder_matches"
  echo "unresolved template placeholder found" >&2
  exit 1
fi

cargo xtask --help
(
  cd crates/template-fixture
  cargo xtask --help
)
cargo xtask check
cargo xtask test
cargo xtask build

# Library releases use cargo-release without forcing binary distribution.
cargo xtask release init
test -f release.toml
test -f CHANGELOG.md
test ! -e dist-workspace.toml
test ! -e .github/workflows/release.yml
if cargo xtask release plan; then
  echo "library-only dist plan unexpectedly succeeded" >&2
  exit 1
fi

# Semantic library crates inherit the workspace version through one root entry.
perl -pi -e 's/version      = "0\.0\.0"/version      = "1.2.3"/' Cargo.toml
cargo xtask scaffold crate resolver
grep -Fq 'template-fixture-resolver = { path = "crates/template-fixture-resolver", version = "1.2.3" }' Cargo.toml
cargo xtask check

# Standalone private binaries are not workspace dependencies until another crate uses them.
cargo xtask scaffold crate worker --bin --private
if grep -Fq 'template-fixture-worker =' Cargo.toml; then
  echo "private standalone binary unexpectedly registered as a workspace dependency" >&2
  exit 1
fi

# The default companion CLI keeps the primary public library dependency-free.
cp crates/template-fixture/Cargo.toml "$destination/public-manifest-before.toml"
cp crates/template-fixture/src/lib.rs "$destination/public-lib-before.rs"
cargo xtask scaffold cli
cmp crates/template-fixture/Cargo.toml "$destination/public-manifest-before.toml"
cmp crates/template-fixture/src/lib.rs "$destination/public-lib-before.rs"
test -f crates/template-fixture-cli/src/main.rs
if cargo tree --package template-fixture | grep -q 'clap'; then
  echo "companion CLI leaked Clap into the primary library" >&2
  exit 1
fi
cargo xtask check
cargo run --package template-fixture-cli -- --version
cargo run --package template-fixture-cli -- hello smoke

# Scaffolds are idempotent.
before=$(git status --porcelain=v1)
cargo xtask scaffold cli
after=$(git status --porcelain=v1)
test "$before" = "$after"

# Optional repository capabilities appear only when requested.
cargo xtask scaffold ci --preset full
test -f .github/workflows/ci.yml
test -f deny.toml
test -f typos.toml
grep -Fq '${{ github.ref }}' .github/workflows/ci.yml
cargo xtask scaffold docs
test -f CONTRIBUTING.md
test -f crates/README.md
grep -Fq 'Targets:' crates/README.md
cargo xtask scaffold agents --claude
test -f AGENTS.md
test -f CLAUDE.md

# Dry runs report changes without writing them.
cargo xtask scaffold crate dry-run-only --dry-run
test ! -e crates/template-fixture-dry-run-only

before=$(git status --porcelain=v1)
if cargo xtask scaffold crate Invalid_Name; then
  echo "invalid semantic name unexpectedly succeeded" >&2
  exit 1
fi
after=$(git status --porcelain=v1)
test "$before" = "$after"

cargo xtask doctor
cargo xtask check

# The explicit primary mode follows the uv-like composition shape.
generate_project primary
primary_project="$destination/primary"
cd "$primary_project"
cargo xtask scaffold cli --entrypoint primary
test -f crates/primary/src/bin/primary.rs
grep -Fq 'primary-cli = { path = "crates/primary-cli" }' Cargo.toml
grep -Fq 'primary-cli = { workspace = true }' crates/primary/Cargo.toml
grep -Fq 'clap = { workspace = true }' crates/primary/Cargo.toml
cargo xtask check
cargo run --package primary -- --version
cargo run --package primary -- hello smoke
