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
  --define license=mit

project="$destination/smoke-project"
cd "$project"

test -d crates/smoke-project
test -d crates/xtask
test ! -e crates/smoke-project-core
test ! -e crates/smoke-project-cli
test -f .cargo/config.toml
grep -Eq '^xtask = "run --package xtask --"$' .cargo/config.toml
placeholder_matches=$(find . -type f \
  ! -path './Cargo.lock' \
  ! -path './crates/xtask/src/tasks/add.rs' \
  -exec grep -EH '\{\{[^}]+\}\}' {} + | grep -Ev '\$\{\{' || true)
if [ -n "$placeholder_matches" ]; then
  printf '%s\n' "$placeholder_matches"
  echo "unresolved template placeholder found" >&2
  exit 1
fi

cargo xtask check
cargo xtask test
cargo xtask build
cargo xtask add core
cargo xtask check
cargo xtask add cli
cargo xtask check
cargo run -- --version
cargo run -- hello smoke

before=$(git status --porcelain=v1)
if cargo xtask add cli; then
  echo "duplicate CLI scaffold unexpectedly succeeded" >&2
  exit 1
fi
after=$(git status --porcelain=v1)
test "$before" = "$after"

before=$(git status --porcelain=v1)
if cargo xtask add Invalid_Name; then
  echo "invalid crate name unexpectedly succeeded" >&2
  exit 1
fi
after=$(git status --porcelain=v1)
test "$before" = "$after"
