use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post, Router};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::extract::{Json, Path};
use crate::handler::account::AccountPathParams;
use crate::handler::project::ProjectPathParams;
use crate::handler::Result;
use crate::service::{AppDatabase, AppState};

#[must_use]
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "project.ts")]
struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub private: Option<bool>,
}

#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "project.ts")]
struct CreateProjectResponse {
    pub project: String,
}

/// `POST ./:account/`
#[tracing::instrument]
async fn create_new_project(
    State(database): State<AppDatabase>,
    Json(request): Json<CreateProjectRequest>,
) -> Result<(StatusCode, Json<CreateProjectResponse>)> {
    let private = request.private.unwrap_or_default();
    let response = CreateProjectResponse {
        project: "".to_owned(),
    };
    Ok((StatusCode::CREATED, Json(response)))
}

#[must_use]
#[derive(Debug, Default, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "project.ts")]
struct ListProjectsRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "project.ts")]
struct ListProjectsResponse {
    pub projects: Vec<RetrieveProjectResponse>,
}

/// `GET ./:account/`
#[tracing::instrument]
async fn list_all_projects(
    State(database): State<AppDatabase>,
    Path(params): Path<AccountPathParams>,
    request: Option<Json<ListProjectsRequest>>,
) -> Result<(StatusCode, Json<ListProjectsResponse>)> {
    let request = request.unwrap_or_default();
    let limit = request.limit.unwrap_or(20);
    let offset = request.offset.unwrap_or_default();

    let response = ListProjectsResponse {
        projects: Vec::new(),
    };

    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "project.ts")]
struct RetrieveProjectResponse {
    pub name: String,
}

/// `GET ./:account/:project/`
#[tracing::instrument]
async fn retrieve_project_details(
    State(database): State<AppDatabase>,
    Path(params): Path<ProjectPathParams>,
) -> Result<(StatusCode, Json<RetrieveProjectResponse>)> {
    let response = RetrieveProjectResponse {
        name: "".to_owned(),
    };

    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "project.ts")]
struct ModifyProjectRequest {
    pub name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub archive: Option<bool>,
}

#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "project.ts")]
struct ModifyProjectResponse {
    pub id: String,
}

/// `PATCH ./:account/:project/`
#[tracing::instrument]
async fn modify_project(
    State(database): State<AppDatabase>,
    Path(params): Path<ProjectPathParams>,
    Json(request): Json<ModifyProjectRequest>,
) -> Result<(StatusCode, Json<ModifyProjectResponse>)> {
    let response = ModifyProjectResponse { id: "".to_owned() };

    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "project.ts")]
struct DeleteProjectResponse {
    pub id: String,
}

/// `DELETE ./:account/:project/`
#[tracing::instrument]
async fn delete_project(
    State(database): State<AppDatabase>,
    Path(params): Path<ProjectPathParams>,
) -> Result<(StatusCode, Json<DeleteProjectResponse>)> {
    let response = DeleteProjectResponse { id: params.project };

    Ok((StatusCode::OK, Json(response)))
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    let path0 = "/accounts/:account/projects/";
    let path1 = "/accounts/:account/projects/:project/";
    Router::new()
        .route(path0, post(create_new_project))
        .route(path0, get(list_all_projects))
        .route(path1, patch(modify_project))
        .route(path1, get(retrieve_project_details))
        .route(path1, delete(delete_project))
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::project::projects;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = projects::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
