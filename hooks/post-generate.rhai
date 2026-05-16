// Post-generate hook for rust-template.
// Requires: cargo-dist, git-cliff, gh installed on the host.
// cargo generate must be invoked with --allow-commands.

// ---------------------------------------------------------------------------
// Fetch LICENSE from GitHub API
// ---------------------------------------------------------------------------
// system::command returns stdout as a String and throws on non-zero exit.
let license_id = variable::get("license");
let author = variable::get("author_name");
// .trim() is not chainable in this Rhai version — assign first, then strip newline.
let current_year = system::command("date", ["+%Y"]);
current_year = current_year.replace("\n", "");

try {
    // Fetch via shell so we can pipe through sed for substitutions —
    // Rhai's String::replace returns () on ImmutableString in this version.
    // GitHub license templates use two placeholder conventions:
    //   [bracket] style — MIT, BSD-2, BSD-3
    //   <angle>   style — AGPL-3.0, GPL-2.0, GPL-3.0, LGPL-2.1
    let text = system::command("sh", [
        "-c",
        `gh api licenses/${license_id} --jq .body \
         | sed 's/\\[year\\]/${current_year}/g' \
         | sed 's/\\[fullname\\]/${author}/g' \
         | sed 's/\\[name of copyright owner\\]/${author}/g' \
         | sed 's/<year>/${current_year}/g' \
         | sed 's/<name of author>/${author}/g'`
    ]);

    file::write("LICENSE", text);
    print(`✓  LICENSE (${license_id}) written.`);
} catch(e) {
    print(`⚠  Could not fetch license '${license_id}' from GitHub API: ${e}`);
    print(`   Run manually: gh api licenses/${license_id} --jq .body > LICENSE`);
}

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
print("     -d author_email=... -d project_description=\"...\" \\");
print("     -d include_cli=true -d license=mit");
