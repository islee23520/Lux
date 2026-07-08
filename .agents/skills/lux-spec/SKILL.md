---
name: lux-spec
description: Manage the game design and delivery specification that drives Lux automation.
category: workflow
source: lux
---

# lux-spec — Spec Management

## Purpose
Manage the game design and delivery specification that drives Lux automation.

## When to Use
- A project needs its gameplay, art, audio, UI, or testing intent captured.
- Before `lux run`, to ensure `spec.json` and domain notes are valid.
- When an AI agent needs to record assumptions, questions, or decisions.
- After scope changes that affect architecture, packages, levels, or verification.
- In CI or review flows that need spec validation.

## Commands
| Command | Use |
| --- | --- |
| `lux spec status` | Show current spec completion and validation state. |
| `lux spec edit <domain>` | Open `$EDITOR` for a domain markdown file. |
| `lux spec validate` | Validate schema version, required domains, and structure. |
| `lux spec edit dialectic` | Capture questions, decisions, and assumptions. |

## Examples
```bash
lux spec status
```
Expected: domain coverage for design, architecture, art-style, audio, narrative, levels, ui-ux, packages, and testing.

```bash
lux spec edit ui-ux
```
Expected: `$EDITOR` opens the UI/UX domain notes for precise edits.

```bash
lux spec validate
```
Expected: success only when `schema_version` and required domains are present.

## Gotchas
- Do not bypass `$EDITOR` by writing contradictory state outside `.lux/`; `.lux/` is the source of truth.
- The nine domains are required context, not optional decoration.
- Record unresolved topics in the dialectic section instead of hiding uncertainty in implementation notes.
