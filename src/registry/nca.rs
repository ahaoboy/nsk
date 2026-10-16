//! NCA identifiers and file names.
//!
//! This mirrors the naming rules of the upstream `RegisteredCache`
//! (`FollowsNcaIdFormat`): a 32-digit hexadecimal id followed by `.nca`
//! (regular content) or `.cnmt.nca` (control metadata).

use std::fmt;

/// Number of bytes in an NCA id.
pub const NCA_ID_LEN: usize = 0x10;

/// Number of hex digits in the textual form of an NCA id.
const NCA_ID_HEX_LEN: usize = NCA_ID_LEN * 2;

/// Raw NCA identifier bytes.
pub type NcaIdBytes = [u8; NCA_ID_LEN];

/// A 16-byte NCA identifier.
///
/// The identifier is the first 16 bytes of the SHA-256 over the whole NCA and
/// is also the hexadecimal prefix of every NCA file name.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NcaId(NcaIdBytes);

impl NcaId {
    /// Parse the 32 hexadecimal digits of an NCA file name.
    ///
    /// Returns `None` when `hex` is not exactly 32 hexadecimal digits.
    pub fn from_hex(hex: &str) -> Option<Self> {
        if hex.len() != NCA_ID_HEX_LEN {
            return None;
        }
        let mut bytes = [0u8; NCA_ID_LEN];
        hex::decode_to_slice(hex, &mut bytes).ok()?;
        Some(Self(bytes))
    }

    /// Raw bytes.
    pub const fn as_bytes(&self) -> &NcaIdBytes {
        &self.0
    }

    /// Lower-case hexadecimal representation.
    pub fn to_hex(self) -> String {
        hex::encode(self.0)
    }
}

impl fmt::Display for NcaId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl fmt::Debug for NcaId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NcaId({self})")
    }
}

/// The flavour of an NCA file, encoded by its suffix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NcaKind {
    /// Regular content (`.nca`).
    Content,
    /// Control metadata (`.cnmt.nca`).
    Meta,
}

/// An NCA file name split into its identifier and flavour.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NcaName {
    /// NCA identifier.
    pub id: NcaId,
    /// File flavour.
    pub kind: NcaKind,
}

impl NcaName {
    /// Parse the base file name of an NCA.
    ///
    /// Any directory component must have been stripped by the caller. Names
    /// that are not NCA files yield `None`, so directories and unrelated files
    /// are skipped silently.
    pub fn parse(name: &str) -> Option<Self> {
        let bytes = name.as_bytes();
        let kind = match bytes.len() {
            36 if bytes[32..].eq_ignore_ascii_case(b".nca") => NcaKind::Content,
            41 if bytes[32..].eq_ignore_ascii_case(b".cnmt.nca") => NcaKind::Meta,
            _ => return None,
        };
        if !bytes[..32].iter().all(u8::is_ascii_hexdigit) {
            return None;
        }
        // The first 32 bytes are known to be ASCII, so this slice cannot split
        // a multi-byte character.
        let id = NcaId::from_hex(&name[..32])?;
        Some(Self { id, kind })
    }

    /// Canonical file name: `<id>.nca` or `<id>.cnmt.nca`.
    pub fn file_name(self) -> String {
        match self.kind {
            NcaKind::Content => format!("{}.nca", self.id),
            NcaKind::Meta => format!("{}.cnmt.nca", self.id),
        }
    }
}
