# Shared Rust Core Crates

## Overview
Reusable Rust packages extracted from gateway responsibilities. Keep them pure enough to serve CLI, server, MCP, and tests without importing gateway wiring.

## Structure
```
crates/
├── lux-ai-core/             # AI context ontology summaries
├── lux-bridge-core/         # Bridge protocol and Unity AST contracts
├── lux-core/                # Atomic IO and JSONL primitives
├── lux-project/             # Unity/Godot detection and capability records
├── lux-run-core/            # Run state, task DAG, ticket dispatch policy
├── lux-spec-core/           # Spec models, validation, ambiguity reports
└── lux-verification-core/   # Evidence and blocker taxonomy
```

## Where To Look
| Task | Location | Notes |
| --- | --- | --- |
| Atomic `.lux` writes | `lux-core/src/lib.rs` | Use synced temp file plus rename |
| Event log append/read | `lux-core/src/lib.rs` | JSONL path rejects symlink/hardlink risks |
| Bridge protocol shape | `lux-bridge-core/src/protocol.rs` | Must stay compatible with Unity C# protocol |
| Unity AST contract | `lux-bridge-core/src/ast.rs` | Scene, asset, selection AST payloads |
| Engine detection | `lux-project/src/detection.rs` | Unity and Godot project detection |
| Capability status | `lux-project/src/capability.rs` | Mature/partial/planned routing data |
| Ticket execution rules | `lux-run-core/src/ticket.rs` | Dispatch and blocker policy |
| Spec validation | `lux-spec-core/src/validation.rs` | Supported schema/version constraints |
| Evidence classes | `lux-verification-core/src/lib.rs` | Canonical evidence and blocker labels |

## Conventions
- Public exports belong in each crate's `src/lib.rs`.
- Prefer `serde` models with explicit schema/version constants where data crosses process or file boundaries.
- Keep gateway orchestration in `gateway/`; crates should not import Axum, Clap, `tokio::process`, `std::process::Command`, `gateway::`, or `crate::gateway`.
- Add or update crate-local tests when a shared contract changes.

## Anti-Patterns
- Do not duplicate gateway route or CLI behavior inside a core crate.
- Do not add side-channel state paths that compete with `.lux/`.
- Do not weaken serde shapes to hide compatibility breaks.

## Commands
```bash
cargo test -p lux-core
cargo test -p lux-bridge-core
cargo test -p lux-project
cargo test -p lux-run-core
cargo test -p lux-spec-core
cargo test -p lux-verification-core
```
