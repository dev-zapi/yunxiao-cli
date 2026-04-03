//! Cache layer for the YunXiao CLI.
//!
//! Stores JSON files under `~/.cache/yunxiao-cli/` keyed by a caller-chosen
//! string. Each cached value is written as `{key}.json`.

use crate::error::{CliError, Result};
use log::debug;
use std::path::PathBuf;

/// Returns the cache directory: `~/.cache/yunxiao-cli/`.
pub fn cache_dir() -> PathBuf {
    let base = dirs::cache_dir().unwrap_or_else(|| {
        dirs::home_dir()
            .expect("Cannot determine home directory")
            .join(".cache")
    });
    base.join("yunxiao-cli")
}

/// Ensures the cache directory exists, creating it if necessary.
pub fn ensure_cache_dir() -> Result<()> {
    let dir = cache_dir();
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
        debug!("Created cache directory at {}", dir.display());
    }
    Ok(())
}

/// Write a JSON value to the cache under the given key.
///
/// The value is stored at `<cache_dir>/<key>.json`.
pub fn write_cache(key: &str, value: &serde_json::Value) -> Result<()> {
    ensure_cache_dir()?;
    let path = cache_dir().join(format!("{key}.json"));
    let content = serde_json::to_string_pretty(value)?;
    std::fs::write(&path, content)?;
    debug!("Wrote cache entry '{}' to {}", key, path.display());
    Ok(())
}

/// Read a cached JSON value by key.
///
/// Returns `Ok(None)` if the cache entry does not exist.
pub fn read_cache(key: &str) -> Result<Option<serde_json::Value>> {
    let path = cache_dir().join(format!("{key}.json"));
    if !path.exists() {
        debug!("Cache miss for key '{}'", key);
        return Ok(None);
    }

    let content = std::fs::read_to_string(&path).map_err(|e| {
        CliError::Cache(format!("Failed to read cache file {}: {e}", path.display()))
    })?;

    let value: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| CliError::Cache(format!("Corrupt cache file {} : {e}", path.display())))?;

    debug!("Cache hit for key '{}'", key);
    Ok(Some(value))
}

/// Remove all cached entries by deleting every file in the cache directory.
pub fn clear_cache() -> Result<()> {
    let dir = cache_dir();
    if dir.exists() {
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            if entry.path().is_file() {
                std::fs::remove_file(entry.path())?;
            }
        }
        debug!("Cleared cache directory {}", dir.display());
    }
    Ok(())
}

/// Delete a single cache entry by key.
///
/// Silently succeeds if the entry does not exist.
pub fn delete_cache(key: &str) -> Result<()> {
    let path = cache_dir().join(format!("{key}.json"));
    if path.exists() {
        std::fs::remove_file(&path)?;
        debug!("Deleted cache entry '{}'", key);
    }
    Ok(())
}
