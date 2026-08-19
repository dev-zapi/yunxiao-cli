use super::*;
use clap::Parser;
use mockito::{Matcher, Server};
use serde_json::json;

#[test]
fn resolve_search_categories_defaults_to_three_types() {
    assert_eq!(resolve_search_categories(&[]), "Req,Task,Bug");
}

#[test]
fn resolve_search_categories_joins_multiple_values() {
    let categories = vec!["Req".to_string(), "Task".to_string(), "Bug".to_string()];
    assert_eq!(resolve_search_categories(&categories), "Req,Task,Bug");
}

#[test]
fn search_args_support_repeated_short_category_flags() {
    #[derive(Parser)]
    struct TestCli {
        #[command(subcommand)]
        command: WorkitemsCmds,
    }

    let cli = TestCli::parse_from([
        "test",
        "search",
        "--space-id",
        "proj-1",
        "-c",
        "Req",
        "-c",
        "Task",
        "-k",
        "login",
        "-n",
        "MMCL-123",
        "-v",
        "ver-1",
        "-S",
        "sprint-1",
        "-p",
        "2",
        "-P",
        "50",
    ]);

    let WorkitemsCmds::Search(args) = cli.command else {
        panic!("expected search command");
    };

    assert_eq!(args.space_id, "proj-1");
    assert_eq!(args.category, vec!["Req", "Task"]);
    assert_eq!(args.keyword.as_deref(), Some("login"));
    assert_eq!(args.serial_number.as_deref(), Some("MMCL-123"));
    assert_eq!(args.version_id.as_deref(), Some("ver-1"));
    assert_eq!(args.sprint_id.as_deref(), Some("sprint-1"));
    assert_eq!(args.page, 2);
    assert_eq!(args.page_size, 50);
}

#[test]
fn search_args_default_categories_when_omitted() {
    #[derive(Parser)]
    struct TestCli {
        #[command(subcommand)]
        command: WorkitemsCmds,
    }

    let cli = TestCli::parse_from(["test", "search", "--space-id", "proj-1"]);

    let WorkitemsCmds::Search(args) = cli.command else {
        panic!("expected search command");
    };

    assert!(args.category.is_empty());
    assert_eq!(resolve_search_categories(&args.category), "Req,Task,Bug");
    assert_eq!(args.page, 1);
    assert_eq!(args.page_size, 20);
}

#[test]
fn resolve_search_categories_preserves_custom_values() {
    let categories = vec!["CustomReq".to_string(), "Spike".to_string()];
    assert_eq!(resolve_search_categories(&categories), "CustomReq,Spike");
}

#[test]
fn build_search_body_uses_default_categories_when_omitted() {
    let args = WiSearchArgs {
        space_id: "proj-1".to_string(),
        category: Vec::new(),
        keyword: None,
        serial_number: None,
        version_id: None,
        sprint_id: None,
        page_size: 20,
        page: 1,
    };

    let body = build_workitems_search_body(&args);

    assert_eq!(body["category"], "Req,Task,Bug");
    assert_eq!(body["spaceId"], "proj-1");
    assert_eq!(body["page"], 1);
    assert_eq!(body["perPage"], 20);
    assert!(body.get("conditions").is_none());
}

#[test]
fn build_search_body_joins_categories_and_includes_conditions() {
    let args = WiSearchArgs {
        space_id: "proj-1".to_string(),
        category: vec!["Req".to_string(), "Task".to_string()],
        keyword: Some("login".to_string()),
        serial_number: Some("MMCL-123".to_string()),
        version_id: Some("ver-1".to_string()),
        sprint_id: Some("sprint-1".to_string()),
        page_size: 50,
        page: 2,
    };

    let body = build_workitems_search_body(&args);

    assert_eq!(body["category"], "Req,Task");
    assert_eq!(body["spaceId"], "proj-1");
    assert_eq!(body["page"], 2);
    assert_eq!(body["perPage"], 50);

    let conditions = body["conditions"].as_str().unwrap();
    assert!(conditions.contains("\"fieldIdentifier\":\"subject\""));
    assert!(conditions.contains("\"fieldIdentifier\":\"serialNumber\""));
    assert!(conditions.contains("\"fieldIdentifier\":\"version\""));
    assert!(conditions.contains("\"fieldIdentifier\":\"sprint\""));
}

