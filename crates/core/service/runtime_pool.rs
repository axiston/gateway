use crate::handler::Error;

/// TODO.
#[must_use]
#[derive(Debug, Clone)]
pub struct RuntimePool {}

impl RuntimePool {
    /// Creates a new [`RuntimePool`].
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for RuntimePool {
    fn default() -> Self {
        Self {}
    }
}

// impl CheckHealth for RuntimePool {
//     type Error = Error;
// }
