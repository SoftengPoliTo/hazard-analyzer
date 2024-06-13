use std::collections::HashSet;
use std::io::Write;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::firmware::FileManifest;

const RED: Color = Color::Rgb(232, 72, 85);
const YELLOW: Color = Color::Rgb(249, 220, 92);
const GREEN: Color = Color::Rgb(147, 255, 150);
const BLUE: Color = Color::Rgb(13, 31, 45);
const GREY: Color = Color::Rgb(247, 247, 249);
const CYAN: Color = Color::Rgb(84, 106, 123);

fn write_hazards(
    stdout: &mut StandardStream,
    color: Color,
    hazards: &HashSet<&str>,
) -> std::io::Result<()> {
    if !hazards.is_empty() {
        write!(stdout, "{:indent$}hazards: ", "", indent = 16)?;
        stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
        writeln!(
            stdout,
            "{}",
            hazards.iter().copied().collect::<Vec<_>>().join(", ")
        )?;
        stdout.reset()?;
    }

    Ok(())
}

fn write_not_allowed_hazards(
    stdout: &mut StandardStream,
    not_allowed_hazards: &HashSet<String>,
) -> std::io::Result<()> {
    if !not_allowed_hazards.is_empty() {
        write_colored(
            stdout,
            RED,
            16,
            &format!(
                "not allowed hazards: {}",
                not_allowed_hazards
                    .iter()
                    .map(|nah| nah.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        )?;
    }

    Ok(())
}

fn write_colored(
    stdout: &mut StandardStream,
    color: Color,
    indent: usize,
    content: &str,
) -> std::io::Result<()> {
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    writeln!(stdout, "{:indent$}{}", "", content, indent = indent)?;
    stdout.reset()
}

pub(crate) fn print_manifest(manifest: &[FileManifest]) -> std::io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Write files.
    for file in manifest.iter().filter(|file| !file.devices.is_empty()) {
        write_colored(&mut stdout, BLUE, 0, &format!("\n{}", file.file.display()))?;

        // Write devices.
        for device in &file.devices {
            stdout.set_color(ColorSpec::new().set_fg(Some(CYAN)))?;
            write!(stdout, "{:4}{} ", "", &device.name)?;
            stdout.reset()?;
            writeln!(stdout, "({}, {})", device.position.0, device.position.1)?;

            write_colored(
                &mut stdout,
                Color::Ansi256(15),
                8,
                "defined mandatory actions:",
            )?;

            // Write defined mandatory actions.
            for action in &device.mandatory_actions {
                write_colored(&mut stdout, GREY, 12, action.name)?;
                write_hazards(&mut stdout, GREEN, &action.hazards)?;
                write_not_allowed_hazards(&mut stdout, &action.not_allowed_hazards)?;

                if !action.missing_hazards.is_empty() {
                    write_colored(
                        &mut stdout,
                        RED,
                        16,
                        &format!(
                            "missing hazards: {}",
                            action
                                .missing_hazards
                                .iter()
                                .map(|mh| mh.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    )?;
                }
            }

            // Write missing mandatory actions.
            if let Some(missing_actions) = &device.missing_mandatory_actions {
                if !missing_actions.is_empty() {
                    write_colored(
                        &mut stdout,
                        RED,
                        12,
                        &format!(
                            "missing mandatory actions: {}",
                            missing_actions.to_vec().join(", ")
                        ),
                    )?;
                }
            }

            // Write optional actions.
            write_colored(&mut stdout, Color::Ansi256(15), 8, "optional actions:")?;
            for action in &device.optional_actions {
                write_colored(&mut stdout, GREY, 12, action.name)?;
                write_hazards(&mut stdout, YELLOW, &action.hazards)?;
                write_not_allowed_hazards(&mut stdout, &action.not_allowed_hazards)?;
            }
        }
    }
    Ok(())
}
