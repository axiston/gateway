use std::fmt;

use cron::Schedule;
use derive_more::{Deref, DerefMut, From};
use petgraph::Graph;

use crate::inputs::{InputEdgeId, InputNodeId};
use crate::outputs::{HookRegistry, ReportError, TaskRegistry};

/// TODO.
#[must_use = "graph does nothing unless you use it"]
#[derive(Default, From, Deref, DerefMut)]
pub struct ProcessGraph {
    inner: Graph<ProcessNode, ProcessEdge>,
}

impl ProcessGraph {
    /// Returns a new [`ProcessGraph`].
    #[inline]
    pub fn new(inner: Graph<ProcessNode, ProcessEdge>) -> Self {
        Self { inner }
    }

    /// Returns the underlying [`Graph`]<[`ProcessNode`], [`ProcessEdge`]>.
    #[inline]
    pub fn into_inner(self) -> Graph<ProcessNode, ProcessEdge> {
        self.inner
    }
}

impl fmt::Debug for ProcessGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProcessGraph").finish_non_exhaustive()
    }
}

#[must_use = "graph node does nothing unless you use it"]
#[derive(Debug, Clone)]
pub struct ProcessNode {
    execute_priority: u32,
    input_node: InputNodeId,
    inner_data: ProcessNodeData,
    // started exec, ended exec
    // inputs
    // outputs
}

impl ProcessNode {
    /// Returns a new [`ProcessNode`].
    fn new(input_node_id: InputNodeId, inner_data: ProcessNodeData) -> Self {
        Self {
            execute_priority: 0,
            input_node: input_node_id,
            inner_data,
        }
    }

    /// Attempts to create a new [`ProcessNodeData::TriggerOnManual`].
    pub fn try_from_manual_trigger(input_node_id: InputNodeId) -> Result<Self, ReportError> {
        Ok(Self::new(input_node_id, ProcessNodeData::TriggerOnManual))
    }

    /// Attempts to create a new [`ProcessNode`] with on schedule trigger.
    pub fn try_from_cron_trigger(
        input_node_id: InputNodeId,
        cron_format: &str,
    ) -> Result<Self, ReportError> {
        cron_format
            .parse::<Schedule>()
            .map(|x| Self::new(input_node_id, ProcessNodeData::TriggerOnSchedule(x)))
            .map_err(|_| ReportError::incorrect_cron_format(input_node_id, cron_format))
    }

    /// Attempts to create a new [`ProcessNodeData::TriggerOnWebhook`].
    pub fn try_from_hook_trigger(
        input_node_id: InputNodeId,
        hook_id: &str,
        hook_registry: &HookRegistry,
    ) -> Result<Self, ReportError> {
        // TODO: Check the existence of the hook with a provided id.
        Ok(Self::new(
            input_node_id,
            ProcessNodeData::TriggerOnWebhook(hook_id.to_owned()),
        ))
    }

    /// Attempts to create a new [`ProcessNodeData::Action`].
    pub fn try_from_action(
        input_node_id: InputNodeId,
        task_registry: &TaskRegistry,
    ) -> Result<Self, ReportError> {
        todo!()
    }

    /// Prioritizes this node over others with lower priority.
    pub fn with_execute_priority(mut self, priority: u32) -> Self {
        self.execute_priority = priority;
        self
    }

    /// Returns the [`InputNodeId`] of the processed node.
    #[inline]
    pub fn id(&self) -> InputNodeId {
        self.input_node
    }
}

#[must_use = "graph node does nothing unless you use it"]
#[derive(Debug, Clone)]
struct ProcessTask {}

#[must_use = "graph node does nothing unless you use it"]
#[derive(Debug, Clone)]
pub(crate) enum ProcessNodeData {
    TriggerOnManual,
    TriggerOnSchedule(Schedule),
    TriggerOnWebhook(String),
    Action(()),
}

impl ProcessNodeData {}

#[must_use = "graph edge does nothing unless you use it"]
#[derive(Debug, Clone)]
pub struct ProcessEdge {
    input_edge: InputEdgeId,
}

impl ProcessEdge {
    /// Returns a new [`ProcessEdge`].
    pub fn new(input_edge: InputEdgeId) -> Self {
        Self { input_edge }
    }

    /// Returns the [`InputEdgeId`] of the processed edge.
    #[inline]
    pub fn id(&self) -> InputEdgeId {
        self.input_edge
    }
}

#[cfg(test)]
mod test {
    use crate::worker::ProcessGraph;
    use crate::Result;

    #[test]
    fn build_empty_graph() -> Result<()> {
        let _ = ProcessGraph::default();
        Ok(())
    }
}
