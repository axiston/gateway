use std::any::Any;
use std::future::ready;
use std::time::Duration;

use axum::error_handling::HandleErrorLayer;
use axum::response::{IntoResponse, Response};
use axum::{BoxError, Router};
use futures::future::{BoxFuture, FutureExt};
use tower::timeout::TimeoutLayer;
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer;

use crate::handler::{Error, ErrorKind};

type ResponseFut = BoxFuture<'static, Response>;

/// Transforms any known [`tower::BoxError`] into the [`Response`].
pub fn handle_error(err: tower::BoxError) -> ResponseFut {
    use tower::timeout::error::Elapsed;
    if err.downcast_ref::<Elapsed>().is_some() {
        tracing::error!(target: "server:error", "service timeout: exceeded waiting time");
    }

    // TODO: Unboxed future: async handle_error(...) -> Response.
    ready(ErrorKind::InternalServerError.into_response()).boxed()
}

type Panic = Box<dyn Any + Send + 'static>;

/// Transforms any panic into the [`Error`] and then [`Response`].
pub fn catch_panic(err: Panic) -> Response {
    if let Some(s) = err.downcast_ref::<String>() {
        tracing::error!(target: "server:panic", "service panic: {}", s);
    } else if let Some(s) = err.downcast_ref::<&str>() {
        tracing::error!(target: "server:panic", "service panic: {}", s);
    } else if let Some(s) = err.downcast_ref::<Error>() {
        tracing::error!(target: "server:panic", "service panic: {}", s);
    }

    ErrorKind::InternalServerError.into_response()
}

pub fn setup_error_handling<S>(router: Router<S>, timeout: Duration) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let middlewares = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .layer(CatchPanicLayer::custom(catch_panic))
        .layer(TimeoutLayer::new(timeout));

    router.layer(middlewares)
}
