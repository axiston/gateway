//! All `axum::`[`Router`]s with related `axum::`[`Handler`]s.
//!
//! [`Router`]: axum::routing::Router
//! [`Handler`]: axum::handler::Handler

mod accounts;
mod integrations;
mod notifications;
mod projects;
mod workflows;

use std::borrow::Cow;
use std::fmt;

use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, IntoResponseParts, Response};
use axum::routing::Router;
use serde::Serialize;

use crate::service::AppState;

/// Common fallback handler (404 Not Found).
#[inline]
async fn fallback() -> Response {
    // TODO: Static files.
    ErrorKind::NotFound.into_response()
}

/// Returns a [`Router`] with all routes.
pub fn routes() -> Router<AppState> {
    Router::new().fallback(fallback)
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

/// A specialized [`Result`] type for the [`Error`] type. Used by [`Handler`]s.
///
/// [`Result`]: std::result::Result
/// [`Handler`]: axum::handler::Handler
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Comprehensive list of all possible [`Error`]s.
#[must_use = "errors do nothing unless serialized"]
#[derive(Debug, Default, Clone, Copy)]
pub enum ErrorKind {
    /// 400 Bad Request.
    MissingPathParam,
    /// 400 Bad Request.
    BadRequestBody,

    /// 400 Bad Request.
    MissingAuthToken,
    /// 400 Bad Request.
    MalformedAuthToken,

    /// 401 Unauthorized.
    Unauthorized,
    /// 403 Forbidden.
    Forbidden,
    /// 404 Not Found.
    NotFound,
    /// 500 Internal Server Error.
    #[default]
    InternalServerError,
}

impl ErrorKind {
    /// Explicitly converts into the [`Error`].
    #[inline]
    pub fn into_error(self) -> Error {
        self.into()
    }

    /// Transforms [`ErrorKind`] into [`ErrorRepr`].
    fn into_repr(self) -> ErrorRepr<'static> {
        match self {
            Self::MissingPathParam => ErrorRepr::MISSING_PATH_PARAM,
            Self::MissingAuthToken => ErrorRepr::MISSING_AUTH_TOKEN,
            Self::MalformedAuthToken => ErrorRepr::MALFORMED_AUTH_TOKEN,
            Self::BadRequestBody => ErrorRepr::BAD_REQUEST_BODY,
            Self::Unauthorized => ErrorRepr::UNAUTHORIZED,
            Self::Forbidden => ErrorRepr::FORBIDDEN,
            Self::NotFound => ErrorRepr::NOT_FOUND,
            Self::InternalServerError => ErrorRepr::INTERNAL_SERVER_ERROR,
        }
    }

    /// TODO: Attach context, additional message text.
    fn into_repr_cx(self) -> ErrorRepr<'static> {
        self.into_repr()
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
#[derive(Debug, Clone, Serialize)]
struct ErrorRepr<'a> {
    pub name: Cow<'a, str>,
    pub message: Cow<'a, str>,
    #[serde(skip)]
    pub status: StatusCode,
}

impl<'a> ErrorRepr<'a> {
    const MISSING_PATH_PARAM: Self = Self::new(
        "missing_path_param",
        "The request path is missing one of more required parameters.",
        StatusCode::BAD_REQUEST,
    );

    const BAD_REQUEST_BODY: Self = Self::new(
        "bad_request_body",
        "The request body cannot be correctly deserialized.",
        StatusCode::BAD_REQUEST,
    );

    const MISSING_AUTH_TOKEN: Self = Self::new(
        "missing_auth_token",
        "The authentication header is not found.",
        StatusCode::BAD_REQUEST,
    );

    const MALFORMED_AUTH_TOKEN: Self = Self::new(
        "malformed_auth_token",
        "The authentication header value can't be parsed.",
        StatusCode::BAD_REQUEST,
    );

    const UNAUTHORIZED: Self = Self::new(
        "unauthorized",
        "The authentication token can't be verified.",
        StatusCode::UNAUTHORIZED,
    );

    const FORBIDDEN: Self = Self::new(
        "forbidden",
        "The authorization token is not verified",
        StatusCode::FORBIDDEN,
    );

    const NOT_FOUND: Self = Self::new(
        "not_found",
        "The requested endpoint does not exist.",
        StatusCode::NOT_FOUND,
    );

    const INTERNAL_SERVER_ERROR: Self = Self::new(
        "internal_server_error",
        "An unexpected error occurred.",
        StatusCode::INTERNAL_SERVER_ERROR,
    );

    /// Returns a new [`ErrorRepr`].
    #[inline]
    pub const fn new(name: &'a str, message: &'a str, status: StatusCode) -> Self {
        Self {
            name: Cow::Borrowed(name),
            message: Cow::Borrowed(message),
            status,
        }
    }

    // TODO.
    // pub fn with_context(mut self, cx: &str) -> Self {}
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
