// This hook operates on cargo-generate's temporary template copy.

fn stable_version(version) {
    let core = version.split("+")[0];
    if core.contains("-") {
        throw `Expected a stable release, received ${version}`;
    }
    let parts = core.split(".");
    if parts.len() != 3 {
        throw `Expected a release version, received ${version}`;
    }
    for part in parts {
        let number = parse_int(part);
        if number < 0 || number.to_string() != part {
            throw `Invalid release version: ${version}`;
        }
    }
    version
}

// A current stable compiler also determines the generated project's baseline.
print("Resolving current stable Rust, dependencies, and GitHub Actions...");
system::command("rustup", ["toolchain", "install", "stable", "--profile", "minimal", "--component", "clippy", "--component", "rustfmt"]);
let compiler = system::command("rustc", ["+stable", "--version"]);
variable::set("rust_version", stable_version(compiler.split(" ")[1]));
variable::set("project_version", "0.0.0");

// Isolate resolution from the template's not-yet-rendered workspace manifest.
file::write("hooks/dependency-resolver/Cargo.toml", "[workspace]\n[package]\nname = \"template-dependency-resolver\"\nversion = \"0.0.0\"\nedition = \"2024\"\n[lib]\npath = \"lib.rs\"\n");
system::command("cargo", ["+stable", "add", "--manifest-path", "hooks/dependency-resolver/Cargo.toml", "--registry", "crates-io", "--ignore-rust-version", "cargo_metadata", "clap", "toml_edit", "--features", "clap/derive"]);
let dependencies = system::command("cargo", ["+stable", "tree", "--manifest-path", "hooks/dependency-resolver/Cargo.toml", "--depth", "1", "--prefix", "none", "--format", "{p}"]);
let found = [];
for line in dependencies.split("\n") {
    line.trim();
    let fields = line.split(" ");
    let name = fields[0];
    if ["cargo_metadata", "clap", "toml_edit"].contains(name) {
        if found.contains(name) || !fields[1].starts_with("v") {
            throw `Unexpected dependency resolution: ${line}`;
        }
        // Cargo ignores build metadata in requirements; keep it in Cargo.lock only.
        let version = stable_version(fields[1].sub_string(1));
        variable::set(`${name}_version`, version.split("+")[0]);
        found.push(name);
    }
}
if found.len() != 3 {
    throw "Cargo did not resolve every required dependency";
}
file::delete("hooks/dependency-resolver");

// GitHub's latest-release redirect supplies a stable tag without API credentials.
let actions = [
    ["checkout", "actions/checkout"],
    ["setup_rust", "moonrepo/setup-rust"],
    ["install", "taiki-e/install-action"],
    ["audit", "rustsec/audit-check"]
];
for action in actions {
    let repository = action[1];
    let release = system::command("curl", ["--fail", "--silent", "--show-error", "--head", "--location", "--proto", "=https", "--proto-redir", "=https", "--connect-timeout", "15", "--max-time", "30", "--output", "hooks/action-response", "--write-out", "\"%{url_effective}\"", `https://github.com/${repository}/releases/latest`]);
    let prefix = `https://github.com/${repository}/releases/tag/v`;
    if !release.starts_with(prefix) {
        throw `Could not resolve a stable release for ${repository}: ${release}`;
    }
    let version = stable_version(release.sub_string(prefix.len()));
    if version.contains("+") {
        throw `Unexpected action tag: ${release}`;
    }
    variable::set(`${action[0]}_version`, version);
}
file::delete("hooks/action-response");

// Licenses remain bundled; only version resolution requires network access.
variable::set("copyright_year", system::date().year.to_string());
let license_expression = variable::get("license");
file::delete("LICENSE");
file::rename(`hooks/licenses/${license_expression}.txt`, "LICENSE");
file::delete("hooks/licenses");
