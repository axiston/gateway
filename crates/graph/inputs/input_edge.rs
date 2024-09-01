use serde::{Deserialize, Serialize};
use uuid::{Error, Uuid};

use crate::inputs::input_node::InputNodeId;

/// Opaque and unique [`InputEdge`] identifier.
#[must_use = "graph identifier does nothing unless you use it"]
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct InputEdgeId(pub Uuid);

impl InputEdgeId {
    /// Returns a new [`InputEdgeId`].
    pub fn new(inner: &str) -> Result<Self, Error> {
        inner.parse::<Uuid>().map(Self)
    }
}

#[must_use = "graph edge does nothing unless you use it"]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputEdge {
    pub origin: InputNodeId,
    pub destination: InputNodeId,
}

impl InputEdge {
    /// Returns a new [`InputEdge`].
    #[inline]
    pub fn new<T0, T1>(origin: T0, destination: T1) -> Self
    where
        T0: Into<InputNodeId>,
        T1: Into<InputNodeId>,
    {
        Self {
            origin: origin.into(),
            destination: destination.into(),
        }
    }
}
