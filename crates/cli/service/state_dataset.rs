use crate::handler::instance::CheckHealth;
use crate::handler::Error;

/// TODO.
#[must_use]
#[derive(Debug, Clone)]
pub struct Dataset {}

impl Dataset {
    /// Creates a new [`Dataset`].
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Dataset {
    fn default() -> Self {
        Self {}
    }
}

impl CheckHealth for Dataset {
    type Error = Error;
}
