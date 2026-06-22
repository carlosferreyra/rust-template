use crate::flags::Add;
use std::fs;
use std::path::{Path, PathBuf};

const WORKSPACE_DEPENDENCIES: &str = "[workspace.dependencies]\n";
const PACKAGE_DEPENDENCIES: &str = "[dependencies]\n";

pub fn add(cmd: Add) {
    if let Err(error) = add_crate(&cmd.name) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn add_crate(suffix: &str) -> Result<(), String> {
    validate_suffix(suffix)?;
    let project_name = project_name()?;
    if suffix == "cli" {
        add_cli(&project_name)
    } else {
        add_library(&project_name, suffix)
    }
}

fn project_name() -> Result<String, String> {
    let workspace = read("Cargo.toml")?;
    let document: toml::Value = workspace
        .parse()
        .map_err(|error| format!("failed to parse Cargo.toml: {error}"))?;
    let repository = document
        .get("workspace")
        .and_then(|value| value.get("package"))
        .and_then(|value| value.get("repository"))
        .and_then(toml::Value::as_str)
        .ok_or("workspace.package.repository must be set")?;
    repository
        .trim_end_matches(".git")
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| "could not derive project name from workspace.package.repository".into())
}

fn validate_suffix(suffix: &str) -> Result<(), String> {
    let mut chars = suffix.chars();
    if !matches!(chars.next(), Some('a'..='z'))
        || !chars.all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
        || suffix.ends_with('-')
        || suffix.contains("--")
    {
        return Err("name must begin with a lowercase letter and contain only lowercase letters, digits, and single hyphens".into());
    }
    if suffix == "xtask" || suffix.starts_with("cli-") {
        return Err(format!("{suffix:?} is reserved"));
    }
    Ok(())
}

fn read(path: impl AsRef<Path>) -> Result<String, String> {
    let path = path.as_ref();
    fs::read_to_string(path).map_err(|error| format!("failed to read {}: {error}", path.display()))
}

fn insert_after(content: &str, header: &str, line: &str) -> Result<String, String> {
    let index = content
        .find(header)
        .ok_or_else(|| format!("missing {header:?} section"))?
        + header.len();
    let mut updated = content.to_owned();
    updated.insert_str(index, line);
    Ok(updated)
}

fn crate_dir(crate_name: &str) -> PathBuf {
    Path::new("crates").join(crate_name)
}

fn ensure_absent(crate_name: &str, directory: &Path, workspace: &str) -> Result<(), String> {
    if directory.exists() || workspace.contains(&format!("{crate_name} =")) {
        return Err(format!("crate {crate_name:?} already exists"));
    }
    Ok(())
}

fn workspace_dependency(crate_name: &str) -> String {
    format!("{crate_name} = {{ path = \"crates/{crate_name}\", version = \"0.0.0\" }}\n")
}

fn add_library(project_name: &str, suffix: &str) -> Result<(), String> {
    let crate_name = format!("{project_name}-{suffix}");
    let directory = crate_dir(&crate_name);
    let workspace = read("Cargo.toml")?;
    ensure_absent(&crate_name, &directory, &workspace)?;
    let updated_workspace = insert_after(
        &workspace,
        WORKSPACE_DEPENDENCIES,
        &workspace_dependency(&crate_name),
    )?;

    let manifest = library_manifest(&crate_name);
    let source = format!(
        "//! TODO: describe the `{crate_name}` crate.\n\n/// Starter value for `{crate_name}`.\n#[must_use]\npub const fn hello() -> &'static str {{\n    \"Hello from {crate_name}!\"\n}}\n"
    );

    fs::create_dir_all(directory.join("src"))
        .map_err(|error| format!("failed to create {}: {error}", directory.display()))?;
    fs::write(directory.join("Cargo.toml"), manifest).map_err(|error| error.to_string())?;
    fs::write(directory.join("src/lib.rs"), source).map_err(|error| error.to_string())?;
    fs::write("Cargo.toml", updated_workspace).map_err(|error| error.to_string())?;
    append_crates_readme(
        &crate_name,
        "Library crate. Add its dependency explicitly where needed.",
    )?;

    println!("created crates/{crate_name}");
    println!("  → add `{crate_name} = {{ workspace = true }}` to each consuming crate");
    Ok(())
}

fn library_manifest(crate_name: &str) -> String {
    format!(
        r#"[package]
name        = "{crate_name}"
description = "TODO: describe {crate_name}."
version.workspace      = true
edition.workspace      = true
rust-version.workspace = true
authors.workspace      = true
license.workspace      = true
repository.workspace   = true
homepage.workspace     = true
readme.workspace       = true
keywords.workspace     = true
categories.workspace   = true

[lints]
workspace = true
"#
    )
}

