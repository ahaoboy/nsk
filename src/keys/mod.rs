//! Reading and installing key packages.
//!
//! [`KeysPackage`] copies the `*.keys` files of a key package into a
//! [`KeyStore`]. The `cli` feature adds the `nsk keys install` command on top.

mod package;
mod store;

pub use package::{InstallReport, KeysPackage};
pub use store::{InstalledKey, KeyStore};

#[cfg(feature = "cli")]
mod command;

#[cfg(feature = "cli")]
pub use command::{InstallArgs, KeysCmd, KeysCommand, run};
