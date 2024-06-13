// This module defines functions
// for managing devices
// that use method calls to define the mandatory actions.

use rust_code_analysis::{Node, Rust, Search};

use super::{
    get_allowed_hazards, hazards_from_const, node_where_child, AscotDevice, DeviceAction,
    MandatoryActions,
};

// Retrieves `Actions` enum.
pub(crate) fn find_actions_enum<'a>(root: Node<'a>, source_code: &[u8]) -> Option<Node<'a>> {
    root.first_child(|node| {
        node_where_child(
            node,
            Rust::EnumItem,
            Rust::TypeIdentifier,
            "Actions",
            source_code,
        )
    })
}

// Converts `Actions` `enum_variant` from CamelCase to SNAKE_CASE to match const name that defines
// the hazards.
fn to_snake_case(enum_variant: &str) -> String {
    let mut action = String::new();
    for (i, ch) in enum_variant.chars().enumerate() {
        if ch.is_uppercase() {
            if i != 0 {
                action.push('_');
            }
            action.push(ch.to_ascii_lowercase());
        } else {
            action.push(ch);
        }
    }
    action
}

// Retrieves all the mandatory actions names by parsing the `Actions` enum.
fn get_actions_names(actions_enum: Node, source_code: &[u8]) -> Vec<String> {
    actions_enum
        .all_occurrences(|occ| occ.kind_id() == Rust::Identifier)
        .into_iter()
        .filter_map(|identifier| identifier.utf8_text(source_code).map(to_snake_case))
        .collect()
}

fn get_mandatory_actions<'a>(
    root: Node<'a>,
    actions_names: Vec<String>,
    source_code: &'a [u8],
) -> Vec<DeviceAction<'a>> {
    actions_names
        .into_iter()
        .filter_map(|name| {
            let hazards = hazards_from_const(root, &name.to_ascii_uppercase(), source_code)?;
            Some(DeviceAction {
                name: name.into(),
                hazards,
            })
        })
        .collect()
}

// Creates an `AscotDevice` starting from a device that uses
// methods to define the mandatory actions.
pub(crate) fn handle<'a>(
    device_name: &'a str,
    root: Node<'a>,
    actions_enum: Node,
    source_code: &'a [u8],
) -> Option<AscotDevice<'a>> {
    // // Get allowed hazards.
    let allowed_hazards = get_allowed_hazards(root, source_code)?;

    // Get mandatory actions.
    let actions_names = get_actions_names(actions_enum, source_code);
    let mandatory_actions = get_mandatory_actions(root, actions_names, source_code);

    Some(AscotDevice {
        name: device_name,
        mandatory_actions: MandatoryActions::Methods(mandatory_actions),
        allowed_hazards,
    })
}
