//! TODO.
//!

pub use axiston_db_connect::Database;
use axiston_db_migrate::DatabaseMigrator;
pub use axiston_graph::worker::GraphWorker;
pub use axiston_runtime::RuntimeConn;

pub use crate::service::app_config::{AppBuilder, AppConfig};
pub use crate::service::argon2_hasher::Argon2Hasher;
pub use crate::service::scheduler::{SchedulerError, SchedulerRuntime};
pub use crate::service::socket_room::{WebsocketRoom, WebsocketServer};

mod app_config;
mod argon2_hasher;
mod graph_queue;
mod scheduler;
mod socket_room;

/// Application state.
///
/// Used for the [`State`] extraction (dependency injection).
///
/// [`State`]: axum::extract::State
#[must_use = "state does nothing unless you use it"]
#[derive(Debug, Clone)]
pub struct AppState {
    database: Database,
    hasher: Argon2Hasher,
    client_runtime: RuntimeConn,
    websocket_room: WebsocketServer,
}

impl AppState {
    /// Returns a new [`AppState`].
    #[inline]
    pub async fn connect(app_config: AppConfig) -> anyhow::Result<Self> {
        let database = Self::connect_database(&app_config).await?;
        let runtime = Self::connect_runtime(&app_config).await?;

        Ok(Self {
            database,
            hasher: Argon2Hasher::new(),
            client_runtime: runtime,
            websocket_room: WebsocketServer::new(),
        })
    }

    async fn connect_database(app_config: &AppConfig) -> anyhow::Result<Database> {
        let database = if app_config.multiple {
            Database::new_multiple_gateways(&app_config.database)
        } else {
            Database::new_single_gateway(&app_config.database)
        };

        let _ = {
            let connection = database.get_connection().await?;
            let mut migrator = DatabaseMigrator::new(connection);
            let migrations = migrator.apply_migrations().await?;
            tracing::info!(target: "database", migrations);
        };

        Ok(database)
    }

    async fn connect_runtime(app_config: &AppConfig) -> anyhow::Result<RuntimeConn> {
        // TODO: Load startups clients.
        let runtime = RuntimeConn::builder().build();
        Ok(runtime)
    }

    async fn run_trigger_daemon(database: Database) -> anyhow::Result<()> {
        // TODO: Load all tasks with checked triggers.
        let connection = database.get_connection().await?;

        Ok(())
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

impl_di!(database: Database);
impl_di!(hasher: Argon2Hasher);
impl_di!(client_runtime: RuntimeConn);
impl_di!(websocket_room: WebsocketServer);

#[cfg(test)]
mod test {
    use crate::service::{AppConfig, AppState};

    #[test]
    fn instance_app_state() {
        let config = AppConfig::builder();
        let _ = AppState::connect(config.build());
    }
}
