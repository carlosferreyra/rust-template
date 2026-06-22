use crate::flags::Publish;
use std::process::Command;

pub fn publish(cmd: Publish) {
    let level = cmd.level.as_deref().unwrap_or("patch");
    let mut args = vec!["release", level, "--workspace", "--no-publish"];
    if cmd.execute {
        args.push("--execute");
    }
    let status = Command::new("cargo")
        .args(&args)
        .status()
        .unwrap_or_else(|e| panic!("failed to run cargo release: {e}"));
    if !status.success() {
        eprintln!("hint: install with `cargo install cargo-release`");
        std::process::exit(status.code().unwrap_or(1));
    }
}
