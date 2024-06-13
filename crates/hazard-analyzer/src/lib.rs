#![deny(missing_docs, unsafe_code)]

//! # `hazard-analyzer`
//! Analyzes the source code of a smart home firmware implemented using the
//! `Ascot` interface to certify whether it is `Ascot` compliant or not,
//! according to certain conditions.

mod concurrent;
mod device;
mod firmware;
mod print;
mod re;

use std::path::Path;

use code_certifier::error::Result;
use code_certifier::git::{ascot_devices, ascot_firmware};
use code_certifier::manifest::{check_manifest_path, create_manifest};
use concurrent::ConcurrentRunner;
use device::{file::get_device_files, DeviceProducer};
use firmware::{file::get_fw_files, Analyzer};
use print::print_manifest;

/// Creates a json manifest with all
/// the device instances inside the firmware.
///
/// - `devices_path`: path to **ascot-firmware/ascot-axum/devices/**.
/// - `firmware_path`: path to the firmware to analyze.
/// - `manifest_path`: path to the output manifest. It should be a `JSON` file.
/// - `quiet`: if set to true, the analysis output will not be printed on the terminal.
///
/// If `devices_path` is `None` the tool will clone the
/// [ascot-firmware](https://github.com/SoftengPoliTo/ascot-firmware) repository
/// and use the **ascot-axum/devices/** inside it.
pub fn hazard_analyzer<D: AsRef<Path>>(
    devices_path: Option<D>,
    firmware_path: &Path,
    manifest_path: &Path,
    quiet: bool,
) -> Result<()> {
    // Check manifest path.
    check_manifest_path(manifest_path)?;

    // Set number of threads.
    let n_threads = (rayon::current_num_threads() - 1).max(1);

    // Get device files.
    let device_files = match devices_path {
        Some(devices_path) => get_device_files(devices_path.as_ref())?,
        None => {
            let ascot_firmware = ascot_firmware()?;
            let devices_path = ascot_devices(ascot_firmware);
            get_device_files(&devices_path)?
        }
    };

    // Get ascot devices.
    let ascot_devices = DeviceProducer::new().run(&device_files, n_threads)?;

    // Get firmware files.
    let firmware_files = get_fw_files(firmware_path)?;

    // Get the manifest.
    let manifest = Analyzer::new(&ascot_devices).run(&firmware_files, n_threads)?;

    // Print the manifest.
    if !quiet {
        print_manifest(&manifest)?;
    }

    // Create the manifest json.
    create_manifest(&manifest, manifest_path)
}
