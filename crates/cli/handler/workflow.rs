//! TODO.
//!

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::{delete, get, patch, post, Router};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

use crate::extract::Json;
use crate::handler::Result;
use crate::server::AppConnectInfo;
use crate::service::{AppState, Dataset, Runtime};

#[must_use]
#[derive(Debug, Deserialize)]
struct CreateWorkflowRequest {
    pub name: String,
    pub tags: Vec<String>,
    pub secure: Option<bool>,
    pub active: Option<bool>,
}

#[must_use]
#[derive(Debug, Serialize)]
struct CreateWorkflowResponse {
    pub id: String,
}

/// TODO.
async fn create_workflow(
    Json(request): Json<CreateWorkflowRequest>,
) -> Result<(StatusCode, Json<CreateWorkflowResponse>)> {
    let response = CreateWorkflowResponse { id: "".to_owned() };

    Ok((StatusCode::CREATED, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct ListWorkflowRequest {}

#[must_use]
#[derive(Debug, Serialize)]
struct ListWorkflowResponse {}

/// TODO.
async fn list_workflow(
    Json(request): Json<ListWorkflowRequest>,
) -> Result<(StatusCode, Json<ListWorkflowResponse>)> {
    let response = ListWorkflowResponse {};
    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct RetrieveWorkflowRequest {}

#[must_use]
#[derive(Debug, Serialize)]
struct RetrieveWorkflowResponse {}

/// TODO.
async fn retrieve_workflow(
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
    pub secure: Option<bool>,
    pub active: Option<bool>,
}

#[must_use]
#[derive(Debug, Serialize)]
struct ModifyWorkflowResponse {
    pub id: String,
}

/// TODO.
async fn modify_workflow(
    Json(request): Json<ModifyWorkflowRequest>,
) -> Result<(StatusCode, Json<ModifyWorkflowResponse>)> {
    let response = ModifyWorkflowResponse { id: "".to_owned() };

    Ok((StatusCode::OK, Json(response)))
}

#[must_use]
#[derive(Debug, Deserialize)]
struct DeleteWorkflowRequest {}

#[must_use]
#[derive(Debug, Serialize)]
struct DeleteWorkflowResponse {}

/// TODO.
async fn delete_workflow(
    Json(request): Json<DeleteWorkflowRequest>,
) -> Result<(StatusCode, Json<DeleteWorkflowResponse>)> {
    let response = DeleteWorkflowResponse {};
    Ok((StatusCode::OK, Json(response)))
}

/// HTTP request handler.
#[tracing::instrument]
async fn websocket_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(info): ConnectInfo<AppConnectInfo>,
    State(dataset): State<Dataset>,
    State(runtime): State<Runtime>,
) -> Response {
    ws.on_upgrade(move |socket| websocket_worker(socket, info, dataset, runtime))
}

/// Websocket state machine.
#[tracing::instrument]
async fn websocket_worker(
    mut socket: WebSocket,
    conn: AppConnectInfo,
    dataset: Dataset,
    runtime: Runtime,
) {
    let (sink, mut stream) = socket.split();
    tracing::info!(
        target: "workflow:ws",
        conn = conn.addr.to_string(),
        "connected socket"
    );

    while let Some(message) = stream.next().await {
        let message = message.unwrap();
        match message {
            Message::Text(_) => {}
            Message::Binary(_) => {}
            Message::Ping(_) => {}
            Message::Pong(_) => {}
            Message::Close(_) => {}
        }
    }

    tracing::info!(
        target: "workflow:ws",
        conn = conn.addr.to_string(),
        "disconnected socket"
    );
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_workflow))
        .route("/", get(list_workflow))
        .route("/:id", get(retrieve_workflow))
        .route("/:id", patch(modify_workflow))
        .route("/:id", delete(delete_workflow))
        .route("/ws", get(websocket_handler))
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::workflow;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = workflow::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
