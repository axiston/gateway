use deadpool::managed::{HookResult, Metrics};

use crate::manager::{RuntimeClient, RuntimeError};

/// Custom hook called after a new connection has been established.
///
/// See [`PoolBuilder`] for more details.
///
/// [`PoolBuilder`]: deadpool::managed::PoolBuilder
pub fn post_create(_conn: &mut RuntimeClient, _metrics: &Metrics) -> HookResult<RuntimeError> {
    tracing::trace!(target: "runtime", "post_create");

    // Note: should never return an error.
    Ok(())
}

/// Custom hook called before a connection has been recycled.
///
/// See [`PoolBuilder`] for more details.
///
/// [`PoolBuilder`]: deadpool::managed::PoolBuilder
pub fn pre_recycle(_conn: &mut RuntimeClient, _metrics: &Metrics) -> HookResult<RuntimeError> {
    tracing::trace!(target: "runtime", "pre_recycle");

    // Note: should never return an error.
    Ok(())
}

/// Custom hook called after a connection has been recycled.
///
/// See [`PoolBuilder`] for more details.
///
/// [`PoolBuilder`]: deadpool::managed::PoolBuilder
pub fn post_recycle(_conn: &mut RuntimeClient, _metrics: &Metrics) -> HookResult<RuntimeError> {
    tracing::trace!(target: "runtime", "post_recycle");

    // Note: should never return an error.
    Ok(())
}
