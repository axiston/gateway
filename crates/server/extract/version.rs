use std::convert::Infallible;
use std::num::NonZeroU32;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use serde::Deserialize;

use crate::extract::Path;

/// Convenient [`Path`] `:version` parameter Extractor.
///
/// ##### Examples
///
/// Extracts `:version` [`Path`] parameter and parses it according to a `v{u32}`
/// schema. `v0` is treated as an unstable version.
///
/// Any other schema is treated as an unrecognized version.
///
/// ```rust,no_run
/// use axiston_server::extract::Version;
///
/// async fn read_auth_token(version: Version) {
///     // User has (probably) called a versioned endpoint.
/// }
/// ```
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

#[axum::async_trait]
impl<S> FromRequestParts<S> for Version {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        #[derive(Debug, Clone, Deserialize)]
        struct VersionParams {
            pub version: String,
        }

        Ok(match parts.extract::<Path<VersionParams>>().await {
            Ok(params) => Version::new(params.version.as_str()),
            Err(_) => Version::default(),
        })
    }
}
