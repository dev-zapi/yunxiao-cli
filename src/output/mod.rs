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
            table.set_header(columns.iter().map(Cell::new));

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
            table.set_header(["Key", "Value"].iter().map(Cell::new));
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn format_cell_value_string() {
        let v = json!("hello");
        assert_eq!(format_cell_value(&v), "hello");
    }

    #[test]
    fn format_cell_value_null() {
        let v = json!(null);
        assert_eq!(format_cell_value(&v), "");
    }

    #[test]
    fn format_cell_value_bool() {
        assert_eq!(format_cell_value(&json!(true)), "true");
        assert_eq!(format_cell_value(&json!(false)), "false");
    }

    #[test]
    fn format_cell_value_number() {
        assert_eq!(format_cell_value(&json!(42)), "42");
        assert_eq!(format_cell_value(&json!(3.15)), "3.15");
    }

    #[test]
    fn format_cell_value_nested_object() {
        let v = json!({"a": 1});
        let result = format_cell_value(&v);
        assert!(result.contains("\"a\""));
        assert!(result.contains("1"));
    }

    #[test]
    fn print_json_outputs_pretty() {
        // Just verify it doesn't panic
        let data = json!({"key": "value", "num": 123});
        let result = print_json(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn print_text_object() {
        let data = json!({"name": "test", "count": 5});
        let result = print_text(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn print_text_array() {
        let data = json!([{"a": 1}, {"b": 2}]);
        let result = print_text(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn print_text_scalar() {
        let result = print_text(&json!("just a string"));
        assert!(result.is_ok());
    }

    #[test]
    fn print_text_null() {
        let result = print_text(&json!(null));
        assert!(result.is_ok());
    }

    #[test]
    fn print_table_array_of_objects() {
        let data = json!([
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"}
        ]);
        let result = print_table(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn print_table_single_object() {
        let data = json!({"key1": "val1", "key2": "val2"});
        let result = print_table(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn print_table_empty_array() {
        let data = json!([]);
        let result = print_table(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn print_table_scalar() {
        let result = print_table(&json!(42));
        assert!(result.is_ok());
    }

    #[test]
    fn print_markdown_array_of_objects() {
        let data = json!([
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"}
        ]);
        let result = print_markdown(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn print_markdown_single_object() {
        let data = json!({"key": "value"});
        let result = print_markdown(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn print_markdown_null() {
        let result = print_markdown(&json!(null));
        assert!(result.is_ok());
    }

    #[test]
    fn print_output_dispatches_correctly() {
        let data = json!({"test": true});
        assert!(print_output(&data, &OutputFormat::Json).is_ok());
        assert!(print_output(&data, &OutputFormat::Text).is_ok());
        assert!(print_output(&data, &OutputFormat::Table).is_ok());
        assert!(print_output(&data, &OutputFormat::Markdown).is_ok());
    }
}
