# Verification Scripts

## Overview
Repository maintenance and verification scripts. These are guardrails for structure, policy, website claims, bridge contract drift, and end-to-end gateway smoke behavior.

## Where To Look
| Task | Location | Notes |
| --- | --- | --- |
| Full verification bundle | `test-all.sh` | Rust build/test, CLI smoke, structure, contracts, policy |
| Quick local gate | `test-all.sh --quick` | Skips full Cargo test suite but keeps smoke and policy checks |
| Source hierarchy guard | `check-project-structure.sh` | Required roots, removed roots, skill category layout |
| README/bridge claims | `check-readme-bridge-contract.sh` | Engine maturity and bridge file contract |
| Website claims | `check-website-contract.sh` | Static site must not overclaim support |
| Policy markers | `policy-scan.mjs` | Invariant scan and allow marker handling |
| Sequential smoke | `e2e-lux-sequential-smoke.sh` | Starts gateway and drives a temporary `.lux` workflow |

## Conventions
- Scripts run from repository root unless they compute `ROOT_DIR` themselves.
- Keep checks observable; failures should name the missing path or drifted claim.
- `test-all.sh --quick` is the fastest broad gate for AGENTS/doc-only changes.
- Full `test-all.sh` may run a known-flaky server test retry; preserve the explicit retry behavior.

## Anti-Patterns
- Do not make policy scans silently ignore violations.
- Do not add generated dependency or build output paths as source roots.
- Do not weaken structure checks when a real source boundary moved; update docs and checks together.

## Commands
```bash
./scripts/test-all.sh --quick
./scripts/test-all.sh
node scripts/policy-scan.mjs --advisory-only
bash scripts/check-project-structure.sh
```
