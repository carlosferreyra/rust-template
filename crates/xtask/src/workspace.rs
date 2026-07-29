use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use toml_edit::DocumentMut;

use crate::Result;

pub struct Workspace {
    pub root: PathBuf,
    pub public_crate: String,
    pub version: String,
}

impl Workspace {
    pub fn discover() -> Result<Self> {
        let start = env::current_dir()
            .map_err(|error| format!("failed to read current directory: {error}"))?;
        let root = start
            .ancestors()
            .find(|directory| {
                fs::read_to_string(directory.join("Cargo.toml"))
                    .is_ok_and(|manifest| manifest.contains("[workspace]"))
            })
            .map(Path::to_path_buf)
            .ok_or("run this command inside the project workspace")?;
        let manifest = read_document(&root.join("Cargo.toml"))?;
        let public_crate = manifest["workspace"]["metadata"]["xtask"]["public-crate"]
            .as_str()
            .ok_or("workspace.metadata.xtask.public-crate must be set")?
            .to_owned();
        let version = manifest["workspace"]["package"]["version"]
            .as_str()
            .ok_or("workspace.package.version must be set")?
            .to_owned();
        Ok(Self {
            root,
            public_crate,
            version,
        })
    }

    pub fn path(&self, path: impl AsRef<Path>) -> PathBuf {
        self.root.join(path)
    }

    pub fn manifest(&self) -> Result<DocumentMut> {
        read_document(&self.path("Cargo.toml"))
    }

    pub fn has_publishable_binary(&self) -> Result<bool> {
        for entry in fs::read_dir(self.path("crates"))
            .map_err(|error| format!("failed to read workspace crates: {error}"))?
        {
            let directory = entry
                .map_err(|error| format!("failed to read workspace crate: {error}"))?
                .path();
            let manifest_path = directory.join("Cargo.toml");
            if !manifest_path.is_file() {
                continue;
            }
            let manifest = read_document(&manifest_path)?;
            let unpublished = manifest
                .get("package")
                .and_then(toml_edit::Item::as_table)
                .and_then(|package| package.get("publish"))
                .and_then(toml_edit::Item::as_bool)
                == Some(false);
            if unpublished {
                continue;
            }
            let conventional =
                directory.join("src/main.rs").is_file() || directory.join("src/bin").is_dir();
            let explicit = manifest
                .get("bin")
                .and_then(toml_edit::Item::as_array_of_tables)
                .is_some_and(|targets| !targets.is_empty());
            if conventional || explicit {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

pub fn read(path: &Path) -> Result<String> {
    fs::read_to_string(path).map_err(|error| format!("failed to read {}: {error}", path.display()))
}

pub fn read_document(path: &Path) -> Result<DocumentMut> {
    read(path)?
        .parse()
        .map_err(|error| format!("failed to parse {}: {error}", path.display()))
}
