// This module defines functions
// for managing devices
// that use the new() function to define the mandatory actions.

use std::collections::HashMap;

use rust_code_analysis::{Node, Rust, Search};

use super::{
    get_allowed_hazards, get_new_function, hazards_from_const, AscotDevice, DeviceAction,
    MandatoryActions,
};

// Retrieves new() parameters names, which correspond to the device action names.
fn get_new_parameters<'a>(new_function: Node<'a>, source_code: &'a [u8]) -> Vec<&'a str> {
    // Find the Parameters node.
    let parameters = new_function.first_child(|child| child.kind_id() == Rust::Parameters);

    // For each Parameter get its Identifier (action name).
    parameters
        .map_or(Vec::new(), |parameters| {
            parameters.all_occurrences(|occ| occ.kind_id() == Rust::Parameter)
        })
        .into_iter()
        .filter_map(|parameter| {
            parameter
                .first_child(|child| child.kind_id() == Rust::Identifier)
                .and_then(|identifier| identifier.utf8_text(source_code))
        })
        .collect()
}

fn get_mandatory_actions<'a>(
    root: Node<'a>,
    new_function: Node<'a>,
    source_code: &'a [u8],
) -> HashMap<usize, DeviceAction<'a>> {
    // Get mandatory actions names.
    let actions_names = get_new_parameters(new_function, source_code);

    // For each action search the mandatory hazards associated to it.
    actions_names
        .into_iter()
        .enumerate()
        .filter_map(|(param_num, name)| {
            let hazards = hazards_from_const(root, &name.to_ascii_uppercase(), source_code)?;
            Some((
                param_num,
                DeviceAction {
                    name: name.into(),
                    hazards,
                },
            ))
        })
        .collect()
}

// Creates an `AscotDevice` starting from a device that uses
// the new() function to define the mandatory actions.
pub(crate) fn handle<'a>(
    device_name: &'a str,
    root: Node<'a>,
    source_code: &'a [u8],
) -> Option<AscotDevice<'a>> {
    // Get allowed hazards.
    let allowed_hazards = get_allowed_hazards(root, source_code)?;

    // Get mandatory actions.
    let new_function = get_new_function(root, source_code)?;
    let mandatory_actions = get_mandatory_actions(root, new_function, source_code);

    Some(AscotDevice {
        name: device_name,
        mandatory_actions: MandatoryActions::New(mandatory_actions),
        allowed_hazards,
    })
}
