//! `axum::`[`Router`] and `tower::`[`ServiceBuilder`] extension traits.
//!

use std::time::Duration;

use axum::Router;
use tower::ServiceBuilder;

use crate::middleware::error_handling::setup_error_handling;
pub use crate::middleware::observability::initialize_tracing;
use crate::middleware::observability::setup_observability;

mod authenticate;
mod authorize;
mod error_handling;
mod observability;
mod utility;

/// Extension trait for `tower::`[`ServiceBuilder`] for layering middleware.
pub trait ServiceBuilderExt<L> {}

impl<L> ServiceBuilderExt<L> for ServiceBuilder<L> {}

/// Extension trait for `axum::`[`Router`] for layering middleware.
pub trait RouterExt<S> {
    /// Stacks [`HandleError`], [`CatchPanic`] and [`Timeout`] layers.
    ///
    /// [`HandleError`]: axum::error_handling::HandleErrorLayer
    /// [`CatchPanic`]: tower_http::catch_panic::CatchPanicLayer
    /// [`Timeout`]: tower::timeout::TimeoutLayer
    fn with_error_handling_layer(self, timeout: Duration) -> Self;

    /// Stacks [`SetRequestId`], [`Trace`] and [`PropagateRequestId`] layers.
    ///
    /// [`SetRequestId`]: tower_http::request_id::SetRequestIdLayer
    /// [`Trace`]: tower_http::trace::TraceLayer
    /// [`PropagateRequestId`]: tower_http::request_id::PropagateRequestIdLayer
    fn with_observability_layer(self) -> Self;
}

impl<S> RouterExt<S> for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    #[inline]
    fn with_error_handling_layer(self, timeout: Duration) -> Self {
        setup_error_handling(self, timeout)
    }

    #[inline]
    fn with_observability_layer(self) -> Self {
        setup_observability(self)
    }
}
