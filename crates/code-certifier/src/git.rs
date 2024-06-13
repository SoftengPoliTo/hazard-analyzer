//! This module handles the cloning of `ascot-firmware`.

use std::{env::temp_dir, path::PathBuf};

use git2::Repository;
use toml::Value;

use crate::error::{Error, ErrorKind, Result};

const CARGO_TOML: &str = include_str!("../Cargo.toml");

/// Clones ascot-firmware (https://github.com/SoftengPoliTo/ascot-firmware)
/// inside the temporary directory and returns the path to it.
pub fn ascot_firmware() -> Result<PathBuf> {
    // Parse Cargo.toml file.
    let cargo_toml: Value = CARGO_TOML.parse()?;

    // Get ascot-firmware repository URL.
    let repo_url = cargo_toml
        .get("package")
        .and_then(|pkg| pkg.get("metadata"))
        .and_then(|metadata| metadata.get("ascot-firmware"))
        .and_then(|ascot_firmware| ascot_firmware.get("url"))
        .and_then(|url| url.as_str())
        .ok_or(Error::new(
            ErrorKind::Git,
            "Invalid ascot-firmware repository URL Cargo.toml",
        ))?;

    // Clone ascot-firmware in the temporary directory.
    let repo_path = temp_dir().join("ascot-firmware");
    if repo_path.exists() {
        std::fs::remove_dir_all(&repo_path)?;
    }
    Repository::clone(repo_url, &repo_path)?;

    Ok(repo_path)
}

/// Returns `ascot-firmware/ascot-axum/src/devices` path.
#[inline(always)]
pub fn ascot_devices(ascot_firmware: PathBuf) -> PathBuf {
    ascot_firmware
        .join("ascot-axum")
        .join("src")
        .join("devices")
}
