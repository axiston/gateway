use std::ops::{Deref, DerefMut};

use axum::extract::rejection::JsonRejection;
use axum::extract::{FromRequest, Json as AxumJson, Request};
use axum::response::{IntoResponse, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::handler::{Error, ErrorKind};

/// Customized rejection for the `axum::extract::`[`Json`].
///
/// [`Json`]: AxumJson
#[must_use]
#[derive(Debug, Clone, Copy, Default)]
pub struct Json<T>(pub T);

impl<T> Json<T> {
    /// Creates a new [`Json`] extractor or response.
    #[inline]
    pub fn new(inner: T) -> Self {
        Self(inner)
    }
}

impl<T> Deref for Json<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Json<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for Json<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

#[axum::async_trait]
impl<T, S> FromRequest<S> for Json<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let extractor = AxumJson::<T>::from_request(req, state).await;
        extractor.map(|x| Self(x.0)).map_err(Into::into)
    }
}

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    #[inline]
    fn into_response(self) -> Response {
        AxumJson(self.0).into_response()
    }
}

impl From<JsonRejection> for Error {
    fn from(rejection: JsonRejection) -> Self {
        // TODO: More specific error messages.
        let rejection = match rejection {
            JsonRejection::JsonDataError(_) => ErrorKind::BadRequestBody,
            JsonRejection::JsonSyntaxError(_) => ErrorKind::BadRequestBody,
            JsonRejection::MissingJsonContentType(_) => ErrorKind::BadRequestBody,
            JsonRejection::BytesRejection(_) => ErrorKind::BadRequestBody,
            _ => ErrorKind::InternalServerError,
        };

        rejection.into()
    }
}
