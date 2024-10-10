use axiston_database::AppDatabase;
use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware::from_fn_with_state;
use axum::routing::{delete, get, patch, put, Router};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::extract::{AuthState, Json, Path};
use crate::handler::accounts::{verify_account_access, AccountPathParams};
use crate::handler::Result;
use crate::service::AppState;

/// See [`retrieve_account`] handler.
#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "accounts.ts")]
struct RetrieveAccountResponse {
    /// Unique identifier of the account.
    pub account_id: Uuid,
    pub display_name: String,
    pub email_address: String,
}

/// Retrieves account data.
///
/// #### Notes
///
/// - If no path parameter is specified, the endpoint defaults to the account identifier
///   associated with the access token.
/// - Privileged access is required to perform actions on the account not associated
///   with provided access token.
///
/// #### Endpoints
///
/// - `GET ./accounts/`
/// - `GET ./accounts/:account/`
#[tracing::instrument]
async fn retrieve_account(
    authentication: AuthState,
    params: Option<Path<AccountPathParams>>,
    State(database): State<AppDatabase>,
) -> Result<(StatusCode, Json<RetrieveAccountResponse>)> {
    let account_id = params
        .map(|params| params.account)
        .unwrap_or(authentication.account_id);

    // TODO: Retrieve an account with account_id.

    let response = RetrieveAccountResponse {
        account_id,
        display_name: "".to_owned(),
        email_address: "".to_owned(),
    };

    Ok((StatusCode::OK, Json(response)))
}

/// See [`update_account`] handler.
#[must_use]
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "accounts.ts")]
struct UpdateAccountRequest {
    pub display_name: Option<String>,
    pub next_password: Option<String>,
    pub email_address: Option<String>,

    /// Confirms user's identity.
    pub plain_password: String,
    /// Invalidates all sessions except the one used to update the account.
    /// Defaults to `true` if the password was changed, `false` otherwise.
    pub force_logout: Option<bool>,
}

/// See [`update_account`] handler.
#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "account.ts")]
struct UpdateAccountResponse {
    /// Unique identifier of the account.
    pub account_id: Uuid,
    pub force_logout: bool,
}

/// Updates account fields.
///
/// #### Notes
///
/// - If no path parameter is specified, the endpoint defaults to the account identifier
///   associated with the access token.
/// - Privileged access is required to perform actions on the account not associated
///   with provided access token.
///
/// #### Endpoints
///
/// - `PATCH ./accounts/`
/// - `PATCH ./accounts/:account/`
#[tracing::instrument]
async fn update_account(
    authentication: AuthState,
    params: Option<Path<AccountPathParams>>,
    State(database): State<AppDatabase>,
    Json(request): Json<UpdateAccountRequest>,
) -> Result<(StatusCode, Json<UpdateAccountResponse>)> {
    let account_id = params
        .map(|params| params.account)
        .unwrap_or(authentication.account_id);

    let force_logout = request
        .force_logout
        .unwrap_or_else(|| request.next_password.is_some());

    // TODO: Update an account with account_id.

    let response = UpdateAccountResponse {
        account_id,
        force_logout,
    };
    Ok((StatusCode::OK, Json(response)))
}

/// See [`delete_account`] handler.
#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "accounts.ts")]
struct DeleteAccountResponse {
    /// Unique identifier of the account.
    pub account_id: Uuid,
}

/// Flags an account as deleted.
///
/// #### Notes
///
/// - If no path parameter is specified, the endpoint defaults to the account identifier
///   associated with the access token.
/// - Privileged access is required to perform actions on the account not associated
///   with provided access token.
///
/// #### Endpoints
///
/// - `DELETE ./accounts/`
/// - `DELETE ./accounts/:account/`
#[tracing::instrument]
async fn delete_account(
    authentication: AuthState,
    params: Option<Path<AccountPathParams>>,
    State(database): State<AppDatabase>,
) -> Result<(StatusCode, Json<DeleteAccountResponse>)> {
    let account_id = params
        .map(|params| params.account)
        .unwrap_or(authentication.account_id);

    // TODO: Delete an account with account_id.

    let response = DeleteAccountResponse { account_id };
    Ok((StatusCode::OK, Json(response)))
}

/// Returns a [`Router`] with all related routes.
pub fn routes(state: AppState) -> Router<AppState> {
    let verify_account_access = from_fn_with_state(state, verify_account_access);

    Router::new()
        // For non-admin accounts:
        .route("/accounts/", get(retrieve_account))
        .route("/accounts/", patch(update_account))
        .route("/accounts/", delete(delete_account))
        // For admin accounts:
        .route("/accounts/:account/", get(retrieve_account))
        .route("/accounts/:account/", patch(update_account))
        .route("/accounts/:account/", delete(delete_account))
        // Authorization:
        .route_layer(verify_account_access)
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::accounts::accounts::routes;
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
