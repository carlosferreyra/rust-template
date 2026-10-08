// Reject incompatible current releases before reporting generation success.
system::command("cargo", ["+stable", "check", "--workspace", "--all-targets", "--target-dir", ".generation-target"]);
file::delete(".generation-target");

// ---------------------------------------------------------------------------
// Next-step instructions
// ---------------------------------------------------------------------------
print("");
print("✓  scaffolded successfully.");
print("");
print("   Run these commands to finish setup:");
print("");
print("   cd <project-name>");
print("   git add -A && git commit -m 'chore: initial scaffold'");
print("   gh repo create <name> --source=. --push");
print("");
print("   Development:");
print("   cargo xtask check          — fmt + check + clippy");
print("   cargo xtask test           — check + test suite");
print("   cargo xtask test <filter>  — scoped test run");
print("   cargo xtask build          — test + release build");
print("   cargo xtask scaffold crate <semantic-name>  — add a responsibility crate");
print("   cargo xtask scaffold cli                     — add a companion CLI");
print("   cargo xtask scaffold cli --entrypoint primary — make the primary crate the CLI");
print("   cargo xtask scaffold ci            — add lean CI");
print("   cargo xtask doctor                  — inspect optional tools");
print("");
print("   Optional release tooling:");
print("   cargo xtask tools sync release");
print("   cargo xtask release init");
print("   cargo xtask release prepare");
print("");
print("   Re-generate non-interactively (every prompt has a `-d key=value` flag):");
print("   cargo generate --git ... --allow-commands \\");
print("     --name <name> -d github_username=<u> -d author_name=\"...\" \\");
print("     -d author_email=... -d project_description=\"...\" \\");
print("     -d license=MIT");
