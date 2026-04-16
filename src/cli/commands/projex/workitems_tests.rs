use super::*;
use clap::Parser;

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
