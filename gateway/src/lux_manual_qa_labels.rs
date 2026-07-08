use std::path::Path;

use crate::lux_manual_qa_types::{ManualQaCapabilityStatus, ManualQaPhase};

pub(crate) const fn phase_label(phase: ManualQaPhase) -> &'static str {
    match phase {
        ManualQaPhase::Compile => "compile",
        ManualQaPhase::Test => "test",
        ManualQaPhase::DynamicCode => "dynamic_code",
        ManualQaPhase::Screenshot => "screenshot",
    }
}

pub(crate) const fn phase_requires_screenshot_path(phase: ManualQaPhase) -> bool {
    matches!(phase, ManualQaPhase::Screenshot)
}

pub(crate) const fn capability_blocks(status: ManualQaCapabilityStatus) -> bool {
    matches!(status, ManualQaCapabilityStatus::Blocker)
}

pub(crate) fn evidence_label(label: &str) -> String {
    label
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
                character
            } else {
                '-'
            }
        })
        .collect()
}

pub(crate) fn path_to_slash(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

pub(crate) fn screenshot_path_from_stdout(stdout: &str) -> Option<String> {
    stdout.lines().find_map(|line| {
        line.strip_prefix("screenshot_path=")
            .map(str::trim)
            .filter(|path| !path.is_empty())
            .map(ToOwned::to_owned)
    })
}
