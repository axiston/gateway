use std::collections::HashMap;
use std::error::Error;

use petgraph::graph::NodeIndex;
use petgraph::Graph;

use crate::inputs::{InputEdge, InputEdgeId, InputGraph, InputNode, InputNodeId, InputNodeKind};
use crate::outputs::{HookRegistry, ReportBundle, ReportError, TaskRegistry};
use crate::worker::{ProcessEdge, ProcessGraph, ProcessNode};

/// TODO.
pub trait CompileGraph {
    /// Transforms all nodes and edges of [`InputGraph`] into [`ProcessGraph`].
    fn compile_input_graph(
        &self,
        input_graph: InputGraph,
        task_registry: &TaskRegistry,
        hook_registry: &HookRegistry,
    ) -> (ProcessGraph, ReportBundle);
}

/// Default [`CompileGraph`] implementation.
#[derive(Debug, Clone, Default)]
pub struct DefaultGraphCompiler {
    /// Unique compiler instance identifier.
    ///
    /// Used for debugging and improved observability.
    pub compiler_instance: u32,

    /// Aborts the process if it has encountered any issues.
    ///
    /// Defaults to `false`.
    pub short_circuit: bool,
}

impl DefaultGraphCompiler {
    /// Returns a new [`DefaultGraphCompiler`].
    #[inline]
    pub fn new(compiler_instance: u32) -> Self {
        Self::default().with_compiler_instance(compiler_instance)
    }

    /// Sets the `compiler_instance` to the provided value.
    pub fn with_compiler_instance(mut self, compiler_instance: u32) -> Self {
        self.compiler_instance = compiler_instance;
        self
    }

    /// Sets the `short_circuit` flag to the provided value.
    pub fn with_short_circuit(mut self, short_circuit: bool) -> Self {
        self.short_circuit = short_circuit;
        self
    }

    fn input_node_into_output_node(
        &self,
        input_node_id: InputNodeId,
        input_node: InputNode,
        task_registry: &TaskRegistry,
        hook_registry: &HookRegistry,
    ) -> Result<ProcessNode, ReportError> {
        tracing::trace!(
            instance = self.compiler_instance,
            node = input_node_id.as_u128(),
            "node processed"
        );

        let process_node_result = match input_node.inner {
            InputNodeKind::TriggerOnManual => ProcessNode::try_from_manual_trigger(input_node_id),
            InputNodeKind::TriggerOnSchedule(data) => {
                ProcessNode::try_from_cron_trigger(input_node_id, &data.cron)
            }
            InputNodeKind::TriggerOnWebhook(data) => {
                ProcessNode::try_from_hook_trigger(input_node_id, &data.hook, hook_registry)
            }
            InputNodeKind::Action(data) => {
                ProcessNode::try_from_action(input_node_id, task_registry)
            }
        };

        process_node_result
            .map(|process_node| {
                process_node.with_execute_priority(input_node.priority.unwrap_or_default())
            })
            .inspect_err(|report_error| {
                tracing::error!(
                    instance = self.compiler_instance,
                    node = input_node_id.as_u128(),
                    error = report_error.report_message(),
                    "node error"
                );
            })
    }

    fn input_edge_into_output_edge(
        &self,
        input_edge_id: InputEdgeId,
        input_edge: InputEdge,
        _task_registry: &TaskRegistry,
        _hook_registry: &HookRegistry,
    ) -> Result<ProcessEdge, ReportError> {
        tracing::trace!(
            instance = self.compiler_instance,
            edge = input_edge_id.as_u128(),
            tail = input_edge.tail.as_u128(),
            head = input_edge.head.as_u128(),
            "edge processed"
        );

        Ok(ProcessEdge::new(input_edge_id))
    }

