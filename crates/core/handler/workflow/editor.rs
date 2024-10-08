use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::response::Response;
use axum::routing::Router;
use futures::StreamExt;

use crate::service::{
    AppConnectInfo, AppDatabase, AppState, RuntimePool, WebsocketRoom, WebsocketServer,
};

/// HTTP request handler.
#[tracing::instrument]
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(info): ConnectInfo<AppConnectInfo>,
    State(database): State<AppDatabase>,
    State(runtime): State<RuntimePool>,
    State(websocket): State<WebsocketServer>,
) -> Response {
    let room = websocket.find_room_by_id(0).unwrap();
    ws.on_upgrade(move |socket| websocket_worker(socket, info, database, runtime, room))
}

/// Websocket state machine.
#[tracing::instrument]
async fn websocket_worker(
    mut socket: WebSocket,
    conn: AppConnectInfo,
    database: AppDatabase,
    runtime: RuntimePool,
    websocket: WebsocketRoom,
) {
    let (sink, mut stream) = socket.split();
    tracing::info!(
        target: "workflow:ws", conn = conn.addr.to_string(), "connected socket",
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
        target: "workflow:ws", conn = conn.addr.to_string(), "disconnected socket",
    );
}

/// Returns a [`Router`] with all related routes.
pub fn routes() -> Router<AppState> {
    Router::new()
}

#[cfg(test)]
mod test {
    use axum_test::TestServer;

    use crate::handler::workflow::editor;
    use crate::service::{AppConfig, AppState};

    #[tokio::test]
    async fn routes() -> anyhow::Result<()> {
        let config = AppConfig::default();
        let state = AppState::connect(config).await?;
        let app = editor::routes().with_state(state);
        let server = TestServer::new(app)?;

        Ok(())
    }
}