#[test]
fn fields_args_accept_space_id_and_project_id_alias() {
    #[derive(Parser)]
    struct TestCli {
        #[command(subcommand)]
        command: WorkitemsCmds,
    }

    let cli = TestCli::parse_from([
        "test",
        "fields",
        "--space-id",
        "space-1",
        "--type-id",
        "type-1",
    ]);
    let WorkitemsCmds::Fields(args) = cli.command else {
        panic!("expected fields command");
    };
    assert_eq!(resolve_fields_space_id(&args).unwrap(), "space-1");

    let cli = TestCli::parse_from([
        "test",
        "fields",
        "--project-id",
        "space-1",
        "--type-id",
        "type-1",
    ]);
    let WorkitemsCmds::Fields(args) = cli.command else {
        panic!("expected fields command");
    };
    assert_eq!(resolve_fields_space_id(&args).unwrap(), "space-1");
}

#[test]
fn fields_args_reject_conflicting_alias_values() {
    #[derive(Parser)]
    struct TestCli {
        #[command(subcommand)]
        command: WorkitemsCmds,
    }

    let cli = TestCli::parse_from([
        "test",
        "fields",
        "--space-id",
        "space-1",
        "--project-id",
        "space-2",
        "--type-id",
        "type-1",
    ]);
    let WorkitemsCmds::Fields(args) = cli.command else {
        panic!("expected fields command");
    };
    assert!(resolve_fields_space_id(&args).is_err());
}

#[test]
fn labels_cannot_be_set_through_dynamic_fields() {
    let fields = vec![("labels".to_string(), "ready".to_string())];
    let reserved = HashSet::new();
    let error = validate_dynamic_field_overrides(&fields, &reserved).unwrap_err();
    assert!(error.to_string().contains("Use --labels"));
}

#[test]
fn required_fields_ignore_values_with_defaults() {
    let configs = HashMap::from([
        (
            "assignedTo".to_string(),
            FieldConfig {
                field_id: "assignedTo".into(),
                field_identifier: "assignedTo".into(),
                field_name: "负责人".into(),
                field_format: "user".into(),
                required: true,
                default_value: None,
            },
        ),
        (
            "priority".to_string(),
            FieldConfig {
                field_id: "priority".into(),
                field_identifier: "priority".into(),
                field_name: "优先级".into(),
                field_format: "list".into(),
                required: true,
                default_value: Some(json!("medium")),
            },
        ),
    ]);
    assert!(validate_required_create_fields(&configs, &HashSet::new()).is_err());
    assert!(
        validate_required_create_fields(&configs, &HashSet::from(["assignedTo".to_string()]))
            .is_ok()
    );
}

#[test]
fn required_fields_match_field_identifiers() {
    let config = FieldConfig {
        field_id: "internal-field-id".into(),
        field_identifier: "customRequired".into(),
        field_name: "Required custom field".into(),
        field_format: "input".into(),
        required: true,
        default_value: None,
    };
    let configs = HashMap::from([(config.field_identifier.clone(), config)]);

    assert!(validate_required_create_fields(
        &configs,
        &HashSet::from(["customRequired".to_string()])
    )
    .is_ok());
    let error = validate_required_create_fields(&configs, &HashSet::new()).unwrap_err();
    let message = error.to_string();
    assert!(message.contains("fieldIdentifier: customRequired"));
    assert!(message.contains("id: internal-field-id"));
    assert!(message.contains("--field customRequired=<value>"));
}

#[test]
fn stable_detail_requires_requested_fields() {
    let labels = vec![ResolvedLabel {
        id: "abcdef0123456789abcdef0123".into(),
        name: "ready".into(),
    }];
    let detail = json!({
        "id": "work-1",
        "subject": "subject",
        "assignedTo": {"id": "user-1"},
        "labels": [{"id": "abcdef0123456789abcdef0123"}]
    });
    assert!(
        workitem_detail_mismatches(&detail, "work-1", "subject", Some("user-1"), &labels)
            .is_empty()
    );
}

fn create_args(space_id: String, type_id: String) -> WiCreateArgs {
    WiCreateArgs {
        space_id,
        type_id,
        subject: "Reliable work item".into(),
        assignee: None,
        sprint_id: None,
        priority: None,
        labels: Some("ready-for-agent".into()),
        description: None,
        description_file: None,
        description_format: DescriptionFormat::Markdown,
        wait_timeout: 1,
        fields: Vec::new(),
    }
}

