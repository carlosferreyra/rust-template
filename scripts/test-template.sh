#!/usr/bin/env bash
set -euo pipefail

template_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
destination=$(mktemp -d)
trap 'rm -rf "$destination"' EXIT

cargo generate \
  --path "$template_root" \
  --name smoke-project \
  --destination "$destination" \
  --allow-commands \
  --define github_username=smoke-user \
  --define author_name="Smoke Test" \
  --define author_email=smoke@example.com \
  --define project_description="Generated smoke test" \
  --define license=MIT

project="$destination/smoke-project"
cd "$project"

test -d crates/smoke-project
test -d crates/xtask
test ! -e crates/smoke-project-core
test ! -e crates/smoke-project-cli
test ! -e .github
test ! -e .config
test ! -e scripts
test ! -e CONTEXT.md
test ! -e CONTRIBUTING.md
test ! -e crates/README.md
test ! -e deny.toml
test ! -e typos.toml
test -f .cargo/config.toml
grep -Eq '^xtask = "run --package xtask --"$' .cargo/config.toml
placeholder_matches=$(find . -type f \
  ! -path './Cargo.lock' \
  ! -path './crates/xtask/src/scaffold.rs' \
  -exec grep -EH '\{\{[^}]+\}\}' {} + | grep -Ev '\$\{\{' || true)
if [ -n "$placeholder_matches" ]; then
  printf '%s\n' "$placeholder_matches"
  echo "unresolved template placeholder found" >&2
  exit 1
fi

cargo xtask --help
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

# Added dependency versions follow the current workspace version.
perl -pi -e 's/version      = "0\.0\.0"/version      = "1.2.3"/' Cargo.toml
cargo xtask scaffold crate core
grep -Fq 'smoke-project-core = { path = "crates/smoke-project-core", version = "1.2.3" }' Cargo.toml
cargo xtask check

# Adding a CLI must not overwrite an established public library.
cp crates/smoke-project/src/lib.rs "$destination/public-lib-before.rs"
cargo xtask scaffold cli
cmp crates/smoke-project/src/lib.rs "$destination/public-lib-before.rs"
cargo xtask check
cargo run -- --version
cargo run -- hello smoke

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
cargo xtask scaffold docs
test -f CONTRIBUTING.md
test -f crates/README.md
cargo xtask scaffold agents --claude
test -f AGENTS.md
test -f CLAUDE.md

# Dry runs report changes without writing them.
cargo xtask scaffold crate dry-run-only --dry-run
test ! -e crates/smoke-project-dry-run-only

before=$(git status --porcelain=v1)
if cargo xtask scaffold crate Invalid_Name; then
  echo "invalid crate name unexpectedly succeeded" >&2
  exit 1
fi
after=$(git status --porcelain=v1)
test "$before" = "$after"

cargo xtask doctor
cargo xtask check
