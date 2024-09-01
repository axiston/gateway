#![forbid(unsafe_code)]
#![doc = include_str!("./README.md")]

//! TODO.

pub mod database;
pub mod repo;
pub mod entity;

// TODO: accounts
// TODO: projects
// TODO: accounts

/// Unrecoverable failure of the [`Database`].
///
/// Includes all error types that may occur.
///
/// [`Database`]: database::Database
#[derive(Debug, thiserror::Error)]
#[must_use = "errors do nothing unless you use them"]
pub enum Error {
    /// Underlying [`sqlx`] driver failure.
    #[error("underlying sql driver failure: {0}")]
    Sql(#[from] sqlx::Error),
}

/// Specialized [`Result`] alias for the [`Error`] type.
///
/// [`Result`]: std::result::Result
pub type Result<T, E = Error> = std::result::Result<T, E>;
