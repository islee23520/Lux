# Skills Tooling

## Overview
Validation and synchronization tools for the bundled Skills library. These scripts protect category routing, frontmatter, manifests, and external skill projections.

## Where To Look
| Task | Location | Notes |
| --- | --- | --- |
| Validate skill schema | `validate-skills.sh` | Checks categories, `AGENTS.md`, frontmatter, manifest category, line limits |
| Sync OpenCode copy | `sync-opencode.sh` | Mirrors skills into OpenCode-compatible locations |
| Category routing rules | `../AGENTS.md` | Select category before individual skills |
| Skill bodies | `../skills/<category>/<skill>/SKILL.md` | English generated content |

## Conventions
- Default `SKILLS_ROOT` is `Skills/skills`; override only for explicit validation targets.
- Required categories are `architecture`, `review`, `workflow`, `unity`, `studio`, `quality`, and `bugs`.
- `SKILL.md` frontmatter must contain `name`, `description`, `category`, and `source`.
- Skill directory name, frontmatter `name`, and manifest category must agree.
- Keep validation output explicit: `PASS` or `FAIL` with the checked item.

## Anti-Patterns
- Do not add flat skill directories directly under `Skills/skills`.
- Do not use absolute `SKILL.md` symlinks.
- Do not let a category exist without its routing `AGENTS.md`.
- Do not bypass `validate-skills.sh` after changing skill structure.

## Commands
```bash
cd Skills && ./tools/validate-skills.sh
SKILLS_ROOT=/path/to/skills ./tools/validate-skills.sh
```
