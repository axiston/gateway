use std::borrow::Cow;

use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::inputs::{InputEdgeId, InputNodeId};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct ErrorReport {
    pub errors: Vec<Error>,
}

impl ErrorReport {
    /// Returns an empty [`ErrorReport`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds another [`Error`] into the [`ErrorReport`].
    #[inline]
    pub fn with_error(&mut self, error: Error) {
        self.errors.push(error)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, From)]
pub enum EventTarget {
    Node(InputNodeId),
    Edge(InputEdgeId),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum EventClass {
    Warning(Option<u32>),
    Error(Option<u32>),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Error {
    target: EventTarget,
    class: EventClass,
    message: Cow<'static, str>,
}

impl Error {}

// Event: Clear
// Event: Validate response

#[cfg(test)]
mod test {}
