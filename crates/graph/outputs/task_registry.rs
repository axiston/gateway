use std::collections::HashMap;

use serde::Serialize;
use ts_rs::TS;

// TODO: Secrets.
// Map with loaded Tasks (contains secret ids).
// WeakMap with secrets on a client.

pub struct ServiceRegistry {}

/// Contains all the tasks available to the worker.
#[derive(Debug, Clone, Default, Serialize, TS)]
#[ts(export, export_to = "registry.ts")]
pub struct TaskRegistry {}

impl TaskRegistry {
    /// Returns an empty [`TaskRegistry`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    fn find_by_name(&self, name: &str) -> TaskRegistryChunk {
        TaskRegistryChunk::new([].into_iter())
    }

    fn find_by_tags(&self, tags: &str) -> TaskRegistryChunk {
        TaskRegistryChunk::new([].into_iter())
    }

    pub fn find(&self, query: &str) -> TaskRegistryChunk {
        let mut chunk = TaskRegistryChunk::default();
        chunk.merge(self.find_by_name(query));
        chunk.merge(self.find_by_tags(query));
        chunk
    }
}

#[derive(Debug, Default, Clone, Serialize, TS)]
#[ts(export, export_to = "registry.ts")]
pub struct TaskRegistryChunk {
    #[serde(rename = "tasks")]
    tasks: Vec<TaskRecord>,
}

impl TaskRegistryChunk {
    /// Returns a new [`TaskRegistryChunk`].
    pub fn new<T>(iter: T) -> Self
    where
        T: Iterator<Item = TaskRecord>,
    {
        Self {
            tasks: iter.collect(),
        }
    }

    /// Inserts all [`TaskRecord`]s from the other [`TaskRegistryChunk`].
    pub fn merge(&mut self, other_chunk: TaskRegistryChunk) {
        self.tasks.extend(other_chunk.tasks);
    }
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export, export_to = "registry.ts")]
pub struct TaskRecord {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    pub service: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export, export_to = "registry.ts")]
pub struct TaskFields {
    /// oauth token, jwt, password
    #[serde(rename = "secrets")]
    pub secrets: Vec<String>,
    #[serde(rename = "inputs")]
    pub inputs: HashMap<String, String>,
    #[serde(rename = "outputs")]
    pub outputs: HashMap<String, String>,
}

#[cfg(test)]
mod test {}
