//! `{{project-name}}` command-line interface.
#![allow(clippy::print_stdout, clippy::print_stderr)]

mod logging;

use anyhow::Context;
use clap::Parser;

/// {{project-name}} CLI.
#[derive(Debug, Parser)]
#[command(name = "{{project-name}}", version, about)]
struct Cli {
    /// Name to greet.
    #[arg(default_value = "world")]
    name: String,
}

fn main() -> anyhow::Result<()> {
    logging::init()?;
    let cli = Cli::parse();
    let greeting = {{crate_name}}::greet(&cli.name).context("greet failed")?;
    println!("{greeting}");
    Ok(())
}
