use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::Router;
use serde::{Deserialize, Serialize};

use crate::extract::Json;
use crate::handler::Result;
use crate::service::{AppDatabase, AppState};

// TODO: Move local auth with invitation here.
// TODO: Move into account handlers.

#[must_use]
#[derive(Debug, Deserialize)]
struct CreateLocalAuthRequest {
    pub username: String,
    pub password: String,
}

#[must_use]
#[derive(Debug, Serialize)]
struct CreateLocalAuthResponse {}

/// TODO.
#[tracing::instrument]
async fn create_local_auth(
    State(database): State<AppDatabase>,
    Json(request): Json<CreateLocalAuthRequest>,
) -> Result<(StatusCode, Json<CreateLocalAuthResponse>)> {
    let response = CreateLocalAuthResponse {};
    Ok((StatusCode::CREATED, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct CreateInviteAuthRequest {
    pub invitation: String,
    pub username: String,
    pub password: String,
}

#[must_use]
#[derive(Debug, Serialize)]
struct CreateInviteAuthResponse {}

/// TODO.
#[tracing::instrument]
async fn create_invite_auth(
    State(database): State<AppDatabase>,
    Json(request): Json<CreateInviteAuthRequest>,
) -> Result<(StatusCode, Json<CreateInviteAuthResponse>)> {
    let response = CreateInviteAuthResponse {};
    Ok((StatusCode::CREATED, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct UseLocalAuthRequest {
    pub username: String,
    pub password: String,
}

#[must_use]
#[derive(Debug, Serialize)]
struct UseLocalAuthResponse {}

/// TODO.
#[tracing::instrument]
async fn use_local_auth(
    State(database): State<AppDatabase>,
    Json(request): Json<UseLocalAuthRequest>,
) -> Result<(StatusCode, Json<UseLocalAuthResponse>)> {
    let response = UseLocalAuthResponse {};
    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct UpdateLocalAuthRequest {
    pub username: Option<String>,
    pub password: Option<String>,
}

#[must_use]
#[derive(Debug, Serialize)]
struct UpdateLocalAuthResponse {}

/// TODO.
#[tracing::instrument]
async fn update_local_auth(
    State(database): State<AppDatabase>,
    Json(request): Json<UpdateLocalAuthRequest>,
) -> Result<(StatusCode, Json<UpdateLocalAuthResponse>)> {
    let response = UpdateLocalAuthResponse {};
    Ok((StatusCode::OK, Json(response)))
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/password", post(create_local_auth))
        .route("/invite", post(create_invite_auth))
        .route("/", post(update_local_auth))
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::account::password;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = password::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
