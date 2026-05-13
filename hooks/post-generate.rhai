// Post-generate hook for rust-template.
// Requires: cargo-dist, git-cliff installed on the host.
// cargo generate must be invoked with --allow-commands.

// git init + stage all generated files (user makes the first commit)
system::command("git", ["init", "-q"]);
system::command("git", ["add", "-A"]);

// cargo dist init -y  →  dist-workspace.toml + .github/workflows/release.yml
try {
    system::command("cargo", ["dist", "init", "-y"]);
} catch {
    print("⛔  cargo dist init failed. Is cargo-dist installed?");
    print("    Run: cargo install cargo-dist");
    throw "cargo dist init failed";
}

// git-cliff --init  →  cliff.toml
try {
    system::command("git-cliff", ["--init"]);
} catch {
    print("⛔  git-cliff --init failed. Is git-cliff installed?");
    print("    Run: cargo install git-cliff");
    throw "git-cliff --init failed";
}

// Stage any files added by cargo dist / git-cliff
system::command("git", ["add", "-A"]);

print("");
print("✓  scaffolded successfully. Run: git commit -m 'chore: initial scaffold'");
print("");
print("   cargo xtask check          — fmt + check + clippy");
print("   cargo xtask test           — check + test suite");
print("   cargo xtask test <filter>  — scoped test run");
print("   cargo xtask build          — test + release build");
print("   cargo xtask add <name>     — scaffold a new crate");
print("");
print("   cargo release patch --execute   — bump, changelog, tag, push");
