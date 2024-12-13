use std::collections::{HashMap, VecDeque};

use axiston_graph::worker::GraphWorker;
use uuid::Uuid;

/// TODO.
#[derive(Debug)]
struct UserGraphData {
    graph_id: Uuid,
    graph_data: (),
}

/// TODO.
#[derive(Debug)]
struct UserGraphWorker {
    user_graph: UserGraphData,
    graph_worker: GraphWorker,
}

/// TODO.
#[derive(Debug, Default)]
pub struct GraphQueue {
    waiting: VecDeque<UserGraphData>,
    working: HashMap<Uuid, UserGraphWorker>,
}

impl GraphQueue {
    /// Creates a new [`GraphQueue`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    // TODO.
    // pub fn is_active(&self, graph_id: Uuid) -> bool {
    //     self.working.contains_key(&graph_id)
    //         || self.waiting.iter().any(|data| data.graph_id == graph_id)
    // }

    /// Returns `true` if the [`GraphQueue`] is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.waiting.is_empty()
    }

    /// Returns `true` if the [`GraphQueue`] is full.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.waiting.capacity() == self.waiting.len()
    }
}
