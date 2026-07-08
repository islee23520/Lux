---
name: lux-init
description: Initialize or repair the Lux workspace state for a Unity project.
category: workflow
source: lux
---

# lux-init — .lux Workspace Initialization

## Purpose
Initialize or repair the Lux workspace state for a Unity project.

## When to Use
- First time an AI agent starts work in a Unity project that should use Lux.
- `.lux/` is missing, incomplete, or suspected to be corrupted.
- A team profile must be applied before automated Lux workflows start.
- A controlled reinitialization is needed with `--force`.

## Commands
| Command | Use |
| --- | --- |
| `lux init` | Create `.lux/`, write `spec.json`, and prepare server/MCP state. |
| `lux init --force` | Reinitialize generated Lux state without deleting project work. |
| `lux init --team-profile <name>` | Initialize using a named AI team profile. |
| `lux doctor` | Confirm initialization health after setup. |

## Examples
```bash
lux init
```
Expected: `.lux/spec.json` exists and the project can use Lux CLI/API/MCP surfaces.

```bash
lux init --team-profile small-team
```
Expected: workspace is initialized with the selected team defaults.

```bash
lux init --force && lux doctor
```
Expected: regenerated Lux metadata and a clean diagnostic report.

## Gotchas
- `lux init` is project-scoped; run it from the Unity project root or pass the correct project path through the caller.
- `--force` repairs Lux state but must not be treated as a request to delete worktrees, tickets, or user assets.
- Post-init success means `.lux/spec.json` and server/MCP state are present; verify before running automation.
