// Post-generate hook for rust-template.
// Requires: cargo-dist, git-cliff, gh installed on the host.
// cargo generate must be invoked with --allow-commands.

// ---------------------------------------------------------------------------
// Fetch LICENSE from GitHub API
// ---------------------------------------------------------------------------
let license_id = variable::get("license");
let gh_result = system::command("gh", ["api", `licenses/${license_id}`]);

if gh_result.exit_code == 0 {
    // Parse the body field out of the JSON response using gh's --jq flag.
    let body_result = system::command("gh", ["api", `licenses/${license_id}`, "--jq", ".body"]);
    if body_result.exit_code == 0 {
        let author = variable::get("author_name");
        let year = system::command("date", ["+%Y"]);
        let current_year = year.stdout.trim();

        let text = body_result.stdout;
        // GitHub license templates use two placeholder conventions:
        //   [bracket] style — MIT, BSD-2, BSD-3
        //   <angle>   style — AGPL-3.0, GPL-2.0, GPL-3.0, LGPL-2.1
        text = text.replace("[year]", current_year);
        text = text.replace("[fullname]", author);
        text = text.replace("[name of copyright owner]", author);
        text = text.replace("<year>", current_year);
        text = text.replace("<name of author>", author);

        file::write("LICENSE", text);
        print(`✓  LICENSE (${license_id}) written.`);
    } else {
        print(`⚠  Could not parse license body — skipping LICENSE generation.`);
        print(`   Run: gh api licenses/${license_id} --jq .body > LICENSE`);
    }
} else {
    print(`⚠  Could not fetch license '${license_id}' from GitHub API.`);
    print(`   Run: gh api licenses/${license_id} --jq .body > LICENSE`);
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
