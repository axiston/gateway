use std::time::Duration;

use tower::Layer;

/// TODO. tower::`Service`
#[derive(Clone)]
#[must_use = "services do nothing unless you `.poll_ready` or `.call` them"]
pub struct ErrorHandler<S> {
    inner: S,
}

impl<S> ErrorHandler<S> {
    /// Returns the new [`ErrorHandler`].
    #[inline]
    pub fn new(inner: S, timeout: Duration) -> Self {
        Self { inner }
    }
}

/// /// A `tower::`[`Layer`] that produces a [`ErrorHandler`] service.
#[derive(Debug, Default, Clone)]
#[must_use = "layers do nothing unless you `.layer` them"]
pub struct ErrorHandlerLayer {
    timeout: Option<Duration>,
}

impl ErrorHandlerLayer {
    /// Returns the new [`ErrorHandlerLayer`].
    #[inline]
    pub fn new(timeout: Duration) -> Self {
        let timeout = Some(timeout);
        Self { timeout }
    }
}

impl<S> Layer<S> for ErrorHandlerLayer {
    type Service = ErrorHandler<S>;

    fn layer(&self, inner: S) -> Self::Service {
        let get_10_minutes = || Duration::from_secs(60 * 10);
        ErrorHandler::new(inner, self.timeout.unwrap_or_else(get_10_minutes))
    }
}
