//! Installed-content primitives: NCA identifiers, storage layouts and the
//! writer that installs NCAs into a registered content directory.

mod cache;
mod layout;
mod nca;

pub use cache::{InstalledNca, RegisteredCache};
pub use layout::RegistryLayout;
pub use nca::{NCA_ID_LEN, NcaId, NcaIdBytes, NcaKind, NcaName};
