//! Custom [`axum::extract`]ors (including rejection remapping).

pub use crate::extract::reject_json::Json;
pub use crate::extract::reject_path::Path;
pub use crate::extract::version::RouteVersion;

mod reject_json;
mod reject_path;
mod version;
