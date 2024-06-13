use std::path::Path;

use rust_code_analysis::{read_file, Node, RustCode, Tree};

use code_certifier::error::Result;

// Ascot device file.
pub(crate) struct DeviceFile {
    // Device name.
    pub(crate) name: String,
    // File source code.
    pub(crate) source_code: Vec<u8>,
    // File AST.
    pub(crate) ast: Tree,
}

impl DeviceFile {
    #[inline(always)]
    fn new(name: &str, source_code: Vec<u8>, ast: Tree) -> Self {
        Self {
            name: to_camel_case(name),
            source_code,
            ast,
        }
    }

    #[inline(always)]
    pub(crate) fn root(&self) -> Node {
        self.ast.get_root()
    }
}

// Retrieves the device name by converting
// file name from snake_case to CamelCase.
fn to_camel_case(file_name: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for ch in file_name.chars() {
        if ch == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }

    result
}

// Retrieves list of `DeviceFile` starting from `devices_path`.
pub(crate) fn get_device_files(devices_path: &Path) -> Result<Vec<DeviceFile>> {
    let device_files = std::fs::read_dir(devices_path)?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            let name = path.file_stem()?.to_str()?;
            if name == "mod" {
                return None;
            }
            let source_code = read_file(&path).ok()?;
            let ast = Tree::new::<RustCode>(&source_code);
            Some(DeviceFile::new(name, source_code, ast))
        })
        .collect();

    Ok(device_files)
}
