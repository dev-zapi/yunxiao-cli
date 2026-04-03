//! yunxiao-cli – library root.
//!
//! Re-exports all public modules so they can be used both from the binary
//! crate (`main.rs`) and from integration tests / downstream consumers.

/// Authentication / token management layer.
pub mod auth;

/// Cache layer for persisting API responses.
pub mod cache;

/// CLI definition – root command, global flags, and subcommands.
pub mod cli;

/// HTTP client wrapper for the Yunxiao API.
pub mod client;

/// Configuration management (load, save, resolve).
pub mod config;

/// Unified error types and result alias.
pub mod error;

/// Output formatting (JSON, text, table, Markdown).
pub mod output;
