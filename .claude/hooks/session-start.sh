#!/bin/bash
# Runs once at the start of every Claude Code session.
# Ensures the local toolchain has the components needed for hooks and xtasks.
set -euo pipefail

# Dispatch to the remote variant when running in Claude.ai web or a cloud runner.
if [ "${CLAUDE_CODE_REMOTE:-}" = "true" ]; then
  exec "$(dirname "$0")/session-start-remote.sh"
fi

# Local session: ensure clippy and rustfmt are present for the active toolchain.
rustup component add clippy rustfmt --quiet
