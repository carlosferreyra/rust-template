#!/bin/bash
# Remote/cloud variant of session-start.sh.
# Runs when CLAUDE_CODE_REMOTE=true (Claude.ai web, CI runners).
# Installs tools that are pre-installed locally but absent in cloud environments.
set -euo pipefail

# gh CLI — needed for any xtask or hook that calls `gh api`.
if ! command -v gh &>/dev/null; then
  apt-get update -qq
  apt-get install -y -qq gh
fi

# Rust toolchain components required by hooks and xtasks.
rustup component add clippy rustfmt --quiet

# Point gh at this repo so `gh` works even when the git remote is a local proxy.
# Adjust the value after instantiating the template.
if [ -n "${CLAUDE_ENV_FILE:-}" ]; then
  echo 'export GH_REPO={{github_username}}/{{project-name}}' >> "$CLAUDE_ENV_FILE"
fi
