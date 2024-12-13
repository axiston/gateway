use std::fmt;

use axiston_rt_schema::instance::instance_client::InstanceClient;
use axiston_rt_schema::registry::registry_client::RegistryClient;
use derive_more::From;
use tonic::transport::{Channel, Endpoint};
use uuid::Uuid;

/// Represents a client for interacting with runtime services.
///
/// The `RuntimeClient` is responsible for managing communication with instance
/// and registry services, identified by a unique endpoint ID. It wraps generated
/// gRPC clients for both instance and registry operations, providing a cohesive
/// interface for runtime service interactions.
pub struct RuntimeClient {
    pub(crate) endpoint_id: Uuid,
    pub(crate) instance_client: InstanceClient<Channel>,
    pub(crate) registry_client: RegistryClient<Channel>,
}

impl RuntimeClient {
    /// Returns a new [`RuntimeClient`].
    #[inline]
    pub fn new(id: Uuid, channel: Channel) -> Self {
        Self {
            endpoint_id: id,
            instance_client: InstanceClient::new(channel.clone()),
            registry_client: RegistryClient::new(channel),
        }
    }

    /// Returns a new [`RuntimeClient`].
    pub async fn connect(id: Uuid, endpoint: Endpoint) -> RuntimeResult<Self> {
        let channel = endpoint.connect().await?;
        Ok(Self::new(id, channel))
    }

    /// Returns the reference to the underlying unique endpoint identifier.
    #[inline]
    pub fn as_endpoint_id(&self) -> &Uuid {
        &self.endpoint_id
    }

    /// Returns the reference to the underlying (generated) instance client.
    #[inline]
    pub fn as_instance_client(&self) -> &InstanceClient<Channel> {
        &self.instance_client
    }

    /// Returns the reference to the underlying (generated) instance client.
    #[inline]
    pub fn as_registry_client(&self) -> &RegistryClient<Channel> {
        &self.registry_client
    }
}

impl fmt::Debug for RuntimeClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RuntimeClient").finish_non_exhaustive()
    }
}

/// Unrecoverable failure of the [`RuntimeClient`].
///
/// Includes all error types that may occur.
/// Used to remap from [`PoolError`].
///
/// [`PoolError`]: deadpool::managed::PoolError
#[derive(Debug, From, thiserror::Error)]
#[must_use = "errors do nothing unless you use them"]
pub enum RuntimeError {
    /// All endpoints have reached the limit.
    #[error("all endpoints have reached the limit")]
    EndpointsLimit,
    /// Connection pool has no endpoints.
    #[error("connection pool has no endpoints")]
    NoEndpoints,
    /// Transport failure (from the client or server).
    #[error("transport failure: {0}")]
    Transport(tonic::transport::Error),
}

/// Specialized [`Result`] alias for the [`RuntimeError`] type.
pub type RuntimeResult<T, E = RuntimeError> = Result<T, E>;
