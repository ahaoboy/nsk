//! Reading and installing firmware packages.
//!
//! [`FirmwarePackage`] unpacks the NCA files of a firmware zip package. The
//! `cli` feature adds the `nsk firmware install` command on top of it.

mod package;

pub use package::{FirmwarePackage, InstallReport};

#[cfg(feature = "cli")]
mod command;

#[cfg(feature = "cli")]
pub use command::{FirmwareCmd, FirmwareCommand, InstallArgs, run};
