// Post-generate hook for rust-template.
// Requires: cargo-dist, git-cliff installed on the host.
// cargo generate must be invoked with --allow-commands.

// git init + stage all generated files (user makes the first commit)
system::command("git", ["init", "-q"]);
system::command("git", ["add", "-A"]);

print("");
print("✓  scaffolded successfully.");
print("");
print("   Next steps:");
print("   1. git commit -m 'chore: initial scaffold'");
print("   2. gh repo create <name> --source=. --push   (or connect an existing remote)");
print("   3. cargo dist init -y                        (sets up dist-workspace.toml)");
print("   4. cargo dist generate                       (generates .github/workflows/release.yml)");
print("   5. git-cliff --init                          (generates cliff.toml)");
print("");
print("   cargo xtask check          — fmt + check + clippy");
print("   cargo xtask test           — check + test suite");
print("   cargo xtask test <filter>  — scoped test run");
print("   cargo xtask build          — test + release build");
print("   cargo xtask add <name>     — scaffold a new crate");
print("");
print("   cargo release patch --execute   — bump, changelog, tag, push");
