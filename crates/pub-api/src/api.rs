use std::path::{Path, PathBuf};

use crate::ds::{api_data_structures, get_data_structure, ApiDataStructure};
use crate::functions::{get_functions, Function};
use crate::macros::Macro;
use crate::traits::{get_trait, Trait};
use rustdoc_types::{Crate, ItemEnum, Visibility};
use serde::Serialize;

use code_certifier::error::{Error, ErrorKind, Result};

use code_certifier::git::ascot_firmware;

const LIB_TOML_PATH: &str = "Cargo.toml";
const AXUM_TOML_PATH: &str = "ascot-axum/Cargo.toml";

// APIs.
#[derive(Serialize)]
struct Api<'a> {
    // Structs.
    structs: Vec<ApiDataStructure<'a>>,
    // Enums.
    enums: Vec<ApiDataStructure<'a>>,
    // Traits.
    traits: Vec<Trait<'a>>,
    // Functions.
    functions: Vec<Function<'a>>,
    // Macros.
    macros: Vec<Macro<'a>>,
}

// Public APIs.
#[derive(Serialize)]
pub(crate) struct PublicApis<'a> {
    // ascot-library public APIs.
    ascot_library: Api<'a>,
    // ascot-axum public APIs.
    ascot_axum: Api<'a>,
}

fn ascot_paths(
    library_path: Option<PathBuf>,
    axum_path: Option<PathBuf>,
) -> Result<(PathBuf, PathBuf)> {
    match (library_path, axum_path) {
        (None, None) => {
            let ascot_firmware = ascot_firmware()?;
            Ok((
                ascot_firmware.join(LIB_TOML_PATH),
                ascot_firmware.join(AXUM_TOML_PATH),
            ))
        }
        (None, Some(axum_path)) => {
            let ascot_firmware = ascot_firmware()?;
            Ok((ascot_firmware.join(LIB_TOML_PATH), axum_path))
        }
        (Some(library_path), None) => {
            let ascot_firmware = ascot_firmware()?;
            Ok((library_path, ascot_firmware.join(AXUM_TOML_PATH)))
        }
        (Some(library_path), Some(axum_path)) => Ok((library_path, axum_path)),
    }
}

fn rustdoc_crate<M: AsRef<Path>>(manifest_path: M) -> Result<Crate> {
    // Create json rustdoc for the project corresponding to the given manifest_path.
    let doc_path = rustdoc_json::Builder::default()
        .toolchain("nightly")
        .manifest_path(manifest_path)
        .build()?;

    let doc = std::fs::read_to_string(doc_path)?;

    // Extract the documentation Crate.
    let doc_crate: Crate = serde_json::from_str(&doc)?;

    Ok(doc_crate)
}

pub(crate) fn doc_crates(
    library_path: Option<PathBuf>,
    axum_path: Option<PathBuf>,
) -> Result<(Crate, Crate)> {
    let (library_path, axum_path) = ascot_paths(library_path, axum_path)?;

    Ok((rustdoc_crate(library_path)?, rustdoc_crate(axum_path)?))
}

fn get_api(doc_crate: &Crate) -> Api {
    let mut structs = vec![];
    let mut enums = vec![];
    let mut traits = vec![];
    let mut macros = vec![];

    // Explore one by one the crate public items and create the corresponding objects.
    doc_crate
        .index
        .iter()
        .filter(|(_, item)| item.crate_id == 0 && item.visibility == Visibility::Public)
        .for_each(|(_, item)| {
            if let (Some(name), Some(span)) = (&item.name, &item.span) {
                match &item.inner {
                    ItemEnum::Struct(struct_item) => {
                        let struct_ =
                            get_data_structure(name, &span.filename, &struct_item.impls, doc_crate);
                        structs.push(struct_);
                    }
                    ItemEnum::Enum(enum_item) => {
                        let enum_ =
                            get_data_structure(name, &span.filename, &enum_item.impls, doc_crate);
                        enums.push(enum_);
                    }
                    ItemEnum::Trait(trait_item) => {
                        let trait_ = get_trait(name, &span.filename, trait_item, doc_crate);
                        traits.push(trait_);
                    }
                    ItemEnum::Macro(_) => macros.push(Macro {
                        name,
                        file: &span.filename,
                    }),
                    _ => (),
                }
            }
        });

    // Retrieve the public functions of the project.
    let functions = get_functions(doc_crate, &structs, &enums);

    // Convert structs and enums to the manifest representation.
    let structs = api_data_structures(structs);
    let enums = api_data_structures(enums);

    Api {
        structs,
        enums,
        traits,
        functions,
        macros,
    }
}

