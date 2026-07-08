---
name: lux-triage
description: Turn raw Unity, AI, and Lux events into deduplicated actionable tickets.
category: workflow
source: lux
---

# lux-triage — Event Triage Pipeline

## Purpose
Turn raw Unity, AI, and Lux events into deduplicated actionable tickets.

## When to Use
- Verification, build, or run output contains errors that need classification.
- Unity console logs or AI logs are noisy and repetitive.
- Tickets should be created automatically from events.
- A project has accumulated unresolved event streams in `.lux/`.
- You need to reduce duplicate failure reports before planning work.

## Commands
| Command | Use |
| --- | --- |
| `lux triage` | Ingest events, classify them, deduplicate, and create tickets. |
| `lux kanban` | Review tickets created by triage. |
| `lux verify` | Produce fresh verification events before triage. |
| `lux status` | Confirm project and run state before event processing. |

## Examples
```bash
lux verify || lux triage
```
Expected: compile, bridge, or batchmode failures become categorized tickets.

```bash
lux triage && lux kanban
```
Expected: board shows new or updated tickets without duplicate spam.

```bash
lux triage
```
Expected: events are classified as domains such as compile-error, ai-log, or unity-console.

## Gotchas
- Deduplication uses Jaccard plus Levenshtein similarity with a 0.75 threshold; similar logs may merge.
- Triage should classify and ticket events, not silently discard confusing output.
- Review generated tickets before running broad automation from them.
