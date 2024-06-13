pub(crate) mod file;
mod methods;
mod new;

use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use code_certifier::error::Result;
use crossbeam::channel::{Receiver, Sender};
use file::DeviceFile;
use methods::find_actions_enum;
use rust_code_analysis::{Node, Rust, Search};
use serde::Serialize;

use crate::{concurrent::ConcurrentRunner, re::HAZARD_RE};

// Mandatory actions that must be performed by an Ascot device.
#[derive(Debug, Serialize)]
pub(crate) enum MandatoryActions<'a> {
    // Mandatory actions for an Ascot device using the new() function to define the actions.
    New(HashMap<usize, DeviceAction<'a>>),
    // Mandatory actions for an Ascot device using method calls to define the actions.
    Methods(Vec<DeviceAction<'a>>),
}

// Ascot device action.
#[derive(Debug, Serialize)]
pub(crate) struct DeviceAction<'a> {
    // Name of the action.
    pub(crate) name: Cow<'a, str>,
    // Hazards of the action.
    pub(crate) hazards: HashSet<&'a str>,
}

// Ascot device.
//
// The parsing of Ascot devices directory will produce
// a list of AscotDevice.
#[derive(Debug, Serialize)]
pub(crate) struct AscotDevice<'a> {
    // Name of the device.
    pub(crate) name: &'a str,
    // Mandatory actions of the device.
    pub(crate) mandatory_actions: MandatoryActions<'a>,
    // Allowed hazards for this device.
    pub(crate) allowed_hazards: HashSet<&'a str>,
}

#[inline(always)]
// Returns `true` if all of the
// following conditions are met:
//
// - `node` is of kind `kind_id`.
// - `node` has a child of kind `child_kind`.
// - child text is equal to `child_text`.
fn node_where_child(
    node: &Node,
    node_kind: Rust,
    child_kind: Rust,
    child_text: &str,
    source_code: &[u8],
) -> bool {
    node.kind_id() == node_kind
        && node
            .first_child(|c| {
                c.kind_id() == child_kind
                    && c.utf8_text(source_code).map_or(false, |t| t == child_text)
            })
            .is_some()
}

// Retrieves the new() function instance of the device impl.
#[inline(always)]
fn get_new_function<'a>(root: Node<'a>, source_code: &'a [u8]) -> Option<Node<'a>> {
    root.first_occurence(|n| {
        node_where_child(n, Rust::FunctionItem, Rust::Identifier, "new", source_code)
    })
}

// Given a text correspondonding to a piece of code of the file, retrieve all the hazards contained in it.
fn hazards_from_text(text: &str) -> Option<HashSet<&str>> {
    let hazards = HAZARD_RE
        .captures_iter(text)
        .filter_map(|capture| capture.get(1).map(|re_match| re_match.as_str()))
        .collect::<HashSet<_>>();

    Some(hazards)
}

// Search the file for a const with name equal to the given `name`.
fn search_const<'a>(root: Node<'a>, name: &str, source_code: &'a [u8]) -> Option<&'a str> {
    // Search the first node occurence of type ConstItem that contains `name` in its name.
    root.first_occurence(|occ| {
        occ.kind_id() == Rust::ConstItem
            && occ
                .utf8_text(source_code)
                .map_or(false, |const_text| const_text.contains(name))
    })
    .and_then(|const_action| const_action.utf8_text(source_code))
}

// Retrieves all the hazards defined in a const definition.
fn hazards_from_const<'a>(
    root: Node<'a>,
    const_name: &str,
    source_code: &'a [u8],
) -> Option<HashSet<&'a str>> {
    // Search if the file contains a const definition with name equal to const_name.
    let action_const = search_const(root, const_name, source_code);

    // If the const exists then retrieve the hazards from its value.
    let hazards = match action_const {
        Some(const_text) => hazards_from_text(const_text)?,
        None => HashSet::new(),
    };

    Some(hazards)
}

#[inline(always)]
// Retrieves the list of allowed hazards.
fn get_allowed_hazards<'a>(root: Node<'a>, source_code: &'a [u8]) -> Option<HashSet<&'a str>> {
    hazards_from_const(root, "ALLOWED_HAZARDS", source_code)
}

