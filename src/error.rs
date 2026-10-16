//! Crate-wide error handling.

use std::path::PathBuf;

/// Convenient result alias used across the crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Every failure `nsk` can report.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An underlying filesystem operation failed.
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    /// The firmware package could not be read as a zip archive.
    #[error("invalid firmware package: {0}")]
    Zip(#[from] zip::result::ZipError),

    /// The package did not contain a single NCA file.
    #[error("no NCA files found in `{path}`")]
    EmptyPackage { path: PathBuf },

    /// A reserved command has no implementation yet.
    #[cfg(feature = "cli")]
    #[error("`{command}` is not implemented yet")]
    NotImplemented { command: &'static str },
}
