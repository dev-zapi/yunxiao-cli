//! Output formatting for CLI results.
//!
//! Supports four output modes: JSON, plain text, ASCII table, and Markdown.
//! The format is selected by the resolved [`OutputFormat`].

use crate::config::types::OutputFormat;
use crate::error::Result;
use comfy_table::{Cell, ContentArrangement, Table};

/// Print `data` to stdout in the requested format.
///
/// Delegates to one of the specialised formatters below.
pub fn print_output(data: &serde_json::Value, format: &OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Json => print_json(data),
        OutputFormat::Text => print_text(data),
        OutputFormat::Table => print_table(data),
        OutputFormat::Markdown => print_markdown(data),
    }
}

/// Pretty-print as indented JSON.
fn print_json(data: &serde_json::Value) -> Result<()> {
    let formatted = serde_json::to_string_pretty(data)?;
    println!("{formatted}");
    Ok(())
}

/// Print as plain text.
///
/// - Objects are rendered as `key: value` pairs (one per line).
/// - Arrays delegate each element recursively.
/// - Scalars are printed directly.
fn print_text(data: &serde_json::Value) -> Result<()> {
    match data {
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                match v {
                    serde_json::Value::String(s) => println!("{k}: {s}"),
                    serde_json::Value::Null => println!("{k}: (null)"),
                    other => println!("{k}: {other}"),
                }
            }
        }
        serde_json::Value::Array(arr) => {
            for (i, item) in arr.iter().enumerate() {
                if i > 0 {
                    println!("---");
                }
                print_text(item)?;
            }
        }
        serde_json::Value::String(s) => println!("{s}"),
        serde_json::Value::Null => println!("(null)"),
        other => println!("{other}"),
    }
    Ok(())
}

/// Print as an ASCII table using `comfy-table`.
///
/// - Arrays of objects: columns = union of keys; rows = values.
/// - Single objects: two-column key/value table.
/// - Scalars: simple single-cell table.
fn print_table(data: &serde_json::Value) -> Result<()> {
    match data {
        serde_json::Value::Array(arr) if !arr.is_empty() => {
            // Collect all unique keys to form column headers.
            let mut columns: Vec<String> = Vec::new();
            for item in arr {
                if let serde_json::Value::Object(map) = item {
                    for key in map.keys() {
                        if !columns.contains(key) {
                            columns.push(key.clone());
                        }
                    }
                }
            }

            if columns.is_empty() {
                // Array of non-objects – just print each element.
                for item in arr {
                    println!("{item}");
                }
                return Ok(());
            }

            let mut table = Table::new();
            table.set_content_arrangement(ContentArrangement::Dynamic);
            table.set_header(columns.iter().map(|c| Cell::new(c)));

            for item in arr {
                let row: Vec<Cell> = columns
                    .iter()
                    .map(|col| {
                        let v = item.get(col).unwrap_or(&serde_json::Value::Null);
                        Cell::new(format_cell_value(v))
                    })
                    .collect();
                table.add_row(row);
            }

            println!("{table}");
        }
        serde_json::Value::Object(map) => {
            let mut table = Table::new();
            table.set_content_arrangement(ContentArrangement::Dynamic);
            table.set_header(["Key", "Value"].iter().map(|h| Cell::new(h)));
            for (k, v) in map {
                table.add_row(vec![Cell::new(k), Cell::new(format_cell_value(v))]);
            }
            println!("{table}");
        }
        other => {
            println!("{other}");
        }
    }
    Ok(())
}

/// Print as Markdown.
///
/// - Arrays of objects → Markdown table.
/// - Single objects → key/value list.
/// - Scalars → plain text.
fn print_markdown(data: &serde_json::Value) -> Result<()> {
    match data {
        serde_json::Value::Array(arr) if !arr.is_empty() => {
            // Gather columns
            let mut columns: Vec<String> = Vec::new();
            for item in arr {
                if let serde_json::Value::Object(map) = item {
                    for key in map.keys() {
                        if !columns.contains(key) {
                            columns.push(key.clone());
                        }
                    }
                }
            }

            if columns.is_empty() {
                for item in arr {
                    println!("- {item}");
                }
                return Ok(());
            }

            // Header row
            let header: String = columns
                .iter()
                .map(|c| c.as_str())
                .collect::<Vec<_>>()
                .join(" | ");
            let separator: String = columns
                .iter()
                .map(|_| "---")
                .collect::<Vec<_>>()
                .join(" | ");
            println!("| {header} |");
            println!("| {separator} |");

            // Data rows
            for item in arr {
                let cells: Vec<String> = columns
                    .iter()
                    .map(|col| {
                        let v = item.get(col).unwrap_or(&serde_json::Value::Null);
                        format_cell_value(v)
                    })
                    .collect();
                println!("| {} |", cells.join(" | "));
            }
        }
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                println!("- **{k}**: {}", format_cell_value(v));
            }
        }
        serde_json::Value::String(s) => println!("{s}"),
        serde_json::Value::Null => println!("*(empty)*"),
        other => println!("{other}"),
    }
    Ok(())
}

/// Format a JSON value for display inside a table cell.
///
/// Strings are shown without quotes; nulls as empty; nested structures as
/// compact JSON.
fn format_cell_value(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Null => String::new(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => n.to_string(),
        // Nested arrays / objects – compact JSON
        other => other.to_string(),
    }
}
