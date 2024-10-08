use axum::http::StatusCode;
use axum::routing::{delete, get};
use axum::Router;
use serde::{Deserialize, Serialize};

use crate::extract::{Json, Path};
use crate::handler::workflow::WorkflowPathParams;
use crate::handler::Result;
use crate::service::AppState;

#[derive(Debug, Deserialize)]
pub struct GetRevisionRequest {}

#[derive(Debug, Serialize)]
pub struct GetRevisionResponse {}

#[tracing::instrument]
async fn load_workflow_revision(
    Path(params): Path<WorkflowPathParams>,
    request: Option<Json<GetRevisionRequest>>,
) -> Result<(StatusCode, Json<GetRevisionResponse>)> {
    let response = GetRevisionResponse {};
    Ok((StatusCode::OK, Json(response)))
}

#[derive(Debug, Default, Deserialize)]
pub struct ListRevisionRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct ListRevisionResponse {}

#[tracing::instrument]
async fn list_workflow_revisions(
    Path(params): Path<WorkflowPathParams>,
    request: Option<Json<ListRevisionRequest>>,
) -> Result<(StatusCode, Json<ListRevisionResponse>)> {
    let request = request.unwrap_or_default();
    let limit = request.limit.unwrap_or(20);
    let offset = request.offset.unwrap_or_default();

    let response = ListRevisionResponse {};
    Ok((StatusCode::OK, Json(response)))
}

#[derive(Debug, Deserialize)]
pub struct DeleteRevisionRequest {}

#[derive(Debug, Serialize)]
pub struct DeleteRevisionResponse {}

#[tracing::instrument]
async fn mark_revision_as_usable(
    Path(params): Path<WorkflowPathParams>,
    request: Option<Json<DeleteRevisionRequest>>,
) -> Result<(StatusCode, Json<DeleteRevisionResponse>)> {
    let response = DeleteRevisionResponse {};
    Ok((StatusCode::OK, Json(response)))
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    let path0 = "/accounts/:account/projects/:project/workflows/:workflow/revisions/";
    let path1 = "/accounts/:account/projects/:project/workflows/:workflow/revisions/:revision/";
    Router::new()
        .route(path0, get(list_workflow_revisions))
        .route(path1, get(load_workflow_revision))
        .route(path1, delete(mark_revision_as_usable))
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::workflow::revisions;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = revisions::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
