use std::collections::HashMap;

use serde::Serialize;
use ts_rs::TS;

use crate::inputs::{InputEdgeId, InputNodeId};

/// Contains inputs and outputs of an executed input graph.
#[must_use = "graph does nothing unless you use it"]
#[derive(Debug, Clone, Default, Serialize, TS)]
#[ts(export, export_to = "snapshot.ts")]
pub struct OutputGraph {
    #[serde(rename = "nodes")]
    pub nodes: HashMap<InputNodeId, OutputNode>,
    #[serde(rename = "edges")]
    pub edges: HashMap<InputEdgeId, OutputEdge>,
}

impl OutputGraph {
    /// Returns an empty [`OutputGraph`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Blindly overwrites graph nodes and edges with provided deltas.
    pub fn merge_deltas(&mut self, diff: OutputGraphDelta) {
        if let Some(nodes) = diff.nodes {
            for (node_id, node_data) in nodes.into_iter() {
                let _ = self.nodes.insert(node_id, node_data);
            }
        }

        if let Some(edges) = diff.edges {
            for (edge_id, edge_data) in edges.into_iter() {
                let _ = self.edges.insert(edge_id, edge_data);
            }
        }
    }

    /// Inserts another [`OutputNode`] to the [`OutputGraph`].
    pub fn with_node<I, T>(&mut self, id: I, node: T)
    where
        I: Into<InputNodeId>,
        T: Into<OutputNode>,
    {
        self.nodes.insert(id.into(), node.into());
    }

    /// Inserts another [`OutputEdge`] to the [`OutputGraph`].
    pub fn with_edge<I, T>(&mut self, id: I, edge: T)
    where
        I: Into<InputEdgeId>,
        T: Into<OutputEdge>,
    {
        self.edges.insert(id.into(), edge.into());
    }
}

/// Represents a restricted set of changes applied to the [`OutputGraph`].
///
/// Used to limit the amount of data sent through the gateway.
#[must_use = "graph does nothing unless you use it"]
#[derive(Debug, Clone, Default, Serialize, TS)]
#[ts(export, export_to = "snapshot.ts")]
pub struct OutputGraphDelta {
    #[serde(rename = "nodes")]
    pub nodes: Option<HashMap<InputNodeId, OutputNode>>,
    #[serde(rename = "edges")]
    pub edges: Option<HashMap<InputEdgeId, OutputEdge>>,
}

impl OutputGraphDelta {
    /// Returns an empty [`OutputGraphDelta`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts another [`OutputNode`] to the [`OutputGraphDelta`].
    pub fn with_node<I, T>(&mut self, id: I, node: T)
    where
        I: Into<InputNodeId>,
        T: Into<OutputNode>,
    {
        let nodes = self.nodes.get_or_insert_with(HashMap::default);
        nodes.insert(id.into(), node.into());
    }

    /// Inserts another [`OutputEdge`] to the [`OutputGraphDelta`].
    pub fn with_edge<I, T>(&mut self, id: I, edge: T)
    where
        I: Into<InputEdgeId>,
        T: Into<OutputEdge>,
    {
        let edges = self.edges.get_or_insert_with(HashMap::default);
        edges.insert(id.into(), edge.into());
    }
}

/// Contains inputs and outputs of an executed input node.
#[must_use = "graph node does nothing unless you use it"]
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export, export_to = "snapshot.ts")]
pub struct OutputNode {
    /// Tracks the current step number of the execution.
    #[serde(rename = "counter")]
    pub counter: u32,
    /// Contains inputs of an executed input node.
    #[serde(rename = "inputs")]
    pub inputs: OutputFields,
    /// Contains outputs of an executed input node.
    #[serde(rename = "outputs")]
    pub outputs: OutputFields,
}

impl OutputNode {
    /// Returns a new [`OutputNode`].
    pub fn new(counter: u32) -> Self {
        Self {
            counter,
            inputs: OutputFields::new(),
            outputs: OutputFields::new(),
        }
    }
}

/// Contains inputs and outputs of an executed input edge.
#[must_use = "graph edge does nothing unless you use it"]
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export, export_to = "snapshot.ts")]
pub struct OutputEdge {}

impl OutputEdge {}

/// Contains a packaged set of inputs or outputs.
#[must_use = "graph edge does nothing unless you use it"]
#[derive(Debug, Default, Clone, Serialize, TS)]
#[ts(export, export_to = "snapshot.ts")]
pub struct OutputFields {}

impl OutputFields {
    /// Returns an empty [`OutputFields`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod test {
    use crate::outputs::OutputGraph;
    use crate::Result;

    #[test]
    fn build_empty() -> Result<()> {
        let _ = OutputGraph::new();
        Ok(())
    }

    #[test]
    fn build_from_graph() -> Result<()> {
        let _ = OutputGraph::new();
        Ok(())
    }

    #[test]
    fn build_from_delta() -> Result<()> {
        let _ = OutputGraph::new();
        Ok(())
    }
}
