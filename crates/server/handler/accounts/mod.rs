//! All handlers for an `:account` router.
//!

mod accounts;
mod auth;

use axum::extract::Request;
use axum::middleware::{from_fn_with_state, Next};
use axum::response::Response;
use axum::routing::Router;
use serde::Deserialize;
use ts_rs::TS;
use uuid::Uuid;

use crate::extract::{AuthRole, AuthState, Path};
use crate::handler::{ErrorKind, Result};
use crate::middleware::authentication_guard;
use crate::service::AppState;

/// `Path` param for an `:account` router.
///
/// #### Endpoints
///
/// - `./accounts/:account/`
#[must_use]
#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "accounts.ts")]
pub struct AccountPathParams {
    /// Unique identifier of the account.
    pub account: Uuid,
}

/// Interrupts the request if the account is not allowed to perform an action on
/// the requested account.
///
/// #### Notes
///
/// - If no path parameter is specified, the endpoint defaults to the account identifier
///   associated with the access token.
/// - Privileged access is required to perform actions on the account not associated
///   with provided access token.
async fn verify_account_access(
    authentication: AuthState,
    params: Option<Path<AccountPathParams>>,
    request: Request,
    next: Next,
) -> Result<Response> {
    // Defaults to the account identifier associated with the access token.
    let account_id_param = params
        .map(|params| params.account)
        .unwrap_or(authentication.account_id);

    match authentication.auth_role {
        // Unprivileged user tries to perform an action on someone's else account.
        AuthRole::Unprivileged if authentication.account_id != account_id_param => {
            return Err(ErrorKind::Forbidden.into());
        }
        // Unprivileged user tries to perform an action on their own account.
        AuthRole::Unprivileged => Ok(next.run(request).await),
        // Privileged user tries to perform an action on (possibly) someone's else account.
        AuthRole::Privileged => Ok(next.run(request).await),
    }
}

/// Returns a [`Router`] with all related routes.
pub fn routes(state: AppState) -> Router<AppState> {
    let authenticate = from_fn_with_state(state.clone(), authentication_guard);
    Router::new()
        .merge(accounts::routes(state.clone()))
        .route_layer(authenticate)
        .merge(auth::routes(state))
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::accounts::routes;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
