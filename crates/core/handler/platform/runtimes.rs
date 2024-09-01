use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post, Router};
use serde::{Deserialize, Serialize};

use crate::extract::{Json, Path};
use crate::handler::Result;
use crate::service::{AppDatabase, AppState, RuntimePool};

/// `./:runtime/`
#[must_use]
#[derive(Debug, Deserialize)]
pub struct RuntimePathParams {
    pub runtime: String,
}

#[must_use]
#[derive(Debug, Deserialize)]
struct CreateRuntimeRequest {}

#[must_use]
#[derive(Debug, Serialize)]
struct CreateRuntimeResponse {}

/// TODO.
#[tracing::instrument]
async fn create_runtime(
    State(database): State<AppDatabase>,
    State(runtime): State<RuntimePool>,
    Json(request): Json<CreateRuntimeRequest>,
) -> Result<(StatusCode, Json<CreateRuntimeResponse>)> {
    let response = CreateRuntimeResponse {};
    Ok((StatusCode::CREATED, Json(response)))
}

#[must_use]
#[derive(Debug, Serialize)]
struct ListRuntimeResponse {}

/// TODO.
#[tracing::instrument]
async fn list_runtimes(
    State(database): State<AppDatabase>,
    State(runtime): State<RuntimePool>,
) -> Result<(StatusCode, Json<ListRuntimeResponse>)> {
    let response = ListRuntimeResponse {};
    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Serialize)]
struct RetrieveRuntimeResponse {}

/// TODO.
#[tracing::instrument]
async fn retrieve_runtime(
    State(database): State<AppDatabase>,
    State(runtime): State<RuntimePool>,
    Path(params): Path<RuntimePathParams>,
) -> Result<(StatusCode, Json<RetrieveRuntimeResponse>)> {
    let response = RetrieveRuntimeResponse {};
    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct ModifyRuntimeRequest {}

#[must_use]
#[derive(Debug, Serialize)]
struct ModifyRuntimeResponse {}

/// TODO.
#[tracing::instrument]
async fn modify_runtime(
    State(database): State<AppDatabase>,
    State(runtime): State<RuntimePool>,
    Path(params): Path<RuntimePathParams>,
    Json(request): Json<ModifyRuntimeRequest>,
) -> Result<(StatusCode, Json<ModifyRuntimeResponse>)> {
    let response = ModifyRuntimeResponse {};
    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct DeleteRuntimeRequest {}

#[must_use]
#[derive(Debug, Serialize)]
struct DeleteRuntimeResponse {}

/// TODO.
#[tracing::instrument]
async fn delete_runtime(
    State(database): State<AppDatabase>,
    State(runtime): State<RuntimePool>,
    Path(params): Path<RuntimePathParams>,
    Json(request): Json<DeleteRuntimeRequest>,
) -> Result<(StatusCode, Json<DeleteRuntimeResponse>)> {
    let response = DeleteRuntimeResponse {};
    Ok((StatusCode::OK, Json(response)))
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_runtime))
        .route("/", get(list_runtimes))
        .route("/:runtime", post(retrieve_runtime))
        .route("/:runtime", patch(modify_runtime))
        .route("/:runtime", delete(delete_runtime))
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::platform::runtimes;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = runtimes::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
