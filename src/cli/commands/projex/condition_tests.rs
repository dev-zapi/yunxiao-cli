use super::ConditionBuilder;

#[test]
fn build_empty_returns_none() {
    assert!(ConditionBuilder::new().build().is_none());
}

#[test]
fn string_contains_single() {
    let result = ConditionBuilder::new()
        .string_contains("subject", "test")
        .build()
        .unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    let cond = &parsed["conditionGroups"][0][0];
    assert_eq!(cond["fieldIdentifier"], "subject");
    assert_eq!(cond["operator"], "CONTAINS");
    assert_eq!(cond["value"][0], "test");
    assert_eq!(cond["className"], "string");
    assert_eq!(cond["format"], "input");
    assert!(cond["toValue"].is_null());
}

#[test]
fn string_between() {
    let result = ConditionBuilder::new()
        .string_between("name", "demo")
        .build()
        .unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    let cond = &parsed["conditionGroups"][0][0];
    assert_eq!(cond["fieldIdentifier"], "name");
    assert_eq!(cond["operator"], "BETWEEN");
    assert_eq!(cond["value"][0], "demo");
}

#[test]
fn list_contains() {
    let result = ConditionBuilder::new()
        .list_contains("sprint", "sprint", "sprint-123")
        .build()
        .unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    let cond = &parsed["conditionGroups"][0][0];
    assert_eq!(cond["fieldIdentifier"], "sprint");
    assert_eq!(cond["className"], "sprint");
    assert_eq!(cond["format"], "list");
    assert_eq!(cond["value"][0], "sprint-123");
}

#[test]
fn multi_list_contains() {
    let result = ConditionBuilder::new()
        .multi_list_contains("version", "version", "ver-abc")
        .build()
        .unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    let cond = &parsed["conditionGroups"][0][0];
    assert_eq!(cond["fieldIdentifier"], "version");
    assert_eq!(cond["className"], "version");
    assert_eq!(cond["format"], "multiList");
    assert_eq!(cond["value"][0], "ver-abc");
}

#[test]
fn user_contains_delegates_to_list() {
    let result = ConditionBuilder::new()
        .user_contains("creator", "user-001")
        .build()
        .unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    let cond = &parsed["conditionGroups"][0][0];
    assert_eq!(cond["fieldIdentifier"], "creator");
    assert_eq!(cond["className"], "user");
    assert_eq!(cond["format"], "list");
    assert_eq!(cond["value"][0], "user-001");
}

#[test]
fn multiple_conditions() {
    let result = ConditionBuilder::new()
        .string_contains("subject", "bug")
        .list_contains("status", "status", "100005")
        .user_contains("assignedTo", "user-002")
        .build()
        .unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    let group = parsed["conditionGroups"][0].as_array().unwrap();
    assert_eq!(group.len(), 3);
    assert_eq!(group[0]["fieldIdentifier"], "subject");
    assert_eq!(group[1]["fieldIdentifier"], "status");
    assert_eq!(group[2]["fieldIdentifier"], "assignedTo");
}

#[test]
fn opt_methods_skip_none() {
    let result = ConditionBuilder::new()
        .opt_string_contains("subject", None)
        .opt_string_between("name", None)
        .opt_list_contains("sprint", "sprint", None)
        .opt_multi_list_contains("version", "version", None)
        .opt_user_contains("creator", None)
        .build();
    assert!(result.is_none());
}

#[test]
fn opt_methods_add_some() {
    let direct = ConditionBuilder::new()
        .string_contains("subject", "test")
        .build()
        .unwrap();
    let via_opt = ConditionBuilder::new()
        .opt_string_contains("subject", Some("test"))
        .build()
        .unwrap();
    assert_eq!(direct, via_opt);
}

#[test]
fn chained_opt_mixed() {
    // Simulates real workitem search usage
    let result = ConditionBuilder::new()
        .opt_string_contains("subject", Some("login"))
        .opt_string_contains("serialNumber", None)
        .opt_multi_list_contains("version", "version", Some("ver-001"))
        .opt_list_contains("sprint", "sprint", None)
        .build()
        .unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    let group = parsed["conditionGroups"][0].as_array().unwrap();
    assert_eq!(group.len(), 2);
    assert_eq!(group[0]["fieldIdentifier"], "subject");
    assert_eq!(group[1]["fieldIdentifier"], "version");
}

#[test]
fn default_trait() {
    let result = ConditionBuilder::default().build();
    assert!(result.is_none());
}
