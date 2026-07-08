# Gateway Integration Tests

## Overview
Rust integration tests for the `lux` CLI, Axum API, `.lux` persistence, event schemas, hooks, sessions, tickets, and verification surfaces.

## Where To Look
| Task | Location | Notes |
| --- | --- | --- |
| CLI smoke coverage | `gateway_cli_smoke.rs` | Command help, bridge install, lifecycle behavior |
| API contract smoke | `cli_api_contract_smoke.rs` | CLI/API shape compatibility |
| Server lifecycle | `lux_lifecycle_test.rs`, `sessions_api_smoke.rs` | Health, session, and shutdown behavior |
| Event schema and JSONL | `event_schema_smoke.rs`, `jsonl_persistence_smoke.rs` | Runtime log compatibility |
| Spec and roadmap APIs | `lux_spec_api_test.rs`, `lux_roadmap_registration_test.rs` | `.lux` projection endpoints |
| Tickets and run state | `lux_ticket_test.rs`, `lux_run_state_test.rs` | Dispatch and state transitions |
| Manual QA and verification | `lux_manual_qa_test.rs`, `lux_verification_test.rs` | Evidence-gated completion paths |
| Hooks governance | `lux_hooks_*` | Symlink, blocker, policy behavior |
| Redaction | `redact_*` | Secret-safe output checks |
| Shared helpers | `common/`, `mod.rs` | Reuse temp project and server setup helpers |

## Conventions
- Tests should drive public CLI/API behavior, not private implementation details.
- Preserve `.lux/` as the asserted state root in temp projects.
- Prefer focused smoke or regression tests for new endpoints and command behavior.
- If a test starts a server, assert health or the matching endpoint, then clean up the process.
- Keep Unity Editor-dependent validation outside Cargo unless the test can run without Unity.

## Anti-Patterns
- Do not delete or weaken failing tests to make the workspace green.
- Do not use generated `target/` or checked-in local runtime state as fixtures.
- Do not claim Unity behavior is verified from a Cargo-only test.

## Commands
```bash
cd gateway && cargo test --test gateway_cli_smoke
cd gateway && cargo test --test lux_spec_api_test
cd gateway && cargo test --test lux_verification_test
cargo test --workspace
```
