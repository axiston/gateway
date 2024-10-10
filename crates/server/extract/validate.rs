use axum::extract::{FromRequest, FromRequestParts, Request};
use axum::http::request::Parts;
use derive_more::{Deref, DerefMut, From};
use validator::{Validate, ValidationError, ValidationErrors};

use crate::handler::Error;

/// TODO.
#[must_use]
#[derive(Debug, Clone, Copy, Default, Deref, DerefMut, From)]
pub struct Validated<T>(pub T);

impl<T> Validated<T> {
    /// Creates a new [`Validated`].
    #[inline]
    pub fn new(inner: T) -> Self {
        Self(inner)
    }
}

#[axum::async_trait]
impl<T, S> FromRequestParts<S> for Validated<T>
where
    T: FromRequestParts<S> + Validate,
    Error: From<T::Rejection>,
    S: Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let inner = T::from_request_parts(parts, state).await?;
        inner.validate()?;
        Ok(Self::new(inner))
    }
}

#[axum::async_trait]
impl<T, S> FromRequest<S> for Validated<T>
where
    T: FromRequest<S> + Validate,
    Error: From<T::Rejection>,
    S: Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let inner = T::from_request(req, state).await?;
        inner.validate()?;
        Ok(Self::new(inner))
    }
}

impl From<ValidationErrors> for Error {
    fn from(value: ValidationErrors) -> Self {
        todo!()
    }
}

impl From<ValidationError> for Error {
    fn from(value: ValidationError) -> Self {
        todo!()
    }
}
