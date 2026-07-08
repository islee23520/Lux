use std::{
    fs,
    path::{Component, Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use serde::Deserialize;
use serde_json::{json, Value};

const PROTOCOL: &str = "lux.unity_selection_context.v1";
const LATEST_REL: &str = ".lux/context/selection-context.json";
const EVENTS_REL: &str = ".lux/context/selection-context-events.jsonl";

#[derive(Debug, Deserialize)]
struct RegisterArgs {
    #[serde(default)]
    project_path: Option<PathBuf>,
    context: Value,
}

#[derive(Debug, Deserialize)]
struct LatestArgs {
    #[serde(default)]
    project_path: Option<PathBuf>,
}

pub(super) fn register(arguments: &Value, default_project_path: Option<&Path>) -> Result<Value> {
    let args: RegisterArgs = serde_json::from_value(arguments.clone())
        .context("invalid Unity selection context register arguments")?;
    validate_context(&args.context)?;

    let project_path = resolve_project_path(args.project_path.as_deref(), default_project_path)?;
    let latest_context_path = latest_path(&project_path);
    let events_path = events_path(&project_path);
    crate::lux_io::atomic_write_json(&latest_context_path, &args.context).with_context(|| {
        format!(
            "failed to write latest Unity selection context {}",
            latest_context_path.display()
        )
    })?;

    let event = json!({
        "schemaVersion": 1,
        "eventType": "unity.selection_context_registered",
        "protocol": PROTOCOL,
        "capturedAtUtc": Utc::now().to_rfc3339(),
        "projectPath": project_path,
        "latestContextPath": latest_context_path,
    });
    crate::lux_io::append_jsonl(&events_path, &event).with_context(|| {
        format!(
            "failed to append Unity selection context event {}",
            events_path.display()
        )
    })?;

    Ok(json!({
        "ok": true,
        "protocol": PROTOCOL,
        "projectPath": project_path,
        "latestContextPath": latest_context_path,
        "eventsPath": events_path,
        "context": args.context,
        "message": "Unity selection context registered"
    }))
}

pub(super) fn latest(arguments: &Value, default_project_path: Option<&Path>) -> Result<Value> {
    let input = if arguments.is_null() {
        json!({})
    } else {
        arguments.clone()
    };
    let args: LatestArgs = serde_json::from_value(input)
        .context("invalid Unity selection context latest arguments")?;
    let project_path = resolve_project_path(args.project_path.as_deref(), default_project_path)?;
    let latest_context_path = latest_path(&project_path);
    let events_path = events_path(&project_path);

    if !latest_context_path.is_file() {
        return Ok(json!({
            "ok": false,
            "protocol": PROTOCOL,
            "projectPath": project_path,
            "latestContextPath": latest_context_path,
            "eventsPath": events_path,
            "stopReason": "unity_selection_context_missing",
            "message": format!("Latest Unity selection context is missing: {}", latest_context_path.display())
        }));
    }

    let context: Value = serde_json::from_str(
        &fs::read_to_string(&latest_context_path)
            .with_context(|| format!("failed to read {}", latest_context_path.display()))?,
    )
    .with_context(|| format!("failed to parse {}", latest_context_path.display()))?;

    Ok(json!({
        "ok": true,
        "protocol": PROTOCOL,
        "projectPath": project_path,
        "latestContextPath": latest_context_path,
        "eventsPath": events_path,
        "context": context,
        "message": "Unity selection context loaded"
    }))
}

fn resolve_project_path(
    argument_project_path: Option<&Path>,
    default_project_path: Option<&Path>,
) -> Result<PathBuf> {
    let project_path = argument_project_path
        .or(default_project_path)
        .ok_or_else(|| anyhow!("project_path is required"))?;
    reject_unsafe_project_path(project_path)?;
    if !project_path.exists() {
        anyhow::bail!("project_path does not exist: {}", project_path.display());
    }
    if !project_path.is_dir() {
        anyhow::bail!(
            "project_path is not a directory: {}",
            project_path.display()
        );
    }
    Ok(project_path.to_path_buf())
}

fn reject_unsafe_project_path(project_path: &Path) -> Result<()> {
    if !project_path.is_absolute() {
        anyhow::bail!(
            "project_path must be an absolute project root: {}",
            project_path.display()
        );
    }
    if project_path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        anyhow::bail!(
            "project_path must not contain parent directory components: {}",
            project_path.display()
        );
    }
    Ok(())
}

fn validate_context(context: &Value) -> Result<()> {
    let object = context
        .as_object()
        .ok_or_else(|| anyhow!("context must be a JSON object"))?;
    if !object.contains_key("schemaVersion") {
        anyhow::bail!("context.schemaVersion is required");
    }
    object
        .get("summary")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|summary| !summary.is_empty())
        .ok_or_else(|| anyhow!("context.summary must be a non-empty string"))?;
    Ok(())
}

fn latest_path(project_path: &Path) -> PathBuf {
    project_path.join(LATEST_REL)
}

fn events_path(project_path: &Path) -> PathBuf {
    project_path.join(EVENTS_REL)
}

#[cfg(test)]
#[path = "lux_mcp_selection_context_tests.rs"]
mod tests;
