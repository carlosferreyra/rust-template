// Post-generate hook for rust-template.
// Requires: cargo-dist, git-cliff installed on the host.
// cargo generate must be invoked with --allow-commands.

print("");
print("✓  scaffolded successfully.");
print("");
print("   Run these commands to finish setup:");
print("");
print("   cd <project-name>");
print("   git add -A && git commit -m 'chore: initial scaffold'");
print("   gh repo create <name> --source=. --push");
print("   cargo dist init -y && cargo dist generate");
print("   git-cliff --init");
print("   git add -A && git commit -m 'chore: dist and changelog config'");
print("");
print("   Development:");
print("   cargo xtask check          — fmt + check + clippy");
print("   cargo xtask test           — check + test suite");
print("   cargo xtask test <filter>  — scoped test run");
print("   cargo xtask build          — test + release build");
print("   cargo xtask add <name>     — scaffold a new crate");
print("   cargo xtask coverage       — HTML coverage report (needs cargo-llvm-cov)");
print("");
print("   Release (workspace-lockstep, all crates publish together):");
print("   cargo xtask publish              — dry-run patch bump");
print("   cargo xtask publish --execute    — bump, changelog, tag, push, publish");
print("");
print("   Re-generate non-interactively (every prompt has a `-d key=value` flag):");
print("   cargo generate --git ... --allow-commands \\");
print("     --name <name> -d github_username=<u> -d author_name=\"...\" \\");
print("     -d author_email=... -d project_description=\"...\" -d include_cli=true");
