#!/usr/bin/env python3
"""Refresh upstream stable versions and render the template's CI workflow."""

import argparse
import json
import os
from pathlib import Path
import re
import tomllib
from urllib.request import Request, urlopen


ROOT = Path(__file__).resolve().parent.parent
REGISTRY = ROOT / "tools/xtask/assets/tooling.toml"
TEMPLATE = ROOT / "tools/xtask/assets/scaffold/ci/template.yml"
WORKFLOW = ROOT / ".github/workflows/ci.yml"


def latest(repository):
    headers = {"User-Agent": "rust-template-tooling"}
    token = os.environ.get("GH_TOKEN") or os.environ.get("GITHUB_TOKEN")
    if token:
        headers["Authorization"] = f"Bearer {token}"
    request = Request(
        f"https://api.github.com/repos/{repository}/releases/latest", headers=headers
    )
    with urlopen(request, timeout=30) as response:
        release = json.load(response)
    if release["draft"] or release["prerelease"]:
        raise ValueError(f"{repository}: expected a stable release")
    return release["tag_name"]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--latest", action="store_true", help="fetch stable upstream releases")
    mode.add_argument("--check", action="store_true", help="check rendering offline")
    args = parser.parse_args()
    text = REGISTRY.read_text()
    registry = tomllib.loads(text)
    if args.latest:
        # Fetch every release before changing files, so a network failure leaves them intact.
        replacements = []
        for name, reference in registry["actions"].items():
            repository = reference.split("@", 1)[0]
            updated = f"{repository}@{latest(repository)}"
            replacements.append((f'{name} = "{reference}"', f'{name} = "{updated}"'))
        for package, tool in registry["tools"].items():
            tag = latest(tool["repository"])
            match = re.search(r"(?:^|[^0-9])(\d+\.\d+\.\d+)$", tag)
            if not match:
                raise ValueError(f"{package}: unexpected release tag {tag!r}")
            old = f'{package} = {{ version = "{tool["version"]}"'
            new = f'{package} = {{ version = "{match.group(1)}"'
            replacements.append((old, new))
        for old, new in replacements:
            if old != new:
                print(f"{old} -> {new}")
            text = text.replace(old, new)
        registry = tomllib.loads(text)

    rendered = TEMPLATE.read_text()
    for name, reference in registry["actions"].items():
        rendered = rendered.replace(f"@{name}@", reference)
    for package, tool in registry["tools"].items():
        rendered = rendered.replace(f"@{package}@", tool["version"])
    if args.check:
        if WORKFLOW.read_text() != rendered:
            parser.exit(1, "CI differs from the registry; run python3 scripts/update-tooling.py\n")
    else:
        REGISTRY.write_text(text)
        WORKFLOW.write_text(rendered)


if __name__ == "__main__":
    main()
