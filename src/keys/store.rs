//! Writer for a key directory.

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::archive::write_atomic;
use crate::error::Result;

/// A key file that has been written to disk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstalledKey {
    /// File name of the installed key file.
    pub file_name: String,
    /// Path the key file was written to.
    pub path: PathBuf,
    /// Number of bytes written.
    pub bytes: u64,
}

/// A directory holding key files (`prod.keys`, `title.keys`, ...).
#[derive(Debug, Clone)]
pub struct KeyStore {
    root: PathBuf,
}

impl KeyStore {
    /// Target the key directory `root`.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Root of the key directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Create the root directory if needed.
    pub fn ensure_root(&self) -> Result<()> {
        fs::create_dir_all(&self.root)?;
        Ok(())
    }

    /// Destination path of a key file under this store.
    ///
    /// Key files are stored flat, so `file_name` must not contain a directory
    /// component.
    pub fn path_for(&self, file_name: &str) -> PathBuf {
        self.root.join(file_name)
    }

    /// Stream one key file into the store.
    ///
    /// Returns `Ok(None)` when the file already exists and `replace` is
    /// `false`, so callers can report it as skipped.
    pub fn install<R: Read>(
        &self,
        file_name: &str,
        source: &mut R,
        replace: bool,
    ) -> Result<Option<InstalledKey>> {
        let path = self.path_for(file_name);
        if !replace && path.is_file() {
            return Ok(None);
        }

        self.ensure_root()?;
        if path.is_dir() {
            fs::remove_dir_all(&path)?;
        }

        let bytes = write_atomic(&path, source)?;
        Ok(Some(InstalledKey {
            file_name: file_name.to_owned(),
            path,
            bytes,
        }))
    }
}
