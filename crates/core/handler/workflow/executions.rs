use axum::http::StatusCode;
use axum::routing::{get, Router};
use serde::{Deserialize, Serialize};

use crate::extract::{Json, Path};
use crate::handler::workflow::WorkflowPathParams;
use crate::handler::Result;
use crate::service::AppState;

/// `Path` param for a `:execution` router.
///
/// `./:account/:project/:workflow/:execution/`
#[must_use]
#[derive(Debug, Deserialize)]
struct ExecutionPathParams {
    pub account: String,
    pub project: String,
    pub workflow: String,
    pub execution: String,
}

#[derive(Debug, Deserialize)]
struct GetExecutionRequest {}

#[derive(Debug, Serialize)]
struct GetExecutionResponse {
    pub execution: String,
}

#[tracing::instrument]
async fn get_execution_details(
    Path(params): Path<ExecutionPathParams>,
    Json(request): Json<GetExecutionRequest>,
) -> Result<(StatusCode, Json<GetExecutionResponse>)> {
    let response = GetExecutionResponse {
        execution: params.execution,
    };
    Ok((StatusCode::OK, Json(response)))
}

#[derive(Debug, Default, Deserialize)]
struct ListExecutionRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Serialize)]
struct ListExecutionResponse {}

#[tracing::instrument]
async fn list_all_executions(
    Path(params): Path<WorkflowPathParams>,
    request: Option<Json<ListExecutionRequest>>,
) -> Result<(StatusCode, Json<ListExecutionResponse>)> {
    let request = request.unwrap_or_default();
    let limit = request.limit.unwrap_or(20);
    let offset = request.offset.unwrap_or_default();

    let response = ListExecutionResponse {};
    Ok((StatusCode::OK, Json(response)))
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    let path0 = "/accounts/:account/projects/:project/workflows/:workflow/revisions/";
    let path1 = "/accounts/:account/projects/:project/workflows/:workflow/revisions/:revision/";
    Router::new()
        .route(path0, get(list_all_executions))
        .route(path1, get(get_execution_details))
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::workflow::executions;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = executions::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
