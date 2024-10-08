use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post};
use axum::Router;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::extract::{Json, Path};
use crate::handler::account::AccountPathParams;
use crate::handler::Result;
use crate::service::AppState;

#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "account.ts")]
struct RetrieveAccountResponse {
    pub account: String,
}

/// `GET ./:account/`
#[tracing::instrument]
fn retrieve_account(
    Path(params): Path<AccountPathParams>,
) -> Result<(StatusCode, Json<RetrieveAccountResponse>)> {
    let response = RetrieveAccountResponse {
        account: params.account,
    };

    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "account.ts")]
struct UpdateAccountRequest {
    pub name: Option<String>,
}

#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "account.ts")]
struct UpdateAccountResponse {
    pub id: String,
}

/// `PATCH ./:account/`
#[tracing::instrument]
async fn update_account(
    Path(params): Path<AccountPathParams>,
    Json(request): Json<UpdateAccountRequest>,
) -> Result<(StatusCode, Json<UpdateAccountResponse>)> {
    let response = UpdateAccountResponse { id: params.account };
    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "account.ts")]
struct DeleteAccountRequest {
    pub confirm: bool,
}

#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "account.ts")]
struct DeleteAccountResponse {
    pub account: String,
}

/// `DELETE ./:account/`
#[tracing::instrument]
async fn delete_account(
    Path(params): Path<AccountPathParams>,
    Json(request): Json<DeleteAccountRequest>,
) -> Result<(StatusCode, Json<DeleteAccountResponse>)> {
    let response = DeleteAccountResponse {
        account: params.account,
    };

    Ok((StatusCode::OK, Json(response)))
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/accounts/:account", get(retrieve_account))
        .route("/accounts/:account", patch(update_account))
        .route("/accounts/:account", delete(delete_account))
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::account::accounts;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = accounts::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
