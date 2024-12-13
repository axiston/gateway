use std::borrow::Cow;

use derive_more::From;
use serde::Serialize;
use thiserror::Error;
use ts_rs::TS;

use crate::inputs::{InputEdgeId, InputNodeId};

/// Bundle of unrecoverable [`ReportError`] failures.
#[must_use = "errors do nothing unless you use them"]
#[derive(Debug, Clone, Default, From, Error, Serialize, TS)]
#[error("underlying compilation or execution failure")]
#[ts(export, export_to = "error.ts")]
pub struct ReportBundle {
    #[serde(rename = "errors")]
    pub errors: Vec<ReportError>,
}

impl ReportBundle {
    /// Returns an empty [`ReportBundle`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts another [`ReportError`] into the [`ReportBundle`].
    pub fn with_error(&mut self, error: ReportError) {
        self.errors.push(error)
    }

    /// Extends this [`ReportBundle`] with another [`ReportBundle`].
    pub fn with_error_bundle(&mut self, error_report: ReportBundle) {
        self.errors.extend(error_report.errors);
    }

    /// Returns `true` if contains no hard errors.
    #[inline]
    pub fn is_only_warnings(&self) -> bool {
        self.errors
            .iter()
            .all(|x| matches!(x.event_class, EventClass::Warning(_)))
    }

    /// Returns the number of errors in the report bundle.
    #[inline]
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Returns `true` if the report contains no errors.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Unrecoverable failure of the compiler or the executor.
///
/// Includes all error types that may occur.
#[must_use = "errors do nothing unless you use them"]
#[derive(Debug, Clone, Error, Serialize, TS)]
#[error("underlying compilation or execution failure")]
#[ts(export, export_to = "error.ts")]
pub struct ReportError {
    /// Cause of an occurred error.
    #[serde(rename = "target")]
    event_target: EventTarget,
    /// Kind of an occurred error.
    #[serde(rename = "class")]
    event_class: EventClass,
    /// Description of an occurred error.
    #[serde(rename = "message")]
    report_message: Cow<'static, str>,
}

impl ReportError {
    /// Returns a new [`ReportError`].
    pub fn new(
        event_target: EventTarget,
        event_class: EventClass,
        report_message: Cow<'static, str>,
    ) -> Self {
        Self {
            event_target,
            event_class,
            report_message,
        }
    }

    /// Returns a description of an occurred error.
    #[inline]
    pub fn report_message(&self) -> &str {
        self.report_message.as_ref()
    }

    /// Creates a new [`ReportError`], reasons: graph edge references a non-existent node.
    pub fn missing_edge_target(input_edge_id: InputEdgeId, input_node_id: InputNodeId) -> Self {
        Self::new(
            EventTarget::Edge(input_edge_id),
            EventClass::Error(Cow::Borrowed("missing_referenced_node")),
            Cow::Owned(format!(
                "graph edge references a non-existent node: {}",
                *input_node_id
            )),
        )
    }

    /// Creates a new [`ReportError`], reasons: incorrect schedule trigger cron format.
    pub fn incorrect_cron_format(input_node_id: InputNodeId, cron_format: &str) -> Self {
        Self::new(
            EventTarget::Node(input_node_id),
            EventClass::Error(Cow::Borrowed("incorrect_trigger_schedule")),
            Cow::Owned(format!(
                "incorrect schedule trigger cron format: {}",
                cron_format
            )),
        )
    }
}

/// Represents a cause of an occurred error.
#[derive(Debug, Clone, From, Serialize, TS)]
#[ts(export, export_to = "error.ts")]
enum EventTarget {
    /// An identifier of a malformed node.
    #[serde(rename = "node")]
    Node(InputNodeId),
    /// An identifier of a malformed edge.
    #[serde(rename = "edge")]
    Edge(InputEdgeId),
}

/// Represents a kind or severity of an occurred error.
#[derive(Debug, Clone, Serialize, TS)]
#[ts(export, export_to = "error.ts")]
enum EventClass {
    /// Low severity, able to compile the graph without fixing.
    #[serde(rename = "warning")]
    Warning(Cow<'static, str>),
    /// High severity, unable to compile the graph without fixing.
    #[serde(rename = "error")]
    Error(Cow<'static, str>),
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::inputs::InputNodeId;
    use crate::outputs::{ReportBundle, ReportError};
    use crate::Result;

    #[test]
    fn build_empty_bundle() -> Result<()> {
        let _ = ReportBundle::new();
        Ok(())
    }

    #[test]
    fn build_with_reports() -> Result<()> {
        let mut bundle = ReportBundle::new();
        bundle.with_error(ReportError::incorrect_cron_format(
            InputNodeId(Uuid::new_v4()),
            "* * * *",
        ));

        Ok(())
    }
}
