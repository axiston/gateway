use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Response};
use axum_extra::headers::authorization::{Authorization, Bearer};
use axum_extra::typed_header::{TypedHeader, TypedHeaderRejectionReason};
use base64::Engine;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::handler::{Error, ErrorKind, Result};

/// Authentication & authorization token Extractor / Response.
///
/// ##### Examples
///
/// When used as an extractor, it can decode request `Authorization` headers from the `base64`
/// format and then deserialize the result from the `json` format into itself.
///
/// ```rust,no_run
/// use axiston_server::extract::AuthToken;
///
/// async fn read_auth_token(auth_token: AuthToken) {
///     // User has provided a valid (but not verified) auth token.
/// }
/// ```
///
/// When used as a response, it can serialize itself into the `json` format and then encode
/// the result into the `base64` format, and will automatically set `Authorization: Bearer` header.
///
/// ```rust,no_run
/// use time::ext::NumericalDuration;
/// use uuid::Uuid;
/// use axiston_server::extract::AuthToken;
///
///  async fn write_auth_token() -> AuthToken {
///     AuthToken::new(Uuid::new_v4(), Uuid::new_v4(), 7.days())
/// }
/// ```
///
#[must_use]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthToken {
    // TODO: Attach region identifier.
    // TODO: Encode iat/eat as timestamps.
    #[serde(rename = "pid")]
    pub account_id: Uuid,
    #[serde(rename = "seq")]
    pub token_seq: Uuid,
    #[serde(rename = "iat")]
    pub issued_at: OffsetDateTime,
    #[serde(rename = "eat")]
    pub expired_at: OffsetDateTime,
}

impl AuthToken {
    /// Returns a new [`AuthToken`].
    pub fn new(account_id: Uuid, token_seq: Uuid, expires_in: Duration) -> Self {
        Self {
            account_id,
            token_seq,
            issued_at: OffsetDateTime::now_utc(),
            expired_at: OffsetDateTime::now_utc() + expires_in.abs(),
        }
    }

    /// Returns the duration the token is valid for.
    #[inline]
    pub fn expires_in(&self) -> Duration {
        (self.expired_at - OffsetDateTime::now_utc()).abs()
    }

    /// Decodes a token from `base64` and then deserializes from `json`.
    fn from_base64(token: &str) -> Result<Self> {
        let Ok(token) = base64::prelude::BASE64_STANDARD.decode(token) else {
            return Err(ErrorKind::InternalServerError.into_error());
        };

        serde_json::from_slice(&token).map_err(|_| ErrorKind::InternalServerError.into())
    }

    /// Serializes a token into `json` and then encodes in `base64`.
    fn into_base64(self) -> Result<String> {
        serde_json::to_string(&self)
            .map(|token| base64::prelude::BASE64_STANDARD.encode(token))
            .map_err(|_| ErrorKind::InternalServerError.into())
    }

    /// Returns `true` if the authorization token has already expired.
    #[inline]
    pub fn is_expired(&self) -> bool {
        OffsetDateTime::now_utc() < self.expired_at
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthToken
where
    S: Sync + Send,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        type AuthBearerHeader = TypedHeader<Authorization<Bearer>>;
        match AuthBearerHeader::from_request_parts(parts, state).await {
            Ok(typed_header) => Self::from_base64(typed_header.token()),
            Err(typed_header_error) => match typed_header_error.reason() {
                TypedHeaderRejectionReason::Missing => Err(ErrorKind::MissingAuthToken.into()),
                TypedHeaderRejectionReason::Error(_) => Err(ErrorKind::MalformedAuthToken.into()),
                _ => Err(ErrorKind::InternalServerError.into()),
            },
        }
    }
}

// TODO: IntoResponseParts, ignore panic/error.
impl IntoResponse for AuthToken {
    fn into_response(self) -> Response {
        let base64_token = match self.into_base64() {
            Err(error) => return error.into_response(),
            Ok(base64_token) => base64_token,
        };

        match Authorization::bearer(&base64_token) {
            Ok(typed_header) => TypedHeader(typed_header).into_response(),
            Err(_) => ErrorKind::InternalServerError.into_response(),
        }
    }
}
