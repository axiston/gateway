//! Custom `axum::`[`extract`]ors (including rejection remapping).
//!
//! [`extract`]: axum::extract::FromRequest

pub use crate::extract::auth_state::{AuthRole, AuthState};
pub use crate::extract::auth_token::AuthToken;
pub use crate::extract::reject_json::Json;
pub use crate::extract::reject_path::Path;
pub use crate::extract::validate::Validated;
pub use crate::extract::version::Version;

/// Type alias for [`Validated`]<[`Json`]>.
pub type ValidatedJson<T> = Validated<Json<T>>;

/// Type alias for [`Validated`]<[`Path`]>.
pub type ValidatedPath<T> = Validated<Path<T>>;

mod auth_state;
mod auth_token;
mod reject_json;
mod reject_path;
mod validate;
mod version;