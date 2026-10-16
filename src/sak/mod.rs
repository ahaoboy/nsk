//! Switch Army Knife style content operations.
//!
//! These commands are flattened into the top-level CLI, so they run as
//! `nsk <command>` rather than under a `sak` subcommand. The argument structs
//! already fix the interface (inputs, outputs and options) that the real
//! implementations will use, while every handler currently only reports that it
//! is not implemented yet.

use std::path::PathBuf;

use clap::{Args, Subcommand};

use crate::error::Result;

/// Report a reserved command without executing it.
macro_rules! unsupported {
    ($command:literal, $args:expr) => {{
        eprintln!("note: `{}` is reserved but not implemented yet", $command);
        eprintln!("      prepared parameters: {:?}", &$args);
        Err(crate::error::Error::NotImplemented { command: $command })
    }};
}

/// Run a Switch Army Knife style command.
pub fn run(cmd: SakCommand) -> Result<()> {
    match cmd {
        SakCommand::PatchXci(args) => patch_xci(args),
        SakCommand::SplitXci(args) => split_xci(args),
        SakCommand::MergeXci(args) => merge_xci(args),
        SakCommand::UpdateXci(args) => update_xci(args),
        SakCommand::XciToNsp(args) => xci_to_nsp(args),
        SakCommand::CompressXci(args) => compress_xci(args),
        SakCommand::DecompressXcz(args) => decompress_xcz(args),
        SakCommand::ExtractFirmware(args) => extract_firmware(args),
        SakCommand::PatchNsp(args) => patch_nsp(args),
        SakCommand::SplitNsp(args) => split_nsp(args),
        SakCommand::MergeNsp(args) => merge_nsp(args),
        SakCommand::UpdateNsp(args) => update_nsp(args),
        SakCommand::NspToXci(args) => nsp_to_xci(args),
        SakCommand::CompressNsp(args) => compress_nsp(args),
        SakCommand::DecompressNsz(args) => decompress_nsz(args),
        SakCommand::ExtractSave(args) => extract_save(args),
        SakCommand::Forwarder(args) => forwarder(args),
        SakCommand::FormatSd(args) => format_sd(args),
    }
}

/// Switch Army Knife style operations, flattened into the top-level CLI.
#[derive(Debug, Subcommand)]
pub enum SakCommand {
    /// Patch an XCI so it boots without valid signatures.
    PatchXci(ContentArgs),
    /// Split an XCI into 4 GiB parts.
    SplitXci(SplitArgs),
    /// Merge split XCI parts back into one file.
    MergeXci(MergeArgs),
    /// Merge game updates and DLC into an XCI.
    UpdateXci(UpdateArgs),
    /// Convert an XCI into an NSP.
    XciToNsp(ContentArgs),
    /// Compress an XCI into an XCZ.
    CompressXci(CompressArgs),
    /// Decompress an XCZ back into an XCI.
    DecompressXcz(ContentArgs),
    /// Extract the firmware from an XCI.
    ExtractFirmware(ExtractArgs),

    /// Patch an NSP so it installs without valid signatures.
    PatchNsp(ContentArgs),
    /// Split an NSP into 4 GiB parts.
    SplitNsp(SplitArgs),
    /// Merge split NSP parts back into one file.
    MergeNsp(MergeArgs),
    /// Merge game updates and DLC into an NSP.
    UpdateNsp(UpdateArgs),
    /// Convert an NSP into an XCI.
    NspToXci(ContentArgs),
    /// Compress an NSP into an NSZ.
    CompressNsp(CompressArgs),
    /// Decompress an NSZ back into an NSP.
    DecompressNsz(ContentArgs),

    /// Extract Switch savegames copied from the user partition.
    ExtractSave(ExtractArgs),
    /// Build an installable NSP forwarder from an NRO.
    Forwarder(ForwarderArgs),
    /// Format a device as FAT32.
    FormatSd(FormatSdArgs),
}

/// Common arguments for an operation that converts or patches one container.
#[derive(Debug, Args)]
pub struct ContentArgs {
    /// Input container (`.xci` or `.nsp`).
    #[arg(value_name = "INPUT")]
    pub input: PathBuf,

    /// Output file. Defaults to a name derived from the input.
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// `prod.keys` used to decrypt content.
    #[arg(short, long, value_name = "FILE")]
    pub keys: Option<PathBuf>,
}

/// Arguments for splitting a container into parts.
#[derive(Debug, Args)]
pub struct SplitArgs {
    /// Input container (`.xci` or `.nsp`).
    #[arg(value_name = "INPUT")]
    pub input: PathBuf,

    /// Maximum size of each part, in bytes.
    #[arg(
        short = 's',
        long,
        value_name = "BYTES",
        default_value_t = 4_294_967_295
    )]
    pub part_size: u64,

    /// Directory that receives the parts. Defaults to the input directory.
    #[arg(short, long, value_name = "DIR")]
    pub output: Option<PathBuf>,
}

/// Arguments for merging split parts.
#[derive(Debug, Args)]
pub struct MergeArgs {
    /// Split parts, in order.
    #[arg(value_name = "PART", required = true, num_args = 1..)]
    pub parts: Vec<PathBuf>,

    /// Reassembled container to write.
    #[arg(short, long, value_name = "FILE")]
    pub output: PathBuf,
}

/// Arguments for merging updates and DLC into a container.
#[derive(Debug, Args)]
pub struct UpdateArgs {
    /// Input container (`.xci` or `.nsp`).
    #[arg(value_name = "INPUT")]
    pub input: PathBuf,

