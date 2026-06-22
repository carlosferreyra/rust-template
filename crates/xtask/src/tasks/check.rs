use std::process::Command;

pub fn check() {
    run(&["fmt", "--all", "--check"]);
    run(&["check", "--workspace"]);
    run(&[
        "clippy",
        "--workspace",
        "--all-targets",
        "--",
        "-D",
        "warnings",
    ]);
}

pub(crate) fn run(args: &[&str]) {
    let status = Command::new("cargo")
        .args(args)
        .status()
        .unwrap_or_else(|e| panic!("failed to run cargo: {e}"));
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}
