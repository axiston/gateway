use serde::{Deserialize, Serialize};

#[must_use = "graph node does nothing unless you use it"]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OutputEdge {
    // Remaps input to output.
}

impl OutputEdge {
    /// Returns a new [`OutputEdge`].
    #[inline]
    pub fn new() -> Self {
        Self {}
    }
}
