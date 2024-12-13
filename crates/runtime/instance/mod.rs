use std::fmt;

use deadpool::managed::{Hook, Object, Pool};
use derive_more::{Deref, DerefMut, From};
use uuid::Uuid;

use crate::instance::custom_hooks::{post_create, post_recycle, pre_recycle};
pub use crate::instance::pool_config::RuntimeConfig;
use crate::manager::{RuntimeEndpoint, RuntimeManager, RuntimeManagerConfig};
use crate::Result;

mod custom_hooks;
mod pool_config;

/// Asynchronous `runtime` connection pool.
pub struct Runtime {
    inner: Pool<RuntimeManager>,
}

/// `RuntimeConnection` wrapper.
///
/// Hides connection pool manager types.
#[derive(Debug, From, Deref, DerefMut)]
pub struct RuntimeObject {
    inner_object: Object<RuntimeManager>,
}

impl RuntimeObject {
    /// Removes this runtime endpoint from the pool.
    pub async fn unregister_self(&self) -> Result<()> {
        let Some(runtime_pool) = Object::pool(&self.inner_object) else {
            return Ok(());
        };

        let runtime_manager = runtime_pool.manager();
        runtime_manager
            .unregister_endpoint(self.as_endpoint_id())
            .await?;

        Ok(())
    }
}

impl Runtime {
    /// Returns a new [`Runtime`].
    pub fn new(config: RuntimeConfig) -> Self {
        let manager_config =
            RuntimeManagerConfig::new().with_recycling_method(config.recycling_method);
        let manager = RuntimeManager::new(manager_config);
        let pool = Pool::builder(manager)
            .max_size(config.max_conn.unwrap_or(64))
            .create_timeout(config.create_timeout)
            .wait_timeout(config.wait_timeout)
            .recycle_timeout(config.recycle_timeout)
            .post_create(Hook::sync_fn(post_create))
            .pre_recycle(Hook::sync_fn(pre_recycle))
            .post_recycle(Hook::sync_fn(post_recycle))
            .runtime(deadpool::Runtime::Tokio1);

        let pool = pool.build().expect("should not require runtime");
        Self { inner: pool }
    }

    /// Adds the runtime endpoint into the pool.
    pub async fn register_endpoint<E: Into<RuntimeEndpoint>>(&self, rt: E) -> Result<()> {
        self.inner
            .manager()
            .register_endpoint(rt.into())
            .await
            .map_err(Into::into)
    }

    /// Removes the runtime endpoint from the pool.
    pub async fn unregister_endpoint<E: Into<Uuid>>(&self, rt: E) -> Result<()> {
        self.inner
            .manager()
            .unregister_endpoint(&rt.into())
            .await
            .map_err(Into::into)
    }

    pub async fn get_connection(&self) -> Result<RuntimeObject> {
        self.inner.get().await.map(Into::into).map_err(Into::into)
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new(RuntimeConfig::default())
    }
}

impl fmt::Debug for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Runtime").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod test {
    use crate::instance::Runtime;
    use crate::{Result, RuntimeConfig};

    #[test]
    fn build_default_runtime() -> Result<()> {
        let config = RuntimeConfig::new();
        let _runtime = Runtime::new(config);
        Ok(())
    }
}
