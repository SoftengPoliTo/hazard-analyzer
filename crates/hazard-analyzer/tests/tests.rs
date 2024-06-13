use hazard_analyzer::hazard_analyzer;
use insta::sorted_redaction;
use serde_json::Value;
use std::{env::temp_dir, fs, path::Path};

const SNAPSHOTS_PATH: &str = "./snapshots/firmwares/";
const TEST_DEVICES_PATH: &str = "./tests/devices/";

#[test]
fn ascot_light() {
    compare(
        "ascot_light",
        Path::new(TEST_DEVICES_PATH),
        Path::new("./tests/firmwares/ascot_light.rs"),
    )
}

#[test]
fn ascot_fridge() {
    compare(
        "ascot_fridge",
        Path::new(TEST_DEVICES_PATH),
        Path::new("./tests/firmwares/ascot_fridge.rs"),
    )
}

#[test]
fn firmware_with_new_device() {
    compare(
        "with_new_device",
        Path::new(TEST_DEVICES_PATH),
        Path::new("./tests/firmwares/with_new_device.rs"),
    )
}

#[test]
fn firmware_with_methods_device() {
    compare(
        "with_methods_device",
        Path::new(TEST_DEVICES_PATH),
        Path::new("./tests/firmwares/with_methods_device.rs"),
    )
}

#[test]
fn firmware_with_multiple_devices() {
    compare(
        "with_multiple_devices",
        Path::new(TEST_DEVICES_PATH),
        Path::new("./tests/firmwares/with_multiple_devices.rs"),
    )
}

#[test]
fn firmware_without_mandatory_actions() {
    compare(
        "without_mandatory_actions",
        Path::new(TEST_DEVICES_PATH),
        Path::new("./tests/firmwares/without_mandatory_actions.rs"),
    )
}

#[test]
fn firmware_without_mandatory_hazards() {
    compare(
        "without_mandatory_hazards",
        Path::new(TEST_DEVICES_PATH),
        Path::new("./tests/firmwares/without_mandatory_hazards.rs"),
    )
}

#[test]
fn firmware_with_not_allowed_hazards() {
    compare(
        "with_not_allowed_hazards",
        Path::new(TEST_DEVICES_PATH),
        Path::new("./tests/firmwares/with_not_allowed_hazards.rs"),
    )
}

fn compare(snapshot_name: &str, devices_path: &Path, firmware_path: &Path) {
    let output_path = temp_dir().join(Path::new(snapshot_name));

    hazard_analyzer(Some(devices_path), firmware_path, &output_path, true).unwrap();

    let manifest_str = fs::read_to_string(&output_path).unwrap();
    let manifest: Value = serde_json::from_str(&manifest_str).unwrap();

    insta::with_settings!({
        snapshot_path => Path::new(SNAPSHOTS_PATH),
        prepend_module_to_snapshot => false,
    },{
        insta::assert_yaml_snapshot!(snapshot_name, manifest, {
            "." => sorted_redaction(),
            "[].devices" => sorted_redaction(),
            "[].devices.*.mandatoryActions" => sorted_redaction(),
            "[].devices.*.mandatoryActions.*.hazards" => sorted_redaction(),
            "[].devices.*.mandatoryActions.*.mandatoryHazards" => sorted_redaction(),
            "[].devices.*.mandatoryActions.*.missingHazards" => sorted_redaction(),
            "[].devices.*.mandatoryActions.*.notAllowdHazards" => sorted_redaction(),
            "[].devices.*.missingMandatoryActions" => sorted_redaction(),
            "[].devices.*.optionalActions" => sorted_redaction(),
            "[].devices.*.optionalActions.*.hazards" => sorted_redaction(),
            "[].devices.*.optionalActions.*.notAllowedHazards" => sorted_redaction(),
            "[].devices.*.allowedHazards" => sorted_redaction(),
        });
    });
}
