//! TODO.
//!

pub use crate::service::app_config::{AppBuilder, AppConfig};
pub use crate::service::app_database::AppDatabase;
pub use crate::service::conn_info::AppConnectInfo;
pub use crate::service::runtime_pool::RuntimePool;
pub use crate::service::socket_room::{WebsocketRoom, WebsocketServer};

mod app_config;
mod app_database;
mod conn_info;
mod runtime_pool;
mod socket_room;

/// Application state.
///
/// Used for the [`State`] extraction (dependency injection).
///
/// [`State`]: axum::extract::State
#[must_use = "state does nothing unless you use it"]
#[derive(Debug, Clone)]
pub struct AppState {
    dataset: AppDatabase,
    runtime: RuntimePool,
    room: WebsocketServer,
}

impl AppState {
    /// Returns a new [`AppState`].
    #[inline]
    pub async fn connect(config: AppConfig) -> anyhow::Result<Self> {
        // TODO: Load all tasks with checked triggers.
        let dataset = AppDatabase::connect(config.database).await?;
        let runtime = RuntimePool::new();
        let room = WebsocketServer::new();
        Ok(Self {
            dataset,
            runtime,
            room,
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

impl_di!(dataset: AppDatabase);
impl_di!(runtime: RuntimePool);
impl_di!(room: WebsocketServer);

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
