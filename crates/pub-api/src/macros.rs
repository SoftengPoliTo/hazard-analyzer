use std::path::Path;

use serde::Serialize;

// Macro.
#[derive(Serialize)]
pub(crate) struct Macro<'a> {
    // Name.
    pub(crate) name: &'a str,
    // File.
    pub(crate) file: &'a Path,
}
