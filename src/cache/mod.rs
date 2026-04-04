//! Cache layer for the YunXiao CLI.
//!
//! Stores JSON files under `$XDG_CACHE_HOME/yunxiao-cli/` (defaults to
//! `~/.cache/yunxiao-cli/`) keyed by a caller-chosen string.
//! Each cached value is written as `{key}.json`.
//!
//! The cache directory follows the XDG Base Directory Specification:
//! - Path: `$XDG_CACHE_HOME/yunxiao-cli/` (defaults to `~/.cache/yunxiao-cli/`)
//! - Contents: Token-bound user ID, organization info, project info,
//!   temporary API response cache, runtime intermediate data.
//! - Cleanup: Supports manual `clear_cache()` command, no automatic deletion.

use crate::error::{CliError, Result};
use log::debug;
use std::path::PathBuf;

/// Returns the cache directory: `$XDG_CACHE_HOME/yunxiao-cli/`.
///
/// Per the XDG Base Directory Specification, runtime cache data is stored under
/// `$XDG_CACHE_HOME/yunxiao-cli/` (defaults to `~/.cache/yunxiao-cli/`),
/// distinct from the config directory at `~/.config/yunxiao-cli/`.
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn cache_dir_is_under_cache() {
        let dir = cache_dir();
        let home = dirs::home_dir().unwrap();
        // Should be ~/.cache/yunxiao-cli/ (XDG_CACHE_HOME)
        assert!(dir.starts_with(&home));
        assert!(dir.to_string_lossy().contains(".cache"));
        assert!(dir.to_string_lossy().contains("yunxiao-cli"));
    }

    #[test]
    fn write_and_read_cache_roundtrip() {
        let key = "test_roundtrip";
        let value = json!({"name": "test", "count": 42});

        // Write
        write_cache(key, &value).unwrap();

        // Read
        let result = read_cache(key).unwrap();
        assert!(result.is_some());
        let cached = result.unwrap();
        assert_eq!(cached["name"], "test");
        assert_eq!(cached["count"], 42);

        // Cleanup
        delete_cache(key).unwrap();
    }

    #[test]
    fn read_cache_returns_none_for_missing_key() {
        let result = read_cache("nonexistent_key_xyz_12345").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn delete_cache_removes_entry() {
        let key = "test_delete";
        let value = json!({"temp": true});

        write_cache(key, &value).unwrap();
        assert!(read_cache(key).unwrap().is_some());

        delete_cache(key).unwrap();
        assert!(read_cache(key).unwrap().is_none());
    }

    #[test]
    fn delete_cache_silent_on_missing() {
        // Should not error when deleting a non-existent key
        let result = delete_cache("no_such_key_abc_999");
        assert!(result.is_ok());
    }

    #[test]
    fn ensure_cache_dir_creates_directory() {
        ensure_cache_dir().unwrap();
        assert!(cache_dir().exists());
    }

    #[test]
    fn clear_cache_removes_all_entries() {
        // Write two entries
        write_cache("clear_test_a", &json!({"a": 1})).unwrap();
        write_cache("clear_test_b", &json!({"b": 2})).unwrap();

        // Clear
        clear_cache().unwrap();

        // Both should be gone
        assert!(read_cache("clear_test_a").unwrap().is_none());
        assert!(read_cache("clear_test_b").unwrap().is_none());
    }
}
