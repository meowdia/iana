// SPDX-FileCopyrightText: 2026 Meowdia Community
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::{
    Result,
    output::{project_path, write_if_changed},
};
use std::{
    collections::BTreeSet,
    fs,
    process::{Command, Stdio},
};

pub fn run() -> Result<()> {
    let root = project_path("");
    let git = |args: &[&str]| -> Result<String> {
        Ok(String::from_utf8(
            Command::new("git")
                .args(args)
                .current_dir(&root)
                .output()?
                .stdout,
        )?)
    };
    let changed = git(&["diff", "--name-only", "HEAD", "--", "src", "crates"])?;
    let added = git(&[
        "ls-files",
        "--others",
        "--exclude-standard",
        "--",
        "src",
        "crates",
    ])?;
    let mut crates = BTreeSet::new();
    for path in changed
        .lines()
        .chain(added.lines())
        .filter(|path| path.ends_with(".rs"))
    {
        if path.starts_with("src/") {
            crates.insert("Cargo.toml".to_owned());
        } else if let Some((family, _)) = path
            .strip_prefix("crates/")
            .and_then(|path| path.split_once("/src/"))
        {
            crates.insert(format!("crates/{family}/Cargo.toml"));
        }
    }
    if crates.is_empty() {
        return Ok(());
    }
    let baseline = git(&["show", "HEAD:Cargo.toml"])?;
    for path in crates {
        let previous = git(&["show", &format!("HEAD:{path}")])?;
        let path = root.join(path);
        if !path.exists() || previous.is_empty() {
            continue;
        }
        let previous = package_version(&previous)
            .unwrap_or_else(|| section_version(&baseline, "[workspace.package]").unwrap());
        let manifest = fs::read_to_string(&path)?;
        if package_version(&manifest).is_some_and(|current| current != previous) {
            continue;
        }
        let (prefix, _) = previous.rsplit_once('.').unwrap();
        let (major, minor) = prefix.split_once('.').unwrap();
        let next = format!("{major}.{}.0", minor.parse::<u64>()? + 1);
        let field = manifest
            .lines()
            .find(|line| line.starts_with("version = ") || *line == "version.workspace = true")
            .unwrap();
        write_if_changed(
            &path,
            &manifest.replacen(field, &format!("version = \"{next}\""), 1),
        )?;
    }
    Command::new("cargo")
        .args(["metadata", "--offline", "--format-version", "1"])
        .current_dir(&root)
        .stdout(Stdio::null())
        .status()?;
    Ok(())
}

pub fn package_version(manifest: &str) -> Option<&str> {
    section_version(manifest, "[package]")
}
fn section_version<'a>(manifest: &'a str, section: &str) -> Option<&'a str> {
    manifest
        .split_once(section)?
        .1
        .lines()
        .skip(1)
        .take_while(|line| !line.starts_with('['))
        .find_map(|line| line.strip_prefix("version = \"")?.strip_suffix('"'))
}
