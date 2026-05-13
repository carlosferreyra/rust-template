use crate::flags::Add;
use std::fs;
use std::path::Path;

const PROJECT_NAME: &str = "{{project_name}}";

pub fn add(cmd: Add) {
    let suffix = &cmd.name;
    let crate_name = format!("{PROJECT_NAME}-{suffix}");
    let dir = Path::new("crates").join(&crate_name);

    if dir.exists() {
        eprintln!("error: crates/{crate_name} already exists");
        std::process::exit(1);
    }

    // Create directory structure
    fs::create_dir_all(dir.join("src"))
        .unwrap_or_else(|e| panic!("failed to create crates/{crate_name}/src: {e}"));

    // Write Cargo.toml
    let cargo_toml = format!(
        r#"[package]
name        = "{crate_name}"
description = "TODO: describe {crate_name}."
version.workspace    = true
edition.workspace    = true
authors.workspace    = true
license.workspace    = true
repository.workspace = true
homepage.workspace   = true

[lints]
workspace = true
"#
    );
    fs::write(dir.join("Cargo.toml"), cargo_toml)
        .unwrap_or_else(|e| panic!("failed to write crates/{crate_name}/Cargo.toml: {e}"));

    // Write src/lib.rs
    let lib_rs = format!("// {crate_name}\n");
    fs::write(dir.join("src/lib.rs"), lib_rs)
        .unwrap_or_else(|e| panic!("failed to write crates/{crate_name}/src/lib.rs: {e}"));

    // Append to crates/README.md
    let readme_path = Path::new("crates/README.md");
    if readme_path.exists() {
        let mut readme = fs::read_to_string(readme_path)
            .unwrap_or_else(|e| panic!("failed to read crates/README.md: {e}"));
        let section = format!(
            "\n## [{crate_name}](./{crate_name})\n\nTODO: describe {crate_name}.\n"
        );
        readme.push_str(&section);
        fs::write(readme_path, readme)
            .unwrap_or_else(|e| panic!("failed to update crates/README.md: {e}"));
    }

    println!("created crates/{crate_name}");
    println!("  → update crates/README.md with a real description");
    println!("  → add it as a dependency in any crate that needs it");
}
