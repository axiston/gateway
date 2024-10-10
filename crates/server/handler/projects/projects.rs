use axiston_database::AppDatabase;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post, Router};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::extract::{AuthState, Json, Path};
use crate::handler::accounts::AccountPathParams;
use crate::handler::projects::ProjectPathParams;
use crate::handler::Result;
use crate::service::AppState;

/// See [`create_new_project`].
#[must_use]
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "projects.ts")]
struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// See [`create_new_project`].
#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "projects.ts")]
struct CreateProjectResponse {
    /// Unique identifier of the created project.
    pub project: Uuid,
}

/// #### Endpoints
///
/// - `POST ./:account/`
#[tracing::instrument]
async fn create_new_project(
    authentication: AuthState,
    State(database): State<AppDatabase>,
    Json(request): Json<CreateProjectRequest>,
) -> Result<(StatusCode, Json<CreateProjectResponse>)> {
    let response = CreateProjectResponse {
        project: Uuid::new_v4(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

#[must_use]
#[derive(Debug, Default, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "projects.ts")]
struct ListProjectsRequest {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "projects.ts")]
struct ProjectStatisticsResponse {
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

#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "projects.ts")]
struct ProjectDataResponse {
    // todo: id
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,

    pub show_order: i32,
    pub is_admin: bool,
    pub is_pinned: bool,
    pub is_hidden: bool,
}

#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "projects.ts")]
struct ListProjectsResponse {
    pub projects: Vec<ProjectDataResponse>,
}

/// `GET ./:account/`
#[tracing::instrument]
async fn list_all_projects(
    authentication: AuthState,
    State(database): State<AppDatabase>,
    Path(params): Path<AccountPathParams>,
    request: Option<Json<ListProjectsRequest>>,
) -> Result<(StatusCode, Json<ListProjectsResponse>)> {
    let Json(request) = request.unwrap_or_default();
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
#[ts(export, export_to = "projects.ts")]
struct RetrieveProjectResponse {
    #[serde(flatten)]
    pub data: ProjectDataResponse,
}

/// `GET ./:account/:project/`
#[tracing::instrument]
async fn retrieve_project_details(
    authentication: AuthState,
    State(database): State<AppDatabase>,
    Path(params): Path<ProjectPathParams>,
) -> Result<(StatusCode, Json<RetrieveProjectResponse>)> {
    let response = RetrieveProjectResponse {
        data: ProjectDataResponse {
            name: "".to_string(),
            description: None,
            tags: Vec::new(),
            show_order: 0,
            is_admin: false,
            is_pinned: false,
            is_hidden: false,
        },
    };

    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "projects.ts")]
struct ModifyProjectRequest {
    pub name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub archive: Option<bool>,
}

#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "projects.ts")]
struct ModifyProjectResponse {
    pub project_id: Uuid,
}

/// `PATCH ./:account/:project/`
#[tracing::instrument]
async fn modify_project(
    authentication: AuthState,
    State(database): State<AppDatabase>,
    Path(params): Path<ProjectPathParams>,
    Json(request): Json<ModifyProjectRequest>,
) -> Result<(StatusCode, Json<ModifyProjectResponse>)> {
    let response = ModifyProjectResponse {
        project_id: Uuid::new_v4(),
    };

    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "projects.ts")]
struct DeleteProjectResponse {
    /// Identifier of the deleted project.
    pub project: Uuid,
}

/// - `DELETE ./:account/:project/`
#[tracing::instrument]
async fn delete_project(
    authentication: AuthState,
    State(database): State<AppDatabase>,
    Path(params): Path<ProjectPathParams>,
) -> Result<(StatusCode, Json<DeleteProjectResponse>)> {
    let response = DeleteProjectResponse {
        project: params.project,
    };

    Ok((StatusCode::OK, Json(response)))
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/projects/", post(create_new_project))
        .route("/projects/", get(list_all_projects))
        .route("/projects/:project/", patch(modify_project))
        .route("/projects/:project/", get(retrieve_project_details))
        .route("/projects/:project/", delete(delete_project))
}

#[cfg(test)]
mod test {
    use axum::handler::Handler;
    use axum_test::TestServer;

    use crate::handler::projects::projects::routes;
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
