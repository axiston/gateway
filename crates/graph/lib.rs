#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

//! TODO.

mod compile;
mod inputs;
mod outputs;

#[derive(Debug)]
pub struct Error {}

pub type Result<T, E = Error> = std::result::Result<T, E>;
