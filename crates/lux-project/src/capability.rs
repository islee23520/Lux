use std::{
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngineKind {
    Unity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityStatus {
    Verified,
    Partial,
    Planned,
    Unsupported,
}

impl CapabilityStatus {
    pub const fn blocks_completion(self) -> bool {
        !matches!(self, Self::Verified | Self::Partial)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngineCapabilityStatus {
    Detected,
    Limited,
    Unsupported,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseCapabilityError {
    kind: &'static str,
    value: String,
}

impl ParseCapabilityError {
    fn new(kind: &'static str, value: &str) -> Self {
        Self {
            kind,
            value: value.to_string(),
        }
    }
}

impl fmt::Display for ParseCapabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unknown {} '{}'", self.kind, self.value)
    }
}

impl Error for ParseCapabilityError {}

impl FromStr for EngineKind {
    type Err = ParseCapabilityError;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        match value {
            "unity" => Ok(Self::Unity),
            _ => Err(ParseCapabilityError::new("engine", value)),
        }
    }
}

impl FromStr for CapabilityStatus {
    type Err = ParseCapabilityError;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        match value {
            "verified" => Ok(Self::Verified),
            "partial" => Ok(Self::Partial),
            "planned" => Ok(Self::Planned),
            "unsupported" => Ok(Self::Unsupported),
            _ => Err(ParseCapabilityError::new("capability status", value)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EngineCapability {
    pub engine: EngineKind,
    pub capability: String,
    pub status: CapabilityStatus,
    pub reason: String,
}

impl EngineCapability {
    pub fn new(
        engine: EngineKind,
        capability: impl Into<String>,
        status: CapabilityStatus,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            engine,
            capability: capability.into(),
            status,
            reason: reason.into(),
        }
    }

    pub const fn blocks_completion(&self) -> bool {
        self.status.blocks_completion()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EngineCapabilityBlocker {
    pub engine: EngineKind,
    pub capability: String,
    pub status: CapabilityStatus,
    pub reason: String,
    pub evidence_path: String,
    pub recommended_next_supported_action: String,
}

impl EngineCapabilityBlocker {
    pub fn new(
        engine: EngineKind,
        capability: impl Into<String>,
        status: CapabilityStatus,
        reason: impl Into<String>,
        evidence_path: impl Into<String>,
        recommended_next_supported_action: impl Into<String>,
    ) -> Self {
        Self {
            engine,
            capability: capability.into(),
            status,
            reason: reason.into(),
            evidence_path: evidence_path.into(),
            recommended_next_supported_action: recommended_next_supported_action.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EngineCapabilityRecord {
    pub engine: EngineKind,
    pub status: EngineCapabilityStatus,
    pub reason: String,
    pub detected: bool,
    pub tool_available: bool,
    pub manual_qa_supported: bool,
    pub screenshot_supported: bool,
    pub video_supported: bool,
    pub blocker_reason: Option<String>,
}

impl EngineCapabilityRecord {
    fn detected(
        engine: EngineKind,
        reason: impl Into<String>,
        tool_available: bool,
        manual_qa_supported: bool,
        screenshot_supported: bool,
        video_supported: bool,
    ) -> Self {
        Self {
            engine,
            status: EngineCapabilityStatus::Detected,
            reason: reason.into(),
            detected: true,
            tool_available,
            manual_qa_supported,
            screenshot_supported,
            video_supported,
            blocker_reason: None,
        }
    }

    fn unsupported(
        engine: EngineKind,
        reason: impl Into<String>,
        blocker_reason: impl Into<String>,
    ) -> Self {
        Self {
            engine,
            status: EngineCapabilityStatus::Unsupported,
            reason: reason.into(),
            detected: false,
            tool_available: false,
            manual_qa_supported: false,
            screenshot_supported: false,
            video_supported: false,
            blocker_reason: Some(blocker_reason.into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EngineCapabilityCatalog {
    pub schema_version: u32,
    pub engine: EngineKind,
    pub status: EngineCapabilityStatus,
    pub reason: String,
    pub unity: EngineCapabilityRecord,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EngineCapabilityInventory {
    pub path: PathBuf,
    pub engines: Vec<EngineCapabilityRecord>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
struct EngineCapabilitySnapshot {
    pub engine: EngineKind,
    pub status: &'static str,
    pub capabilities: Vec<EngineCapability>,
    pub engines: Vec<EngineCapabilityRecord>,
}

pub const ENGINE_CAPABILITY_SCHEMA_VERSION: u32 = 1;

pub fn recommended_capability_blockers(engine: Option<EngineKind>) -> Vec<EngineCapabilityBlocker> {
    match engine {
        Some(EngineKind::Unity) | None => Vec::new(),
    }
}

pub fn persist_engine_capabilities(
    project_root: &Path,
    active_engine: EngineKind,
) -> Result<EngineCapabilityCatalog> {
    let catalog = build_catalog(project_root, active_engine)?;
    write_json(&capabilities_path(project_root), &catalog)?;
    Ok(catalog)
}

pub fn detect_engine_capabilities(project_root: &Path) -> Result<EngineCapabilityInventory> {
    let catalog = build_catalog(project_root, EngineKind::Unity)?;
    let engines = vec![catalog.unity.clone()];
    write_json(
        &capabilities_path(project_root),
        &serde_json::json!({
            "schema_version": ENGINE_CAPABILITY_SCHEMA_VERSION,
            "engines": engines,
        }),
    )?;
    Ok(EngineCapabilityInventory {
        path: capabilities_path(project_root),
        engines,
    })
}

pub fn persist_engine_status_snapshot(project_root: &Path, engine: EngineKind) -> Result<()> {
    let catalog = build_catalog(project_root, engine)?;
    let active = match engine {
        EngineKind::Unity => &catalog.unity,
    };
    let snapshot = EngineCapabilitySnapshot {
        engine,
        status: if active.detected {
            "supported"
        } else {
            "unsupported"
        },
        capabilities: snapshot_capabilities(engine, active.detected),
        engines: vec![catalog.unity],
    };
    write_json(&capabilities_path(project_root), &snapshot)
}

fn snapshot_capabilities(engine: EngineKind, detected: bool) -> Vec<EngineCapability> {
    match (engine, detected) {
        (EngineKind::Unity, true) => vec![EngineCapability::new(
            EngineKind::Unity,
            "project_detection",
            CapabilityStatus::Verified,
            "Unity project markers are detected locally",
        )],
        (EngineKind::Unity, false) => vec![EngineCapability::new(
            EngineKind::Unity,
            "project_detection",
            CapabilityStatus::Unsupported,
            "Unity project markers were not found at the requested path",
        )],
    }
}

fn build_catalog(
    project_root: &Path,
    active_engine: EngineKind,
) -> Result<EngineCapabilityCatalog> {
    let unity = if project_root
        .join("ProjectSettings/ProjectVersion.txt")
        .is_file()
    {
        EngineCapabilityRecord::detected(
            EngineKind::Unity,
            "Unity markers found in ProjectSettings/ProjectVersion.txt",
            true,
            true,
            true,
            false,
        )
    } else {
        EngineCapabilityRecord::unsupported(
            EngineKind::Unity,
            "Unity markers not found",
            format!(
                "Missing {}",
                project_root
                    .join("ProjectSettings/ProjectVersion.txt")
                    .display()
            ),
        )
    };
    let active = match active_engine {
        EngineKind::Unity => &unity,
    };
    Ok(EngineCapabilityCatalog {
        schema_version: ENGINE_CAPABILITY_SCHEMA_VERSION,
        engine: active_engine,
        status: active.status,
        reason: active.reason.clone(),
        unity,
    })
}

fn capabilities_path(project_root: &Path) -> PathBuf {
    project_root.join(".lux/engines/capabilities.json")
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create parent directory {}", parent.display()))?;
    }
    let content = serde_json::to_string_pretty(value)
        .context("failed to serialize engine capability payload")?;
    let tmp_path = path.with_extension("json.tmp");
    fs::write(&tmp_path, content)
        .with_context(|| format!("failed to write temp file {}", tmp_path.display()))?;
    fs::rename(&tmp_path, path)
        .with_context(|| format!("failed to atomically replace file {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::{
        detect_engine_capabilities, persist_engine_capabilities, recommended_capability_blockers,
        CapabilityStatus, EngineCapability, EngineCapabilityStatus, EngineKind,
        ENGINE_CAPABILITY_SCHEMA_VERSION,
    };
    use std::{fs, str::FromStr};

    #[test]
    fn capability_status_levels() {
        let capabilities = [
            EngineCapability::new(
                EngineKind::Unity,
                "project detection",
                CapabilityStatus::Verified,
                "Unity project markers are detected locally",
            ),
            EngineCapability::new(
                EngineKind::Unity,
                "project detection",
                CapabilityStatus::Unsupported,
                "Unity project markers are absent",
            ),
        ];

        assert_eq!(capabilities[0].status, CapabilityStatus::Verified);
        assert_eq!(capabilities[1].status, CapabilityStatus::Unsupported);
        assert!(capabilities[1].blocks_completion());
    }

    #[test]
    fn invalid_capability_status_is_rejected() {
        let error = CapabilityStatus::from_str("certified").expect_err("status should be rejected");
        assert_eq!(error.to_string(), "unknown capability status 'certified'");
    }

    #[test]
    fn capability_blockers_are_empty_for_unity_only_surface() {
        assert!(recommended_capability_blockers(None).is_empty());
        assert!(recommended_capability_blockers(Some(EngineKind::Unity)).is_empty());
    }

    #[test]
    fn persist_engine_capabilities_writes_unity_only_catalog() {
        let temp = tempfile::tempdir().expect("tempdir should be created");
        let project_root = temp.path();
        fs::create_dir_all(project_root.join("ProjectSettings")).expect("create unity marker dir");
        fs::write(
            project_root.join("ProjectSettings/ProjectVersion.txt"),
            "m_EditorVersion: 6000.0.0f1\n",
        )
        .expect("write unity marker");

        let catalog = persist_engine_capabilities(project_root, EngineKind::Unity)
            .expect("capabilities should persist");
        let payload: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(project_root.join(".lux/engines/capabilities.json"))
                .expect("persisted capabilities json should be readable"),
        )
        .expect("persisted capabilities json should parse");

        assert_eq!(catalog.schema_version, ENGINE_CAPABILITY_SCHEMA_VERSION);
        assert_eq!(payload["engine"], "unity");
        assert_eq!(payload["status"], "detected");
        assert_eq!(payload["unity"]["status"], "detected");
        assert!(payload.get("godot").is_none());
        assert!(payload.get("three_js").is_none());
        assert_eq!(
            payload["unity"]["reason"],
            "Unity markers found in ProjectSettings/ProjectVersion.txt"
        );
    }

    #[test]
    fn detect_engine_capabilities_writes_inventory_file() {
        let temp = tempfile::tempdir().expect("tempdir should be created");
        fs::create_dir_all(temp.path().join("ProjectSettings")).expect("create unity marker dir");
        fs::write(
            temp.path().join("ProjectSettings/ProjectVersion.txt"),
            "m_EditorVersion: 6000.0.0f1\n",
        )
        .expect("write unity marker");

        let inventory = detect_engine_capabilities(temp.path()).expect("inventory should persist");
        let payload: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(&inventory.path).expect("persisted inventory should be readable"),
        )
        .expect("persisted inventory should parse");

        assert_eq!(
            payload["engines"].as_array().map(std::vec::Vec::len),
            Some(1)
        );
        assert!(inventory
            .engines
            .iter()
            .any(|engine| engine.engine == EngineKind::Unity
                && engine.status == EngineCapabilityStatus::Detected));
    }
}