pub(crate) fn apis<'a>(library_crate: &'a Crate, axum_crate: &'a Crate) -> PublicApis<'a> {
    let library_apis = get_api(library_crate);
    let axum_apis = get_api(axum_crate);

    PublicApis {
        ascot_library: library_apis,
        ascot_axum: axum_apis,
    }
}

pub(crate) fn check_ascot_path(ascot_path: &Path) -> Result<()> {
    // Check if ascot_path is a path to a .toml manifest.
    if ascot_path
        .extension()
        .map_or(false, |ext| ext.to_ascii_lowercase() != "toml")
    {
        return Err(Error::new(
            ErrorKind::Io,
            "Ascot path must be a path to the .toml manifest.",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use insta::sorted_redaction;

    use super::{get_api, rustdoc_crate};

    const SNAPSHOTS_PATH: &str = "../tests/snapshots/";

    fn test_apis(snapshot_name: &str, manifest_path: &Path) {
        let doc_crate = rustdoc_crate(manifest_path).unwrap();
        let apis = get_api(&doc_crate);

        insta::with_settings!({
            snapshot_path => Path::new(SNAPSHOTS_PATH),
            prepend_module_to_snapshot => false,
            sort_maps => true
        }, {
            insta::assert_yaml_snapshot!(snapshot_name, apis,
                {
                    ".structs" => sorted_redaction(),
                    ".structs.*.functions" => sorted_redaction(),
                    ".structs.*.traits" => sorted_redaction(),
                    ".structs.*.traits.*.functions" => sorted_redaction(),
                    ".enums" => sorted_redaction(),
                    ".enums.*.functions" => sorted_redaction(),
                    ".enums.*.traits" => sorted_redaction(),
                    ".enums.*.traits.*.functions" => sorted_redaction(),
                    ".traits" => sorted_redaction(),
                    ".traits.*.functions" => sorted_redaction(),
                    ".functions" => sorted_redaction(),
                    ".macros" => sorted_redaction()
                })
        });
    }

    #[test]
    fn structs() {
        test_apis("structs", Path::new("./tests/projects/structs/Cargo.toml"));
    }

    #[test]
    fn enums() {
        test_apis("enums", Path::new("./tests/projects/enums/Cargo.toml"));
    }

    #[test]
    fn traits() {
        test_apis("traits", Path::new("./tests/projects/traits/Cargo.toml"));
    }

    #[test]
    fn functions() {
        test_apis(
            "functions",
            Path::new("./tests/projects/functions/Cargo.toml"),
        );
    }

    #[test]
    fn macros() {
        test_apis("macros", Path::new("./tests/projects/macros/Cargo.toml"));
    }

    #[test]
    fn impl_traits() {
        test_apis(
            "impl_traits",
            Path::new("./tests/projects/impl_traits/Cargo.toml"),
        );
    }

    #[test]
    fn derive_traits() {
        test_apis(
            "derive_traits",
            Path::new("./tests/projects/derive_traits/Cargo.toml"),
        );
    }

    #[test]
    fn aliases() {
        test_apis("aliases", Path::new("./tests/projects/aliases/Cargo.toml"));
    }
}
