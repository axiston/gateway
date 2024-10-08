use std::collections::HashMap;
use std::convert::Infallible;
use std::num::NonZeroU32;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;

use crate::extract::Path;

/// [`Version`] Extractor.
#[must_use]
#[derive(Debug, Default, Clone, Copy)]
pub struct RouteVersion(pub Version);

/// Route `:id` Extractor.
#[must_use]
#[derive(Debug, Default, Clone, Copy)]
pub enum Version {
    /// ...
    #[default]
    Unrecognized,
    /// v0
    Unstable,
    /// v1, v2...
    Stable(NonZeroU32),
}

impl Version {
    /// Creates a new [`Version`].
    pub fn new(version: &str) -> Self {
        let number = version
            .strip_prefix('v')
            .and_then(|x| x.parse::<u32>().ok());

        match number.map(NonZeroU32::new) {
            None => Self::Unrecognized,
            Some(Some(x)) => Self::Stable(x),
            Some(None) => Self::Unstable,
        }
    }

    /// Returns `true` if it's an unrecognized version.
    #[inline]
    #[must_use]
    pub fn is_unrecognized(&self) -> bool {
        matches!(self, Self::Unrecognized)
    }

    /// Returns `true` if it's an unstable version.
    #[inline]
    #[must_use]
    pub fn is_unstable(&self) -> bool {
        matches!(self, Self::Unstable)
    }

    /// Returns `true` if it's a stable version.
    #[inline]
    #[must_use]
    pub fn is_stable(&self) -> bool {
        matches!(self, Self::Stable(_))
    }

    /// Returns `true` if it matches with a `version`.
    #[must_use]
    pub fn is_v(&self, version: u32) -> bool {
        match self {
            Self::Unstable => version == 0,
            Self::Stable(x) => x.get() == version,
            _ => false,
        }
    }

    /// Returns the underlying version number.
    pub fn into_inner(self) -> Option<u32> {
        match self {
            Self::Unrecognized => None,
            Self::Stable(x) => Some(x.get()),
            Self::Unstable => Some(0),
        }
    }
}

impl From<Version> for bool {
    #[inline]
    fn from(value: Version) -> Self {
        value.is_stable()
    }
}

impl From<&str> for Version {
    #[inline]
    fn from(version: &str) -> Self {
        Self::new(version)
    }
}

impl From<String> for Version {
    #[inline]
    fn from(version: String) -> Self {
        Self::new(version.as_str())
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for Version {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let x: Option<Version> = match parts.extract::<Path<HashMap<String, String>>>().await {
            Ok(x) => x.get("version").map(|x| x.as_str().into()),
            Err(_) => None,
        };

        Ok(x.unwrap_or_default())
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for RouteVersion
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let version = Version::from_request_parts(parts, state).await;
        version.map(RouteVersion)
    }
}
