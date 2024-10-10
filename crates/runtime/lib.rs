#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

//! ### Examples
//!
//! ```rust
//! use axiston_runtime::{RuntimeClient, Result};
//!
//! fn main() -> Result<()> {
//!     let client = RuntimeClient::builder().build();
//!     Ok(())
//! }
//! ```

pub use crate::client::{ClientBuilder, RuntimeClient};

mod client;
mod runtime;

/// Unrecoverable failure of the [`RuntimeClient`].
///
/// Includes all error types that may occur.
#[must_use = "errors do nothing unless you use them"]
#[derive(Debug, thiserror::Error)]
pub enum Error {}

/// Specialized [`Result`] alias for the [`Error`] type.
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;

// TODO: Trait to implement adding new runtimes and picking the right one for the user:
// Use the first available one on local.
// Use the dedicated one on remote.
