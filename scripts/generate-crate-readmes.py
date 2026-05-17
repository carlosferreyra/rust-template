# Generate per-crate README.md files for a Rust workspace.
#
# The "facade" crate (the workspace member whose name matches the repository
# name from `[workspace.package.repository]`) gets a top-level README listing
# the other publishable members. Each remaining publishable member gets a
# README pointing back to the facade crate on crates.io.
#
# Hand-written READMEs (those NOT starting with the generated header) are left
# untouched. Crates with `publish = false` are skipped.
#
# Generated Markdown is formatted with prettier (hermetic: `--prose-wrap
# always` is passed explicitly, so no `.prettierrc` is required). Requires
# `npx` (Node) at runtime for the final formatting pass only.
#
# Usage:
#   uv run scripts/generate-crate-readmes.py
#
# Adapted from astral-sh/uv: https://github.com/astral-sh/uv/tree/main/scripts/

# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///

import json
import pathlib
import subprocess
import tomllib
from urllib.parse import urlparse

GENERATED_HEADER = "<!-- This file is generated. DO NOT EDIT -->"
PRETTIER_VERSION = "3.8.3"

FACADE_TEMPLATE = """{GENERATED_HEADER}

# {facade}

{description}

See the [repository]({repo_url}) for more information.

This crate is the entry point to the workspace. The Rust API exposed here is
not considered a stable public interface.

This is version {version}. The source can be found [here]({source_url}).

The following workspace members are also published:

{members_list}

Workspace members other than the facade crate are considered internal and may
have frequent breaking changes.
"""

MEMBER_TEMPLATE = """{GENERATED_HEADER}

# {name}

This crate is an internal component of
[{facade}](https://crates.io/crates/{facade}). The Rust API exposed here is
unstable and may have frequent breaking changes.

This version ({crate_version}) is a component of
[{facade} {facade_version}]({facade_crates_io_url}). The source can be found
[here]({source_url}).
"""


def load_workspace_repository() -> str:
    """Read `[workspace.package.repository]` from the workspace Cargo.toml."""
    workspace_root = pathlib.Path(__file__).resolve().parent.parent
    manifest = tomllib.loads((workspace_root / "Cargo.toml").read_text())
    repository = manifest.get("workspace", {}).get("package", {}).get("repository")
    if not isinstance(repository, str) or not repository:
        raise RuntimeError(
            "workspace.package.repository is missing from Cargo.toml; "
            "set it to your repository URL"
        )
    return repository


def repo_name_from_url(repository: str) -> str:
    """Parse the repository (project) name from a repository URL."""
    parts = [p for p in urlparse(repository).path.split("/") if p]
    if len(parts) < 2:
        raise RuntimeError(
            f"could not parse a project name from repository URL {repository!r}"
        )
    name = parts[1]
    if name.endswith(".git"):
        name = name[: -len(".git")]
    return name


def main() -> None:
    repo_url = load_workspace_repository().rstrip("/")
    if repo_url.endswith(".git"):
        repo_url = repo_url[: -len(".git")]
    facade = repo_name_from_url(repo_url)

    result = subprocess.run(
        ["cargo", "metadata", "--format-version", "1", "--no-deps"],
        capture_output=True,
        text=True,
        check=True,
    )
    content = json.loads(result.stdout)
    packages = {package["id"]: package for package in content["packages"]}
    workspace_root = pathlib.Path(content["workspace_root"])

    facade_pkg = None
    for member_id in content["workspace_members"]:
        if packages[member_id]["name"] == facade:
            facade_pkg = packages[member_id]
            break
    if facade_pkg is None:
        raise RuntimeError(
            f"could not find a workspace crate named {facade!r} "
            "(expected the facade crate to match the repository name)"
        )

    facade_version = facade_pkg["version"]
    facade_dir = pathlib.Path(facade_pkg["manifest_path"]).parent
    facade_readme_path = facade_dir / "README.md"

    publishable_members = []
    for member_id in content["workspace_members"]:
        package = packages[member_id]
        name = package["name"]
        if name == facade:
            continue
        if package.get("publish") == []:
            continue
        publishable_members.append(name)
    publishable_members.sort()

    members_list = "\n".join(
        f"- [{name}](https://crates.io/crates/{name})" for name in publishable_members
    )

    generated_paths: list[pathlib.Path] = []

    def should_write(path: pathlib.Path, name: str) -> bool:
        if path.exists() and not path.read_text().startswith(GENERATED_HEADER):
            print(f"Skipping {name}: existing README without generated header")
            return False
        return True

    facade_source_url = (
        f"{repo_url}/blob/{facade_version}/{facade_dir.relative_to(workspace_root)}"
    )
    if should_write(facade_readme_path, facade):
        facade_readme_path.write_text(
            FACADE_TEMPLATE.format(
                GENERATED_HEADER=GENERATED_HEADER,
                facade=facade,
                description=facade_pkg.get("description") or facade,
                repo_url=repo_url,
                version=facade_version,
                source_url=facade_source_url,
                members_list=members_list,
            )
        )
        generated_paths.append(facade_readme_path)
        print(f"Generated README for {facade}")

    facade_crates_io_url = f"https://crates.io/crates/{facade}/{facade_version}"
    for member_id in content["workspace_members"]:
        package = packages[member_id]
        name = package["name"]
        if name == facade or package.get("publish") == []:
            continue

        crate_dir = pathlib.Path(package["manifest_path"]).parent
        member_readme_path = crate_dir / "README.md"
        if not should_write(member_readme_path, name):
            continue

        source_url = (
            f"{repo_url}/blob/{facade_version}/{crate_dir.relative_to(workspace_root)}"
        )
        member_readme_path.write_text(
            MEMBER_TEMPLATE.format(
                GENERATED_HEADER=GENERATED_HEADER,
                name=name,
                facade=facade,
                crate_version=package["version"],
                facade_version=facade_version,
                facade_crates_io_url=facade_crates_io_url,
                source_url=source_url,
            )
        )
        generated_paths.append(member_readme_path)
        print(f"Generated README for {name}")

    if generated_paths:
        subprocess.run(
            [
                "npx",
                "--yes",
                f"prettier@{PRETTIER_VERSION}",
                "--prose-wrap",
                "always",
                "--write",
                *[str(path) for path in generated_paths],
            ],
            check=True,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )


if __name__ == "__main__":
    main()
