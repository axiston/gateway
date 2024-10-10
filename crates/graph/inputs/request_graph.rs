use std::collections::HashMap;

use derive_more::{Deref, From};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use ts_rs::TS;
use uuid::Uuid;

/// Represents a user-provided (i.e. input or requested) graph.
#[must_use = "graph does nothing unless you use it"]
#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[ts(export, export_to = "graph.ts")]
pub struct InputGraph {
    #[serde(rename = "version")]
    pub version: u32,
    #[serde(rename = "nodes")]
    pub nodes: HashMap<InputNodeId, InputNode>,
    #[serde(rename = "edges")]
    pub edges: HashMap<InputEdgeId, InputEdge>,
}

impl InputGraph {
    /// Returns an empty [`InputGraph`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Blindly overwrites graph nodes and edges with provided deltas.
    pub fn merge_deltas(&mut self, diff: InputGraphDelta) {
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

    /// Inserts another [`InputNode`] to the [`InputGraph`].
    pub fn with_node<I, T>(&mut self, id: I, node: T)
    where
        I: Into<InputNodeId>,
        T: Into<InputNode>,
    {
        self.nodes.insert(id.into(), node.into());
    }

    /// Inserts another [`InputEdge`] to the [`InputGraph`].
    pub fn with_edge<I, T>(&mut self, id: I, edge: T)
    where
        I: Into<InputEdgeId>,
        T: Into<InputEdge>,
    {
        self.edges.insert(id.into(), edge.into());
    }
}

/// Represents a restricted set of changes applied to the [`InputGraph`].
///
/// Used to limit the amount of data sent through the gateway.
#[must_use = "graph does nothing unless you use it"]
#[derive(Debug, Clone, Default, Serialize, Deserialize, TS)]
#[ts(export, export_to = "graph.ts")]
pub struct InputGraphDelta {
    #[serde(rename = "nodes")]
    pub nodes: Option<HashMap<InputNodeId, InputNode>>,
    #[serde(rename = "edges")]
    pub edges: Option<HashMap<InputEdgeId, InputEdge>>,
}

impl InputGraphDelta {
    /// Returns an empty [`InputGraphDelta`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts another [`InputNode`] to the [`InputGraphDelta`].
    pub fn with_node<I, T>(&mut self, id: I, node: T)
    where
        I: Into<InputNodeId>,
        T: Into<InputNode>,
    {
        let nodes = self.nodes.get_or_insert_with(HashMap::default);
        nodes.insert(id.into(), node.into());
    }

    /// Inserts another [`InputEdge`] to the [`InputGraphDelta`].
    pub fn with_edge<I, T>(&mut self, id: I, edge: T)
    where
        I: Into<InputEdgeId>,
        T: Into<InputEdge>,
    {
        let edges = self.edges.get_or_insert_with(HashMap::default);
        edges.insert(id.into(), edge.into());
    }
}

/// Opaque and unique [`InputNode`] identifier.
#[must_use = "graph id does nothing unless you use it"]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, From, Deref, Serialize, Deserialize, TS)]
#[ts(export, export_to = "graph.ts")]
pub struct InputNodeId(pub Uuid);

/// Opaque and unique [`InputEdge`] identifier.
#[must_use = "graph id does nothing unless you use it"]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, From, Deref, Serialize, Deserialize, TS)]
#[ts(export, export_to = "graph.ts")]
pub struct InputEdgeId(pub Uuid);

/// Represents a single [`InputGraph`] node.
#[must_use = "graph node does nothing unless you use it"]
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "graph.ts")]
#[skip_serializing_none]
pub struct InputNode {
    /// Execution priority expressed as an integer value.
    #[serde(rename = "offset")]
    #[serde(default)]
    pub(crate) priority: Option<u32>,
    /// Contains the node kind and its related data.
    #[serde(flatten)]
    pub(crate) inner: InputNodeKind,
}

impl InputNode {
    /// Returns `true` is it's an action node.
    #[inline]
    pub fn is_action(&self) -> bool {
        matches!(&self.inner, InputNodeKind::Action(_))
    }

    /// Returns `true` is it's a trigger node.
    #[inline]
    pub fn is_trigger(&self) -> bool {
        !self.is_action()
    }
}

/// Contains the node kind and its related data.
#[must_use = "graph node does nothing unless you use it"]
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "graph.ts")]
pub enum InputNodeKind {
    /// Natural root of the [`InputGraph`].
    #[serde(rename = "trigger:self")]
    TriggerOnManual,
    #[serde(rename = "trigger:cron")]
    TriggerOnSchedule(InputNodeSchedule),
    #[serde(rename = "trigger:hook")]
    TriggerOnWebhook(InputNodeWebhook),
    #[serde(rename = "action")]
    Action(InputNodeTask),
}

#[must_use = "graph node does nothing unless you use it"]
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "graph.ts")]
pub struct InputNodeSchedule {
    pub cron: String,
}

