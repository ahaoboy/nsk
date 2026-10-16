//! Reading and installing key packages.

use std::fs::File;
use std::path::{Path, PathBuf};

use zip::ZipArchive;

use super::{InstalledKey, KeyStore};
use crate::archive::base_name;
use crate::error::{Error, Result};

/// A key package: a zip archive that bundles `*.keys` files.
///
/// Key archives often nest the files in a subfolder (for example
/// `Keys-22.1.0/prod.keys`), so every entry is walked and only its base name is
/// used. Files are stored flat in the target directory.
#[derive(Debug)]
pub struct KeysPackage {
    path: PathBuf,
    archive: ZipArchive<File>,
}

/// Summary of an install run.
#[derive(Debug, Default)]
pub struct InstallReport {
    /// Key files written to the target.
    pub written: Vec<InstalledKey>,
    /// Key files skipped because they were already present.
    pub skipped: Vec<String>,
}

impl InstallReport {
    /// Total number of bytes written.
    pub fn bytes_written(&self) -> u64 {
        self.written.iter().map(|key| key.bytes).sum()
    }
}

impl KeysPackage {
    /// Open a key package from disk.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let file = File::open(&path)?;
        let archive = ZipArchive::new(file)?;
        Ok(Self { path, archive })
    }

    /// Install every `*.keys` file into `store`.
    ///
    /// With `replace` unset, files that are already present are skipped instead
    /// of being written again. Returns [`Error::EmptyPackage`] when the archive
    /// does not contain a single key file.
    pub fn install(&mut self, store: &KeyStore, replace: bool) -> Result<InstallReport> {
        store.ensure_root()?;

        let mut report = InstallReport::default();
        for index in 0..self.archive.len() {
            let mut entry = self.archive.by_index(index)?;
            if entry.is_dir() {
                continue;
            }

            let raw_name = entry.name()?.into_owned();
            let Some(base) = base_name(&raw_name) else {
                continue;
            };
            if !is_key_file(base) {
                continue;
            }

            match store.install(base, &mut entry, replace)? {
                Some(installed) => report.written.push(installed),
                None => report.skipped.push(base.to_owned()),
            }
        }

        if report.written.is_empty() && report.skipped.is_empty() {
            return Err(Error::EmptyPackage {
                path: self.path.clone(),
            });
        }
        Ok(report)
    }
}

/// Whether a file name is a key file (`*.keys`, case-insensitive).
fn is_key_file(name: &str) -> bool {
    let bytes = name.as_bytes();
    bytes.len() > 5 && bytes[bytes.len() - 5..].eq_ignore_ascii_case(b".keys")
}
