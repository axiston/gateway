//! `axum::`[`Router`] and `tower::`[`ServiceBuilder`] extension traits.
//!

use std::time::Duration;

use axum::Router;
use tower::ServiceBuilder;

pub use crate::middleware::auth_guards::{authentication_guard, authorization_guard};
use crate::middleware::error_handling::RouterHandlingExt;
use crate::middleware::observability::RouterTracingExt;

mod auth_guards;
mod error_handling;
mod observability;
mod utility;

/// Extension trait for `tower::`[`ServiceBuilder`] for layering middleware.
pub trait ServiceBuilderExt<L> {}

impl<L> ServiceBuilderExt<L> for ServiceBuilder<L> {}

/// Extension trait for `axum::`[`Router`] for layering middleware.
pub trait RouterExt<S> {
    /// Layers [`HandleError`], [`CatchPanic`] and [`Timeout`] middlewares.
    ///
    /// [`HandleError`]: axum::error_handling::HandleErrorLayer
    /// [`CatchPanic`]: tower_http::catch_panic::CatchPanicLayer
    /// [`Timeout`]: tower::timeout::TimeoutLayer
    fn with_error_handling_layer(self, timeout: Duration) -> Self;

    /// Layers [`SetRequestId`], [`Trace`] and [`PropagateRequestId`] middlewares.
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
        self.with_inner_error_handling_layer(timeout)
    }

    #[inline]
    fn with_observability_layer(self) -> Self {
        self.with_inner_observability_layer()
    }
}
