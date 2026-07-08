# Engine Bridge Sources

## Overview
Bridge source files copied into target engine projects by LUX install commands. Unity is verified, Godot is partial, and Three.js is planned unless runtime harness evidence exists.

## Structure
```
bridge/
├── unity/      # Unity package metadata and Editor bridge source
├── godot/      # Godot adapter script for partial support
└── threejs/    # Planned Three.js adapter artifacts
```

## Where To Look
| Task | Location | Notes |
| --- | --- | --- |
| Unity Editor TCP server | `unity/AiBridgeEditor/UnityAiBridgeTcpServer.cs` | Connection, lifecycle, discovery, command dispatch |
| Unity protocol parser | `unity/AiBridgeEditor/UnityAiBridgeProtocol.cs` | Keep aligned with `crates/lux-bridge-core` |
| Unity auto-start | `unity/AiBridgeEditor/UnityAiBridgeBootstrap.cs` | Skips batch mode |
| Unity menu commands | `unity/AiBridgeEditor/UnityAiBridgeMenu.cs` | Tools/Linalab/Lux/AI Bridge |
| Unity compile smoke | `unity/AiBridgeEditor/LuxBatchAutomation.cs` | Writes JSON results under `TestResults/` |
| Unity scene smoke | `unity/AiBridgeEditor/LuxSceneSmoke.cs` | Uses `LUX_SCENE_SMOKE_*` env vars |
| Unity context export | `unity/AiBridgeEditor/LuxUnityContext.cs` | Writes `UserSettings/LuxUnityContext.json` |
| Godot adapter | `godot/bridge.gd` | Partial support only |
| Three.js adapter | `threejs/` | Planned support only |

## Conventions
- Source here is installed into a target project; do not assume the target project is inside this repo.
- Unity code must stay compatible with Unity 6000.0+.
- Bridge install must be idempotent.
- Protocol changes must update Rust and C# contracts together.
- Batch mode outputs JSON evidence; interactive Unity windows are not repository-owned product UI.

## Anti-Patterns
- Do not add Unity Editor window product surfaces such as Workbench or CodexImage.
- Do not present Godot or Three.js as Unity-equivalent maturity.
- Do not vendor target project state under `bridge/`.

## Verification
```bash
cargo test -p lux-bridge-core
./scripts/check-readme-bridge-contract.sh
```

Unity Editor tests run through Unity Test Runner in a target project: `LuxAiActionLogTests`, `LuxAiActionLogBroadcaster` tests, and all `*Tests/Editor/`.
