use std::process::Command;

pub fn readme() {
    let status = Command::new("uv")
        .args(["run", "scripts/generate-crate-readmes.py"])
        .status()
        .unwrap_or_else(|e| panic!("failed to run generate-crate-readmes.py: {e}"));
    if !status.success() {
        eprintln!("hint: install uv — https://docs.astral.sh/uv/");
        std::process::exit(status.code().unwrap_or(1));
    }
}
