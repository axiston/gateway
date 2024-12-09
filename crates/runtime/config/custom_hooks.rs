use deadpool::managed::{HookResult, Metrics};

use crate::client::{RuntimeConn, RuntimeConnError};

/// Custom hook called after a new connection has been established.
///
/// See [`PoolBuilder`] for more details.
///
/// [`PoolBuilder`]: deadpool::managed::PoolBuilder
pub fn post_create(_conn: &mut RuntimeConn, _metrics: &Metrics) -> HookResult<RuntimeConnError> {
    // Note: should never return an error.
    tracing::trace!(target: "runtime", "post_create");
    Ok(())
}

/// Custom hook called before a connection has been recycled.
///
/// See [`PoolBuilder`] for more details.
///
/// [`PoolBuilder`]: deadpool::managed::PoolBuilder
pub fn pre_recycle(_conn: &mut RuntimeConn, _metrics: &Metrics) -> HookResult<RuntimeConnError> {
    // Note: should never return an error.
    tracing::trace!(target: "runtime", "pre_recycle");
    Ok(())
}

/// Custom hook called after a connection has been recycled.
///
/// See [`PoolBuilder`] for more details.
///
/// [`PoolBuilder`]: deadpool::managed::PoolBuilder
pub fn post_recycle(_conn: &mut RuntimeConn, _metrics: &Metrics) -> HookResult<RuntimeConnError> {
    // Note: should never return an error.
    tracing::trace!(target: "runtime", "post_recycle");
    Ok(())
}
