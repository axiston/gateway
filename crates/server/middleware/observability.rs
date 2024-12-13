use axum::Router;
use tower::ServiceBuilder;
use tower_http::request_id::MakeRequestUuid;
use tower_http::trace::TraceLayer;
use tower_http::ServiceBuilderExt;

/// Extension trait for `axum::`[`Router`] for improved observability.
pub trait RouterTracingExt<S> {
    /// Stacks [`SetRequestId`], [`Trace`] and [`PropagateRequestId`] layers.
    ///
    /// [`SetRequestId`]: tower_http::request_id::SetRequestIdLayer
    /// [`PropagateRequestId`]: tower_http::request_id::PropagateRequestIdLayer
    fn with_inner_observability_layer(self) -> Self;
}

impl<S> RouterTracingExt<S> for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn with_inner_observability_layer(self) -> Self {
        let middlewares = ServiceBuilder::new()
            .set_x_request_id(MakeRequestUuid)
            .layer(TraceLayer::new_for_http())
            .propagate_x_request_id();

        self.layer(middlewares)
    }
}
