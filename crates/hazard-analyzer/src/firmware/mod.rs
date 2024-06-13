pub(crate) mod file;

use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use code_certifier::error::Result;
use crossbeam::channel::{Receiver, Sender};
use file::FirmwareFile;
use rust_code_analysis::{Node, Rust, Search};
use serde::Serialize;

use crate::{
    concurrent::ConcurrentRunner,
    device::{AscotDevice, DeviceAction, MandatoryActions},
    re::{method_re, ARGS_RE, HAZARD_RE},
};

// MandatoryAction.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MandatoryAction<'a> {
    // Action name.
    pub(crate) name: &'a str,
    // Action hazards.
    pub(crate) hazards: HashSet<&'a str>,
    // Mandatory hazards that should be set for this action.
    pub(crate) mandatory_hazards: &'a HashSet<&'a str>,
    // Mandatory actions that have not been defined.
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub(crate) missing_hazards: HashSet<String>,
    // Hazards that are not allowed for the device.
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub(crate) not_allowed_hazards: HashSet<String>,
}

// OptionalAction.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OptionalAction<'a> {
    // Action name.
    pub(crate) name: &'a str,
    // Action hazards.
    pub(crate) hazards: HashSet<&'a str>,
    // Hazards that are not allowed for the device.
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    pub(crate) not_allowed_hazards: HashSet<String>,
}

// Instance of a device in the firmware.
struct DeviceInstance<'a> {
    // Code.
    code: &'a str,
    // Row and column of the instance inside the firmware file.
    position: (usize, usize),
}

// Device.
//
// Represents a device found
// in the firmware.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Device<'a> {
    // Device name.
    pub(crate) name: &'a str,
    // Row and column of the device instance inside the firmware file.
    pub(crate) position: (usize, usize),
    // Defined mandatory actions.
    pub(crate) mandatory_actions: Vec<MandatoryAction<'a>>,
    // Mandatory actions that have not been defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) missing_mandatory_actions: Option<Vec<&'a str>>,
    // Defined optional actions.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) optional_actions: Vec<OptionalAction<'a>>,
    // Allowed hazards for this device.
    pub(crate) allowed_hazards: &'a HashSet<&'a str>,
}

// FileManifest.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FileManifest<'a> {
    // File path.
    pub(crate) file: &'a Path,
    // File devices.
    pub(crate) devices: Vec<Device<'a>>,
}

// Returns all the hazards containd in a piece of code.
// It will for example retrieve "FireHazard" and "PowerSurge" from:
//
// add_action(DeviceAction::with_hazards(toggle_config, toggle, &[Hazard::FireHazard, Hazard::PowerSurge]))
fn get_hazards(code: &str) -> Option<HashSet<&str>> {
    let hazards = HAZARD_RE
        .captures_iter(code)
        .filter_map(|capture| capture.get(1).map(|m| m.as_str()))
        .collect::<HashSet<_>>();

    Some(hazards)
}

// Returns the difference between first_set and second_set.
fn sets_difference(first_set: &HashSet<&str>, second_set: &HashSet<&str>) -> HashSet<String> {
    first_set
        .difference(second_set)
        .map(|e| e.to_string())
        .collect()
}

fn get_optional_actions<'a>(
    device_instance: &'a str,
    ascot_device: &'a AscotDevice<'a>,
) -> Option<Vec<OptionalAction<'a>>> {
    // Get optional actions by searching all the add_action() calls of a device instance.
    let optional_actions = method_re("add_action")?
        .captures_iter(device_instance)
        .filter_map(|captures| captures.get(1).map(|re_match| re_match.as_str()))
        .filter_map(|add_action_args| {
            // Get hazards.
            let hazards = get_hazards(add_action_args)?;

            // Get optional action name by extracting the second (nth(1)) argument from add_action().
            let name = crate::re::ARGS_RE
                .captures(add_action_args)?
                .get(1)?
                .as_str()
                .split(',')
                .nth(1)
                .map(|name| name.trim())?;

            // Get eventual not allowed hazards.
            let not_allowed_hazards = sets_difference(&hazards, &ascot_device.allowed_hazards);

            Some(OptionalAction {
                name,
                hazards,
                not_allowed_hazards,
            })
        })
        .collect();

    Some(optional_actions)
}

