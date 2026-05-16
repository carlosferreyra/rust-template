use std::process::Command;

pub fn coverage() {
    let status = Command::new("cargo")
        .args(["llvm-cov", "--workspace", "--html"])
        .status()
        .unwrap_or_else(|e| panic!("failed to run cargo llvm-cov: {e}"));
    if !status.success() {
        eprintln!("hint: install with `cargo install cargo-llvm-cov`");
        std::process::exit(status.code().unwrap_or(1));
    }
}
