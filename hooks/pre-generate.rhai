// Resolve github_username before template rendering so Liquid substitutes it.
// variable::set requires a string literal — use `${}` interpolation to coerce.

let github_username = "";

try {
    let out = system::command("gh", ["api", "user", "--jq", ".login"]);
    github_username = `${out}`.trim();
} catch {}

if github_username == "" {
    try {
        let url = system::command("git", ["config", "--get", "remote.origin.url"]);
        url = `${url}`.trim();
        if url.contains("github.com/") {
            github_username = url.split("github.com/")[1].split("/")[0];
        } else if url.contains("github.com:") {
            github_username = url.split("github.com:")[1].split("/")[0];
        }
    } catch {}
}

if github_username == "" {
    try {
        let out = system::command("git", ["config", "--get", "user.name"]);
        github_username = `${out}`.trim();
    } catch {}
}

variable::set("github_username", `${github_username}`);

// Resolve author_name and author_email from git config.
let author_name = "";
let author_email = "";

try {
    let out = system::command("git", ["config", "--get", "user.name"]);
    author_name = `${out}`.trim();
} catch {}

try {
    let out = system::command("git", ["config", "--get", "user.email"]);
    author_email = `${out}`.trim();
} catch {}

variable::set("author_name", `${author_name}`);
variable::set("author_email", `${author_email}`);

// Exclude the CLI crate when the user opts out.
let include_cli = variable::get("include_cli");
if !include_cli {
    ignore::push("crates/{{project-name}}-cli");
}
