#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

//! ### Examples
//!
//! ```rust,no_run
//! use axiston_database::{Result, AppDatabase};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let addr = "postgresql://usr:pwd@localhost:5432/db";
//!     let conn = AppDatabase::connect_single_instance(addr).await;
//!     Ok(())
//! }
//! ```

mod connect;
mod migrate;

use derive_more::From;
use sea_orm::DbErr as SeaError;

pub use crate::connect::{AppDatabase, ConnectOptionsExt, ConstraintViolation};
pub use crate::migrate::AppDatabaseExt;

/// Unrecoverable failure of the [`Database`].
///
/// Includes all error types that may occur.
///
/// [`Database`]: database::Database
#[derive(Debug, From, thiserror::Error)]
#[error("underlying sql driver failure: {inner}")]
#[must_use = "errors do nothing unless you use them"]
pub struct Error {
    inner: SeaError,
}

impl Error {
    /// Returns a new [`Error`].
    #[inline]
    pub fn new(inner: SeaError) -> Self {
        Self { inner }
    }
}

/// Specialized [`Result`] alias for the [`Error`] type.
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
