use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use tokio::signal::{ctrl_c, unix};

/// Completes once the terminate signal is received.
///
/// See [`ctrl_c`] and [`unix::SignalKind::terminate`].
pub async fn shutdown_signal(timeout: Duration) {
    let ctrl_c = async {
        ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        unix::signal(unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    let t0 = Instant::now();
    tracing::warn!(
        target: "server:terminate",
        timeout = timeout.as_secs(),
        "otel is terminating"
    );

    let (sender, receiver) = mpsc::channel();
    let _ = thread::spawn(move || {
        opentelemetry::global::shutdown_tracer_provider();
        sender.send(()).ok()
    });

    if receiver.recv_timeout(timeout).is_err() {
        tracing::error!(target: "server:terminate", "failed to shutdown otel");
    }

    let t1 = Instant::now().duration_since(t0);
    tracing::warn!(
        target: "server:terminate",
        waiting = t1.as_secs(),
        "server is terminating"
    );
}
