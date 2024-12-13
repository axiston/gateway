#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

//! ### Examples
//!
//! ```rust
//! use axiston_server::handler::Result;
//!
//! fn main() -> Result<()> {
//!     Ok(())
//! }
//! ```

pub mod extract;
pub mod handler;
pub mod middleware;
pub mod service;
