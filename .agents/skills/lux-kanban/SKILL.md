---
name: lux-kanban
description: Inspect and manage Lux tickets that coordinate automated and human work.
category: workflow
source: lux
---

# lux-kanban — Ticket/Kanban Management

## Purpose
Inspect and manage Lux tickets that coordinate automated and human work.

## When to Use
- You need to see the current board before selecting work.
- Triage has created tickets from events.
- A run is blocked and dependencies need inspection.
- Priority or lifecycle state must be checked before automation continues.
- CI or reporting needs a concise board snapshot.

## Commands
| Command | Use |
| --- | --- |
| `lux kanban` | Show board status and ticket distribution. |
| `lux triage` | Create or update tickets from classified events. |
| `lux run` | Execute tickets according to TaskDAG ordering. |
| `lux status` | Check whether a run is already active. |

## Examples
```bash
lux kanban
```
Expected: tickets grouped by Open, InProgress, Done, and Closed.

```bash
lux triage && lux kanban
```
Expected: newly classified failures appear as prioritized tickets.

```bash
lux status && lux kanban
```
Expected: current run state and board state can be compared safely.

## Gotchas
- Ticket lifecycle is Open → InProgress → Done → Closed; avoid skipping states without evidence.
- Priorities are Critical, High, Medium, and Low; Critical blockers should be resolved first.
- Respect blocker relationships or the TaskDAG may execute work in an unsafe order.
