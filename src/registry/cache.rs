//! Writer for a registered content directory.

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use super::layout::RegistryLayout;
use super::nca::NcaName;
use crate::archive::write_atomic;
use crate::error::Result;

/// An NCA that has been written to disk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstalledNca {
    /// Name of the installed NCA.
    pub name: NcaName,
    /// Path the NCA was written to.
    pub path: PathBuf,
    /// Number of bytes written.
    pub bytes: u64,
}

/// A directory holding installed (registered) content.
#[derive(Debug, Clone)]
pub struct RegisteredCache {
    root: PathBuf,
}

impl RegisteredCache {
    /// Target the content directory `root`.
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Root of the content directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Create the root directory if needed.
    pub fn ensure_root(&self) -> Result<()> {
        fs::create_dir_all(&self.root)?;
        Ok(())
    }

    /// Destination path of `name` under this cache.
    pub fn path_for(&self, name: NcaName, layout: RegistryLayout) -> PathBuf {
        layout.path(&self.root, name)
    }

    /// Stream one NCA into the cache.
    ///
    /// Returns `Ok(None)` when the NCA already exists and `replace` is `false`,
    /// so callers can report it as skipped.
    pub fn install<R: Read>(
        &self,
        name: NcaName,
        layout: RegistryLayout,
        source: &mut R,
        replace: bool,
    ) -> Result<Option<InstalledNca>> {
        let path = self.path_for(name, layout);
        if !replace && path.is_file() {
            return Ok(None);
        }

        let Some(parent) = path.parent() else {
            return Ok(None);
        };
        // The flat layout nests the NCA inside `<id>.nca`. A previous,
        // unorganised copy of the firmware can leave a plain `<id>.nca` file in
        // the way — exactly the collision the reference installer works around.
        if parent.is_file() {
            fs::remove_file(parent)?;
        }
        fs::create_dir_all(parent)?;

        let bytes = write_atomic(&path, source)?;
        Ok(Some(InstalledNca { name, path, bytes }))
    }
}
