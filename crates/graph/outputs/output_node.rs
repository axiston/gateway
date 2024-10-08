use croner::Cron;
use derive_more::From;
use serde::{Deserialize, Serialize};

#[must_use = "graph node does nothing unless you use it"]
#[derive(Debug, Clone, Deserialize, Serialize, From)]
pub enum OutputNode {
    Trigger(Trigger),
    Action(Action),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Trigger {
    Task(TaskTrigger),
    Schedule(ScheduleTrigger),
    Webhook(WebhookTrigger),
    Manual,
}

impl Trigger {
    #[inline]
    pub fn is_manual(&self) -> bool {
        matches!(self, Trigger::Manual)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskTrigger {
    pub task: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScheduleTrigger {
    // pub cron: Cron,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebhookTrigger {
    pub hook: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Action {
    pub task: String,
}