// Creates an `AscotDevice` starting from a `DeviceFile`.
fn get_ascot_device(device_file: &DeviceFile) -> Option<AscotDevice> {
    // Get file root.
    let root = device_file.root();

    // Check if the device uses methods or the new() function to define the actions
    // by verifying if it contains `Actions` enum inside the file.
    let ascot_device = if let Some(actions_enum) = find_actions_enum(root, &device_file.source_code)
    {
        // Handle a device that uses methods.
        methods::handle(
            &device_file.name,
            root,
            actions_enum,
            &device_file.source_code,
        )
    } else {
        // Handle a device that uses the new() function.
        new::handle(&device_file.name, root, &device_file.source_code)
    };

    ascot_device
}

// DeviceProducer.
//
// Implements the `ConcurrentRunner` and
// returns the list of `AscotDevice`.
pub(crate) struct DeviceProducer;

impl DeviceProducer {
    pub(crate) const fn new() -> Self {
        Self
    }
}

impl<'a> ConcurrentRunner<'a> for DeviceProducer {
    type Items = &'a [DeviceFile];
    type ProducerItem = &'a DeviceFile;
    type ConsumerItem = AscotDevice<'a>;
    type Output = Vec<AscotDevice<'a>>;

    fn producer(
        &self,
        device_files: Self::Items,
        sender: Sender<Self::ProducerItem>,
    ) -> Result<()> {
        for device_file in device_files {
            sender.send(device_file)?;
        }

        Ok(())
    }

    fn consumer(
        &self,
        receiver: Receiver<Self::ProducerItem>,
        sender: Sender<Self::ConsumerItem>,
    ) -> Result<()> {
        while let Ok(device_file) = receiver.recv() {
            if let Some(ascot_device) = get_ascot_device(device_file) {
                sender.send(ascot_device)?;
            }
        }

        Ok(())
    }

