//! [`Manager`] of [`RuntimeClient`]s.
//!

mod instance_client;
mod manager_config;
mod runtime_endpoint;

use std::collections::HashMap;
use std::fmt;
use std::sync::Mutex;

use deadpool::managed::{Manager, Metrics, RecycleResult};
use tonic::transport::Channel;
use uuid::Uuid;

pub use crate::manager::instance_client::{RuntimeClient, RuntimeError, RuntimeResult};
pub use crate::manager::manager_config::{RecyclingMethod, RuntimeManagerConfig};
pub use crate::manager::runtime_endpoint::RuntimeEndpoint;

/// [`Manager`] of [`RuntimeClient`]s.
pub struct RuntimeManager {
    inner: Mutex<RuntimeManagerInner>,
}

struct RuntimeManagerInner {
    config: RuntimeManagerConfig,
    endpoints: HashMap<Uuid, (RuntimeEndpoint, Option<Channel>)>,
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
    pub(crate) fn register_endpoint(&self, endpoint: RuntimeEndpoint) -> RuntimeResult<()> {
        let mut manager = self.inner.lock().expect("should not be held");
        let mut endpoint_id = Uuid::new_v4();
        while manager.endpoints.contains_key(&endpoint_id) {
            endpoint_id = Uuid::new_v4();
        }

        manager.endpoints.insert(endpoint_id, (endpoint, None));
        Ok(())
    }

    /// Removes the runtime endpoint from the pool.
    pub(crate) fn unregister_endpoint(&self, endpoint_id: &Uuid) -> RuntimeResult<()> {
        let mut manager = self.inner.lock().expect("should not be held");
        // TODO: Maybe don't actually remove it, but use a *disable* flag instead.
        let _ = manager.endpoints.remove(endpoint_id);
        Ok(())
    }

    /// - Returns the next least used channel.
    /// - Increases the counter of current connections by 1.
    async fn next_channel(&self) -> RuntimeResult<(Uuid, Channel)> {
        let mut manager = self.inner.lock().expect("should not be held");
        if manager.endpoints.is_empty() {
            return Err(RuntimeError::NoEndpoints);
        }

        // Returns the endpoint with the least of connections out of the pool
        // of endpoints with no limits or if their limit was not reached yet.
        let endpoint = manager
            .endpoints
            .iter_mut()
            .filter(|(_, (r, _))| r.limit.is_none() || r.limit.is_some_and(|x| x < r.current))
            .min_by(|(_, (l, _)), (_, (r, _))| l.current.cmp(&r.current));

        let Some((id, (runtime_endpoint, runtime_channel))) = endpoint else {
            return Err(RuntimeError::EndpointsLimit);
        };

        let runtime_channel = if let Some(runtime_channel) = runtime_channel {
            runtime_channel.clone()
        } else {
            let channel = runtime_endpoint.endpoint.connect().await?;
            *runtime_channel = Some(channel.clone());
            channel
        };

        runtime_endpoint.current += 1;
        Ok((*id, runtime_channel))
    }

    /// Reduces the counter of current connections by 1.
    fn drop_channel(&self, endpoint_id: &Uuid) {
        let mut manager = self.inner.lock().expect("should not be held");
        if let Some((endpoint, _)) = manager.endpoints.get_mut(endpoint_id) {
            endpoint.current -= 1;
        }
    }

    async fn test_connection(&self, runtime_client: &mut RuntimeClient) -> RuntimeResult<()> {
        let mut manager = self.inner.lock().expect("should not be held");

        // TODO.
        match manager.config.recycling_method {
            RecyclingMethod::Fast => {}
            RecyclingMethod::Verified => {}
        }

        Ok(())
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
        let (id, channel) = self.next_channel().await?;
        Ok(RuntimeClient::new(id, channel))
    }

    async fn recycle(
        &self,
        conn: &mut Self::Type,
        _metrics: &Metrics,
    ) -> RecycleResult<Self::Error> {
        self.drop_channel(&conn.endpoint_id);
        self.test_connection(conn).await?;
        Ok(())
    }
}
