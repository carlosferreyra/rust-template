use crate::flags::Add;
use std::fs;
use std::path::Path;

// Fields under [workspace.package] that cargo supports as inheritable.
const INHERITABLE: &[&str] = &[
    "version",
    "edition",
    "authors",
    "license",
    "license-file",
    "repository",
    "homepage",
    "documentation",
    "description",
    "readme",
    "keywords",
    "categories",
    "publish",
    "rust-version",
    "exclude",
    "include",
];

fn workspace_inheritable_fields() -> Vec<&'static str> {
    let raw = fs::read_to_string("Cargo.toml")
        .unwrap_or_else(|e| panic!("failed to read workspace Cargo.toml: {e}"));
    let doc: toml::Value = raw
        .parse()
        .unwrap_or_else(|e| panic!("failed to parse workspace Cargo.toml: {e}"));

    let pkg = doc
        .get("workspace")
        .and_then(|w| w.get("package"))
        .and_then(|p| p.as_table());

    INHERITABLE
        .iter()
        .copied()
        .filter(|&field| pkg.is_some_and(|t| t.contains_key(field)))
        .collect()
}

const PROJECT_NAME: &str = "{{project-name}}";

pub fn add(cmd: Add) {
    let suffix = &cmd.name;
    let crate_name = format!("{PROJECT_NAME}-{suffix}");
    let dir = Path::new("crates").join(&crate_name);

    if dir.exists() {
        eprintln!("error: crates/{crate_name} already exists");
        std::process::exit(1);
    }

    fs::create_dir_all(dir.join("src"))
        .unwrap_or_else(|e| panic!("failed to create crates/{crate_name}/src: {e}"));

    let fields = workspace_inheritable_fields();
    let max_len = fields.iter().map(|f| f.len()).max().unwrap_or(0);
    let inherited: String = fields
        .iter()
        .map(|&f| format!("{f:<max_len$}.workspace = true\n"))
        .collect();

    let cargo_toml = format!(
        r#"[package]
name        = "{crate_name}"
description = "TODO: describe {crate_name}."
{inherited}
[lints]
workspace = true
"#
    );
    fs::write(dir.join("Cargo.toml"), cargo_toml)
        .unwrap_or_else(|e| panic!("failed to write crates/{crate_name}/Cargo.toml: {e}"));

    let lib_rs = format!(
        r#"//! {crate_name}
//!
//! TODO: replace this line with a one-sentence summary of what the crate does.
//!
//! # Examples
//!
//! ```
//! // TODO: add a minimal working example.
//! ```
"#
    );
    fs::write(dir.join("src/lib.rs"), lib_rs)
        .unwrap_or_else(|e| panic!("failed to write crates/{crate_name}/src/lib.rs: {e}"));

    let readme_path = Path::new("crates/README.md");
    if readme_path.exists() {
        let mut readme = fs::read_to_string(readme_path)
            .unwrap_or_else(|e| panic!("failed to read crates/README.md: {e}"));
        readme.push_str(&format!(
            "\n## [{crate_name}](./{crate_name})\n\nTODO: one-line role description for {crate_name}.\n"
        ));
        fs::write(readme_path, readme)
            .unwrap_or_else(|e| panic!("failed to update crates/README.md: {e}"));
    }

    println!("created crates/{crate_name}");
    println!("  → fill in the TODO in crates/README.md");
    println!("  → fill in the TODO doc comment in crates/{crate_name}/src/lib.rs");
    println!("  → add it as a dependency in any crate that needs it");
}
