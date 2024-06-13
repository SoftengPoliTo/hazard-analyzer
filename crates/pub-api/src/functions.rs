use std::{collections::HashSet, path::Path};

use rustdoc_types::{Crate, Id, Item, ItemEnum, Visibility};
use serde::Serialize;

use super::ds::DataStructure;

// Function.
#[derive(Serialize)]
pub(crate) struct Function<'a> {
    // Name.
    name: &'a str,
    // File.
    file: &'a Path,
}

fn get_function<'a>(id: &Id, item: &'a Item, ds_functions: &HashSet<&Id>) -> Option<Function<'a>> {
    // Filter out the item if it is not from local crate OR if it is not public OR
    // if it is a function from a data structure impl.
    if item.crate_id != 0 || item.visibility != Visibility::Public || ds_functions.contains(&id) {
        return None;
    }

    // Check if item is actually a function and build the corresponding Function object.
    match item.inner {
        ItemEnum::Function(_) => {
            let function_name = item.name.as_ref()?;
            let file = &item.span.as_ref()?.filename;
            Some(Function {
                name: function_name.as_str(),
                file,
            })
        }
        _ => None,
    }
}

// Retrieves public functions by taking all crate public functions
// and filtering out those that are not linked to a Trait or to the impl
// of a data structure.
//
// The JSON output of rustdoc does not in fact provide a direct way
// to distinguish between plain functions and functions contained in impl blocks.
pub(crate) fn get_functions<'a>(
    doc_crate: &'a Crate,
    structs: &[DataStructure<'a>],
    enums: &[DataStructure<'a>],
) -> Vec<Function<'a>> {
    // Functions to filter out.
    let ds_functions: HashSet<_> = structs
        .iter()
        .chain(enums.iter())
        .flat_map(|ds| ds.functions.keys())
        .copied()
        .collect();

    // Get functions.
    doc_crate
        .index
        .iter()
        .filter_map(|(id, item)| get_function(id, item, &ds_functions))
        .collect()
}
