//! `nsk` — a toolbox for Nintendo Switch content.
//!
//! The crate is split into small modules:
//!
//! * [`registry`] models the on-disk layout of installed content.
//! * [`firmware`] installs firmware packages into a [`registry::RegisteredCache`].
//! * `cli` and `sak` provide the command-line front end; they are gated behind
//!   the `cli` feature.
//!
//! [`registry::RegisteredCache`]: registry::RegisteredCache

#[cfg(feature = "cli")]
pub mod cli;
pub mod error;
pub mod firmware;
pub mod registry;
#[cfg(feature = "cli")]
pub mod sak;

#[cfg(feature = "cli")]
pub use cli::Cli;
pub use error::{Error, Result};
