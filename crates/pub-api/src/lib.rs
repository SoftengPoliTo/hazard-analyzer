#![deny(missing_docs, unsafe_code)]

//! # `pub-api`
//! Extracts the public `APIs` from `Ascot` framework `ascot-library` and `ascot-axum`.

mod api;
mod ds;
mod functions;
mod macros;
mod traits;

use api::{apis, check_ascot_path, doc_crates};
use code_certifier::error::Result;
use code_certifier::manifest::{check_manifest_path, create_manifest};
use std::path::{Path, PathBuf};

/// Creates a json manifest with all
/// the public `APIs` of **ascot-library** and **ascot-axum**.
///
/// - `library_path`: path to Cargo.toml of **ascot-library**.
/// - `axum_path`: path to Cargo.toml of **ascot-axum**.
/// - `manifest_path`: path to the output manifest. It should be a `JSON` file.
///
/// If `library_path` or `axum_path` is `None` the tool will use those inside
/// [ascot-firmware](https://github.com/SoftengPoliTo/ascot-firmware) repository.
pub fn pub_apis(
    library_path: Option<PathBuf>,
    axum_path: Option<PathBuf>,
    manifest_path: &Path,
) -> Result<()> {
    // Check library_path.
    if let Some(path) = library_path.as_ref() {
        check_ascot_path(path)?;
    }

    // Check axum_path.
    if let Some(path) = library_path.as_ref() {
        check_ascot_path(path)?;
    }

    // Check manifest_path.
    check_manifest_path(manifest_path)?;

    // Get ascot-library and ascot-axum rustdoc crates from the JSON doc.
    let (library_doc_crate, axum_doc_crate) = doc_crates(library_path, axum_path)?;

    // Get the manifest of public APIs.
    let manifest = apis(&library_doc_crate, &axum_doc_crate);

    // Create the manifest json.
    create_manifest(&manifest, manifest_path)?;

    Ok(())
}
