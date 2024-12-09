#[derive(Debug, Default)]
pub struct TaskRegistry {}

impl TaskRegistry {
    /// Returns an empty [`TaskRegistry`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}
