//! Writer for a registered content directory.

use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use super::layout::RegistryLayout;
use super::nca::NcaName;
use crate::error::Result;

/// Size of the blocks used when copying an NCA, mirroring the upstream
/// `VFS_RC_LARGE_COPY_BLOCK`.
const COPY_BLOCK: usize = 0x40_0000;

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
    /// Returns `Ok(None)` when the NCA already exists and `overwrite` is
    /// `false`, so callers can report it as skipped.
    pub fn install<R: Read>(
        &self,
        name: NcaName,
        layout: RegistryLayout,
        source: &mut R,
        overwrite: bool,
    ) -> Result<Option<InstalledNca>> {
        let path = self.path_for(name, layout);
        if !overwrite && path.is_file() {
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

/// Copy `source` into `path` through a sibling temporary file.
fn write_atomic<R: Read>(path: &Path, source: &mut R) -> Result<u64> {
    let tmp = path.with_extension("nsk-part");
    let written = (|| -> Result<u64> {
        let mut sink = fs::File::create(&tmp)?;
        let bytes = copy(source, &mut sink)?;
        sink.flush()?;
        Ok(bytes)
    })();

    match written {
        Ok(bytes) => {
            if path.exists() {
                fs::remove_file(path)?;
            }
            fs::rename(&tmp, path)?;
            Ok(bytes)
        }
        Err(err) => {
            let _ = fs::remove_file(&tmp);
            Err(err)
        }
    }
}

/// Copy every byte of `source` into `sink`, returning the number of bytes.
fn copy<R: Read, W: Write>(source: &mut R, sink: &mut W) -> Result<u64> {
    let mut buffer = vec![0u8; COPY_BLOCK];
    let mut total = 0u64;
    loop {
        let read = source.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        sink.write_all(&buffer[..read])?;
        total += read as u64;
    }
    Ok(total)
}
