use std::future::Future;
use std::net::{Ipv4Addr, SocketAddr};

use axiston_core::handler::{ErrorKind, Result};
use axum::extract::{Host, State};
use axum::handler::Handler;
use axum::http::Uri;
use axum::response::Redirect;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

/// Application state.
///
/// Used for the [`State`] extraction (dependency injection).
#[derive(Debug, Clone, Copy)]
#[must_use = "state does nothing unless you use it"]
struct RedirectState {
    pub http: u16,
    pub https: u16,
}

/// Starts the optional [`hyper`] server that redirects `http` requests to the `https`.
///
/// [`hyper`]: axum::serve
pub async fn run_redirect_server<F>(
    http_port: u16,
    https_port: u16,
    interrupt_fut: F,
) -> anyhow::Result<()>
where
    F: Future<Output = ()> + Send + 'static,
{
    let app_state = RedirectState {
        http: http_port,
        https: https_port,
    };

    let app_service = redirect_handler.with_state(app_state);
    let server_addr = SocketAddr::from((Ipv4Addr::LOCALHOST, app_state.http));
    let listener = TcpListener::bind(server_addr).await?;

    tracing::debug!(
        target: "server:redirect",
        port = app_state.http,
        redirect = app_state.https,
        "server (redirect) is listening on {}",
        server_addr
    );

    axum::serve(listener, app_service.into_make_service())
        .with_graceful_shutdown(interrupt_fut)
        .await?;

    Ok(())
}

fn make_https_redirect(host: &str, uri: Uri, state: RedirectState) -> anyhow::Result<Uri> {
    let mut parts = uri.into_parts();

    parts.scheme = Some(axum::http::uri::Scheme::HTTPS);
    if parts.path_and_query.is_none() {
        parts.path_and_query = Some("/".parse()?);
    }

    let https_host = host.replace(&state.http.to_string(), &state.https.to_string());
    parts.authority = Some(https_host.parse()?);

    Ok(Uri::from_parts(parts)?)
}

#[tracing::instrument]
async fn redirect_handler(
    Host(host): Host,
    uri: Uri,
    State(state): State<RedirectState>,
) -> Result<Redirect> {
    match make_https_redirect(&host, uri, state) {
        Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
        Err(error) => {
            tracing::warn!(host, %error, "failed to convert URI to HTTPS");
            Err(ErrorKind::InternalServerError.into())
        }
    }
}
