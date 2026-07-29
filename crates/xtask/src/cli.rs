use clap::{Args, Parser, Subcommand, ValueEnum};

/// Develop and grow this workspace.
#[derive(Debug, Parser)]
#[command(name = "xtask", version, propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Check formatting, compilation, and lints.
    Check,
    /// Check the workspace, then run its tests.
    Test(TestArgs),
    /// Test the workspace, then build release artifacts.
    Build,
    /// Run the checks used by generated CI.
    Ci(CiArgs),
    /// Generate an HTML coverage report.
    Coverage,
    /// Add an optional project capability.
    Scaffold {
        /// Show planned writes without changing files.
        #[arg(long, global = true)]
        dry_run: bool,
        #[command(subcommand)]
        command: ScaffoldCommand,
    },
    /// Report the availability of optional development tools.
    Doctor,
    /// Manage pinned, project-local development tools.
    Tools {
        #[command(subcommand)]
        command: ToolsCommand,
    },
    /// Initialize, inspect, or prepare releases.
    Release {
        #[command(subcommand)]
        command: ReleaseCommand,
    },
}

#[derive(Debug, Args)]
pub struct TestArgs {
    /// Optional test-name filter.
    pub filter: Option<String>,
    /// Use cargo-nextest instead of Cargo's built-in test runner.
    #[arg(long)]
    pub nextest: bool,
}

#[derive(Debug, Args)]
pub struct CiArgs {
    /// Also run documentation, dependency-policy, and typo checks.
    #[arg(long)]
    pub full: bool,
}

#[derive(Debug, Subcommand)]
pub enum ScaffoldCommand {
    /// Add a library or binary crate.
    Crate {
        /// Suffix appended to the public crate name.
        name: String,
        /// Generate a binary instead of a library.
        #[arg(long)]
        bin: bool,
        /// Mark the crate as unpublished.
        #[arg(long)]
        private: bool,
    },
    /// Add a Clap model crate and executable entrypoint.
    Cli,
    /// Add a GitHub Actions workflow.
    Ci {
        /// Lean uses Cargo only; full also enables policy and typo checks.
        #[arg(long, value_enum, default_value_t)]
        preset: CiPreset,
    },
    /// Add contribution and crate-index documentation.
    Docs,
    /// Add agent instruction files.
    Agents {
        /// Also add CLAUDE.md pointing at AGENTS.md.
        #[arg(long)]
        claude: bool,
    },
}

#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum CiPreset {
    #[default]
    Lean,
    Full,
}

#[derive(Debug, Subcommand)]
pub enum ToolsCommand {
    /// Install an exact tool group under .xtask/tools.
    Sync {
        #[arg(value_enum, default_value_t)]
        group: ToolGroup,
    },
}

#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum ToolGroup {
    Ci,
    Coverage,
    Release,
    Test,
    #[default]
    All,
}

#[derive(Debug, Subcommand)]
pub enum ReleaseCommand {
    /// Create release configuration and initialize dist for binary projects.
    Init,
    /// Ask dist to preview the release plan.
    Plan {
        /// Optional release tag to plan.
        #[arg(long)]
        tag: Option<String>,
    },
    /// Prepare a version, changelog, commit, and tag without local publishing.
    Prepare {
        #[arg(value_enum, default_value_t)]
        level: ReleaseLevel,
        /// Apply changes. The default is cargo-release's dry run.
        #[arg(long)]
        execute: bool,
    },
    #[command(hide = true)]
    Changelog {
        #[arg(long)]
        tag: String,
    },
}

#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum ReleaseLevel {
    #[default]
    Patch,
    Minor,
    Major,
}

impl ReleaseLevel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Patch => "patch",
            Self::Minor => "minor",
            Self::Major => "major",
        }
    }
}
