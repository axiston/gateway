#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

//! ### Examples
//!
//! ```rust
//! use axiston_graph::inputs::InputGraph;
//! use axiston_graph::Result;
//!
//! fn main() -> Result<()> {
//!     let _ = InputGraph::new();
//!     Ok(())
//! }
//! ```

pub mod inputs;
pub mod outputs;
pub mod worker;

/// Specialized [`Result`] alias for the [`ReportBundle`] type.
///
/// [`ReportBundle`]: crate::outputs::ReportBundle
/// [`Result`]: std::result::Result
pub type Result<T, E = outputs::ReportBundle> = std::result::Result<T, E>;
