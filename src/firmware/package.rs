//! Reading and installing firmware packages.

use std::fs::File;
use std::path::{Path, PathBuf};

use zip::ZipArchive;

use crate::error::{Error, Result};
use crate::registry::{InstalledNca, NcaId, NcaName, RegisteredCache, RegistryLayout};

/// A firmware package: a zip archive that bundles NCA files.
///
/// Firmware archives are usually flat, but the reference installer walks them
/// recursively because the NCAs may also be nested in a subfolder. Every file
/// whose base name is an NCA is installed; everything else is ignored.
#[derive(Debug)]
pub struct FirmwarePackage {
    path: PathBuf,
    archive: ZipArchive<File>,
}

/// Summary of an install run.
#[derive(Debug, Default)]
pub struct InstallReport {
    /// NCAs written to the target.
    pub written: Vec<InstalledNca>,
    /// NCAs skipped because they were already present.
    pub skipped: Vec<NcaId>,
}

impl InstallReport {
    /// Total number of bytes written.
    pub fn bytes_written(&self) -> u64 {
        self.written.iter().map(|nca| nca.bytes).sum()
    }
}

impl FirmwarePackage {
    /// Open a firmware package from disk.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let file = File::open(&path)?;
        let archive = ZipArchive::new(file)?;
        Ok(Self { path, archive })
    }

    /// Install every NCA into `cache`.
    ///
    /// Returns [`Error::EmptyPackage`] when the archive does not contain a
    /// single NCA file.
    pub fn install(
        &mut self,
        cache: &RegisteredCache,
        layout: RegistryLayout,
        overwrite: bool,
    ) -> Result<InstallReport> {
        cache.ensure_root()?;

        let mut report = InstallReport::default();
        for index in 0..self.archive.len() {
            let mut entry = self.archive.by_index(index)?;
            if entry.is_dir() {
                continue;
            }

            let raw_name = entry.name()?.into_owned();
            let base = raw_name
                .rsplit(['/', '\\'])
                .next()
                .unwrap_or(raw_name.as_str());
            let Some(name) = NcaName::parse(base) else {
                continue;
            };

            match cache.install(name, layout, &mut entry, overwrite)? {
                Some(installed) => report.written.push(installed),
                None => report.skipped.push(name.id),
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
