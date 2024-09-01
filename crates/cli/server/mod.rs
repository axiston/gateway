//! TODO.
//!

pub use crate::server::conn_info::AppConnectInfo;
pub use crate::server::inter_signal::shutdown_signal;
#[cfg(not(feature = "support-https"))]
pub use crate::server::run_default::run_http_server;
#[cfg(feature = "support-https")]
use crate::server::run_redirect::run_redirect_server;
#[cfg(feature = "support-https")]
pub use crate::server::run_secure::run_https_server;
pub use crate::server::serv_config::{ServerBuilder, ServerConfig};

mod conn_info;
mod inter_signal;
#[cfg(not(feature = "support-https"))]
mod run_default;
#[cfg(feature = "support-https")]
mod run_redirect;
#[cfg(feature = "support-https")]
mod run_secure;
mod serv_config;
