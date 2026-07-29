use crate::Result;
use crate::cli::TestArgs;
use crate::process;
use crate::tools::{self, Tool};
use crate::workspace::Workspace;

pub fn check(workspace: &Workspace) -> Result {
    cargo(workspace, ["fmt", "--all", "--check"])?;
    cargo(workspace, ["check", "--workspace", "--all-targets"])?;
    cargo(
        workspace,
        [
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )
}

pub fn test(workspace: &Workspace, args: &TestArgs) -> Result {
    check(workspace)?;
    if args.nextest {
        let mut nextest = vec!["run", "--workspace"];
        if let Some(filter) = &args.filter {
            nextest.push(filter);
        }
        tools::execute(workspace, Tool::Nextest, nextest)?;
        cargo(workspace, ["test", "--workspace", "--doc"])
    } else {
        let mut test = vec!["test", "--workspace"];
        if let Some(filter) = &args.filter {
            test.push(filter);
        }
        cargo(workspace, test)
    }
}

pub fn build(workspace: &Workspace) -> Result {
    test(
        workspace,
        &TestArgs {
            filter: None,
            nextest: false,
        },
    )?;
    cargo(workspace, ["build", "--workspace", "--release"])
}

pub fn ci(workspace: &Workspace, full: bool) -> Result {
    test(
        workspace,
        &TestArgs {
            filter: None,
            nextest: false,
        },
    )?;
    cargo(workspace, ["doc", "--workspace", "--no-deps"])?;
    if full {
        tools::execute(workspace, Tool::Deny, ["check"])?;
        tools::execute(workspace, Tool::Typos, ["."])?;
    }
    Ok(())
}

pub fn coverage(workspace: &Workspace) -> Result {
    tools::execute(
        workspace,
        Tool::Coverage,
        ["llvm-cov", "--workspace", "--html"],
    )
}

fn cargo<I, S>(workspace: &Workspace, args: I) -> Result
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    process::run(&workspace.root, "cargo", args)
}
