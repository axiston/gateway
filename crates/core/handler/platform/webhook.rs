use axum::http::{Method, StatusCode};
use axum::routing::any;
use axum::Router;
use serde::{Deserialize, Serialize};

use crate::extract::{Json, Path};
use crate::handler::Result;
use crate::service::AppState;

/// [`Path`] param for a `./::webhook/` router.
///
/// [`Path`]: crate::extract::Path
#[must_use]
#[derive(Debug, Deserialize)]
pub struct WebhookPathParams {
    pub webhook: String,
}

#[derive(Debug, Deserialize)]
struct TriggerWebhookRequest {}

#[derive(Debug, Serialize)]
struct TriggerWebhookResponse {}

/// TODO.
#[tracing::instrument]
async fn trigger_webhook(
    method: Method,
    Path(params): Path<WebhookPathParams>,
    request: Option<Json<TriggerWebhookRequest>>,
) -> Result<(StatusCode, Json<TriggerWebhookResponse>)> {
    let response = TriggerWebhookResponse {};
    Ok((StatusCode::OK, Json(response)))
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    Router::new().route("/:webhook/", any(trigger_webhook))
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::platform::webhook;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = webhook::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
