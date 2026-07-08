---
name: lux-build
description: Trigger and monitor the Unity WebGL build pipeline through Lux.
category: workflow
source: lux
---

# lux-build — Build Pipeline

## Purpose
Trigger and monitor the Unity WebGL build pipeline through Lux.

## When to Use
- A verified feature needs a distributable WebGL build.
- CI or release workflow must confirm the project builds outside the editor.
- Verification requires build artifacts after compile and bridge checks pass.
- Build status must be tracked through the Lux API or MCP surface.
- You need a reproducible build command for scripts.

## Commands
| Command | Use |
| --- | --- |
| `lux build` | Start the configured WebGL build. |
| `lux status` | Monitor build state and project connection details. |
| `lux verify` | Run verification before or after build as appropriate. |
| `lux doctor` | Diagnose build environment issues. |

## Examples
```bash
lux verify && lux build
```
Expected: build starts only after verification succeeds.

```bash
lux build
```
Expected: Lux records build progress and final status through its API.

```bash
lux status
```
Expected: JSON includes current project, bridge, and build-related state.

## Gotchas
- Unity compile errors must be resolved before requesting a WebGL build.
- Build status is asynchronous; poll status instead of assuming immediate completion.
- Treat failed builds as verification failures and triage the underlying Unity output.
