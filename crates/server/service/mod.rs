//! TODO.
//!

pub use axiston_database::AppDatabase;
use axiston_database::AppDatabaseExt;
pub use axiston_graph::worker::GraphWorker;
pub use axiston_runtime::RuntimeClient;

pub use crate::service::app_config::{AppBuilder, AppConfig};
pub use crate::service::app_hashing::AppHashing;
pub use crate::service::conn_info::AppConnectInfo;
pub use crate::service::socket_room::{WebsocketRoom, WebsocketServer};

mod app_config;
mod app_hashing;
mod conn_info;
mod database_err;
mod socket_room;

/// Application state.
///
/// Used for the [`State`] extraction (dependency injection).
///
/// [`State`]: axum::extract::State
#[must_use = "state does nothing unless you use it"]
#[derive(Debug, Clone)]
pub struct AppState {
    app_database: AppDatabase,
    app_hashing: AppHashing,
    client_runtime: RuntimeClient,
    websocket_room: WebsocketServer,
}

impl AppState {
    /// Returns a new [`AppState`].
    #[inline]
    pub async fn connect(app_config: AppConfig) -> anyhow::Result<Self> {
        // TODO: Load all tasks with checked triggers.
        let app_database = if app_config.multiple_gateways {
            AppDatabase::connect_multiple_instances(&app_config.database_conn).await?
        } else {
            AppDatabase::connect_single_instance(&app_config.database_conn).await?
        };

        if let Err(migration_err) = app_database.apply_migrations(None).await {
            let _ = app_database.rollback_migrations(None).await;
            return Err(migration_err.into());
        };

        // TODO: Load startups clients.
        let runtime = RuntimeClient::builder().build();

        Ok(Self {
            app_database,
            app_hashing: AppHashing::new(),
            client_runtime: runtime,
            websocket_room: WebsocketServer::new(),
        })
    }
}

macro_rules! impl_di {
    ($($f:ident: $t:ty),+) => {$(
        impl axum::extract::FromRef<AppState> for $t {
            fn from_ref(state: &AppState) -> Self {
                state.$f.clone()
            }
        }
    )+};
}

impl_di!(app_database: AppDatabase);
impl_di!(app_hashing: AppHashing);
impl_di!(client_runtime: RuntimeClient);
impl_di!(websocket_room: WebsocketServer);

#[cfg(test)]
mod test {
    use crate::service::{AppConfig, AppState};

    #[test]
    fn instance_app_state() {
        let config = AppConfig::builder();
        let _ = AppState::connect(config.build());
    }

    #[test]
    fn configure_app_state() {
        let config = AppConfig::builder();
        let _ = AppState::connect(config.build());
    }
}
