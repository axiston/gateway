//! TODO.
//!

use petgraph::graph::NodeIndex;
use petgraph::{Direction, Graph};

use crate::compile::error_report::ErrorReport;
use crate::inputs::InputGraph;
use crate::outputs::OutputGraph;

mod cache_graph;
pub mod error_report;

pub trait CompileGraph {
    fn compile(self) -> Result<OutputGraph, ErrorReport>;
}

impl CompileGraph for InputGraph {
    fn compile(self) -> Result<OutputGraph, ErrorReport> {
        // TODO: Error report.

        // Constructs a graph of user nodes and edges.
        // let mut graph = Graph::new();

        for (idx, node) in self.nodes.iter().enumerate() {
            // graph.add_node(node);
        }

        for (idx, edge) in self.edges.iter().enumerate() {
            // let lhs = NodeIndex::new(edge.1.origin);
            // let rhs = NodeIndex::new(edge.destination);
            // graph.add_edge(lhs, rhs, edge);
        }

        // All external nodes are triggers.
        // Only one manual trigger.
        // Maximum of n triggers.

        // for trigger_node_id in graph.externals(Direction::Incoming) {
        //     let trigger_node = graph[trigger_node_id];
        // }

        todo!()
    }
}

pub trait CacheGraph {}

impl CacheGraph for OutputGraph {}
