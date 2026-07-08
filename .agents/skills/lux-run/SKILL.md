---
name: lux-run
description: Execute a Lux development run from project spec to verified completion.
category: workflow
source: lux
---

# lux-run — Spec-Driven Automated Dev Run

## Purpose
Execute a Lux development run from project spec to verified completion.

## When to Use
- The workspace is initialized and has a valid Lux spec.
- A feature or fix should be decomposed into tickets and executed by AI agents.
- Existing run state needs controlled recovery after interruption.
- You need Lux to plan, execute, verify, and close work in one lifecycle.
- Task count or complexity suggests adaptive team composition.

## Commands
| Command | Use |
| --- | --- |
| `lux run` | Plan, execute, verify, and complete the next automated run. |
| `lux run --recover <id>` | Resume an interrupted run by run id. |
| `lux spec validate` | Required preflight before a new run. |
| `lux status` | Inspect run and bridge state while automation proceeds. |

## Examples
```bash
lux spec validate && lux run
```
Expected: lifecycle advances through planning, ticket execution, verification, and completion.

```bash
lux run --recover run-2026-05-14-001
```
Expected: Lux reloads run state and continues from the last safe step.

```bash
lux status
```
Expected: JSON shows run state such as `Idle`, `Planning`, `ExecutingTicket`, or `Verifying`.

## Gotchas
- Do not start a run with an invalid or stale spec; fix spec validation first.
- TaskDAG order matters: blocked tickets must not be executed before prerequisites.
- Recovery should use the recorded run id, not a guessed ticket id.
