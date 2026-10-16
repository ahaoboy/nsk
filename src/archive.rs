//! Shared plumbing for installing files out of an archive package.

use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use crate::error::Result;

/// Size of the copy blocks used when writing files, mirroring the upstream
/// `VFS_RC_LARGE_COPY_BLOCK`.
const COPY_BLOCK: usize = 0x40_0000;

/// Base name of an archive entry path.
///
/// Returns `None` for entries that resolve to nothing usable (empty, `.` or
/// `..`), so callers can skip them and a hand-crafted archive cannot escape the
/// target directory.
pub(crate) fn base_name(raw: &str) -> Option<&str> {
    let base = raw.rsplit(['/', '\\']).next().unwrap_or(raw);
    (!base.is_empty() && base != "." && base != "..").then_some(base)
}

/// Copy `source` into `path` through a sibling temporary file.
///
/// The temporary file is renamed over the destination, so an interrupted run
/// never leaves a half-written file in place.
pub(crate) fn write_atomic<R: Read>(path: &Path, source: &mut R) -> Result<u64> {
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
