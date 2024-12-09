use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

use deadpool::managed::{Manager, Metrics, Pool, RecycleResult};
use deadpool::Runtime;
use crate::runtime::connection_metadata::RuntimeMetadata;
use crate::runtime::ConnectionInstance;

#[derive(Debug)]
pub struct Error {}


/// [`ConnectionInstance`] connection manager.
#[derive(Default)]
pub struct RuntimeManager {
    inner: Arc<Mutex<RuntimeManagerInner>>,
}

#[derive(Default)]
struct RuntimeManagerInner {
    recorded: Vec<RuntimeMetadata>,
    connected: HashMap<usize, ConnectionInstance>,
}

impl RuntimeManager {
    /// Returns a new [`RuntimeManager`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a pool using the connection manager.
    pub fn pool(self) -> Pool<RuntimeManager> {
        let pool = Pool::builder(self).runtime(Runtime::Tokio1).build();
        pool.expect("should not require runtime")
    }
}

impl Manager for RuntimeManager {
    type Type = ConnectionInstance;
    type Error = Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        todo!()
    }

    async fn recycle(&self, obj: &mut Self::Type, metrics: &Metrics) -> RecycleResult<Self::Error> {
        todo!()
    }
}

impl fmt::Debug for RuntimeManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RuntimeManager").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod test {
    use crate::runtime::RuntimeManager;
    use crate::RuntimeResult;

    #[test]
    fn build_from_address() -> RuntimeResult<()> {
        let _ = RuntimeManager::new();
        Ok(())
    }
}