#[tokio::test]
async fn create_returns_stable_detail_after_resolving_label_name() {
    let mut server = Server::new_async().await;
    let suffix = chrono::Utc::now().timestamp_nanos_opt().unwrap();
    let org_id = format!("org-{suffix}");
    let space_id = format!("space-{suffix}");
    let type_id = format!("type-{suffix}");
    let label_id = "e4995af971162ddb8faa7dc1a1";

    let fields = server
        .mock(
            "GET",
            format!(
                "/oapi/v1/projex/organizations/{org_id}/projects/{space_id}/workitemTypes/{type_id}/fields"
            )
            .as_str(),
        )
        .with_body(json!([{
            "id": "subject",
            "name": "Title",
            "format": "string",
            "required": true,
            "defaultValue": null
        }]).to_string())
        .create_async()
        .await;
    let label_list = server
        .mock(
            "GET",
            format!("/oapi/v1/projex/organizations/{org_id}/projects/{space_id}/labels").as_str(),
        )
        .with_body(json!([{"id": label_id, "name": "ready-for-agent"}]).to_string())
        .create_async()
        .await;
    let create = server
        .mock(
            "POST",
            format!("/oapi/v1/projex/organizations/{org_id}/workitems").as_str(),
        )
        .match_body(Matcher::PartialJson(json!({
            "subject": "Reliable work item",
            "spaceId": space_id,
            "workitemTypeId": type_id,
            "labels": [label_id]
        })))
        .with_body(json!({"id": "work-1"}).to_string())
        .create_async()
        .await;
    let detail = server
        .mock(
            "GET",
            format!("/oapi/v1/projex/organizations/{org_id}/workitems/work-1").as_str(),
        )
        .with_body(
            json!({
                "id": "work-1",
                "subject": "Reliable work item",
                "labels": [{"id": label_id, "name": "ready-for-agent"}]
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = ApiClient::new("token", &server.url(), 5).unwrap();
    let args = create_args(space_id, type_id);
    let result = create_workitem(&args, &client, &org_id).await.unwrap();

    assert_eq!(result["id"], "work-1");
    fields.assert_async().await;
    label_list.assert_async().await;
    create.assert_async().await;
    detail.assert_async().await;
}

#[tokio::test]
async fn create_with_zero_wait_timeout_returns_the_raw_response() {
    let mut server = Server::new_async().await;
    let suffix = chrono::Utc::now().timestamp_nanos_opt().unwrap();
    let org_id = format!("org-raw-{suffix}");
    let space_id = format!("space-raw-{suffix}");
    let type_id = format!("type-raw-{suffix}");

    let fields = server
        .mock(
            "GET",
            format!(
                "/oapi/v1/projex/organizations/{org_id}/projects/{space_id}/workitemTypes/{type_id}/fields"
            )
            .as_str(),
        )
        .with_body(json!([]).to_string())
        .create_async()
        .await;
    let create = server
        .mock(
            "POST",
            format!("/oapi/v1/projex/organizations/{org_id}/workitems").as_str(),
        )
        .with_body(json!({"id": "work-raw", "queued": true}).to_string())
        .create_async()
        .await;
    let detail = server
        .mock(
            "GET",
            format!("/oapi/v1/projex/organizations/{org_id}/workitems/work-raw").as_str(),
        )
        .expect(0)
        .create_async()
        .await;

    let client = ApiClient::new("token", &server.url(), 5).unwrap();
    let mut args = create_args(space_id, type_id);
    args.labels = None;
    args.wait_timeout = 0;
    let result = create_workitem(&args, &client, &org_id).await.unwrap();

    assert_eq!(result, json!({"id": "work-raw", "queued": true}));
    fields.assert_async().await;
    create.assert_async().await;
    detail.assert_async().await;
}

#[tokio::test]
async fn stable_wait_retries_a_not_found_detail_once() {
    let mut server = Server::new_async().await;
    let suffix = chrono::Utc::now().timestamp_nanos_opt().unwrap();
    let org_id = format!("org-404-{suffix}");
    let space_id = format!("space-404-{suffix}");
    let workitem_path = format!("/oapi/v1/projex/organizations/{org_id}/workitems/work-404");

    let not_found = server
        .mock("GET", workitem_path.as_str())
        .expect(1)
        .with_status(404)
        .with_body(json!({"errorCode": "NotFound", "errorMessage": "not ready"}).to_string())
        .create_async()
        .await;
    let detail = server
        .mock("GET", workitem_path.as_str())
        .expect(1)
        .with_body(json!({"id": "work-404", "subject": "Reliable work item"}).to_string())
        .create_async()
        .await;

    let client = ApiClient::new("token", &server.url(), 5).unwrap();
    let mut args = create_args(space_id, "type-404".into());
    args.labels = None;
    args.wait_timeout = 1;
    let result = wait_for_stable_workitem(&client, &org_id, &args, "work-404", &[], Vec::new())
        .await
        .unwrap();

    assert_eq!(result["id"], "work-404");
    not_found.assert_async().await;
    detail.assert_async().await;
}

#[tokio::test]
async fn create_repairs_missing_labels_on_the_known_work_item() {
    let mut server = Server::new_async().await;
    let suffix = chrono::Utc::now().timestamp_nanos_opt().unwrap();
    let org_id = format!("org-repair-{suffix}");
    let space_id = format!("space-repair-{suffix}");
    let type_id = format!("type-repair-{suffix}");
    let label_id = "e4995af971162ddb8faa7dc1a1";

    let fields = server
        .mock(
            "GET",
            format!(
                "/oapi/v1/projex/organizations/{org_id}/projects/{space_id}/workitemTypes/{type_id}/fields"
            )
            .as_str(),
        )
        .with_body(json!([]).to_string())
        .create_async()
        .await;
    let label_list = server
        .mock(
            "GET",
            format!("/oapi/v1/projex/organizations/{org_id}/projects/{space_id}/labels").as_str(),
        )
        .with_body(json!([{"id": label_id, "name": "ready-for-agent"}]).to_string())
        .create_async()
        .await;
    let create = server
        .mock(
            "POST",
            format!("/oapi/v1/projex/organizations/{org_id}/workitems").as_str(),
        )
        .with_body(json!({"id": "work-2"}).to_string())
        .create_async()
        .await;
    let missing_detail = server
        .mock(
            "GET",
            format!("/oapi/v1/projex/organizations/{org_id}/workitems/work-2").as_str(),
        )
        .expect(1)
        .with_body(
            json!({"id": "work-2", "subject": "Reliable work item", "labels": null}).to_string(),
        )
        .create_async()
        .await;
    let repaired = server
        .mock(
            "PUT",
            format!("/oapi/v1/projex/organizations/{org_id}/workitems/work-2").as_str(),
        )
        .match_body(Matcher::Json(json!({"labels": [label_id]})))
        .with_body(json!({"status": "ok"}).to_string())
        .create_async()
        .await;
    let stable_detail = server
        .mock(
            "GET",
            format!("/oapi/v1/projex/organizations/{org_id}/workitems/work-2").as_str(),
        )
        .expect(1)
        .with_body(
            json!({
                "id": "work-2",
                "subject": "Reliable work item",
                "labels": [{"id": label_id, "name": "ready-for-agent"}]
            })
            .to_string(),
        )
        .create_async()
        .await;

    let client = ApiClient::new("token", &server.url(), 5).unwrap();
    let args = create_args(space_id, type_id);
    let result = create_workitem(&args, &client, &org_id).await.unwrap();

    assert_eq!(result["id"], "work-2");
    fields.assert_async().await;
    label_list.assert_async().await;
    create.assert_async().await;
    missing_detail.assert_async().await;
    repaired.assert_async().await;
    stable_detail.assert_async().await;
}

#[tokio::test]
async fn update_reloads_labels_once_after_a_label_validation_error() {
    let mut server = Server::new_async().await;
    let suffix = chrono::Utc::now().timestamp_nanos_opt().unwrap();
    let org_id = format!("org-update-{suffix}");
    let space_id = format!("space-update-{suffix}");
    let old_label_id = "e4995af971162ddb8faa7dc1a1";
    let refreshed_label_id = "a9df0f9f3937055a2f6d77e198";
    let labels_path = format!("/oapi/v1/projex/organizations/{org_id}/projects/{space_id}/labels");
    let workitem_path = format!("/oapi/v1/projex/organizations/{org_id}/workitems/work-3");

    let initial_labels = server
        .mock("GET", labels_path.as_str())
        .expect(1)
        .with_body(json!([{"id": old_label_id, "name": "ready-for-agent"}]).to_string())
        .create_async()
        .await;
    let initial_update = server
        .mock("PUT", workitem_path.as_str())
        .expect(1)
        .match_body(Matcher::Json(json!({"labels": [old_label_id]})))
        .with_status(400)
        .with_body(
            json!({
                "errorCode": "InvaildData.Failed",
                "errorMessage": "标签未找到"
            })
            .to_string(),
        )
        .create_async()
        .await;
    let refreshed_labels = server
        .mock("GET", labels_path.as_str())
        .expect(1)
        .with_body(json!([{"id": refreshed_label_id, "name": "ready-for-agent"}]).to_string())
        .create_async()
        .await;
    let retried_update = server
        .mock("PUT", workitem_path.as_str())
        .expect(1)
        .match_body(Matcher::Json(json!({"labels": [refreshed_label_id]})))
        .with_body(json!({"status": "ok"}).to_string())
        .create_async()
        .await;

    let args = WiUpdateArgs {
        space_id,
        workitem_id: "work-3".into(),
        type_id: None,
        subject: None,
        assignee: None,
        status: None,
        priority: None,
        labels: Some("ready-for-agent".into()),
        description: None,
        description_file: None,
        description_format: None,
        fields: Vec::new(),
    };
    let client = ApiClient::new("token", &server.url(), 5).unwrap();
    let result = update_workitem(&args, &client, &org_id).await.unwrap();

    assert_eq!(result["status"], "ok");
    initial_labels.assert_async().await;
    initial_update.assert_async().await;
    refreshed_labels.assert_async().await;
    retried_update.assert_async().await;
}
