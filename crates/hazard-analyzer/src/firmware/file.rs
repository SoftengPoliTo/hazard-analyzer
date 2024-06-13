use std::{borrow::Cow, path::Path};

use rust_code_analysis::{read_file, Node, RustCode, Tree};

use code_certifier::error::Result;

// Firmware file.
pub(crate) struct FirmwareFile<'a> {
    // File path.
    pub(crate) path: Cow<'a, Path>,
    // File source code.
    pub(crate) source_code: Vec<u8>,
    // File AST.
    ast: Tree,
}

impl<'a> FirmwareFile<'a> {
    fn new(path: Cow<'a, Path>) -> Result<Self> {
        let source_code = read_file(path.as_ref())?;
        let ast = Tree::new::<RustCode>(&source_code);

        Ok(Self {
            path,
            source_code,
            ast,
        })
    }

    #[inline(always)]
    pub(crate) fn root(&self) -> Node {
        self.ast.get_root()
    }
}

#[inline(always)]
fn is_rust_file(path: &Path) -> bool {
    path.extension().and_then(|ext| ext.to_str()) == Some("rs")
}

pub(crate) fn get_fw_files(fw_path: &Path) -> Result<Vec<FirmwareFile>> {
    // `firmware_path` is a single file.
    if fw_path.is_file() && is_rust_file(fw_path) {
        let fw_file = FirmwareFile::new(fw_path.into())?;
        return Ok(vec![fw_file]);
    }

    // `firmware_path` is a directory, so we have to retrieve
    // all Rust files inside it.
    let mut fw_files = Vec::new();
    let mut stack = vec![fw_path.to_path_buf()];

    while let Some(current_dir) = stack.pop() {
        if let Ok(entries) = std::fs::read_dir(current_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path.to_path_buf());
                } else if is_rust_file(&path) {
                    let fw_file = FirmwareFile::new(path.into())?;
                    fw_files.push(fw_file);
                }
            }
        }
    }

    Ok(fw_files)
}
