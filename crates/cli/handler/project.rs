//! TODO.
//!

use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post, Router};
use serde::{Deserialize, Serialize};

use crate::extract::{Json, Path};
use crate::handler::Result;
use crate::service::{AppState, Dataset};

#[must_use]
#[derive(Debug, Deserialize)]
struct CreateProjectRequest {
    pub name: String,
    pub tags: Vec<String>,
    pub archive: Option<bool>,
}

#[must_use]
#[derive(Debug, Serialize)]
struct CreateProjectResponse {
    pub id: String,
}

/// TODO.
async fn create_project(
    Json(request): Json<CreateProjectRequest>,
) -> Result<(StatusCode, Json<CreateProjectResponse>)> {
    let response = CreateProjectResponse { id: "".to_owned() };

    Ok((StatusCode::CREATED, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct ModifyProjectRequest {
    pub name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub archive: Option<bool>,
}

#[must_use]
#[derive(Debug, Serialize)]
struct ModifyProjectResponse {
    pub id: String,
}

/// TODO.
async fn modify_project(
    Path(id): Path<String>,
    Json(request): Json<ModifyProjectRequest>,
) -> Result<(StatusCode, Json<ModifyProjectResponse>)> {
    let response = ModifyProjectResponse { id: "".to_owned() };

    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct ListProjectRequest {}

#[must_use]
#[derive(Debug, Serialize)]
struct ListProjectResponse {}

/// TODO.
async fn list_project(
    Json(request): Json<ListProjectRequest>,
) -> Result<(StatusCode, Json<ListProjectResponse>)> {
    let response = ListProjectResponse {};
    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct RetrieveProjectRequest {}

#[must_use]
#[derive(Debug, Serialize)]
struct RetrieveProjectResponse {}

/// TODO.
async fn retrieve_project(
    Path(id): Path<String>,
    Json(request): Json<RetrieveProjectRequest>,
) -> Result<(StatusCode, Json<RetrieveProjectResponse>)> {
    let response = RetrieveProjectResponse {};
    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct DeleteProjectRequest {}

#[must_use]
#[derive(Debug, Serialize)]
struct DeleteProjectResponse {}

/// TODO.
async fn delete_project(
    State(dataset): State<Dataset>,
    Path(id): Path<String>,
    Json(request): Json<DeleteProjectRequest>,
) -> Result<(StatusCode, Json<DeleteProjectResponse>)> {
    let response = DeleteProjectResponse {};
    Ok((StatusCode::OK, Json(response)))
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_project))
        .route("/", get(list_project))
        .route("/:id", patch(modify_project))
        .route("/:id", get(retrieve_project))
        .route("/:id", delete(delete_project))
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::project;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = project::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
