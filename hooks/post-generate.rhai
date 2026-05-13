// Post-generate hook for rust-template.
// Requires: cargo-dist, git-cliff installed on the host.
// cargo generate must be invoked with --allow-commands.

print("");
print("✓  scaffolded successfully.");
print("");
print("   Run these commands to finish setup:");
print("");
print("   git init -q && git add -A && git commit -m 'chore: initial scaffold'");
print("   gh repo create <name> --source=. --push");
print("   cargo dist init -y && cargo dist generate");
print("   git-cliff --init");
print("   git add -A && git commit -m 'chore: dist and changelog config'");
print("");
print("   cargo xtask check          — fmt + check + clippy");
print("   cargo xtask test           — check + test suite");
print("   cargo xtask test <filter>  — scoped test run");
print("   cargo xtask build          — test + release build");
print("   cargo xtask add <name>     — scaffold a new crate");
print("");
print("   cargo release patch --execute   — bump, changelog, tag, push");
