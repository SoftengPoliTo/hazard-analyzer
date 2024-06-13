use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use rustdoc_types::{Crate, Id, ItemEnum};
use serde::Serialize;

// Data structure impl.
//
// Can be a data structure implementation or
// a trait implementation.
enum Impl<'a> {
    // Data structure implementation with associated functions.
    // The key corresponds to the id of the function inside json rustdoc.
    Functions(HashMap<&'a Id, &'a str>),
    // Trait implementation.
    Trait(TraitImpl<'a>),
}

// Trait implementaion.
#[derive(Serialize)]
pub(crate) struct TraitImpl<'a> {
    // Trait name.
    name: &'a str,
    // Trait functions.
    // Contains the provided (with deafult implementation) functions
    // and the ones that have to be implemented (required).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    functions: Vec<&'a str>,
}

// Data structure.
pub(crate) struct DataStructure<'a> {
    // Name.
    pub(crate) name: &'a str,
    // File.
    pub(crate) file: &'a Path,
    // Implemented functions.
    pub(crate) functions: HashMap<&'a Id, &'a str>,
    // Implemented traits.
    pub(crate) traits: Vec<TraitImpl<'a>>,
}

// Data structure representation inside output manifest.
#[derive(Serialize)]
pub(crate) struct ApiDataStructure<'a> {
    // Name.
    pub(crate) name: &'a str,
    // File.
    pub(crate) file: &'a Path,
    // Implemented functions.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) functions: Vec<&'a str>,
    // Implemented traits.
    pub(crate) traits: Vec<TraitImpl<'a>>,
}

impl<'a> From<DataStructure<'a>> for ApiDataStructure<'a> {
    fn from(data_structure: DataStructure<'a>) -> ApiDataStructure<'a> {
        Self {
            name: data_structure.name,
            file: data_structure.file,
            functions: data_structure.functions.values().copied().collect(),
            traits: data_structure.traits,
        }
    }
}

pub(crate) fn api_data_structures(data_structures: Vec<DataStructure>) -> Vec<ApiDataStructure> {
    data_structures.into_iter().map(|ds| ds.into()).collect()
}

// Retrieves the functions associated to an impl instance.
// They can be the functions related to a data structure implementation
// or the ones related to a trait impl by a data structure.
fn get_functions<'a, T, M, C>(doc_crate: &'a Crate, impl_: &'a rustdoc_types::Impl, map: M) -> C
where
    M: Fn(&'a Id, &'a str) -> T,
    C: FromIterator<T>,
{
    impl_
        .items
        .iter()
        .filter_map(|id| {
            let item = doc_crate.index.get(id)?;
            match item.inner {
                ItemEnum::Function(_) => {
                    let func_name = item.name.as_ref()?;
                    Some(map(&item.id, func_name))
                }
                _ => None,
            }
        })
        .collect()
}

// Retrieves functions defined in a data structure impl block.
fn ds_functions<'a>(
    doc_crate: &'a Crate,
    impl_: &'a rustdoc_types::Impl,
) -> HashMap<&'a Id, &'a str> {
    get_functions(doc_crate, impl_, |id, func_name| (id, func_name))
}

fn trait_functions<'a>(doc_crate: &'a Crate, impl_: &'a rustdoc_types::Impl) -> Vec<&'a str> {
    // Required functions and reimplementations of provided functions.
    let mut functions: HashSet<&str> = get_functions(doc_crate, impl_, |_, func_name| func_name);

    // Extend the functions list with the functions provided by the Trait that have not been reimplemented.
    functions.extend(impl_.provided_trait_methods.iter().map(|s| s.as_str()));

    functions.into_iter().collect()
}

fn get_impl<'a>(doc_crate: &'a Crate, impl_: &'a rustdoc_types::Impl) -> Impl<'a> {
    // Check if impl corresponds to a data structure implementation or to a trait one,
    // and create the corresponding Impl object.
    if let Some(trait_) = &impl_.trait_ {
        let functions = trait_functions(doc_crate, impl_);
        Impl::Trait(TraitImpl {
            name: &trait_.name,
            functions,
        })
    } else {
        let functions = ds_functions(doc_crate, impl_);
        Impl::Functions(functions)
    }
}

// Get implemented functions from array of data structure implementations.
fn implemented_functions(functions: Vec<Impl>) -> HashMap<&Id, &str> {
    functions
        .into_iter()
        .filter_map(|impl_| match impl_ {
            Impl::Functions(functions) => Some(functions),
            _ => None,
        })
        .flatten()
        .collect()
}

// Get implemented traits from array of data structure implementations.
fn implemented_traits(traits: Vec<Impl>) -> Vec<TraitImpl> {
    traits
        .into_iter()
        .filter_map(|impl_| match impl_ {
            Impl::Trait(trait_) => Some(trait_),
            _ => None,
        })
        .collect()
}

// Returns map of data structure implemented functions and array
// of implemented traits.
fn get_impls<'a>(
    doc_crate: &'a Crate,
    impls: &'a [Id],
) -> (HashMap<&'a Id, &'a str>, Vec<TraitImpl<'a>>) {
    // Retrieve impl.
    let retrieve_impl = |impl_: &Id| -> Option<Impl> {
        doc_crate
            .index
            .get(impl_)
            .and_then(|item| match &item.inner {
                ItemEnum::Impl(impl_item) => Some(get_impl(doc_crate, impl_item)),
                _ => None,
            })
    };

    // Distinguish between data structure implemented functions and implemented traits.
    let (functions, traits) = impls
        .iter()
        .filter_map(retrieve_impl)
        .partition::<Vec<_>, _>(|impl_| matches!(impl_, Impl::Functions(_)));

    // Get implemented functions.
    let functions = implemented_functions(functions);

    // Get implemented traits.
    let traits = implemented_traits(traits);

    (functions, traits)
}

pub(crate) fn get_data_structure<'a>(
    name: &'a str,
    file: &'a Path,
    impls: &'a [Id],
    doc_crate: &'a Crate,
) -> DataStructure<'a> {
    let (functions, traits) = get_impls(doc_crate, impls);

    DataStructure {
        name,
        file,
        functions,
        traits,
    }
}
