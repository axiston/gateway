use serde::Serialize;
use ts_rs::TS;

/// Contains all the hooks available to the worker.
#[derive(Debug, Clone, Default, Serialize, TS)]
#[ts(export, export_to = "registry.ts")]
pub struct HookRegistry {}

impl HookRegistry {
    /// Returns an empty [`HookRegistry`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default, Clone, Serialize, TS)]
#[ts(export, export_to = "registry.ts")]
pub struct HookRegistryChunk {
    #[serde(rename = "hooks")]
    hooks: Vec<()>,
}
