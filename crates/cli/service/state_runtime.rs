use crate::handler::instance::CheckHealth;
use crate::handler::Error;

/// TODO.
#[must_use]
#[derive(Debug, Clone)]
pub struct Runtime {}

impl Runtime {
    /// Creates a new [`Runtime`].
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self {}
    }
}

impl CheckHealth for Runtime {
    type Error = Error;
}
