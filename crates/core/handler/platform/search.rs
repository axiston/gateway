use axum::extract::State;
use axum::http::StatusCode;
use axum::Router;
use serde::{Deserialize, Serialize};

use crate::extract::Json;
use crate::handler::Result;
use crate::service::{AppDatabase, AppState, RuntimePool};

#[must_use]
#[derive(Debug, Deserialize)]
struct SearchIntegrationRequest {}

#[must_use]
#[derive(Debug, Serialize)]
struct SearchIntegrationResponse {}

/// TODO.
#[tracing::instrument]
async fn search_integration(
    State(database): State<AppDatabase>,
    State(runtime): State<RuntimePool>,
    Json(request): Json<SearchIntegrationRequest>,
) -> Result<(StatusCode, Json<SearchIntegrationResponse>)> {
    let response = SearchIntegrationResponse {};
    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct ListRegistryRequest {}

#[must_use]
#[derive(Debug, Serialize)]
struct ListRegistryResponse {}

/// TODO.
#[tracing::instrument]
async fn list_registry(
    State(database): State<AppDatabase>,
    State(runtime): State<RuntimePool>,
    Json(request): Json<ListRegistryRequest>,
) -> Result<(StatusCode, Json<ListRegistryResponse>)> {
    let response = ListRegistryResponse {};
    Ok((StatusCode::OK, Json(response)))
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    Router::new()
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::platform::search;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = search::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
