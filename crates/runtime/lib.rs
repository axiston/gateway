#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

//! ### Examples
//!
//! ```rust
//! use axiston_rt_connect::{Runtime, Result, RuntimeEndpoint};
//!
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let runtime = Runtime::default();
//!     let endpoint = RuntimeEndpoint::try_new("")?;
//!     runtime.register_endpoint(endpoint)?;
//!     let _conn = runtime.get_connection().await?;
//!     Ok(())
//! }
//! ```

mod instance;
mod manager;
mod middleware;

use deadpool::managed::PoolError;
use derive_more::From;

pub use crate::instance::{Runtime, RuntimeConfig};
pub use crate::manager::RuntimeEndpoint;
use crate::manager::RuntimeError;

/// Unrecoverable failure of the [`Runtime`].
///
/// Includes all error types that may occur.
#[non_exhaustive]
#[derive(Debug, From, thiserror::Error)]
#[must_use = "errors do nothing unless you use them"]
pub enum Error {
    /// Timeout happened.
    #[error("timeout happened")]
    Timout(deadpool::managed::TimeoutType),

    /// Runtime: All endpoints have reached the limit.
    #[error("runtime: all endpoints have reached the limit")]
    EndpointsLimit,
    /// Runtime: Connection pool has no endpoints.
    #[error("runtime: connection pool has no endpoints")]
    NoEndpoints,
    /// Runtime: Transport failure (from the client or server).
    #[error("runtime: transport failure: {0}")]
    Transport(tonic::transport::Error),
}

impl From<RuntimeError> for Error {
    fn from(runtime_connection_error: RuntimeError) -> Self {
        match runtime_connection_error {
            RuntimeError::Transport(transport_failure) => Self::Transport(transport_failure),
            RuntimeError::EndpointsLimit => Self::EndpointsLimit,
            RuntimeError::NoEndpoints => Self::NoEndpoints,
        }
    }
}

impl From<PoolError<RuntimeError>> for Error {
    fn from(value: PoolError<RuntimeError>) -> Self {
        match value {
            PoolError::Timeout(timeout_type) => Self::Timout(timeout_type),
            PoolError::Backend(backend_error) => backend_error.into(),
            PoolError::Closed => unreachable!(),
            PoolError::NoRuntimeSpecified => unreachable!(),
            PoolError::PostCreateHook(_) => unreachable!(),
        }
    }
}

/// Specialized [`Result`] alias for the [`Error`] type.
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
