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
