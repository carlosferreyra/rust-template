// Post-generate hook for rust-template.
// Requires: gh and uv installed on the host for license retrieval.
// cargo generate must be invoked with --allow-commands.

// ---------------------------------------------------------------------------
// Fetch LICENSE from GitHub API
// ---------------------------------------------------------------------------
// system::command returns stdout as a String and throws on non-zero exit.
let license_expression = variable::get("license");
let author = variable::get("author_name");
let current_year = system::command("date", ["+%Y"]);

// Write a small Python script to a temp file and run it via uv.
// This avoids quoting/escaping issues with sh -c pipelines that contain
// spaces in author names or brackets in sed patterns.
let py = `
import json, subprocess, sys
license_id = """${license_expression}""".lower().removesuffix("-only")
result = subprocess.run(
    ["gh", "api", f"licenses/{license_id}", "--jq", ".body"],
    capture_output=True, text=True
)
if result.returncode != 0:
    sys.exit(result.returncode)
text = result.stdout
year = "${current_year}".strip()
author = """${author}"""
text = text.replace("[year]", year).replace("[fullname]", author)
text = text.replace("[name of copyright owner]", author)
text = text.replace("<year>", year).replace("<name of author>", author)
print(text, end="")
`;
file::write("_fetch_license.py", py);

try {
    let text = system::command("uv", ["run", "--no-project", "_fetch_license.py"]);
    file::write("LICENSE", text);
    print(`✓  LICENSE (${license_expression}) written.`);
} catch(e) {
    print(`⚠  Could not fetch license '${license_expression}' from GitHub API: ${e}`);
    print("   Fetch the matching license body from the GitHub licenses API.");
}
system::command("rm", ["-f", "_fetch_license.py"]);

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
print("   cargo xtask scaffold crate <name>  — add a crate");
print("   cargo xtask scaffold cli           — add a CLI");
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
