//! On-disk layouts of a registered content directory.

use std::fmt;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use super::nca::{NcaId, NcaName};

/// Directory layout an emulator expects for installed content.
///
/// Two layouts are supported:
///
/// * [`RegistryLayout::Flat`] — the Ryujinx portable layout, where every NCA
///   lives in `<id>.nca/00`. This matches the reference shell installer.
/// * [`RegistryLayout::TwoDigit`] — the yuzu/Eden layout, where content is
///   bucketed by `000000XX` (`XX` is the first byte of the SHA-256 over the NCA
///   id) as described by the upstream `RegisteredCache`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
pub enum RegistryLayout {
    /// `<root>/<id>.nca/00`.
    Flat,
    /// `<root>/000000XX/<id>.nca`.
    TwoDigit,
}

impl RegistryLayout {
    /// Resolve the destination path of `name` below `root`.
    pub fn path(self, root: &Path, name: NcaName) -> PathBuf {
        match self {
            Self::Flat => root.join(format!("{}.nca", name.id)).join("00"),
            Self::TwoDigit => {
                let bucket = format!("000000{:02X}", hash_byte(&name.id));
                root.join(bucket).join(name.file_name())
            }
        }
    }
}

impl fmt::Display for RegistryLayout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Flat => "flat",
            Self::TwoDigit => "two-digit",
        })
    }
}

/// First byte of the SHA-256 over the raw NCA id.
fn hash_byte(id: &NcaId) -> u8 {
    Sha256::digest(id.as_bytes())[0]
}
