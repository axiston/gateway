use serde::Deserialize;
use ts_rs::TS;

use crate::inputs::{InputGraph, InputNodeId};

/// Represents a user-provided (i.e. input or requested) event.
#[must_use = "event does nothing unless you use it"]
#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export, export_to = "event.ts")]
pub enum InputEvent {
    /// Runs from the given node (including the node).
    RunFromNode(InputNodeId),
    /// Runs from the given node (excluding the node).
    RunFromEdge(InputNodeId),
}

impl InputEvent {
    /// Returns a new [`InputEvent::RunFromNode`].
    fn run_from_node(graph: &InputGraph, node: InputNodeId) -> Self {
        Self::RunFromNode(node)
    }

    /// Returns a new [`InputEvent::RunFromEdge`].
    fn run_from_edge(graph: &InputGraph, node0: InputNodeId, node1: InputNodeId) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod test {}