    fn composer(&self, receiver: Receiver<Self::ConsumerItem>) -> Result<Self::Output> {
        let mut ascot_devices = Vec::new();
        while let Ok(ascot_device) = receiver.recv() {
            ascot_devices.push(ascot_device);
        }

        Ok(ascot_devices)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use insta::sorted_redaction;
    use rust_code_analysis::{read_file, RustCode, Tree};

    use crate::concurrent::ConcurrentRunner;

    use super::{AscotDevice, DeviceFile, DeviceProducer};

    const SNAPSHOTS_PATH: &str = "../../tests/snapshots/devices/";

    const NEW_DEVICE: &str = "NewDevice";
    const NEW_DEVICE_PATH: &str = "./tests/devices/new_device.rs";

    const METHODS_DEVICE: &str = "MethodsDevice";
    const METHODS_DEVICE_PATH: &str = "./tests/devices/methods_device.rs";

    const ASCOT_LIGHT: &str = "Light";
    const ASCOT_LIGHT_PATH: &str = "./tests/devices/light.rs";

    const ASCOT_FRIDGE: &str = "Fridge";
    const ASCOT_FRIDGE_PATH: &str = "./tests/devices/fridge.rs";

    struct DeviceData {
        name: String,
        path: &'static Path,
    }

    fn set_device_producer<'a>(device_files: &'a [DeviceFile]) -> Vec<AscotDevice<'a>> {
        DeviceProducer::new()
            .run(&device_files, (rayon::current_num_threads() - 1).max(1))
            .unwrap()
    }

    fn set_device_files(device_files: Vec<DeviceData>) -> Vec<DeviceFile> {
        device_files
            .into_iter()
            .map(|device_data| {
                let source_code = read_file(device_data.path).unwrap();
                let ast = Tree::new::<RustCode>(&source_code);
                DeviceFile {
                    name: device_data.name,
                    source_code,
                    ast,
                }
            })
            .collect()
    }

    #[test]
    fn actions_defined_with_new() {
        let device_files = set_device_files(vec![DeviceData {
            name: NEW_DEVICE.to_string(),
            path: Path::new(NEW_DEVICE_PATH),
        }]);

        let ascot_devices = set_device_producer(&device_files);
        let new_device = ascot_devices.first().unwrap();

        insta::with_settings!({
            snapshot_path => Path::new(SNAPSHOTS_PATH),
            prepend_module_to_snapshot => false,
            sort_maps => true
        }, {
            insta::assert_yaml_snapshot!("actions_defined_with_new", new_device,
                {
                    ".allowed_hazards" => sorted_redaction(),
                    ".mandatory_actions" => sorted_redaction(),
                    ".mandatory_actions.*.hazards" => sorted_redaction()
                })
        });
    }

    #[test]
    fn actions_defined_with_methods() {
        let device_files = set_device_files(vec![DeviceData {
            name: METHODS_DEVICE.to_string(),
            path: Path::new(METHODS_DEVICE_PATH),
        }]);

        let ascot_devices = set_device_producer(&device_files);
        let methods_device = ascot_devices.first().unwrap();

        insta::with_settings!({
            snapshot_path => Path::new(SNAPSHOTS_PATH),
            prepend_module_to_snapshot => false,
            sort_maps => true
        }, {
            insta::assert_yaml_snapshot!("actions_defined_with_methods", methods_device,
                {
                    ".allowed_hazards" => sorted_redaction(),
                    ".mandatory_actions" => sorted_redaction(),
                    ".mandatory_actions.*.hazards" => sorted_redaction(),
                })
        });
    }

    #[test]
    fn test_multiple_devices() {
        let device_files = set_device_files(vec![
            DeviceData {
                name: NEW_DEVICE.to_string(),
                path: Path::new(NEW_DEVICE_PATH),
            },
            DeviceData {
                name: METHODS_DEVICE.to_string(),
                path: Path::new(METHODS_DEVICE_PATH),
            },
        ]);

        let ascot_devices = set_device_producer(&device_files);

        insta::with_settings!({
            snapshot_path => Path::new(SNAPSHOTS_PATH),
            prepend_module_to_snapshot => false,
            sort_maps => true
        }, {
            insta::assert_yaml_snapshot!("multiple_devices", ascot_devices,
                {
                    "." => sorted_redaction(),
                    "[].allowed_hazards" => sorted_redaction(),
                    "[].mandatory_actions" => sorted_redaction(),
                    "[].mandatory_actions.*.hazards" => sorted_redaction(),
                })
        });
    }

    #[test]
    fn ascot_light() {
        let device_files = set_device_files(vec![DeviceData {
            name: ASCOT_LIGHT.to_string(),
            path: Path::new(ASCOT_LIGHT_PATH),
        }]);

        let ascot_devices = set_device_producer(&device_files);
        let new_device = ascot_devices.first().unwrap();

        insta::with_settings!({
            snapshot_path => Path::new(SNAPSHOTS_PATH),
            prepend_module_to_snapshot => false,
            sort_maps => true
        }, {
            insta::assert_yaml_snapshot!("ascot_light", new_device,
                {
                    ".allowed_hazards" => sorted_redaction(),
                    ".mandatory_actions" => sorted_redaction(),
                    ".mandatory_actions.*.hazards" => sorted_redaction()
                })
        });
    }

    #[test]
    fn ascot_fridge() {
        let device_files = set_device_files(vec![DeviceData {
            name: ASCOT_FRIDGE.to_string(),
            path: Path::new(ASCOT_FRIDGE_PATH),
        }]);

        let ascot_devices = set_device_producer(&device_files);
        let new_device = ascot_devices.first().unwrap();

        insta::with_settings!({
            snapshot_path => Path::new(SNAPSHOTS_PATH),
            prepend_module_to_snapshot => false,
            sort_maps => true
        }, {
            insta::assert_yaml_snapshot!("ascot_fridge", new_device,
                {
                    ".allowed_hazards" => sorted_redaction(),
                    ".mandatory_actions" => sorted_redaction(),
                    ".mandatory_actions.*.hazards" => sorted_redaction()
                })
        });
    }
}
