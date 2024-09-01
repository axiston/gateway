use derive_more::From;
use serde::{Deserialize, Serialize};
use uuid::{Error, Uuid};

use crate::inputs::triggers::{InputTriggerOnSchedule, InputTriggerOnTask, InputTriggerOnWebhook};

/// Opaque and unique [`InputNode`] identifier.
#[must_use = "graph identifier does nothing unless you use it"]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InputNodeId(pub Uuid);

impl InputNodeId {
    /// Returns a new [`InputNodeId`].
    pub fn new(inner: &str) -> Result<Self, Error> {
        inner.parse::<Uuid>().map(Self)
    }
}

#[must_use = "graph node does nothing unless you use it"]
#[derive(Debug, Clone, From, Serialize, Deserialize)]
#[serde(tag = "node", rename_all = "lowercase")]
pub enum InputNode {
    Trigger(InputTriggerNode),
    Action(InputActionNode),
}

impl InputNode {
    /// Returns a new [`InputActionNode`].
    #[inline]
    pub fn action(task: &str) -> Self {
        Self::Action(InputActionNode {
            task: task.to_owned(),
        })
    }

    /// Returns a new [`InputTriggerNode::Manual`].
    #[inline]
    pub fn manual() -> Self {
        Self::Trigger(InputTriggerNode::Manual)
    }

    /// Returns a new [`InputTriggerNode::Schedule`].
    #[inline]
    pub fn schedule(cron: &str) -> Self {
        let trigger = InputTriggerOnSchedule {
            cron: cron.to_owned(),
        };
        Self::Trigger(trigger.into())
    }

    /// Returns a new [`InputTriggerNode::Task`].
    #[inline]
    pub fn trigger(task: &str) -> Self {
        let trigger = InputTriggerOnTask {
            task: task.to_owned(),
            field: None,
        };
        Self::Trigger(trigger.into())
    }

    /// Returns a new [`InputTriggerNode::Webhook`].
    #[inline]
    pub fn webhook(hook: &str) -> Self {
        let trigger = InputTriggerOnWebhook {
            hook: hook.to_owned(),
        };
        Self::Trigger(trigger.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputActionNode {
    pub task: String,
    // TODO: Default values.
}

impl InputActionNode {
    /// Returns a new [`InputActionNode`].
    #[inline]
    pub fn new(task: &str) -> Self {
        Self {
            task: task.to_owned(),
        }
    }
}

#[derive(Debug, Default, Clone, From, Serialize, Deserialize)]
#[serde(tag = "trigger", rename_all = "lowercase")]
pub enum InputTriggerNode {
    Schedule(InputTriggerOnSchedule),
    Task(InputTriggerOnTask),
    Webhook(InputTriggerOnWebhook),
    #[default]
    Manual,
}

impl InputTriggerNode {
    /// Returns a new [`InputActionNode`].
    pub fn new(trigger: impl Into<Self>) -> Self {
        trigger.into()
    }
}
