use std::path::Path;

use rustdoc_types::{Crate, ItemEnum};
use serde::Serialize;

// Trait.
#[derive(Serialize)]
pub(crate) struct Trait<'a> {
    // Name.
    name: &'a str,
    // File.
    file: &'a Path,
    // Functions.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    functions: Vec<&'a str>,
}

pub(crate) fn get_trait<'a>(
    name: &'a str,
    file: &'a Path,
    trait_: &'a rustdoc_types::Trait,
    doc_crate: &'a Crate,
) -> Trait<'a> {
    // Get trait functions by getting the name of all
    // items of type Function contained in trait.items.
    let functions: Vec<_> = trait_
        .items
        .iter()
        .filter_map(|item| {
            doc_crate
                .index
                .get(item)
                .and_then(|item| match item.inner {
                    ItemEnum::Function(_) => item.name.as_ref(),
                    _ => None,
                })
                .map(|name| name.as_str())
        })
        .collect();

    Trait {
        name,
        file,
        functions,
    }
}