// Retrieves the method call code that has name equal to the given `method_name`.
fn get_method_call<'a>(device_instance: &'a str, method_name: &str) -> Option<&'a str> {
    method_re(method_name)?
        .captures(device_instance)
        .and_then(|c| c.get(1))
        .map(|c| c.as_str())
}

// Get mandatory actions defined inside DeviceName::new() function call.
fn get_new_actions<'a>(
    actions: &'a HashMap<usize, DeviceAction<'a>>,
    new_call: &'a str,
    allowed_hazards: &'a HashSet<&'a str>,
) -> Option<Vec<MandatoryAction<'a>>> {
    // Get all DeviceAction::..() arguments inside DeviceName::new() call.
    let mandatory_actions = ARGS_RE
        .captures_iter(new_call)
        .enumerate()
        .filter_map(|(pos, capture)| {
            let argument = capture.get(1)?.as_str();

            // Get the action corresponding to the argument by
            // searching in the map the action which has argument number equal to `pos`.
            let action = actions.get(&pos)?;

            // Get hazards.
            let hazards = get_hazards(argument)?;

            // Get missing hazards.
            let missing_hazards = sets_difference(&action.hazards, &hazards);

            // Get eventual not allowed hazards.
            let not_allowed_hazards = sets_difference(&hazards, allowed_hazards);

            Some(MandatoryAction {
                name: &action.name,
                hazards,
                mandatory_hazards: &action.hazards,
                missing_hazards,
                not_allowed_hazards,
            })
        })
        .collect();

    Some(mandatory_actions)
}

// Get mandatory actions defined with methods calls.
fn get_methods_actions<'a>(
    actions: &[&'a DeviceAction],
    device_instance: &DeviceInstance<'a>,
    ascot_device: &AscotDevice,
) -> Option<Vec<MandatoryAction<'a>>> {
    // Build mandatory actions objects starting from defined actions.
    let mandatory_actions = actions
        .iter()
        .filter_map(|action| {
            // Get method call code to `.mandatory_action()`.
            let action_method_call = get_method_call(device_instance.code, &action.name)?;

            // Get hazards.
            let hazards = get_hazards(action_method_call)?;

            // Get missing hazards.
            let missing_hazards = sets_difference(&action.hazards, &hazards);

            // Get eventual not allowed hazards.
            let not_allowed_hazards = sets_difference(&hazards, &ascot_device.allowed_hazards);

            Some(MandatoryAction {
                name: &action.name,
                hazards,
                mandatory_hazards: &action.hazards,
                missing_hazards,
                not_allowed_hazards,
            })
        })
        .collect();

    Some(mandatory_actions)
}

fn build_device<'a>(
    device_instance: DeviceInstance<'a>,
    ascot_device: &'a AscotDevice<'a>,
) -> Option<Device<'a>> {
    let (mandatory_actions, missing_mandatory_actions) = match &ascot_device.mandatory_actions {
        MandatoryActions::New(actions) => {
            // Get text of DeviceName::new() method call.
            let new_call =
                get_method_call(device_instance.code, &format!("{}::new", ascot_device.name))?;

            // Get defined mandatory actions from new() method call.
            let mandatory_actions =
                get_new_actions(actions, new_call, &ascot_device.allowed_hazards)?;

            (mandatory_actions, None)
        }
        MandatoryActions::Methods(actions) => {
            // Search which actions have been defined and which are missing
            // by checking if there is a method call where method name is equal to device action name.
            let (defined, missing): (Vec<_>, Vec<_>) = actions
                .iter()
                .partition(|action| get_method_call(device_instance.code, &action.name).is_some());

            // Build mandatory actions objects starting from defined actions.
            let mandatory_actions = get_methods_actions(&defined, &device_instance, ascot_device)?;

            // Get missing mandatory actions names.
            let missing_mandatory_actions = missing
                .into_iter()
                .map(|action| action.name.as_ref())
                .collect();

            (mandatory_actions, Some(missing_mandatory_actions))
        }
    };

    // Get optional actions.
    let optional_actions = get_optional_actions(device_instance.code, ascot_device)?;

    Some(Device {
        name: ascot_device.name,
        position: device_instance.position,
        mandatory_actions,
        missing_mandatory_actions,
        optional_actions,
        allowed_hazards: &ascot_device.allowed_hazards,
    })
}