fn add_cli(project_name: &str) -> Result<(), String> {
    let cli_name = format!("{project_name}-cli");
    let cli_dir = crate_dir(&cli_name);
    let binary = Path::new("crates")
        .join(project_name)
        .join("src/bin")
        .join(format!("{project_name}.rs"));
    let workspace = read("Cargo.toml")?;
    let project_manifest_path = Path::new("crates").join(project_name).join("Cargo.toml");
    let project_manifest = read(&project_manifest_path)?;

    ensure_absent(&cli_name, &cli_dir, &workspace)?;
    if binary.exists()
        || project_manifest.contains("default-run")
        || project_manifest.contains(&format!("{cli_name} ="))
    {
        return Err("CLI scaffold already exists".into());
    }

    let updated_workspace = insert_after(
        &workspace,
        WORKSPACE_DEPENDENCIES,
        &workspace_dependency(&cli_name),
    )?;
    let with_default_run = project_manifest.replacen(
        "[package]\n",
        &format!("[package]\ndefault-run = \"{project_name}\"\n"),
        1,
    );
    let updated_project_manifest = insert_after(
        &with_default_run,
        PACKAGE_DEPENDENCIES,
        &format!(
            "{cli_name} = {{ workspace = true }}\nclap = {{ workspace = true }}\ntracing-subscriber = {{ workspace = true }}\n"
        ),
    )?;

    fs::create_dir_all(cli_dir.join("src")).map_err(|error| error.to_string())?;
    fs::create_dir_all(binary.parent().expect("binary has parent"))
        .map_err(|error| error.to_string())?;
    fs::write(
        cli_dir.join("Cargo.toml"),
        cli_manifest(project_name, &cli_name),
    )
    .map_err(|error| error.to_string())?;
    fs::write(cli_dir.join("src/lib.rs"), cli_source(project_name))
        .map_err(|error| error.to_string())?;
    fs::write(&binary, binary_source(project_name)).map_err(|error| error.to_string())?;
    fs::write(project_manifest_path, updated_project_manifest)
        .map_err(|error| error.to_string())?;
    fs::write("Cargo.toml", updated_workspace).map_err(|error| error.to_string())?;
    fs::write(
        Path::new("crates").join(project_name).join("src/lib.rs"),
        entrypoint_source(project_name),
    )
    .map_err(|error| error.to_string())?;
    append_crates_readme(
        &cli_name,
        "Clap command model for the project executable. Not published.",
    )?;

    println!("created CLI scaffold");
    println!("  → command model: crates/{cli_name}");
    println!("  → entrypoint: {}", binary.display());
    Ok(())
}

fn cli_manifest(project_name: &str, cli_name: &str) -> String {
    format!(
        r#"[package]
name        = "{cli_name}"
description = "Command-line model for {project_name}."
publish     = false
version.workspace      = true
edition.workspace      = true
rust-version.workspace = true
authors.workspace      = true
license.workspace      = true
repository.workspace   = true
homepage.workspace     = true

[lints]
workspace = true

[dependencies]
clap = {{ workspace = true }}
"#
    )
}

fn cli_source(project_name: &str) -> String {
    format!(
        r#"//! Clap command model for `{project_name}`.

use clap::{{Parser, Subcommand}};

/// `{project_name}` command-line interface.
#[derive(Debug, Parser)]
#[command(name = "{project_name}", version, about)]
pub struct Cli {{
    /// Command to run.
    #[command(subcommand)]
    pub command: Command,
}}

/// Available commands.
#[derive(Debug, Subcommand)]
pub enum Command {{
    /// Print a greeting.
    Hello {{
        /// Name to greet.
        #[arg(default_value = "world")]
        name: String,
    }},
}}
"#
    )
}

fn entrypoint_source(project_name: &str) -> String {
    format!(
        r#"//! Public library and executable dispatch for `{project_name}`.
#![allow(clippy::print_stdout, clippy::print_stderr)]

use std::ffi::OsString;
use std::process::ExitCode;

use clap::Parser;
use {crate_name}_cli::{{Cli, Command}};

/// Returns a starter message.
#[must_use]
pub const fn hello() -> &'static str {{
    "Hello from {project_name}!"
}}

/// Parse command-line arguments and dispatch the selected command.
pub fn main(args: impl IntoIterator<Item = OsString>) -> ExitCode {{
    let _ = tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .try_init();
    let cli = match Cli::try_parse_from(args) {{
        Ok(cli) => cli,
        Err(error) => {{
            let exit_code = error.exit_code();
            let _ = error.print();
            return ExitCode::from(u8::try_from(exit_code).unwrap_or(2));
        }}
    }};
    match cli.command {{
        Command::Hello {{ name }} => println!("Hello, {{name}}!"),
    }}
    ExitCode::SUCCESS
}}
"#,
        crate_name = project_name.replace('-', "_")
    )
}

fn binary_source(project_name: &str) -> String {
    format!(
        r#"//! Thin process entrypoint for `{project_name}`.

use std::process::ExitCode;

fn main() -> ExitCode {{
    {crate_name}::main(std::env::args_os())
}}
"#,
        crate_name = project_name.replace('-', "_")
    )
}

fn append_crates_readme(crate_name: &str, description: &str) -> Result<(), String> {
    let path = Path::new("crates/README.md");
    let mut readme = read(path)?;
    readme.push_str(&format!(
        "\n## [{crate_name}](./{crate_name})\n\n{description}\n"
    ));
    fs::write(path, readme).map_err(|error| error.to_string())
}
