use super::*;

fn valid_context(summary: &str) -> Value {
    json!({
        "schemaVersion": 1,
        "summary": summary,
        "selection": { "gameObjectName": "Player" }
    })
}

#[test]
fn register_writes_latest_and_jsonl_event() {
    let temp = tempfile::tempdir().expect("temp project");
    let project_path = temp.path();
    let payload_internal_path = temp.path().join("payload-controlled");
    let context = json!({
        "schemaVersion": 1,
        "summary": "Player selected",
        "selection": {
            "gameObjectName": "Player",
            "hierarchyPath": "Scene/Player"
        },
        "scenePath": payload_internal_path.join("scene-context.json"),
        "assetPath": payload_internal_path.join("asset-context.json"),
        "prefabPath": payload_internal_path.join("prefab-context.json"),
        "path": payload_internal_path.join("generic-context.json")
    });

    let registered = register(&json!({ "context": context.clone() }), Some(project_path))
        .expect("register selection context");

    assert_eq!(registered["ok"], true);
    assert_eq!(registered["context"], context);
    assert_eq!(
        registered["latestContextPath"],
        json!(latest_path(project_path))
    );
    assert_eq!(registered["eventsPath"], json!(events_path(project_path)));

    let latest: Value = serde_json::from_str(
        &fs::read_to_string(latest_path(project_path)).expect("latest context JSON"),
    )
    .expect("latest context");
    assert_eq!(latest, context);

    let events_text =
        fs::read_to_string(events_path(project_path)).expect("selection context events");
    let events = events_text.lines().collect::<Vec<_>>();
    assert_eq!(events.len(), 1);
    let event: Value = serde_json::from_str(events[0]).expect("selection context event");
    assert_eq!(event["eventType"], "unity.selection_context_registered");
    assert_eq!(event["protocol"], PROTOCOL);
    assert_eq!(event["latestContextPath"], json!(latest_path(project_path)));

    assert!(!project_path.join(".lux/selection-context.json").exists());
    assert!(!project_path
        .join(".lux/selection-context-events.jsonl")
        .exists());
    assert!(!payload_internal_path.exists());
}

#[test]
fn latest_reads_registered_context_without_appending_events() {
    let temp = tempfile::tempdir().expect("temp project");
    let project_path = temp.path();
    let context = valid_context("Enemy selected");
    register(&json!({ "context": context.clone() }), Some(project_path))
        .expect("register selection context");
    let event_count_after_register = fs::read_to_string(events_path(project_path))
        .expect("selection context events")
        .lines()
        .count();

    let first_latest = latest(&Value::Null, Some(project_path)).expect("first latest context");
    let second_latest = latest(&Value::Null, Some(project_path)).expect("second latest context");

    assert_eq!(first_latest["ok"], true);
    assert_eq!(first_latest["context"], context);
    assert_eq!(second_latest["ok"], true);
    assert_eq!(second_latest["context"], context);
    assert_eq!(
        fs::read_to_string(events_path(project_path))
            .expect("selection context events")
            .lines()
            .count(),
        event_count_after_register
    );
}

#[test]
fn latest_missing_returns_mcp_error_result_shape() {
    let temp = tempfile::tempdir().expect("temp project");
    let project_path = temp.path();

    let result = latest(&Value::Null, Some(project_path)).expect("latest tool result");

    assert_eq!(result["ok"], false);
    assert_eq!(result["stopReason"], "unity_selection_context_missing");
    assert!(result["message"]
        .as_str()
        .expect("missing message")
        .contains("Latest Unity selection context is missing"));
    assert_eq!(
        result["latestContextPath"],
        json!(latest_path(project_path))
    );
    assert_eq!(result["eventsPath"], json!(events_path(project_path)));
}

#[test]
fn register_rejects_invalid_payload_without_writing() {
    let temp = tempfile::tempdir().expect("temp project");
    let project_path = temp.path();

    let error = register(&json!({ "context": "not-an-object" }), Some(project_path))
        .expect_err("invalid payload should fail");

    assert!(error.to_string().contains("context must be a JSON object"));
    assert!(!latest_path(project_path).exists());
    assert!(!events_path(project_path).exists());
}

#[test]
fn register_rejects_malformed_context_contract() {
    let temp = tempfile::tempdir().expect("temp project");
    let project_path = temp.path();

    for context in [
        json!({ "summary": "Missing schema version" }),
        json!({ "schemaVersion": 1 }),
        json!({ "schemaVersion": 1, "summary": "" }),
        json!({ "schemaVersion": 1, "summary": "   " }),
        json!({ "schemaVersion": 1, "summary": 42 }),
    ] {
        assert!(register(&json!({ "context": context }), Some(project_path)).is_err());
    }

    assert!(!latest_path(project_path).exists());
    assert!(!events_path(project_path).exists());
}

#[test]
fn register_rejects_relative_project_path_without_writing_default_or_outside() {
    let default_temp = tempfile::tempdir().expect("default project");
    let outside_temp = tempfile::tempdir().expect("outside project");
    let context = valid_context("Relative project rejected");

    let error = register(
        &json!({
            "project_path": "../outside-project",
            "context": context
        }),
        Some(default_temp.path()),
    )
    .expect_err("relative project_path should fail");

    assert!(error
        .to_string()
        .contains("project_path must be an absolute project root"));
    assert!(!latest_path(default_temp.path()).exists());
    assert!(!events_path(default_temp.path()).exists());
    assert!(!latest_path(outside_temp.path()).exists());
    assert!(!events_path(outside_temp.path()).exists());
}

#[test]
fn register_rejects_parent_component_project_path_without_writing_outside() {
    let base_temp = tempfile::tempdir().expect("base project");
    let inside_project = base_temp.path().join("inside");
    let outside_project = base_temp.path().join("outside");
    fs::create_dir_all(&inside_project).expect("inside project");
    fs::create_dir_all(&outside_project).expect("outside project");
    let traversal_path = inside_project.join("..").join("outside");
    let context = valid_context("Traversal project rejected");

    let error = register(
        &json!({
            "project_path": traversal_path,
            "context": context
        }),
        Some(&inside_project),
    )
    .expect_err("parent-component project_path should fail");

    assert!(error
        .to_string()
        .contains("project_path must not contain parent directory components"));
    assert!(!latest_path(&inside_project).exists());
    assert!(!events_path(&inside_project).exists());
    assert!(!latest_path(&outside_project).exists());
    assert!(!events_path(&outside_project).exists());
}
