//! Command-line interface.

use clap::{Parser, Subcommand};

use crate::error::Result;
use crate::firmware::{self, FirmwareCmd};
use crate::sak::{self, SakCommand};

/// Toolbox for Nintendo Switch content: firmware installation and format
/// conversion.
#[derive(Debug, Parser)]
#[command(name = "nsk", version, about, long_about = None)]
pub struct Cli {
    /// Subcommand to run.
    #[command(subcommand)]
    pub command: Command,
}

/// Top-level subcommands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Install firmware packages.
    Firmware(FirmwareCmd),
    /// Switch Army Knife style content operations.
    #[command(flatten)]
    Sak(SakCommand),
}

/// Run the parsed command line.
pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Firmware(cmd) => firmware::run(cmd),
        Command::Sak(cmd) => sak::run(cmd),
    }
}
