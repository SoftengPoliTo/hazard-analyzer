//! This module handles output manifest creation.

use std::path::Path;

use serde::Serialize;
use crate::error::{Error, ErrorKind, Result};

/// Checks if manifest path is a `JSON` file.
pub fn check_manifest_path(manifest_path: &Path) -> Result<()> {
    if manifest_path
        .extension()
        .map_or(false, |ext| ext.to_ascii_lowercase() != "json")
    {
        return Err(Error::new(
            ErrorKind::Io,
            "Manifest path must be a json file",
        ));
    }

    Ok(())
}

/// Creates the `JSON` manifest.
pub fn create_manifest<S: Serialize>(manifest: &S, manifest_path: &Path) -> Result<()> {
    let json = serde_json::to_string(&manifest)?;
    std::fs::write(manifest_path, json.as_bytes())?;

    Ok(())
}