fn get_device_instances<'a>(
    root: Node<'a>,
    ascot_device: &AscotDevice,
    source_code: &'a [u8],
) -> Vec<DeviceInstance<'a>> {
    // Get all nodes of type CallExpression to a DeviceName::new(), where DeviceName is
    // ascot_device.name.
    // To make sure that the node corresponds to the DeviceName::new() root node of the device instance,
    // we have to ensure that it does not have other ancestors of type CallExpression.
    let new_device_instances = root.all_occurrences(|n| {
        n.kind_id() == Rust::CallExpression
            && n.utf8_text(source_code).map_or(false, |t| {
                t.contains(&format!("{}::new", ascot_device.name))
            })
            && !n.has_ancestor(|a| a.kind_id() == Rust::CallExpression)
    });

    // Create a DeviceInstance object with the device instance code, start row and start column.
    new_device_instances
        .into_iter()
        .filter_map(|n| {
            let instance = n.utf8_text(source_code)?;
            Some(DeviceInstance {
                code: instance,
                position: n.start_position(),
            })
        })
        .collect()
}

fn get_file_manifest<'a>(
    firmware_file: &'a FirmwareFile,
    ascot_devices: &'a [AscotDevice],
) -> Option<FileManifest<'a>> {
    let root = firmware_file.root();

    // For each ascot device search in the file all the instances an build the
    // corresponding Device object.
    let devices: Vec<_> = ascot_devices
        .iter()
        .flat_map(|ascot_device| {
            get_device_instances(root, ascot_device, &firmware_file.source_code)
                .into_iter()
                .filter_map(|instance| build_device(instance, ascot_device))
        })
        .collect();

    // Build a FileManifest only if the file instantiates at least one device.
    (!devices.is_empty()).then_some(FileManifest {
        file: &firmware_file.path,
        devices,
    })
}

pub(crate) struct Analyzer<'a>(&'a [AscotDevice<'a>]);

impl<'a> Analyzer<'a> {
    pub(crate) const fn new(ascot_devices: &'a [AscotDevice<'a>]) -> Self {
        Self(ascot_devices)
    }
}

impl<'a> ConcurrentRunner<'a> for Analyzer<'a> {
    type Items = &'a [FirmwareFile<'a>];
    type ProducerItem = &'a FirmwareFile<'a>;
    type ConsumerItem = FileManifest<'a>;
    type Output = Vec<FileManifest<'a>>;

    fn producer(
        &self,
        firmware_files: Self::Items,
        sender: Sender<Self::ProducerItem>,
    ) -> Result<()> {
        for fw_file in firmware_files {
            sender.send(fw_file)?;
        }

        Ok(())
    }

    fn consumer(
        &self,
        receiver: Receiver<Self::ProducerItem>,
        sender: Sender<Self::ConsumerItem>,
    ) -> Result<()> {
        while let Ok(firmware_file) = receiver.recv() {
            if let Some(file_manifest) = get_file_manifest(firmware_file, self.0) {
                sender.send(file_manifest)?;
            }
        }

        Ok(())
    }

    fn composer(&self, receiver: Receiver<Self::ConsumerItem>) -> Result<Self::Output> {
        let mut manifest = Vec::new();
        while let Ok(file_manifest) = receiver.recv() {
            manifest.push(file_manifest);
        }

        Ok(manifest)
    }
}
