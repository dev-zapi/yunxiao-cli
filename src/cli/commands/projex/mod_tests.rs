use super::*;

// ────── parse_dynamic_fields ──────

#[test]
fn parse_dynamic_fields_valid() {
    let fields = vec!["a=1".into(), "b=2".into()];
    let result = parse_dynamic_fields(&fields);
    assert_eq!(
        result,
        vec![
            ("a".to_string(), "1".to_string()),
            ("b".to_string(), "2".to_string()),
        ]
    );
}

#[test]
fn parse_dynamic_fields_with_equals_in_value() {
    let fields = vec!["key=val=ue".into()];
    let result = parse_dynamic_fields(&fields);
    assert_eq!(result, vec![("key".to_string(), "val=ue".to_string())]);
}

#[test]
fn parse_dynamic_fields_invalid_skipped() {
    let fields = vec!["noequals".into(), "valid=ok".into()];
    let result = parse_dynamic_fields(&fields);
    assert_eq!(result, vec![("valid".to_string(), "ok".to_string())]);
}

#[test]
fn parse_dynamic_fields_empty() {
    let result = parse_dynamic_fields(&[]);
    assert!(result.is_empty());
}

// ────── resolve_description ──────

#[test]
fn resolve_description_direct() {
    let desc = "hello\\nworld".to_string();
    let result = resolve_description(Some(&desc), None).unwrap();
    assert_eq!(result, Some("hello\nworld".to_string()));
}

#[test]
fn resolve_description_none() {
    let result = resolve_description(None, None).unwrap();
    assert!(result.is_none());
}

#[test]
fn resolve_description_from_file() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(tmp.path(), "file content").unwrap();
    let path = tmp.path().to_string_lossy().to_string();
    let result = resolve_description(None, Some(&path)).unwrap();
    assert_eq!(result, Some("file content".to_string()));
}

#[test]
fn resolve_description_direct_wins_over_file() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(tmp.path(), "from file").unwrap();
    let desc = "from arg".to_string();
    let path = tmp.path().to_string_lossy().to_string();
    let result = resolve_description(Some(&desc), Some(&path)).unwrap();
    assert_eq!(result, Some("from arg".to_string()));
}

// ────── format_type_to_api ──────

#[test]
fn format_type_to_api_text() {
    assert_eq!(format_type_to_api(DescriptionFormat::Text), "RICHTEXT");
}

#[test]
fn format_type_to_api_markdown() {
    assert_eq!(format_type_to_api(DescriptionFormat::Markdown), "MARKDOWN");
}

// ────── require_org ──────

#[test]
fn require_org_some() {
    let org = Some("org-x".to_string());
    assert_eq!(require_org(&org).unwrap(), "org-x");
}

#[test]
fn require_org_none() {
    let org: Option<String> = None;
    let err = require_org(&org).unwrap_err();
    assert!(matches!(err, crate::error::CliError::Config(_)));
}
