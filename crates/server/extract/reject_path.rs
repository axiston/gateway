use axum::extract::rejection::PathRejection;
use axum::extract::{FromRequestParts, Path as AxumPath};
use axum::http::request::Parts;
use derive_more::{Deref, DerefMut, From};
use serde::de::DeserializeOwned;

use crate::handler::{Error, ErrorKind};

/// Customized rejection for the `axum::extract::`[`Path`].
///
/// [`Path`]: AxumPath
#[must_use]
#[derive(Debug, Clone, Copy, Default, Deref, DerefMut, From)]
pub struct Path<T>(pub T);

impl<T> Path<T> {
    /// Creates a new [`Path`].
    #[inline]
    fn new(inner: T) -> Self {
        Self(inner)
    }
}

#[axum::async_trait]
impl<T, S> FromRequestParts<S> for Path<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let extractor = AxumPath::<T>::from_request_parts(parts, state).await;
        extractor.map(|x| Self(x.0)).map_err(Into::into)
    }
}

impl From<PathRejection> for Error {
    fn from(rejection: PathRejection) -> Self {
        // TODO: Better error messages.
        let rejection = match rejection {
            PathRejection::FailedToDeserializePathParams(_) => ErrorKind::InternalServerError,
            PathRejection::MissingPathParams(_) => ErrorKind::MissingPathParam,
            _ => ErrorKind::InternalServerError,
        };

        rejection.into()
    }
}
