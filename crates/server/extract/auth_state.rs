use axiston_database::AppDatabase;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use derive_more::{Deref, DerefMut};

use crate::extract::AuthToken;
use crate::handler::{Error, Result};

/// Authentication & authorization status Extractor.
///
/// ##### Examples
///
/// Extracts auth token with an [`AuthToken`] extractor, and then
/// verifies the said token with an [`AppDatabase`].
///
/// Stores itself in request extensions.
///azz
/// ```rust,no_run
/// use axiston_server::extract::AuthState;
///
/// async fn read_auth_token(authentication: AuthState) {
///     // User has provided a valid and verified auth token.
/// }
/// ```
#[must_use]
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct AuthState {
    /// User-provided `Authorization` header.
    #[deref]
    #[deref_mut]
    auth_token: AuthToken,
    /// Verification & authentication results.
    pub auth_role: AuthRole,
}

impl AuthState {
    /// Returns a new [`AuthState`].
    fn new(auth_token: AuthToken, auth_role: AuthRole) -> Self {
        Self {
            auth_token,
            auth_role,
        }
    }

    /// Returns the authorized role of the token.
    #[inline]
    pub fn auth_role(&self) -> AuthRole {
        self.auth_role
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthState
where
    S: Sync + Send,
    AppDatabase: FromRef<S>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        if let Some(auth_state) = parts.extensions.get::<Self>() {
            return Ok(auth_state.clone());
        };

        let app_database = AppDatabase::from_ref(state);
        let auth_token = AuthToken::from_request_parts(parts, state).await?;

        #[derive(Debug, Clone, Copy)]
        pub enum AuthReason {
            /// The [`AuthToken`] was explicitly removed or blocked.
            ///
            /// Most likely due to a recent password change.
            Ignored,

            /// The [`AuthToken`] wasn't renewed before it expired.
            Expired,
        }

        // TODO.
        let auth_role = AuthRole::Unprivileged;

        let auth_state = AuthState::new(auth_token, auth_role);
        parts.extensions.insert(auth_state.clone());
        Ok(auth_state)
    }
}

///
#[must_use]
#[derive(Debug, Clone, Copy)]
pub enum AuthRole {
    /// The [`AuthToken`] belongs to a regular user.
    Unprivileged,
    /// The [`AuthToken`] belongs to a gateway admin.
    Privileged,
}

impl AuthRole {
    /// Returns `true` is the [`AuthRole`] is [`AuthRole::Privileged`].
    #[inline]
    pub fn is_privileged(&self) -> bool {
        matches!(self, AuthRole::Privileged)
    }
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthRole
where
    S: Sync + Send,
    AppDatabase: FromRef<S>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_state = AuthState::from_request_parts(parts, state).await?;
        Ok(auth_state.auth_role)
    }
}
