//! All `axum::`[`Router`]s with related `axum::`[`Handler`]s.
//!
//! [`Router`]: axum::routing::Router
//! [`Handler`]: axum::handler::Handler

use std::fmt;

use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub mod account;
#[cfg(feature = "support-invite")]
mod account_invite;
#[cfg(feature = "support-oauth2")]
mod account_oauth2;
pub mod instance;
pub mod project;
pub mod workflow;

/// Common fallback handler (404 Not Found).
#[inline]
pub async fn fallback() -> Response {
    // TODO: Static files.
    ErrorKind::NotFound.into_response()
}

/// The error type for [`Handler`]s.
///
/// [`Handler`]: axum::handler::Handler
#[must_use = "errors do nothing unless serialized"]
#[derive(Debug, Default)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    /// Returns a new [`Error`].
    #[inline]
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }

    /// Returns the underlying [`ErrorKind`].
    #[inline]
    pub fn into_inner(self) -> ErrorKind {
        self.kind
    }
}

impl From<ErrorKind> for Error {
    #[inline]
    fn from(kind: ErrorKind) -> Self {
        Self::new(kind)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = self.kind.into_repr();
        write!(f, "{} ({}): {}", repr.name, repr.status, repr.message)
    }
}

impl IntoResponse for Error {
    #[inline]
    fn into_response(self) -> Response {
        self.kind.into_response()
    }
}

/// A specialized [`Result`] type for [`Handler`]s.
///
/// [`Handler`]: axum::handler::Handler
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Comprehensive list of all possible [`Error`]s.
#[must_use = "errors do nothing unless serialized"]
#[derive(Debug, Default, Clone, Copy)]
pub enum ErrorKind {
    /// 400 Bad Request: [`ErrorRepr::MISSING_PATH_PARAM`]
    MissingPathParam,
    /// 400 Bad Request: [`ErrorRepr::BAD_REQUEST_BODY`]
    BadRequestBody,
    /// 404 Not Found: [`ErrorRepr::NOT_FOUND`]
    NotFound,
    /// 500 Internal Server Error: [`ErrorRepr::INTERNAL_SERVER_ERROR`]
    #[default]
    InternalServerError,
}

impl ErrorKind {
    /// Transforms [`ErrorKind`] into [`ErrorRepr`].
    fn into_repr(self) -> ErrorRepr<'static> {
        match self {
            Self::MissingPathParam => ErrorRepr::MISSING_PATH_PARAM,
            Self::BadRequestBody => ErrorRepr::BAD_REQUEST_BODY,
            Self::NotFound => ErrorRepr::NOT_FOUND,
            Self::InternalServerError => ErrorRepr::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ErrorKind {
    #[inline]
    fn into_response(self) -> Response {
        self.into_repr().into_response()
    }
}

/// Internal representation of a serialized [`Error`] response.
#[must_use = "errors do nothing unless serialized"]
#[derive(Debug, Clone, Copy, Serialize)]
struct ErrorRepr<'a> {
    pub name: &'a str,
    pub message: &'a str,
    #[serde(skip)]
    pub status: StatusCode,
}

impl<'a> ErrorRepr<'a> {
    const BAD_REQUEST_BODY: Self = Self::new(
        "bad_request_body",
        "The request body cannot be correctly deserialized.",
        StatusCode::BAD_REQUEST,
    );
    const INTERNAL_SERVER_ERROR: Self = Self::new(
        "internal_server_error",
        "An unexpected error occurred.",
        StatusCode::INTERNAL_SERVER_ERROR,
    );
    const MISSING_PATH_PARAM: Self = Self::new(
        "missing_path_param",
        "The request path is missing one of more required parameters.",
        StatusCode::BAD_REQUEST,
    );
    const NOT_FOUND: Self = Self::new(
        "not_found",
        "The requested endpoint does not exist.",
        StatusCode::NOT_FOUND,
    );

    /// Returns a new [`ErrorRepr`].
    #[inline]
    pub const fn new(name: &'a str, message: &'a str, status: StatusCode) -> Self {
        Self {
            name,
            message,
            status,
        }
    }
}

impl Default for ErrorRepr<'_> {
    #[inline]
    fn default() -> Self {
        Self::INTERNAL_SERVER_ERROR
    }
}

impl IntoResponse for ErrorRepr<'_> {
    #[inline]
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}

#[cfg(test)]
mod test {
    use axum::response::IntoResponse;

    use crate::handler::{Error, ErrorKind};

    #[test]
    fn build_error_default() {
        let error = Error::default();
        let _ = error.into_response();
    }

    #[test]
    fn build_error_kind() {
        let error = Error::new(ErrorKind::default());
        let _ = error.into_response();
    }
}
