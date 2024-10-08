use serde::{Deserialize, Serialize};

#[must_use = "graph node does nothing unless you use it"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputTriggerOnSchedule {
    pub cron: String,
}

impl InputTriggerOnSchedule {
    /// Returns a new [`InputTriggerOnSchedule`].
    #[inline]
    pub fn new(cron: &str) -> Self {
        Self {
            cron: cron.to_owned(),
        }
    }
}

#[must_use = "graph node does nothing unless you use it"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputTriggerOnTask {
    pub task: String,
    /// Name of the field used to determine if the graph should run.
    pub field: Option<String>,
}

impl InputTriggerOnTask {
    /// Returns a new [`InputTriggerOnTask`].
    #[inline]
    pub fn new(task: &str) -> Self {
        Self {
            task: task.to_owned(),
            field: None,
        }
    }
}

#[must_use = "graph node does nothing unless you use it"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputTriggerOnWebhook {
    pub hook: String,
}

impl InputTriggerOnWebhook {
    /// Returns a new [`InputTriggerOnWebhook`].
    #[inline]
    pub fn new(hook: &str) -> Self {
        Self {
            hook: hook.to_owned(),
        }
    }
}