    /// Update NSPs to merge in.
    #[arg(long = "update", value_name = "NSP", required = true)]
    pub updates: Vec<PathBuf>,

    /// DLC NSPs to merge in.
    #[arg(long = "dlc", value_name = "NSP")]
    pub dlcs: Vec<PathBuf>,

    /// Output file. Defaults to a name derived from the input.
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// `prod.keys` used to decrypt content.
    #[arg(short, long, value_name = "FILE")]
    pub keys: Option<PathBuf>,
}

/// Arguments for compressing a container.
#[derive(Debug, Args)]
pub struct CompressArgs {
    /// Input container (`.xci` or `.nsp`).
    #[arg(value_name = "INPUT")]
    pub input: PathBuf,

    /// Compressed file to write. Defaults to a name derived from the input.
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Compression level (1 = fastest, 22 = smallest).
    #[arg(short = 'l', long, default_value_t = 18)]
    pub level: u8,
}

/// Arguments for extracting data out of a container.
#[derive(Debug, Args)]
pub struct ExtractArgs {
    /// Input container or archive.
    #[arg(value_name = "INPUT")]
    pub input: PathBuf,

    /// Directory that receives the extracted files.
    #[arg(short, long, value_name = "DIR")]
    pub output: PathBuf,
}

/// Arguments for building an NRO forwarder.
#[derive(Debug, Args)]
pub struct ForwarderArgs {
    /// Homebrew executable (`.nro`).
    #[arg(value_name = "NRO")]
    pub nro: PathBuf,

    /// Title shown on the home menu.
    #[arg(long, value_name = "NAME")]
    pub name: String,

    /// Author shown on the home menu.
    #[arg(long, value_name = "AUTHOR")]
    pub author: String,

    /// Icon (`.png`, 256x256).
    #[arg(long, value_name = "PNG")]
    pub icon: Option<PathBuf>,

    /// Installable NSP to create.
    #[arg(short, long, value_name = "FILE")]
    pub output: PathBuf,
}

/// Arguments for formatting a device as FAT32.
#[derive(Debug, Args)]
pub struct FormatSdArgs {
    /// Target device or mount point (e.g. `\\.\PhysicalDrive2` or `/dev/sdb`).
    #[arg(value_name = "DEVICE")]
    pub device: String,

    /// Volume label.
    #[arg(long, default_value = "NSK")]
    pub label: String,

    /// Cluster size in bytes.
    #[arg(long, value_name = "BYTES", default_value_t = 32_768)]
    pub cluster_size: u32,
}

/// Patch an XCI and write a copy that boots without valid signatures.
pub fn patch_xci(args: ContentArgs) -> Result<()> {
    unsupported!("patch-xci", args)
}

/// Split an XCI into parts no larger than `part_size` bytes.
pub fn split_xci(args: SplitArgs) -> Result<()> {
    unsupported!("split-xci", args)
}

/// Merge split XCI parts back into a single container.
pub fn merge_xci(args: MergeArgs) -> Result<()> {
    unsupported!("merge-xci", args)
}

/// Merge game updates and DLC into an XCI.
pub fn update_xci(args: UpdateArgs) -> Result<()> {
    unsupported!("update-xci", args)
}

/// Convert an XCI into an NSP.
pub fn xci_to_nsp(args: ContentArgs) -> Result<()> {
    unsupported!("xci-to-nsp", args)
}

/// Compress an XCI into an XCZ.
pub fn compress_xci(args: CompressArgs) -> Result<()> {
    unsupported!("compress-xci", args)
}

/// Decompress an XCZ back into an XCI.
pub fn decompress_xcz(args: ContentArgs) -> Result<()> {
    unsupported!("decompress-xcz", args)
}

/// Extract the firmware from an XCI.
pub fn extract_firmware(args: ExtractArgs) -> Result<()> {
    unsupported!("extract-firmware", args)
}

/// Patch an NSP and write a copy that installs without valid signatures.
pub fn patch_nsp(args: ContentArgs) -> Result<()> {
    unsupported!("patch-nsp", args)
}

/// Split an NSP into parts no larger than `part_size` bytes.
pub fn split_nsp(args: SplitArgs) -> Result<()> {
    unsupported!("split-nsp", args)
}

/// Merge split NSP parts back into a single container.
pub fn merge_nsp(args: MergeArgs) -> Result<()> {
    unsupported!("merge-nsp", args)
}

/// Merge game updates and DLC into an NSP.
pub fn update_nsp(args: UpdateArgs) -> Result<()> {
    unsupported!("update-nsp", args)
}

/// Convert an NSP into an XCI.
pub fn nsp_to_xci(args: ContentArgs) -> Result<()> {
    unsupported!("nsp-to-xci", args)
}

/// Compress an NSP into an NSZ.
pub fn compress_nsp(args: CompressArgs) -> Result<()> {
    unsupported!("compress-nsp", args)
}

/// Decompress an NSZ back into an NSP.
pub fn decompress_nsz(args: ContentArgs) -> Result<()> {
    unsupported!("decompress-nsz", args)
}

/// Extract Switch savegames copied from the user partition.
pub fn extract_save(args: ExtractArgs) -> Result<()> {
    unsupported!("extract-save", args)
}

/// Build an installable NSP forwarder from an NRO.
pub fn forwarder(args: ForwarderArgs) -> Result<()> {
    unsupported!("forwarder", args)
}

/// Format a device as FAT32.
pub fn format_sd(args: FormatSdArgs) -> Result<()> {
    unsupported!("format-sd", args)
}
