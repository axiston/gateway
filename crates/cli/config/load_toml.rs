use std::path::Path;

use crate::config::Args;

/// - Reads the entire contents of a file into a string buffer.
/// - Deserializes an instance of type [`Args`] from the read content.
///
/// # Errors
///
/// - See [std::fs::read_to_string] and [`serde_toml::from_str`] documentation for details.
pub fn load_toml<P: AsRef<Path>>(path: P) -> anyhow::Result<Args> {
    let file_content = std::fs::read_to_string(path)?;
    let parsed_args = serde_toml::from_str(&file_content)?;
    Ok(parsed_args)
}
