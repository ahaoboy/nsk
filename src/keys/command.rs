//! The `nsk keys install` command.

use std::path::PathBuf;

use clap::{Args, Subcommand};

use super::{KeyStore, KeysPackage};
use crate::error::Result;

/// Key commands.
#[derive(Debug, Args)]
pub struct KeysCmd {
    /// Key subcommand to run.
    #[command(subcommand)]
    pub command: KeysCommand,
}

/// Key subcommands.
#[derive(Debug, Subcommand)]
pub enum KeysCommand {
    /// Install key files from a package into a key directory.
    Install(InstallArgs),
}

/// Arguments for `nsk keys install`.
#[derive(Debug, Args)]
pub struct InstallArgs {
    /// Key package to install (a `.zip` file).
    #[arg(value_name = "PACKAGE")]
    pub package: PathBuf,

    /// Directory that receives the key files.
    #[arg(value_name = "TARGET")]
    pub target: PathBuf,

    /// Keep key files that are already present instead of replacing them.
    #[arg(long)]
    pub skip_existing: bool,
}

/// Run a key command.
pub fn run(cmd: KeysCmd) -> Result<()> {
    match cmd.command {
        KeysCommand::Install(args) => install(args),
    }
}

/// Install a key package and report what happened.
fn install(args: InstallArgs) -> Result<()> {
    let mut package = KeysPackage::open(&args.package)?;
    let store = KeyStore::new(&args.target);
    let report = package.install(&store, !args.skip_existing)?;

    println!(
        "Installed {} key file(s) into {} ({:.1} KiB)",
        report.written.len(),
        store.root().display(),
        report.bytes_written() as f64 / 1024.0,
    );
    if !report.skipped.is_empty() {
        println!(
            "Skipped {} key file(s) that are already present",
            report.skipped.len()
        );
    }
    Ok(())
}
