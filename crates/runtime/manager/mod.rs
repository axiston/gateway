//! [`Manager`] of [`RuntimeClient`]s.
//!

mod instance_client;
mod manager_config;
mod runtime_endpoint;

use std::collections::HashMap;
use std::fmt;
use std::sync::Mutex;

use deadpool::managed::{Manager, Metrics, RecycleResult};
use tonic::transport::{Channel, Endpoint, Uri};

pub use crate::manager::instance_client::{RuntimeClient, RuntimeError, RuntimeResult};
pub use crate::manager::manager_config::{RecyclingMethod, RuntimeManagerConfig};
pub use crate::manager::runtime_endpoint::RuntimeEndpoint;

/// [`Manager`] of [`RuntimeClient`]s.
pub struct RuntimeManager {
    inner: Mutex<RuntimeManagerInner>,
}

struct RuntimeManagerInner {
    config: RuntimeManagerConfig,
    endpoints: HashMap<Uri, RuntimeEndpoint>,
}

impl RuntimeManager {
    /// Returns a new [`RuntimeManager`].
    #[inline]
    pub fn new(config: RuntimeManagerConfig) -> Self {
        let inner = Mutex::new(RuntimeManagerInner {
            endpoints: HashMap::new(),
            config,
        });

        Self { inner }
    }

    /// Adds the runtime endpoint into the pool.
    pub fn register_endpoint(&self, endpoint: RuntimeEndpoint) -> RuntimeResult<()> {
        let mut manager = self.inner.lock().expect("should not be held");
        // TODO.
        Ok(())
    }

    /// Removes the runtime endpoint from the pool.
    pub fn unregister_endpoint(&self, uri: Uri) -> RuntimeResult<()> {
        let mut manager = self.inner.lock().expect("should not be held");
        // TODO.
        Ok(())
    }

    async fn next_channel(&self) -> RuntimeResult<Channel> {
        todo!()
    }
}

impl Default for RuntimeManager {
    fn default() -> Self {
        Self::new(RuntimeManagerConfig::default())
    }
}

impl fmt::Debug for RuntimeManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RuntimeManager").finish_non_exhaustive()
    }
}

impl Manager for RuntimeManager {
    type Type = RuntimeClient;
    type Error = RuntimeError;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        let channel = self.next_channel().await?;
        Ok(RuntimeClient::new(channel))
    }

    async fn recycle(
        &self,
        conn: &mut Self::Type,
        metrics: &Metrics,
    ) -> RecycleResult<Self::Error> {
        todo!()
    }
}
