use axum::http::StatusCode;
use axum::routing::{get, Router};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::extract::{Json, Path};
use crate::handler::project::ProjectPathParams;
use crate::handler::Result;
use crate::service::AppState;

#[must_use]
#[derive(Debug, Default, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "project.ts")]
struct AnalyticsRequest {
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "project.ts")]
struct AnalyticsResponse {
    // Workflows (total/enabled).
    pub workflows_total: u32,
    pub workflows_enabled: u32,

    // Runs and success rate.
    pub runs_total: u32,
    pub runs_success: u32,

    // plan usage/limits
    pub ticks_allocated: u32,
    pub ticks_used: u32,
    // alerts/failures
    // security (maybe different handler)
}

/// `GET ./:account/:project/analytics`
#[tracing::instrument]
async fn recent_analytics(
    Path(params): Path<ProjectPathParams>,
    request: Option<Json<AnalyticsRequest>>,
) -> Result<(StatusCode, Json<AnalyticsResponse>)> {
    let Json(request) = request.unwrap_or_default();
    // Unless specified otherwise, start with the first entry.
    let data_from = request.date_from.unwrap_or_default();
    // Unless specified otherwise, end with the last entry.
    let data_to = request.date_to.unwrap_or_default();

    let response = AnalyticsResponse {
        workflows_total: 0,
        workflows_enabled: 0,
        runs_total: 0,
        runs_success: 0,
        ticks_allocated: 0,
        ticks_used: 0,
    };

    Ok((StatusCode::OK, Json(response)))
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    let path = "/:account/:project/analytics";
    Router::new().route(path, get(recent_analytics))
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::project::analytics;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = analytics::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
