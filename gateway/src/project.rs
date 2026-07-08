use std::path::{Path, PathBuf};

use anyhow::Result;

pub use lux_project::{
    detect_from_cwd, detect_from_path, detect_unity_project, DetectedPackage, ProjectInfo,
    UnityProjectDetection,
};

use crate::cross_platform;
use crate::lux_engines::{self, EngineCapabilityRecord};
use lux_project::EngineKind;

#[derive(Clone, Debug, PartialEq)]
pub struct EngineCapabilityInventory {
    pub path: PathBuf,
    pub engines: Vec<EngineCapabilityRecord>,
}

pub fn detect_engine_capabilities(project_root: &Path) -> Result<EngineCapabilityInventory> {
    let active_engine = active_engine_for_project(project_root);
    let snapshot = lux_engines::write_engine_capability_snapshot(project_root, active_engine)?;
    Ok(EngineCapabilityInventory {
        path: project_root.join(".lux/engines/capabilities.json"),
        engines: snapshot.engines,
    })
}

pub fn resolve_project_root(project_path: &Option<PathBuf>) -> anyhow::Result<PathBuf> {
    match project_path {
        Some(path) => Ok(cross_platform::normalize_path_buf(path.clone())),
        None => {
            let cwd = cross_platform::normalize_path_buf(std::env::current_dir()?);
            if is_unity_project(&cwd) {
                return Ok(cwd);
            }
            if let Some(parent_project) = find_unity_project_root(cwd.clone()) {
                return Ok(parent_project);
            }
            let nested = find_nested_unity_projects(&cwd, 3);
            match nested.len() {
                0 => anyhow::bail!(
                    "Unity project not found in {} or subdirectories. Use --project-path.",
                    cwd.display()
                ),
                1 => {
                    eprintln!("Found Unity project: {}", nested[0].display());
                    Ok(nested[0].clone())
                }
                _ => {
                    eprintln!("Multiple Unity projects found:");
                    for (i, p) in nested.iter().enumerate() {
                        let rel = p.strip_prefix(&cwd).unwrap_or(p).display();
                        eprintln!("  [{i}] {rel}");
                    }
                    eprint!("Select project [0-{}]: ", nested.len() - 1);
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).ok();
                    let idx: usize = input.trim().parse().unwrap_or(0);
                    Ok(nested
                        .get(idx)
                        .cloned()
                        .unwrap_or_else(|| nested[0].clone()))
                }
            }
        }
    }
}

pub fn is_unity_project(path: &Path) -> bool {
    path.join("Assets").is_dir() && path.join("ProjectSettings").is_dir()
}

pub fn find_nested_unity_projects(start: &Path, max_depth: usize) -> Vec<PathBuf> {
    let mut results = Vec::new();
    let skip_dirs = [
        "node_modules",
        "Library",
        "Temp",
        "target",
        ".git",
        "Obj",
        "Build",
        "Builds",
    ];
    fn search(
        current: &Path,
        depth: usize,
        max_depth: usize,
        skip: &[&str],
        results: &mut Vec<PathBuf>,
    ) {
        if depth > max_depth {
            return;
        }
        if is_unity_project(current) {
            results.push(current.to_path_buf());
            return;
        }
        if let Ok(entries) = std::fs::read_dir(current) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if skip.contains(&name_str.as_ref()) {
                    continue;
                }
                let path = entry.path();
                if path.is_dir() {
                    search(&path, depth + 1, max_depth, skip, results);
                }
            }
        }
    }
    search(start, 0, max_depth, &skip_dirs, &mut results);
    results
}

pub fn find_unity_project_root(mut current: PathBuf) -> Option<PathBuf> {
    loop {
        if is_unity_project(&current) {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}

fn active_engine_for_project(project_root: &Path) -> EngineKind {
    if detect_unity_project(project_root).ok().flatten().is_some() {
        return EngineKind::Unity;
    }
    EngineKind::Unity
}
