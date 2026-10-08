#!/usr/bin/env python3
"""Exercise sync upgrades and tool discovery in a generated project without installs."""

import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile


project = Path(sys.argv[1]).resolve()
xtask = project / "target/debug/xtask"
packages = [
    "cargo-nextest", "cargo-llvm-cov", "cargo-deny", "typos-cli",
    "cargo-dist", "cargo-release", "git-cliff",
]
executables = {"typos-cli": "typos", "cargo-dist": "dist"}

with tempfile.TemporaryDirectory() as temporary:
    temporary = Path(temporary)
    env = dict(os.environ, PATH=f"{temporary}{os.pathsep}{os.environ['PATH']}")
    env["TOOL_TEST_LOG"] = str(temporary / "commands.jsonl")
    env["TOOL_TEST_CARGO"] = shutil.which("cargo")
    env["TOOL_TEST_PYTHON"] = sys.executable
    stub = temporary / "stub.py"
    stub.write_text('''import json, os, sys
from pathlib import Path
args = sys.argv[2:]
program = sys.argv[1]
if program == "cargo" and args[:2] != ["+stable", "install"]:
    os.execv(os.environ["TOOL_TEST_CARGO"], ["cargo", *args])
with open(os.environ["TOOL_TEST_LOG"], "a") as log:
    log.write(json.dumps([program, *args]) + "\\n")
if program == "cargo":
    package = args[-1]
    binary = {"typos-cli": "typos", "cargo-dist": "dist"}.get(package, package)
    path = Path(args[args.index("--root") + 1]) / "bin" / binary
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("#!/bin/sh\\necho " + binary + " " + os.environ["TOOL_TEST_RELEASE"] + "\\n")
    path.chmod(0o755)
''')
    for program in ["cargo", "rustup"]:
        wrapper = temporary / program
        wrapper.write_text(
            '#!/bin/sh\nexec "$TOOL_TEST_PYTHON" "'
            + str(stub) + '" ' + program + ' "$@"\n'
        )
        wrapper.chmod(0o755)

    # A second sync must contact the installer again and replace an older release.
    for release in ["98.0.0", "99.0.0"]:
        env["TOOL_TEST_RELEASE"] = release
        subprocess.run([xtask, "tools", "sync", "all"], cwd=project, env=env, check=True)
        for package in packages:
            binary = project / ".xtask/tools/bin" / executables.get(package, package)
            assert release in subprocess.check_output([binary, "--version"], text=True)

    commands = [json.loads(line) for line in Path(env["TOOL_TEST_LOG"]).read_text().splitlines()]
    for offset in [0, 10]:
        assert commands[offset] == ["rustup", "update", "stable"]
        assert commands[offset + 1] == [
            "rustup", "component", "add", "--toolchain", "stable", "clippy", "rustfmt",
        ]
    installs = [command for command in commands if command[0] == "cargo"]
    assert [command[-1] for command in installs] == packages * 2
    assert all(command[1:3] == ["+stable", "install"] for command in installs)
    assert all("--locked" in command and "--version" not in command for command in installs)
    assert commands.count([
        "rustup", "component", "add", "--toolchain", "stable", "llvm-tools-preview",
    ]) == 2

    # Arbitrary working versions are accepted, preferring project-local binaries.
    doctor = subprocess.check_output([xtask, "doctor"], cwd=project, env=env, text=True)
    assert "missing" not in doctor
    assert str(project / ".xtask/tools/bin/cargo-deny") in doctor
    local = project / ".xtask/tools/bin/cargo-deny"
    local.rename(temporary / "cargo-deny")
    doctor = subprocess.check_output([xtask, "doctor"], cwd=project, env=env, text=True)
    assert "missing" not in doctor
    assert str(local) not in doctor

# The fixture's remaining checks should use real tools from PATH.
shutil.rmtree(project / ".xtask/tools")
print("tool sync upgrades and local/global discovery passed")
