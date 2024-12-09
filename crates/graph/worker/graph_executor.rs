use crate::outputs::{OutputGraph, ReportBundle, TaskRegistry};
use crate::worker::internal_graph::ProcessGraph;

/// TODO.
pub trait ExecuteGraph {
    fn execute_graph(
        &self,
        process_graph: ProcessGraph,
        task_registry: &TaskRegistry,
    ) -> (OutputGraph, ReportBundle);
}

/// Default [`ExecuteGraph`] implementation.
#[derive(Debug, Clone, Default)]
pub struct DefaultGraphExecutor {
    /// Limits the number of concurrently ran tasks.
    ///
    /// Defaults to the current `runtime_pool`.
    limit_concurrency: Option<u32>,

    /// Strictly follows the execution priority.
    ///
    /// Defaults to `false`.
    strict_priority: bool,
}

impl DefaultGraphExecutor {
    /// Returns a new [`DefaultGraphExecutor`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    fn schedule_next_batch(&self, runtime_pool: u32) -> Result<(), ReportBundle> {
        let concurrency_limit = self.limit_concurrency.unwrap_or(runtime_pool);
        // TODO: get next n tasks

        Ok(())
    }
}

impl ExecuteGraph for DefaultGraphExecutor {
    fn execute_graph(
        &self,
        process_graph: ProcessGraph,
        task_registry: &TaskRegistry,
    ) -> (OutputGraph, ReportBundle) {
        todo!()
    }
}
