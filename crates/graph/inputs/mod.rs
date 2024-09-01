use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub use crate::inputs::input_edge::{InputEdge, InputEdgeId};
pub use crate::inputs::input_node::{InputNode, InputNodeId};

mod input_edge;
mod input_node;
mod triggers;

/// User-provided graph nodes and edges.
#[must_use = "graph does nothing unless you use it"]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InputGraph {
    pub nodes: HashMap<InputNodeId, InputNode>,
    pub edges: HashMap<InputEdgeId, InputEdge>,
}

/// User-provided graph nodes and edges deltas.
///
/// Used to limit the amount of data sent to the gateway.
#[must_use = "graph does nothing unless you use it"]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InputGraphDiff {
    pub nodes: Option<HashMap<InputNodeId, InputNode>>,
    pub edges: Option<HashMap<InputEdgeId, InputEdge>>,
}

impl InputGraph {
    /// Returns an empty [`InputGraph`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Blindly overwrites user-provided graph nodes with diff nodes.
    pub fn merge_graph_diffs(&mut self, diff: InputGraphDiff) {
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

    /// Adds another [`InputNode`] to the [`InputGraph`].
    pub fn with_node(&mut self, id: impl Into<InputNodeId>, node: impl Into<InputNode>) {
        self.nodes.insert(id.into(), node.into());
    }

    /// Adds another [`InputEdge`] to the [`InputGraph`].
    pub fn with_edge(&mut self, id: impl Into<InputEdgeId>, edge: impl Into<InputEdge>) {
        self.edges.insert(id.into(), edge.into());
    }
}

#[cfg(test)]
mod test {
    use crate::inputs::input_edge::InputEdge;
    use crate::inputs::input_node::InputNode;
    use crate::inputs::InputGraph;
    use crate::Result;

    #[test]
    fn build_empty() -> Result<()> {
        let graph = InputGraph::new();

        let json = serde_json::to_string_pretty(&graph);
        println!("{}", json.expect("should be parsable"));

        Ok(())
    }

    #[test]
    fn build_once() -> Result<()> {
        let mut graph = InputGraph::new();

        graph.with_node("t1", InputNode::manual());
        graph.with_node("t2", InputNode::schedule("cron"));
        graph.with_node("t3", InputNode::trigger("task"));
        graph.with_node("t4", InputNode::webhook("hook"));

        graph.with_node("a1", InputNode::action("task"));
        graph.with_edge("e1", InputEdge::new("t1", "a1"));
        graph.with_edge("e2", InputEdge::new("t2", "a1"));
        graph.with_edge("e3", InputEdge::new("t3", "a1"));
        graph.with_edge("e4", InputEdge::new("t4", "a1"));

        let json = serde_json::to_string_pretty(&graph);
        println!("{}", json.expect("should be parsable"));

        Ok(())
    }

    #[test]
    fn build_diff() -> Result<()> {
        let graph = InputGraph::new();

        let json = serde_json::to_string_pretty(&graph);
        println!("{}", json.expect("should be parsable"));

        Ok(())
    }
}
