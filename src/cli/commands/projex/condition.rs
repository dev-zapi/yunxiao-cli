//! Builder for constructing API filter conditions (conditionGroups format).

use serde_json::json;

/// Builder for constructing API filter conditions (conditionGroups format).
pub struct ConditionBuilder {
    conditions: Vec<serde_json::Value>,
}

impl Default for ConditionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ConditionBuilder {
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
        }
    }

    /// Add a string field condition (format: "input", operator: "CONTAINS").
    /// Used for: subject, serialNumber, name, etc.
    pub fn string_contains(mut self, field: &str, value: &str) -> Self {
        self.conditions.push(json!({
            "fieldIdentifier": field,
            "operator": "CONTAINS",
            "value": [value],
            "toValue": null,
            "className": "string",
            "format": "input"
        }));
        self
    }

    /// Add a string field condition with BETWEEN operator.
    /// Used for: project name search.
    pub fn string_between(mut self, field: &str, value: &str) -> Self {
        self.conditions.push(json!({
            "fieldIdentifier": field,
            "operator": "BETWEEN",
            "value": [value],
            "toValue": null,
            "className": "string",
            "format": "input"
        }));
        self
    }

    /// Add a list field condition (format: "list", operator: "CONTAINS").
    /// Used for: status, sprint, logicalStatus, etc.
    pub fn list_contains(mut self, field: &str, class_name: &str, value: &str) -> Self {
        self.conditions.push(json!({
            "fieldIdentifier": field,
            "operator": "CONTAINS",
            "value": [value],
            "toValue": null,
            "className": class_name,
            "format": "list"
        }));
        self
    }

    /// Add a multi-list field condition (format: "multiList", operator: "CONTAINS").
    /// Used for: version, tag, project.admin, etc.
    pub fn multi_list_contains(mut self, field: &str, class_name: &str, value: &str) -> Self {
        self.conditions.push(json!({
            "fieldIdentifier": field,
            "operator": "CONTAINS",
            "value": [value],
            "toValue": null,
            "className": class_name,
            "format": "multiList"
        }));
        self
    }

    /// Add a user field condition (format: "list", operator: "CONTAINS").
    pub fn user_contains(self, field: &str, user_id: &str) -> Self {
        self.list_contains(field, "user", user_id)
    }

    // Convenience methods that only add condition if value is Some

    pub fn opt_string_contains(self, field: &str, value: Option<&str>) -> Self {
        match value {
            Some(v) => self.string_contains(field, v),
            None => self,
        }
    }

    pub fn opt_string_between(self, field: &str, value: Option<&str>) -> Self {
        match value {
            Some(v) => self.string_between(field, v),
            None => self,
        }
    }

    pub fn opt_list_contains(self, field: &str, class_name: &str, value: Option<&str>) -> Self {
        match value {
            Some(v) => self.list_contains(field, class_name, v),
            None => self,
        }
    }

    pub fn opt_multi_list_contains(
        self,
        field: &str,
        class_name: &str,
        value: Option<&str>,
    ) -> Self {
        match value {
            Some(v) => self.multi_list_contains(field, class_name, v),
            None => self,
        }
    }

    pub fn opt_user_contains(self, field: &str, value: Option<&str>) -> Self {
        match value {
            Some(v) => self.user_contains(field, v),
            None => self,
        }
    }

    /// Build into a JSON string for the "conditions" parameter.
    /// Returns None if no conditions were added.
    pub fn build(self) -> Option<String> {
        if self.conditions.is_empty() {
            None
        } else {
            Some(json!({ "conditionGroups": [self.conditions] }).to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        // ConditionBuilder::default() should work the same as new()
        let result = ConditionBuilder::default().build();
        assert!(result.is_none());
    }
}
