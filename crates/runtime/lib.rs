#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

//! ### Examples
//!
//! ```rust
//! use axiston_runtime::{Runtime, RuntimeConfig, RuntimeResult};
//!
//! #[tokio::main]
//! async fn main() -> RuntimeResult<()> {
//!     let config = RuntimeConfig::new();
//!     let runtime = Runtime::new((), config);
//!     let _conn = runtime.get_connection().await?;
//!     Ok(())
//! }
//! ```

use deadpool::managed::PoolError;
use derive_more::From;

use crate::client::{RuntimeConn, RuntimeConnBuilder, RuntimeConnError};
pub use crate::config::{Runtime, RuntimeConfig, RuntimeObject};

mod client;
mod config;

/// Unrecoverable failure of the [`RuntimeConn`].
///
/// Includes all error types that may occur.
#[derive(Debug, From, thiserror::Error)]
#[must_use = "errors do nothing unless you use them"]
pub enum RuntimeError {
    /// Transport failure (from the client or server).
    #[error("transport failure: {0}")]
    Transport(tonic::transport::Error),
}

impl From<PoolError<RuntimeConnError>> for RuntimeError {
    fn from(value: PoolError<RuntimeConnError>) -> Self {
        todo!()
    }
}

/// Specialized [`Result`] alias for the [`RuntimeError`] type.
pub type RuntimeResult<T, E = RuntimeError> = Result<T, E>;

// TODO: Trait to implement adding new runtimes and picking the right one for the user:
// Use the first available one on local.
// Use the dedicated one on remote.

// TODO: best practices or whatever
// https://github.com/weiznich/diesel_async/blob/main/src/pooled_connection/deadpool.rs
// https://github.com/bikeshedder/deadpool/blob/master/diesel/src/manager.rs
