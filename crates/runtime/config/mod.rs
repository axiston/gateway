mod custom_hooks;
mod pool_manager;
mod runtime_config;

use deadpool::managed::{Hook, Object, Pool};
use derive_more::{Deref, DerefMut, From};

use crate::config::custom_hooks::{post_create, post_recycle, pre_recycle};
pub use crate::config::pool_manager::{RuntimeManager, RuntimeManagerConfig};
pub use crate::config::runtime_config::RuntimeConfig;
use crate::RuntimeResult;

/// Asynchronous `runtime` connection pool.
pub struct Runtime {
    inner: Pool<RuntimeManager>,
}

/// [`RuntimeConnection`] wrapper.
///
/// Hides connection pool manager types.
///
/// [`RuntimeConnection`]: crate::RuntimeConn
#[derive(Debug, From, Deref, DerefMut)]
pub struct RuntimeObject {
    inner: Object<RuntimeManager>,
}

impl Runtime {
    /// Returns a new [`Runtime`].
    pub fn new(addr: (), pool_config: RuntimeConfig) -> Self {
        let manager_config = RuntimeManagerConfig::new();
        let manager = RuntimeManager::new_with_config(addr, manager_config);
        let pool = Pool::builder(manager)
            .max_size(pool_config.max_conn.unwrap_or(8))
            .create_timeout(pool_config.create_timeout)
            .wait_timeout(pool_config.wait_timeout)
            .recycle_timeout(pool_config.recycle_timeout)
            .post_create(Hook::sync_fn(post_create))
            .pre_recycle(Hook::sync_fn(pre_recycle))
            .post_recycle(Hook::sync_fn(post_recycle))
            .runtime(deadpool::Runtime::Tokio1);

        let pool = pool.build().expect("should not require runtime");
        Self { inner: pool }
    }

    pub async fn get_connection(&self) -> RuntimeResult<RuntimeObject> {
        self.inner.get().await.map(Into::into).map_err(Into::into)
    }
}
