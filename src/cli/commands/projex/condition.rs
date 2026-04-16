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
