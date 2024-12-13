use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;

use crate::extract::{AuthRole, AuthState};
use crate::handler::{ErrorKind, Result};

/// Interrupts the request if the `Authorization` header cannot be successfully verified.
///
/// #### Notes
///
/// - [`AuthToken`] can't be extracted from requests without (any) token.
/// - [`AuthState`] can't be extracted from requests without *verified* token.
///
/// #### Examples
///
/// ```rust,no_run
/// use axum::middleware::from_fn_with_state;
/// use axiston_server::middleware::authentication_guard;
/// use axiston_server::service::{AppConfig, AppState};
///
/// let state = AppState::connect(AppConfig::new());
/// let _ = from_fn_with_state(state, authentication_guard);
/// ```
///
/// [`AuthToken`]: crate::extract::AuthToken
pub async fn authentication_guard(
    _auth_state: AuthState,
    request: Request,
    next: Next,
) -> Response {
    next.run(request).await
}

/// Interrupts the request if the `Authorization` header cannot be successfully verified
/// or if the [`AuthToken`] is not associated with a privileged access account.
///
/// #### Notes
///
/// - [`AuthToken`] can't be extracted from requests without (any) token.
/// - [`AuthState`] can't be extracted from requests without *verified* token.
///
/// #### Examples
///
/// ```rust,no_run
/// use axum::middleware::from_fn_with_state;
/// use axiston_server::middleware::authorization_guard;
/// use axiston_server::service::{AppConfig, AppState};
///
/// let state = AppState::connect(AppConfig::new());
/// let _ = from_fn_with_state(state, authorization_guard);
/// ```
///
/// [`AuthToken`]: crate::extract::AuthToken
pub async fn authorization_guard(
    auth_state: AuthState,
    request: Request,
    next: Next,
) -> Result<Response> {
    match auth_state.auth_role() {
        AuthRole::Unprivileged => Err(ErrorKind::Forbidden.into()),
        AuthRole::Privileged => Ok(next.run(request).await),
    }
}