#[must_use = "graph node does nothing unless you use it"]
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "graph.ts")]
pub struct InputNodeWebhook {
    pub hook: String,
}

#[must_use = "graph node does nothing unless you use it"]
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "graph.ts")]
pub struct InputNodeTask {}

impl InputNode {
    /// Returns a new [`InputNode`].
    pub fn new() -> Self {
        Self {
            priority: None,
            inner: InputNodeKind::TriggerOnManual,
        }
    }
}

/// Represents a single [`InputGraph`] edge.
#[must_use = "graph edge does nothing unless you use it"]
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "graph.ts")]
#[skip_serializing_none]
pub struct InputEdge {
    /// [`InputNodeId`] of the source or origin of the [`InputEdge`].
    #[serde(rename = "tail")]
    pub tail: InputNodeId,
    /// [`InputNodeId`] of the destination or terminus of the [`InputEdge`].
    #[serde(rename = "head")]
    pub head: InputNodeId,
    /// Connected [`InputNode`]'s position expressed as an offset from the left i.e. the least important.
    #[serde(rename = "order")]
    #[serde(default)]
    pub offset: Option<u32>,
}

impl InputEdge {
    /// Returns a new [`InputEdge`].
    #[inline]
    pub fn new<T0, T1>(tail: T0, head: T1) -> Self
    where
        T0: Into<InputNodeId>,
        T1: Into<InputNodeId>,
    {
        Self {
            tail: tail.into(),
            head: head.into(),
            offset: None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::LazyLock;

    use uuid::Uuid;

    use crate::inputs::{InputEdge, InputGraph, InputGraphDelta, InputNode};
    use crate::Result;

    static T1: LazyLock<Uuid, fn() -> Uuid> = LazyLock::new(Uuid::new_v4);
    static T2: LazyLock<Uuid, fn() -> Uuid> = LazyLock::new(Uuid::new_v4);
    static T3: LazyLock<Uuid, fn() -> Uuid> = LazyLock::new(Uuid::new_v4);
    static T4: LazyLock<Uuid, fn() -> Uuid> = LazyLock::new(Uuid::new_v4);
    static E1: LazyLock<Uuid, fn() -> Uuid> = LazyLock::new(Uuid::new_v4);
    static E2: LazyLock<Uuid, fn() -> Uuid> = LazyLock::new(Uuid::new_v4);
    static E3: LazyLock<Uuid, fn() -> Uuid> = LazyLock::new(Uuid::new_v4);
    static E4: LazyLock<Uuid, fn() -> Uuid> = LazyLock::new(Uuid::new_v4);
    static A1: LazyLock<Uuid, fn() -> Uuid> = LazyLock::new(Uuid::new_v4);

    #[test]
    fn build_empty_input_graph() -> Result<()> {
        let input_graph = InputGraph::new();

        let json = serde_json::to_string_pretty(&input_graph);
        println!("{}", json.expect("should be parsable"));

        Ok(())
    }

    #[test]
    fn build_from_graph() -> Result<()> {
        let mut graph = InputGraph::new();

        graph.with_node(T1.clone(), InputNode::new());
        graph.with_node(T2.clone(), InputNode::new());
        graph.with_node(T3.clone(), InputNode::new());
        graph.with_node(T4.clone(), InputNode::new());

        graph.with_node(A1.clone(), InputNode::new());
        graph.with_edge(E1.clone(), InputEdge::new(T1.clone(), A1.clone()));
        graph.with_edge(E2.clone(), InputEdge::new(T2.clone(), A1.clone()));
        graph.with_edge(E3.clone(), InputEdge::new(T3.clone(), A1.clone()));
        graph.with_edge(E4.clone(), InputEdge::new(T4.clone(), A1.clone()));

        let json = serde_json::to_string_pretty(&graph);
        println!("{}", json.expect("should be parsable"));

        Ok(())
    }

    #[test]
    fn build_from_delta() -> Result<()> {
        let mut graph = InputGraph::new();

        let mut diff1 = InputGraphDelta::new();
        diff1.with_node(T1.clone(), InputNode::new());
        diff1.with_node(T2.clone(), InputNode::new());
        diff1.with_node(T3.clone(), InputNode::new());
        diff1.with_node(T4.clone(), InputNode::new());

        let mut diff2 = InputGraphDelta::new();
        diff2.with_node(A1.clone(), InputNode::new());
        diff2.with_edge(E1.clone(), InputEdge::new(T1.clone(), A1.clone()));
        diff2.with_edge(E2.clone(), InputEdge::new(T2.clone(), A1.clone()));
        diff2.with_edge(E3.clone(), InputEdge::new(T3.clone(), A1.clone()));
        diff2.with_edge(E4.clone(), InputEdge::new(T4.clone(), A1.clone()));

        graph.merge_deltas(diff1);
        graph.merge_deltas(diff2);

        let json = serde_json::to_string_pretty(&graph);
        println!("{}", json.expect("should be parsable"));

        Ok(())
    }
}
