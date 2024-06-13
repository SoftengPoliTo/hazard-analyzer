use clap::{Args, Parser, Subcommand};
use hazard_analyzer::hazard_analyzer;
use pub_api::pub_apis;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Hazard analyzer command.
    HazardAnalyzer(AnalyzerArgs),

    /// Public API command.
    PubApi(ApisArgs),
}

#[derive(Args)]
struct AnalyzerArgs {
    /// Path to the firmware.
    #[clap(long, short = 'f', required = true, value_hint = clap::ValueHint::DirPath)]
    firmware_path: PathBuf,

    /// Path to the ascot devices.
    #[clap(long, short = 'd', value_hint = clap::ValueHint::DirPath)]
    devices_path: Option<PathBuf>,

    /// Path to the output manifest.
    #[clap(long, short = 'm', required = true, value_hint = clap::ValueHint::FilePath)]
    manifest_path: PathBuf,

    /// If set, the analysis output will not be printed on the terminal.
    #[arg(long, short = 'q', action = clap::ArgAction::SetTrue)]
    quiet: bool,
}

#[derive(Args)]
struct ApisArgs {
    /// Path to the .toml manifest of ascot-library.
    #[clap(long, short = 'l', value_hint = clap::ValueHint::DirPath)]
    library_path: Option<PathBuf>,

    /// Path to the .toml manifest of ascot-axum.
    #[clap(long, short = 'a', value_hint = clap::ValueHint::DirPath)]
    axum_path: Option<PathBuf>,

    /// Path to the output manifest.
    #[clap(long, short = 'm', required = true, value_hint = clap::ValueHint::FilePath)]
    manifest_path: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        // Hazard analyzer command.
        Commands::HazardAnalyzer(args) => {
            hazard_analyzer(
                args.devices_path,
                &args.firmware_path,
                &args.manifest_path,
                args.quiet,
            )
            .unwrap();
        }
        // Public API command.
        Commands::PubApi(args) => {
            pub_apis(args.library_path, args.axum_path, &args.manifest_path).unwrap()
        }
    }
}
