# Unity Bridge Editor Tests

## Overview
Unity Editor/NUnit tests for the installed AI Bridge package. These tests validate the C# bridge behavior that Cargo cannot execute.

## Structure
```
AiBridgeTests/
└── Editor/   # NUnit editor tests and helper scopes
```

## Where To Look
| Task | Location | Notes |
| --- | --- | --- |
| TCP server behavior | `Editor/UnityAiBridgeTcpServerTests.cs` | Ports, requests, lifecycle, command handling |
| Protocol behavior | `Editor/UnityAiBridgeProtocolTests.cs` | Request/response parsing and serialization |
| AST contract | `Editor/UnityAiBridgeAstContractTests.cs` | Scene/asset/selection AST compatibility |
| Bootstrap behavior | `Editor/UnityAiBridgeBootstrapTests.cs` | Editor auto-start and batch-mode boundaries |
| Discovery files | `Editor/UnityAiBridgeDiscoveryTests.cs`, `Editor/DiscoveryFileCleanup.cs` | `Library/UnityAiBridge/server.json` handling |
| Menu integration | `Editor/UnityAiBridgeMenuTests.cs` | Tools/Linalab/Lux menu behavior |
| Temporary assets | `Editor/TemporaryAssetScope.cs` | Test-owned asset cleanup |
| TCP helpers | `Editor/TcpRequestHelper.cs` | Shared request helper |

## Conventions
- Tests run in Unity Editor Test Runner, not Cargo.
- Keep temporary assets scoped and cleaned up.
- Preserve Unity 6000.0+ compatibility.
- Protocol assertions must stay aligned with `bridge/unity/AiBridgeEditor/UnityAiBridgeProtocol.cs` and `crates/lux-bridge-core`.
- Batch-mode assumptions belong in dedicated tests; do not hide skipped behavior as success.

## Anti-Patterns
- Do not require a target game project from this repository.
- Do not make tests depend on user-specific Unity Editor state.
- Do not claim Unity bridge verification from Rust-only commands.

## Verification
Run Unity Editor tests for `Linalab.UnityAiBridge.Tests.Editor`, including all `*Tests.cs` under `Editor/`.
