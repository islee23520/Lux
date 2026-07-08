# LUX Project Knowledge Base

**Generated:** 2026-07-02 12:46:54 KST
**Commit:** 9a03ab4f
**Branch:** develop

## Overview
LUX is a local-first Rust gateway and MCP control plane for Unity game-project automation. It installs the Unity bridge adapter into target projects, exposes CLI/HTTP/WebSocket/MCP surfaces, and records runtime truth under `.lux/`.

Unity is the only active verified engine path. The next product priority after Unity is Ouroforge; this repository does not yet expose an Ouroforge runtime surface.

## Agent Response Rules
- Answer the user in Korean unless a more specific artifact format requires another language.
- If the user's statement is wrong, say why it is wrong before acting on the corrected premise.
- If the request is ambiguous enough to risk the wrong change, ask for the missing detail.
- For files under `Skills/`, generated skill content stays English because `Skills/AGENTS.md` requires it.

## Structure
```
lux/
├── gateway/       # Rust CLI, Axum server, MCP/API/tool orchestration
├── crates/        # Shared Rust core packages with no gateway wiring
├── bridge/        # Engine bridge source copied by lux bridge install
├── Skills/        # Manifest-backed agent workflow library
├── docs/          # Human-facing projection of architecture and support tiers
├── scripts/       # Verification, policy, and structure checks
└── website/       # Small static site, guarded by website contract checks
```

## Where To Look
| Task | Location | Notes |
| --- | --- | --- |
| CLI command tree | `gateway/src/main.rs` | Source of truth for `lux` commands and legacy wrappers |
| HTTP/WS routes | `gateway/src/server.rs` | Axum 0.7 routes, state extraction, SPA fallback |
| MCP tools | `gateway/src/lux_mcp.rs` | Keep tool responses tied to `.lux/` evidence |
| Runtime state IO | `gateway/src/lux_*`, `crates/lux-core` | `.lux/` remains canonical |
| Shared data contracts | `crates/` | Extracted types and pure logic used by gateway/tests |
| Unity bridge source | `bridge/unity/AiBridgeEditor/` | Installed into target Unity projects |
| Unity bridge tests | `bridge/unity/AiBridgeTests/` | Unity Editor/NUnit verification |
| Skill routing | `Skills/AGENTS.md` | Pick category before loading a specific skill |
| Skill validation tools | `Skills/tools/` | Schema and category layout checks |
| Usage | `docs/usage.md` | Docs are projections, not runtime truth |
| Full local verification | `scripts/test-all.sh` | Runs Rust, CLI, structure, website, policy checks |

## Code Map
| Symbol or Surface | Type | Location | Role |
| --- | --- | --- | --- |
| `lux` | CLI binary | `gateway/src/main.rs` | Command dispatch, bridge install, Unity flows |
| `server` | Axum server | `gateway/src/server.rs` | HTTP, WebSocket, API state projection |
| `lux_mcp` | MCP server | `gateway/src/lux_mcp.rs` | JSON-RPC tool exposure for AI clients |
| `try_ping_unity_bridge_backend` | Rust function | `gateway/src/lib.rs` | Verifies Unity TCP backend readiness |
| `atomic_write_json` | Rust function | `crates/lux-core/src/lib.rs` | Atomic `.lux` JSON writes |
| `append_jsonl` | Rust function | `crates/lux-core/src/lib.rs` | Durable event log append path |
| `SpecProject` | Rust model | `crates/lux-spec-core/src/lib.rs` | Versioned spec contract |
| `TicketStore` | Rust model | `crates/lux-run-core/src/lib.rs` | Executable ticket and dispatch state |
| `BridgeProtocolRequest` | Rust model | `crates/lux-bridge-core/src/protocol.rs` | Rust side bridge protocol contract |
| `UnityAiBridgeTcpServer` | C# class | `bridge/unity/AiBridgeEditor/UnityAiBridgeTcpServer.cs` | Unity Editor TCP server |
| `UnityAiBridgeProtocol` | C# class | `bridge/unity/AiBridgeEditor/UnityAiBridgeProtocol.cs` | Unity bridge request/response parser |
| `scripts/policy-scan.mjs` | Policy checker | `scripts/` | Invariant and marker scan |

## Conventions
- Rust stack: Axum 0.7, tokio 1, clap 4.5, anyhow, serde.
- User-facing Rust errors use `anyhow` for propagation and `eprintln!` for CLI output.
- New endpoints need tests in `gateway/src/server.rs` or `gateway/tests/gateway_cli_smoke.rs`.
- Runtime truth lives under `.lux/`; docs, README, and API projections must not override it.
- `gateway/` owns server and CLI wiring. `bridge/` owns engine adapter source. `Skills/` owns workflow documents.
- Core crates must not depend on gateway-only surfaces such as Axum, Clap, process spawning, or `gateway::`.
- Bridge install paths must be idempotent and safe to rerun.
- Unity bridge compatibility target is Unity 6000.0+.
- Do not reintroduce Godot or Three.js surfaces without a new accepted roadmap decision.

## Anti-Patterns
- Do not include Unity Editor window logic such as Workbench or CodexImage in this repo.
- Do not add GUI, dashboard, TUI, or frontend app code here.
- Do not treat a target Unity project as part of this repository.
- Do not present planned or adapter-only behavior as completed support.
- Do not silently fall back to a legacy path without observable logging.
- Do not add `TODO`, `FIXME`, or `HACK` comments.
- Do not reactivate removed roots: `adapters/`, `seeds/`, `plugins/`, `bridge-threejs/`, `gateway/ui`, `gateway/ui-src`.

## Commands
```bash
cargo build --workspace
cargo test --workspace
cd gateway && cargo run -- bridge install --help
cd gateway && cargo run -- serve --help
./scripts/test-all.sh
./scripts/test-all.sh --quick
node scripts/policy-scan.mjs --advisory-only
```

## Notes
- The repository can contain dirty user work; preserve unrelated edits.
- `.lux/` is runtime state and may contain local evidence, tickets, and session artifacts.
- Unity Editor tests require the Unity Test Runner and are not covered by plain Cargo.
- Generated bridge or dependency folders such as `target/` and `node_modules/` are not source hierarchy.
