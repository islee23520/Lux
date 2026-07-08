use std::fs;
use std::path::Path;
use std::process::Command;

/// Bridge install must copy only the DLL (not .cs files) when LUX_BRIDGE_PRECOMPILED_DLL is set.
#[test]
fn test_bridge_install_copies_dll_not_cs() {
    let precompiled_dll = match std::env::var("LUX_BRIDGE_PRECOMPILED_DLL") {
        Ok(path) if Path::new(&path).is_file() => path,
        _ => {
            eprintln!("skipping: LUX_BRIDGE_PRECOMPILED_DLL not set or not a file");
            return;
        }
    };

    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let project_root = temp_dir.path();

    // Create minimal Unity project skeleton
    fs::create_dir_all(project_root.join("Assets")).expect("create Assets");
    fs::create_dir_all(project_root.join("Packages")).expect("create Packages");
    fs::create_dir_all(project_root.join("ProjectSettings")).expect("create ProjectSettings");
    fs::write(
        project_root.join("ProjectSettings/ProjectVersion.txt"),
        "m_EditorVersion: 6000.0.75f1\n",
    )
    .expect("write ProjectVersion");

    let output = Command::new(env!("CARGO_BIN_EXE_lux"))
        .args([
            "bridge",
            "install",
            "--project-path",
            project_root.to_str().unwrap(),
            "--no-opencode-commands",
        ])
        .env("LUX_BRIDGE_PRECOMPILED_DLL", &precompiled_dll)
        .output()
        .expect("run lux bridge install");

    assert!(
        output.status.success(),
        "lux bridge install failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let dll_path = project_root.join("Assets/Editor/LuxBridge/Linalab.UnityAiBridge.Editor.dll");
    let meta_path =
        project_root.join("Assets/Editor/LuxBridge/Linalab.UnityAiBridge.Editor.dll.meta");

    assert!(
        dll_path.is_file(),
        "DLL not found at {}",
        dll_path.display()
    );
    assert!(
        meta_path.is_file(),
        "DLL .meta not found at {}",
        meta_path.display()
    );

    // Verify NO .cs files are installed
    let cs_files: Vec<_> = fs::read_dir(project_root.join("Assets/Editor/LuxBridge"))
        .expect("read LuxBridge dir")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "cs"))
        .collect();
    assert!(
        cs_files.is_empty(),
        "Found .cs files in LuxBridge — only DLL should be installed"
    );

    // Verify NO legacy AiBridgeEditor directory
    assert!(
        !project_root.join("Assets/Editor/AiBridgeEditor").exists(),
        "Legacy AiBridgeEditor directory should be removed"
    );
}

/// Bridge install must remove legacy AiBridgeEditor if it exists.
#[test]
fn test_bridge_install_removes_legacy_cs() {
    let precompiled_dll = match std::env::var("LUX_BRIDGE_PRECOMPILED_DLL") {
        Ok(path) if Path::new(&path).is_file() => path,
        _ => {
            eprintln!("skipping: LUX_BRIDGE_PRECOMPILED_DLL not set or not a file");
            return;
        }
    };

    let temp_dir = tempfile::tempdir().expect("create temp dir");
    let project_root = temp_dir.path();

    // Create minimal Unity project skeleton
    fs::create_dir_all(project_root.join("Assets/Editor")).expect("create Assets/Editor");
    fs::create_dir_all(project_root.join("Packages")).expect("create Packages");
    fs::create_dir_all(project_root.join("ProjectSettings")).expect("create ProjectSettings");
    fs::write(
        project_root.join("ProjectSettings/ProjectVersion.txt"),
        "m_EditorVersion: 6000.0.75f1\n",
    )
    .expect("write ProjectVersion");

    // Create legacy AiBridgeEditor directory with a dummy .cs file
    let legacy_dir = project_root.join("Assets/Editor/AiBridgeEditor");
    fs::create_dir_all(&legacy_dir).expect("create legacy dir");
    fs::write(legacy_dir.join("LegacyScript.cs"), "// legacy").expect("write legacy cs");

    let output = Command::new(env!("CARGO_BIN_EXE_lux"))
        .args([
            "bridge",
            "install",
            "--project-path",
            project_root.to_str().unwrap(),
            "--no-opencode-commands",
        ])
        .env("LUX_BRIDGE_PRECOMPILED_DLL", &precompiled_dll)
        .output()
        .expect("run lux bridge install");

    assert!(
        output.status.success(),
        "lux bridge install failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        !legacy_dir.exists(),
        "Legacy AiBridgeEditor directory should be removed after install"
    );
}
