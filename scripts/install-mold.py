# Install the mold linker and make it the default linker (Linux only).
#
# Retries on transient HTTP errors (429/500/502/503/504) that the
# `rui314/setup-mold` GitHub Action does not handle. No-op on non-Linux.
#
# Usage:
#   uv run scripts/install-mold.py
#
# Adapted from astral-sh/uv: https://github.com/astral-sh/uv/tree/main/scripts/

# /// script
# requires-python = ">=3.11"
# dependencies = ["httpx"]
# ///

import os
import platform
import subprocess
import sys
import tempfile
import time

import httpx  # ty: ignore  # PEP 723 dep, not resolvable by the linter

MOLD_VERSION = os.environ.get("MOLD_VERSION", "2.40.4")
RETRY_STATUS = {429, 500, 502, 503, 504}
MAX_TRIES = 5
RETRY_WAIT_SECS = 3


def main() -> None:
    if platform.system() != "Linux":
        print("Not on Linux, skipping mold install", file=sys.stderr)
        return

    arch = platform.machine()
    url = (
        f"https://github.com/rui314/mold/releases/download/"
        f"v{MOLD_VERSION}/mold-{MOLD_VERSION}-{arch}-linux.tar.gz"
    )
    sudo = [] if os.geteuid() == 0 else ["sudo"]

    print(f"Installing mold {MOLD_VERSION} ({arch})...")

    with tempfile.NamedTemporaryFile(suffix=".tar.gz") as tmp:
        last_error: Exception | None = None
        for attempt in range(1, MAX_TRIES + 1):
            try:
                with httpx.stream(
                    "GET", url, follow_redirects=True, timeout=30
                ) as resp:
                    if resp.status_code in RETRY_STATUS:
                        resp.read()
                        raise httpx.HTTPStatusError(
                            f"HTTP {resp.status_code}",
                            request=resp.request,
                            response=resp,
                        )
                    resp.raise_for_status()
                    for chunk in resp.iter_bytes():
                        tmp.write(chunk)
                break
            except (httpx.HTTPError, httpx.HTTPStatusError) as exc:
                last_error = exc
                if attempt == MAX_TRIES:
                    raise
                print(
                    f"download failed (attempt {attempt}/{MAX_TRIES}): {exc}; "
                    f"retrying in {RETRY_WAIT_SECS}s",
                    file=sys.stderr,
                )
                time.sleep(RETRY_WAIT_SECS)
        else:  # pragma: no cover - defensive
            raise RuntimeError(f"failed to download mold: {last_error}")

        tmp.flush()
        # Extract via system tar: writing into /usr/local needs sudo, and
        # --strip-components mirrors the upstream shell behavior exactly.
        subprocess.run(
            sudo
            + [
                "tar",
                "-C",
                "/usr/local",
                "--strip-components=1",
                "--no-overwrite-dir",
                "-xzf",
                tmp.name,
            ],
            check=True,
        )

    current_ld = os.path.realpath("/usr/bin/ld")
    if current_ld != "/usr/local/bin/mold":
        subprocess.run(
            sudo + ["ln", "-sf", "/usr/local/bin/mold", current_ld], check=True
        )

    print(f"mold {MOLD_VERSION} installed successfully")
    subprocess.run(["mold", "--version"], check=True)


if __name__ == "__main__":
    main()
