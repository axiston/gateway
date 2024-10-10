use axiston_database::AppDatabase;
use axum::extract::State;
use axum::http::StatusCode;
use axum::middleware::from_fn_with_state;
use axum::routing::{post, Router};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::extract::{AuthToken, Json};
use crate::handler::Result;
use crate::middleware::authentication_guard;
use crate::service::{AppHashing, AppState};

/// See [`sign_up`].
#[must_use]
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "auth.ts")]
struct SignUpRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

/// See [`sign_up`].
#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "auth.ts")]
struct SignUpResponse {
    pub ignore: (),
}

#[tracing::instrument]
async fn sign_up(
    State(database): State<AppDatabase>,
    State(hashing): State<AppHashing>,
    Json(request): Json<SignUpRequest>,
) -> Result<(StatusCode, Json<SignUpResponse>)> {
    todo!()
}

/// See [`sign_in`].
#[must_use]
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "auth.ts")]
struct SignInRequest {
    pub email: String,
    pub password: String,
}

/// Can be used as a sign-in method.
#[tracing::instrument]
async fn sign_in(
    State(database): State<AppDatabase>,
    State(hashing): State<AppHashing>,
    Json(request): Json<SignInRequest>,
) -> Result<(StatusCode, AuthToken)> {
    todo!()
}

/// See [`sign_out`].
#[must_use]
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "auth.ts")]
struct SignOutRequest {
    pub email: String,
    pub password: String,
}

/// See [`sign_out`].
#[must_use]
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "auth.ts")]
struct SignOutResponse {
    pub ignore: (),
}

/// Can be used as a sign-out.
#[tracing::instrument]
async fn sign_out(
    State(database): State<AppDatabase>,
    Json(request): Json<SignOutRequest>,
) -> Result<(StatusCode, Json<SignOutResponse>)> {
    todo!()
}

/// Returns a [`Router`] with all related routes.
pub fn routes(state: AppState) -> Router<AppState> {
    let authenticate = from_fn_with_state(state, authentication_guard);
    Router::new()
        .route("/accounts/auth/signout", post(sign_out))
        .route_layer(authenticate)
        .route("/accounts/auth/signup", post(sign_up))
        .route("/accounts/auth/signin", post(sign_in))
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::accounts::auth::routes;
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
