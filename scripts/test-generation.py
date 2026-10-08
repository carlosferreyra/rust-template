#!/usr/bin/env python3
"""Exercise generation-time upgrades and failures without contacting upstreams."""

import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import tomllib


ROOT = Path(__file__).resolve().parent.parent
GENERATOR = shutil.which("cargo-generate")


def main():
    assert GENERATOR, "cargo-generate is required"
    with tempfile.TemporaryDirectory(prefix="template-generation-") as directory:
        destination = Path(directory)
        binaries = destination / "bin"
        binaries.mkdir()
        wrapper = f"#!{sys.executable}\n" + '''
import os
from pathlib import Path
import sys

tool = Path(sys.argv[0]).name
arguments = sys.argv[1:]
version = os.environ["GENERATION_TEST_VERSION"]
failure = os.environ.get("GENERATION_TEST_FAILURE", "")
if tool == "rustup":
    assert arguments[:3] == ["toolchain", "install", "stable"]
elif tool == "rustc":
    assert arguments == ["+stable", "--version"]
    print("rustc 9.0.0 (mock)")
elif tool == "curl":
    if failure == "network":
        sys.exit("upstream unavailable")
    tag = version if failure != "prerelease" else version + "-beta.1"
    print(arguments[-1].replace("/latest", "/tag/v" + tag))
elif tool == "cargo":
    assert arguments[0] == "+stable"
    command = arguments[1]
    if command == "add":
        assert "--ignore-rust-version" in arguments
        assert arguments[arguments.index("--registry") + 1] == "crates-io"
        if failure == "registry":
            sys.exit("registry unavailable")
    elif command == "tree":
        print("template-dependency-resolver v0.0.0")
        for name in ["cargo_metadata", "clap", "toml_edit"]:
            print(name + " v" + version)
    elif command == "check":
        assert "--workspace" in arguments and "--all-targets" in arguments
        if failure == "incompatible":
            sys.exit("current dependency API is incompatible")
        Path("Cargo.lock").write_text("version = 4\\n")
        Path(".generation-target").mkdir()
    else:
        sys.exit("unexpected cargo command: " + command)
else:
    sys.exit("unexpected tool: " + tool)
'''
        for name in ["cargo", "rustc", "rustup", "curl"]:
            path = binaries / name
            path.write_text(wrapper)
            path.chmod(0o755)

        def generate(name, version="5.0.0", license="MIT", failure="", allow=True):
            environment = dict(os.environ, PATH=f"{binaries}{os.pathsep}{os.environ['PATH']}",
                               GENERATION_TEST_VERSION=version, GENERATION_TEST_FAILURE=failure)
            command = [GENERATOR, "generate", "--path", str(ROOT), "--name", name,
                       "--destination", str(destination), "--silent",
                       "-d", "github_username=smoke-user", "-d", "author_name=Smoke Test",
                       "-d", "author_email=smoke@example.com", "-d", "license=" + license]
            if allow:
                command.append("--allow-commands")
            result = subprocess.run(command, env=environment, capture_output=True, text=True)
            return result, destination / name

        # A new generation resolves new major releases for both crates and actions.
        for index, version in enumerate(["5.0.0", "6.0.1"]):
            result, project = generate(f"upgrade-{index}", version)
            assert result.returncode == 0, result.stdout + result.stderr
            manifest = tomllib.loads((project / "Cargo.toml").read_text())
            dependencies = manifest["workspace"]["dependencies"]
            assert dependencies["clap"]["version"] == version
            assert dependencies["cargo_metadata"] == version
            assert dependencies["toml_edit"] == version
            assert manifest["workspace"]["package"]["rust-version"] == "9.0.0"
            registry = tomllib.loads((project / "tools/xtask/assets/tooling.toml").read_text())
            assert all(reference.endswith("@v" + version) for reference in registry["actions"].values())
            assert (project / "Cargo.lock").is_file()
            assert not (project / "hooks/dependency-resolver").exists()
            assert not (project / "hooks/action-response").exists()
            assert not (project / ".generation-target").exists()

        for failure, message in [("registry", "registry unavailable"),
                                 ("network", "upstream unavailable"),
                                 ("prerelease", "Expected a stable release"),
                                 ("incompatible", "current dependency API is incompatible")]:
            result, project = generate("failure-" + failure, failure=failure)
            assert result.returncode != 0, failure
            assert message in result.stdout + result.stderr
            assert not (project / "Cargo.toml").exists(), "failed generation copied a project"
        result, _ = generate("denied", allow=False)
        assert result.returncode != 0
        assert "--allow-commands" in result.stdout + result.stderr

        for index, source in enumerate(sorted((ROOT / "hooks/licenses").glob("*.txt"))):
            result, project = generate(f"license-{index}", license=source.stem)
            assert result.returncode == 0, result.stdout + result.stderr
            license = (project / "LICENSE").read_text()
            assert license and "{{" not in license
            assert not (project / "hooks/licenses").exists()
    print("Generation upgrades, failure paths, permissions, and bundled licenses passed.")


if __name__ == "__main__":
    main()
