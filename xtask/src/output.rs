// SPDX-FileCopyrightText: 2026 Meowdia Community
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::Result;
use std::{
    fs,
    io::Write as _,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub const GENERATED: &str = "src/generated.rs";
pub const SNAPSHOTS: &str = "target/iana/snapshots";
pub const CATALOG: &str = "iana/catalog.txt";

pub fn write_if_changed(path: &Path, contents: &str) -> Result<()> {
    if fs::read_to_string(path).ok().as_deref() == Some(contents) {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents).map_err(|error| format!("write {}: {error}", path.display()).into())
}

pub fn format_rust_source(source: &str) -> Result<String> {
    let mut rustfmt = Command::new("rustfmt")
        .args(["--edition", "2024"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|error| format!("failed to spawn `rustfmt`: {error}"))?;

    rustfmt
        .stdin
        .take()
        .expect("piped stdin")
        .write_all(source.as_bytes())?;

    let output = rustfmt.wait_with_output()?;

    if !output.status.success() {
        return Err(format!("`rustfmt` exited unsuccessfully: {}", output.status).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

pub fn project_path(relative_path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join(relative_path)
}
