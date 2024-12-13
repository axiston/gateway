//! Deserializable datatypes (client to server).
//!

mod request_event;
mod request_graph;

pub use crate::inputs::request_event::InputEvent;
pub use crate::inputs::request_graph::{
    InputEdge, InputEdgeId, InputGraph, InputGraphDelta, InputNode, InputNodeId, InputNodeKind,
    InputNodeSchedule, InputNodeTask, InputNodeWebhook,
};
