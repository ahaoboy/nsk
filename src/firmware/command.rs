//! The `nsk firmware install` command.

use std::path::PathBuf;

use clap::{Args, Subcommand};

use super::FirmwarePackage;
use crate::error::Result;
use crate::registry::{RegisteredCache, RegistryLayout};

/// Firmware commands.
#[derive(Debug, Args)]
pub struct FirmwareCmd {
    /// Firmware subcommand to run.
    #[command(subcommand)]
    pub command: FirmwareCommand,
}

/// Firmware subcommands.
#[derive(Debug, Subcommand)]
pub enum FirmwareCommand {
    /// Install a firmware package into a registered content directory.
    Install(InstallArgs),
}

/// Arguments for `nsk firmware install`.
#[derive(Debug, Args)]
pub struct InstallArgs {
    /// Firmware package to install (a `.zip` file).
    #[arg(value_name = "PACKAGE")]
    pub package: PathBuf,

    /// Directory that receives the installed content.
    #[arg(value_name = "TARGET")]
    pub target: PathBuf,

    /// Directory layout the emulator expects.
    #[arg(short, long, value_enum, default_value = "flat")]
    pub layout: RegistryLayout,

    /// Keep NCAs that are already present instead of replacing them.
    #[arg(long)]
    pub skip_existing: bool,
}

/// Run a firmware command.
pub fn run(cmd: FirmwareCmd) -> Result<()> {
    match cmd.command {
        FirmwareCommand::Install(args) => install(args),
    }
}

/// Install a firmware package and report what happened.
fn install(args: InstallArgs) -> Result<()> {
    let mut package = FirmwarePackage::open(&args.package)?;
    let cache = RegisteredCache::new(&args.target);
    let report = package.install(&cache, args.layout, !args.skip_existing)?;

    println!(
        "Installed {} NCA file(s) into {} ({}, {:.1} MiB)",
        report.written.len(),
        cache.root().display(),
        args.layout,
        report.bytes_written() as f64 / (1024.0 * 1024.0),
    );
    if !report.skipped.is_empty() {
        println!(
            "Skipped {} NCA file(s) that are already present",
            report.skipped.len()
        );
    }
    Ok(())
}
