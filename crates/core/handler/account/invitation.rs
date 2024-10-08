use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{post, Router};
use serde::{Deserialize, Serialize};

use crate::extract::Json;
use crate::handler::Result;
use crate::service::{AppDatabase, AppState};

#[must_use]
#[derive(Debug, Deserialize)]
struct SignUpRequest {
    pub username: String,
    pub password: String,
    pub invite: String,
}

#[must_use]
#[derive(Debug, Serialize)]
struct SignUpResponse {}

/// TODO.
#[tracing::instrument]
async fn sign_up(
    State(database): State<AppDatabase>,
    Json(request): Json<SignUpRequest>,
) -> Result<(StatusCode, Json<SignUpResponse>)> {
    let response = SignUpResponse {};
    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct SignInRequest {
    pub username: String,
    pub password: String,
}

#[must_use]
#[derive(Debug, Serialize)]
struct SignInResponse {}

/// TODO.
#[tracing::instrument]
async fn sign_in(
    State(database): State<AppDatabase>,
    Json(request): Json<SignInRequest>,
) -> Result<(StatusCode, Json<SignInResponse>)> {
    let response = SignInResponse {};
    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct SignOutRequest {}

#[must_use]
#[derive(Debug, Serialize)]
struct SignOutResponse {}

/// TODO.
#[tracing::instrument]
async fn sign_out(
    State(database): State<AppDatabase>,
    Json(request): Json<SignOutRequest>,
) -> Result<(StatusCode, Json<SignOutResponse>)> {
    let response = SignOutResponse {};
    Ok((StatusCode::OK, Json(response)))
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/up", post(sign_up))
        .route("/in", post(sign_in))
        .route("/out", post(sign_out))
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::account::invitation;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = invitation::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
