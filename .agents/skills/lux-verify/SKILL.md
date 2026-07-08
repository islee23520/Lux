---
name: lux-verify
description: Run Lux verification tiers that prove the Unity project is ready for the next workflow step.
category: workflow
source: lux
---

# lux-verify — Full Verification Suite

## Purpose
Run Lux verification tiers that prove the Unity project is ready for the next workflow step.

## When to Use
- Before `lux run` completion or before `lux build`.
- After code, scene, package, or bridge changes.
- When CI needs a single verification entry point.
- After recovering from a crashed or interrupted run.
- When Unity behavior must be checked beyond static file edits.

## Commands
| Command | Use |
| --- | --- |
| `lux verify` | Run the standalone verification suite. |
| `lux run` | Includes verification in its lifecycle. |
| `lux status` | Inspect current verification/run state. |
| `lux triage` | Classify verification failures into actionable tickets. |

## Examples
```bash
lux verify
```
Expected: T1 compile, T2 bridge, and T3 batchmode results are reported.

```bash
lux run
```
Expected: verification occurs after ticket execution and before completion.

```bash
lux verify || lux triage
```
Expected: failed signals become classified events and tickets.

## Gotchas
- T1 compile checks script compilation; do not ignore warnings that block Unity compilation.
- T2 bridge checks Lux-to-Unity connectivity, not gameplay correctness by itself.
- T3 batchmode catches editor/runtime issues that may not appear in a quick file check.
