use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post, Router};
use serde::{Deserialize, Serialize};

use crate::extract::{Json, Path};
use crate::handler::project::ProjectPathParams;
use crate::handler::workflow::WorkflowPathParams;
use crate::handler::Result;
use crate::service::{AppDatabase, AppState};

#[must_use]
#[derive(Debug, Deserialize)]
struct CreateWorkflowRequest {
    pub name: String,
    pub tags: Vec<String>,
}

#[must_use]
#[derive(Debug, Serialize)]
struct CreateWorkflowResponse {
    pub id: String,
}

/// `POST ./:account/:project/`
#[tracing::instrument]
async fn create_workflow(
    State(database): State<AppDatabase>,
    Path(params): Path<ProjectPathParams>,
    Json(request): Json<CreateWorkflowRequest>,
) -> Result<(StatusCode, Json<CreateWorkflowResponse>)> {
    let response = CreateWorkflowResponse { id: "".to_owned() };

    Ok((StatusCode::CREATED, Json(response)))
}

#[must_use]
#[derive(Debug, Default, Deserialize)]
struct ListWorkflowRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[must_use]
#[derive(Debug, Serialize)]
struct ListWorkflowResponse {}

/// `GET ./:account/:project/`
#[tracing::instrument]
async fn list_workflow(
    State(database): State<AppDatabase>,
    Path(params): Path<ProjectPathParams>,
    request: Option<Json<ListWorkflowRequest>>,
) -> Result<(StatusCode, Json<ListWorkflowResponse>)> {
    let request = request.unwrap_or_default();
    let limit = request.limit.unwrap_or(20);
    let offset = request.offset.unwrap_or_default();

    let response = ListWorkflowResponse {};
    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct RetrieveWorkflowRequest {}

#[must_use]
#[derive(Debug, Serialize)]
struct RetrieveWorkflowResponse {}

/// `GET ./:account/:project/:workflow/`
#[tracing::instrument]
async fn retrieve_workflow(
    State(database): State<AppDatabase>,
    Path(params): Path<WorkflowPathParams>,
    Json(request): Json<RetrieveWorkflowRequest>,
) -> Result<(StatusCode, Json<RetrieveWorkflowResponse>)> {
    let response = RetrieveWorkflowResponse {};
    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct ModifyWorkflowRequest {
    pub name: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[must_use]
#[derive(Debug, Serialize)]
struct ModifyWorkflowResponse {
    pub id: String,
}

/// `PATCH ./:account/:project/:workflow/`
#[tracing::instrument]
async fn modify_workflow(
    State(database): State<AppDatabase>,
    Path(params): Path<WorkflowPathParams>,
    Json(request): Json<ModifyWorkflowRequest>,
) -> Result<(StatusCode, Json<ModifyWorkflowResponse>)> {
    let response = ModifyWorkflowResponse {
        id: params.workflow.to_owned(),
    };

    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct DeleteWorkflowRequest {}

#[must_use]
#[derive(Debug, Serialize)]
struct DeleteWorkflowResponse {}

/// `DELETE ./:account/:project/:workflow/`
#[tracing::instrument]
async fn delete_workflow(
    State(database): State<AppDatabase>,
    Path(params): Path<WorkflowPathParams>,
    Json(request): Json<DeleteWorkflowRequest>,
) -> Result<(StatusCode, Json<DeleteWorkflowResponse>)> {
    let response = DeleteWorkflowResponse {};
    Ok((StatusCode::OK, Json(response)))
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    let path0 = "/accounts/:account/projects/:project/";
    let path1 = "/accounts/:account/projects/:project/workflows/:workflow/";
    Router::new()
        .route(path0, post(create_workflow))
        .route(path0, get(list_workflow))
        .route(path1, patch(modify_workflow))
        .route(path1, get(retrieve_workflow))
        .route(path1, delete(delete_workflow))
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::workflow::workflows;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = workflows::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