    fn transform_input_graph(
        &self,
        input_graph: InputGraph,
        task_registry: &TaskRegistry,
        hook_registry: &HookRegistry,
    ) -> (ProcessGraph, ReportBundle) {
        let mut output_graph = ProcessGraph::default();
        let mut report_bundle = ReportBundle::new();

        // Moves nodes from input graph into output graph.
        let mut node_ids = HashMap::with_capacity(input_graph.nodes.len());
        for (input_node_id, input_node) in input_graph.nodes.into_iter() {
            // Transform input edge to output edge.
            let output_node = match self.input_node_into_output_node(
                input_node_id,
                input_node,
                task_registry,
                hook_registry,
            ) {
                Ok(output_node) => output_node,
                Err(report) if self.short_circuit => {
                    report_bundle.with_error(report);
                    return (output_graph, report_bundle);
                }
                Err(report) => {
                    report_bundle.with_error(report);
                    continue;
                }
            };

            let output_graph_node_id = output_graph.add_node(output_node);
            debug_assert_eq!(output_graph_node_id.index(), node_ids.len() + 1);
            node_ids.insert(input_node_id, node_ids.len() + 1);
        }

        // Moves edges from input graph into output graph.
        for (input_edge_id, input_edge) in input_graph.edges.into_iter() {
            // Verify tail node.
            let tail_node_id = input_edge.tail.clone();
            debug_assert!(node_ids.contains_key(&tail_node_id));
            let tail_node_idx = match node_ids
                .get(&tail_node_id)
                .ok_or_else(|| ReportError::missing_edge_target(input_edge_id, tail_node_id))
            {
                Ok(tail_node_idx) => NodeIndex::new(*tail_node_idx),
                Err(report) if self.short_circuit => {
                    report_bundle.with_error(report);
                    return (output_graph, report_bundle);
                }
                Err(report) => {
                    report_bundle.with_error(report);
                    continue;
                }
            };

            // Verify head node.
            let head_node_id = input_edge.head.clone();
            debug_assert!(node_ids.contains_key(&head_node_id));
            let head_node_idx = match node_ids
                .get(&head_node_id)
                .ok_or_else(|| ReportError::missing_edge_target(input_edge_id, head_node_id))
            {
                Ok(head_node_idx) => NodeIndex::new(*head_node_idx),
                Err(report) if self.short_circuit => {
                    report_bundle.with_error(report);
                    return (output_graph, report_bundle);
                }
                Err(report) => {
                    report_bundle.with_error(report);
                    continue;
                }
            };

            // Transform input edge to output edge.
            let output_edge = match self.input_edge_into_output_edge(
                input_edge_id,
                input_edge,
                task_registry,
                hook_registry,
            ) {
                Ok(output_edge) => output_edge,
                Err(report) if self.short_circuit => {
                    report_bundle.with_error(report);
                    return (output_graph, report_bundle);
                }
                Err(report) => {
                    report_bundle.with_error(report);
                    continue;
                }
            };

            output_graph.add_edge(tail_node_idx, head_node_idx, output_edge);
        }

        (output_graph, report_bundle)
    }

    fn verify_output_graph(
        &self,
        process_graph: &ProcessGraph,
        task_registry: &TaskRegistry,
        hook_registry: &HookRegistry,
    ) -> ReportBundle {
        let mut report_bundle = ReportBundle::new();
        // TODO.
        report_bundle
    }
}

impl CompileGraph for DefaultGraphCompiler {
    fn compile_input_graph(
        &self,
        input_graph: InputGraph,
        task_registry: &TaskRegistry,
        hook_registry: &HookRegistry,
    ) -> (ProcessGraph, ReportBundle) {
        let (process_graph, mut report_bundle) =
            self.transform_input_graph(input_graph, task_registry, hook_registry);
        if self.short_circuit && !report_bundle.is_empty() {
            return (process_graph, report_bundle);
        }

        report_bundle.with_error_bundle(self.verify_output_graph(
            &process_graph,
            task_registry,
            hook_registry,
        ));

        (process_graph, report_bundle)
    }
}
