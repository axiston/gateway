mod output_edge;
mod output_node;

use std::fmt;

use derive_more::From;
use petgraph::{Direction, Graph};

pub use crate::outputs::output_edge::OutputEdge;
pub use crate::outputs::output_node::OutputNode;

// TODO: Id of the next node?

pub struct OutputGraph {
    inner: Graph<OutputNode, OutputEdge>,
}

impl OutputGraph {
    pub fn is_manual_trigger_enabled(&self) -> bool {
        todo!()
    }

    // pub fn run_from_node
}

impl fmt::Debug for OutputGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
