use std::path::Path;

use crate::config::Args;

/// - Reads the entire contents of a file into a bytes vector.
/// - Deserializes an instance of type [`Args`] from the read content.
///
/// # Errors
///
/// - See [std::fs::read] and [`serde_json::from_slice`] documentation for details.
pub fn load_json<P: AsRef<Path>>(path: P) -> anyhow::Result<Args> {
    let file_content = std::fs::read(path)?;
    let parsed_args = serde_json::from_slice(&file_content)?;
    Ok(parsed_args)
}
