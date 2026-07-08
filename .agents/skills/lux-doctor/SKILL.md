---
name: lux-doctor
description: Diagnose and optionally repair Lux workspace, Unity, bridge, and agent integration issues.
category: workflow
source: lux
---

# lux-doctor — Self-Diagnosis & Repair

## Purpose
Diagnose and optionally repair Lux workspace, Unity, bridge, and agent integration issues.

## When to Use
- Before starting significant work in an unfamiliar project.
- After crashes, interrupted runs, missing plugins, or strange status output.
- As a CI/CD gate before automated Lux workflows.
- When `.agents/skills/` appears incomplete.
- Before using `--fix` to let Lux propose safe repairs.

## Commands
| Command | Use |
| --- | --- |
| `lux doctor` | Run diagnostic checks and report failures. |
| `lux doctor --fix` | Auto-fix supported issues through `opencode -p`. |
| `lux status` | Compare live system state before or after diagnostics. |
| `lux init --force` | Repair initialization issues when doctor recommends it. |

## Examples
```bash
lux doctor
```
Expected: checks cover workspace, spec, run-state, unity-project, bridge, plugin, agents-skills, lux-binary, and integrity.

```bash
lux doctor --fix
```
Expected: supported failures are repaired through an observable OpenCode prompt flow.

```bash
lux doctor && lux verify
```
Expected: environment health is confirmed before full verification.

## Gotchas
- `--fix` is for supported repairs; do not assume it can resolve gameplay or design ambiguity.
- Doctor failures are signals to investigate, not reasons to invent fallback state outside `.lux/`.
- Run doctor after crashes before restarting automation to avoid compounding partial state.
