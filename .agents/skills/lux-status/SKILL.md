---
name: lux-status
description: Read Lux server, project, bridge, run, and build state in script-friendly JSON.
category: workflow
source: lux
---

# lux-status — System Status

## Purpose
Read Lux server, project, bridge, run, and build state in script-friendly JSON.

## When to Use
- Before starting automation to ensure the correct project is active.
- During `lux run`, `lux build`, or verification to monitor progress.
- In scripts or CI that need machine-readable status.
- When diagnosing bridge connectivity or server lifecycle issues.
- Before deciding whether recovery or doctor is needed.

## Commands
| Command | Use |
| --- | --- |
| `lux status` | Print current Lux status as JSON. |
| `lux doctor` | Diagnose problems discovered from status. |
| `lux run --recover <id>` | Recover a run identified from status output. |
| `lux verify` | Validate the project after status looks healthy. |

## Examples
```bash
lux status
```
Expected: JSON describes server, project, bridge, run, and build state.

```bash
lux status | jq '.bridge.connected'
```
Expected: `true` when the Unity bridge is connected.

```bash
lux status | jq '.run.state'
```
Expected: state such as `Idle`, `Planning`, `ExecutingTicket`, or `Verifying`.

## Gotchas
- Status is observational; it should not mutate `.lux/` or repair state by itself.
- Always confirm the project path in JSON before acting on tickets or builds.
- In CI, parse explicit fields instead of scraping human text.
