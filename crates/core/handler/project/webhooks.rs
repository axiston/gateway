use axum::http::StatusCode;
use axum::Router;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::extract::{Json, Path};
use crate::handler::project::ProjectPathParams;
use crate::handler::Result;
use crate::service::AppState;

/// [`Path`] param for a `:webhook` router.
///
/// `./:account/:project/:webhook/`
#[must_use]
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "project.ts")]
pub struct WebhookPathParams {
    pub account: String,
    pub project: String,
    pub webhook: String,
}

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "project.ts")]
struct CreateWebhookRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "project.ts")]
struct CreateWebhookResponse {
    pub hook: String,
}

/// TODO.
#[tracing::instrument]
async fn create_webhook(
    Path(params): Path<ProjectPathParams>,
    Json(request): Json<CreateWebhookRequest>,
) -> Result<(StatusCode, Json<CreateWebhookResponse>)> {
    let response = CreateWebhookResponse {
        hook: "".to_owned(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "project.ts")]
struct DeleteWebhookResponse {
    pub id: String,
}

/// `DELETE ./:account/:project/:webhook/`
#[tracing::instrument]
async fn delete_webhook(
    Path(params): Path<WebhookPathParams>,
) -> Result<(StatusCode, Json<DeleteWebhookResponse>)> {
    let response = DeleteWebhookResponse { id: "".to_owned() };
    Ok((StatusCode::OK, Json(response)))
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    Router::new()
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::project::webhooks;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = webhooks::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
