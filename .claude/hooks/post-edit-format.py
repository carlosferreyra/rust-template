# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///

"""Post-edit hook: auto-format files after Claude edits them.

Fires on every Edit/Write/MultiEdit tool call. Reads the tool event from stdin,
extracts the file path, and dispatches to the right formatter based on extension.

Formatters used:
  .rs          -> cargo fmt -- <file>
  .py / .pyi   -> uvx ruff format <file>
  .toml        -> taplo format <file>  (if installed, otherwise skipped)
"""

import json
import os
import subprocess
import sys
from pathlib import Path


def run(cmd: list[str], cwd: str) -> None:
    try:
        subprocess.run(cmd, cwd=cwd, capture_output=True)
    except FileNotFoundError:
        pass


def main() -> None:
    event = json.load(sys.stdin)

    if event.get("tool_name") not in ("Write", "Edit", "MultiEdit"):
        return

    file_path = event.get("tool_input", {}).get("file_path")
    if not file_path:
        return

    cwd = os.environ.get("CLAUDE_PROJECT_DIR", os.getcwd())
    ext = Path(file_path).suffix

    if ext == ".rs":
        # cargo fmt targets a single file when passed after '--'
        run(["cargo", "fmt", "--", file_path], cwd)
    elif ext in (".py", ".pyi"):
        run(["uvx", "ruff", "format", file_path], cwd)
    elif ext == ".toml":
        # taplo is the standard TOML formatter; skipped silently if not installed.
        # Install: cargo install taplo-cli --locked
        run(["taplo", "format", file_path], cwd)


if __name__ == "__main__":
    main()
