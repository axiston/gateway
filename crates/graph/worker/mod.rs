mod graph_compiler;
mod graph_executor;
mod internal_graph;

use std::collections::HashMap;

use crate::inputs::InputGraph;
use crate::outputs::{HookRegistry, ReportBundle, TaskRegistry};
use crate::worker::graph_compiler::{CompileGraph, DefaultGraphCompiler};
use crate::worker::graph_executor::{DefaultGraphExecutor, ExecuteGraph};
use crate::worker::internal_graph::{ProcessEdge, ProcessGraph, ProcessNode};

/// TODO.
#[derive(Debug)]
pub struct GraphWorker<C = DefaultGraphCompiler, E = DefaultGraphExecutor> {
    graph_compiler: C,
    graph_executor: E,
    task_registry: TaskRegistry,
    hook_registry: HookRegistry,
}

struct ActiveGraphs {
    last_graph_id: u32,
    active_graphs: HashMap<u32, ProcessGraph>,
}

impl<C, E> GraphWorker<C, E> {
    /// Returns a new [`GraphWorker`].
    pub fn new(
        graph_compiler: C,
        graph_executor: E,
        task_registry: TaskRegistry,
        hook_registry: HookRegistry,
    ) -> Self
    where
        C: CompileGraph,
        E: ExecuteGraph,
    {
        Self {
            graph_compiler,
            graph_executor,
            task_registry,
            hook_registry,
        }
    }

    /// Returns a reference to the inner [`CompileGraph`].
    #[inline]
    pub fn as_graph_compiler(&self) -> &C {
        &self.graph_compiler
    }

    /// Returns a reference to the inner [`ExecuteGraph`].
    #[inline]
    pub fn as_graph_executor(&self) -> &E {
        &self.graph_executor
    }

    /// Returns a reference to the inner [`TaskRegistry`].
    #[inline]
    pub fn as_task_registry(&self) -> &TaskRegistry {
        &self.task_registry
    }

    /// Returns a reference to the inner [`HookRegistry`].
    #[inline]
    pub fn as_hook_registry(&self) -> &HookRegistry {
        &self.hook_registry
    }

    pub fn load_graph(&mut self, input_graph: InputGraph) -> Result<u32, ReportBundle>
    where
        C: CompileGraph,
        E: ExecuteGraph,
    {
        let (process_graph, report_bundle) = self.graph_compiler.compile_input_graph(
            input_graph,
            &self.task_registry,
            &self.hook_registry,
        );
        // self.active_graphs.insert()

        todo!()
    }

    pub fn unload_graph(&self, graph: u32)
    where
        C: CompileGraph,
        E: ExecuteGraph,
    {
        todo!()
    }
}

impl Default for GraphWorker<DefaultGraphCompiler, DefaultGraphExecutor> {
    fn default() -> Self {
        let graph_compiler = DefaultGraphCompiler::default();
        let graph_executor = DefaultGraphExecutor::default();
        let task_registry = TaskRegistry::default();
        let hook_registry = HookRegistry::default();
        Self::new(graph_compiler, graph_executor, task_registry, hook_registry)
    }
}

#[cfg(test)]
mod test {
    use crate::worker::GraphWorker;
    use crate::Result;

    #[test]
    fn build_default() -> Result<()> {
        let worker = GraphWorker::default();
        let _ = worker.as_graph_compiler();
        let _ = worker.as_graph_executor();
        Ok(())
    }
}
