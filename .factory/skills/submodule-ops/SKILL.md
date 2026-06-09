---
name: submodule-ops
description: "Make safe, consistent changes in a workspace with many git submodules under orgs/**"
---

# Skill: Submodule Operations

## Goal
Make safe, consistent changes in a workspace with many git submodules.

## Use This Skill When
- You touch files under `orgs/**` or `.gitmodules`.
- You need to run `submodule` commands or update submodule pointers.
- You are asked to change code in multiple submodules.
- You need to initialize, update, align, or capture submodule state.

## Do Not Use This Skill When
- The change is confined to the REDACTED_SECRET workspace only.
- You are only editing documentation in `docs/`.

## Inputs
- The target submodule path(s).
- Any required submodule commands from `AGENTS.md`.

## Steps
1. Identify the exact submodule(s) impacted.
2. Work inside the submodule path; avoid cross-submodule edits.
3. Note any required sync/update steps before finishing.
4. Mention follow-up commands for the user if needed.

---

## Branch-Based Submodule Alignment Protocol

This workspace uses a **branch-based submodule alignment protocol**. All agents must follow these rules.

### Rule 1: Default Branches (`main`, `staging`)
When the parent repo is on `main` or `staging`:
- All submodules track their own `main` branch (or `staging` if available).
- Command: `git submodule update --remote --recursive`
- This ensures integration branches are stable and predictable.

### Rule 2: Feature Branches (everything else)
When the parent repo is on any non-default branch (`feat/*`, `fix/*`, `wip/*`, `pi/*`, etc.):
- Submodules follow whatever branch is currently checked out in each child.
- Each submodule operates independently.
- The parent repo's submodule pointer records the exact commit of each submodule's current checkout.
- Submodules may be on different branches, detached HEADs, or feature branches.

### Rule 3: Fork Tax (State Capture)
Before ending a session or switching context:
- Run the "fork tax" to persist submodule states into the parent repo.
- This updates the parent repo's submodule pointers to match current submodule checkouts.
- Command:
  ```bash
  git submodule foreach 'cd $toplevel && git add $sm_path'
  git commit -m "fork tax: align submodule pointers to current checkouts"
  ```

### Rule 4: Initialization Protocol
For fresh clones or after branch switches:
```bash
# Remove orphaned refs (not in .gitmodules)
git ls-tree -r HEAD | grep '^160000' | awk '{print $4}' | while read path; do
  grep -q "path = $path" .gitmodules || git rm --cached "$path"
done

# Initialize all valid submodules
git submodule update --init --recursive
```

### Rule 5: `.gitmodules` Branch Configuration
Every submodule should have a `branch =` field in `.gitmodules`:
- Default for most submodules: `branch = main`
- If a submodule uses a different default branch, document it.
- The branch field is used by `git submodule update --remote` on default branches.

### Rule 6: Safety Rules
- Never commit broken submodule refs (missing URLs, unreachable commits).
- Always verify `git submodule status` shows no `-` (uninitialized) before committing.
- Keep submodule refs clean: remove orphaned refs promptly.

---

## Submodule Commands

```bash
# Sync .gitmodules mappings and initialize submodules
submodule sync [--recursive] [--jobs <n>]

# Update to latest tracked commits
submodule update [--recursive] [--jobs <n>]

# Show pinned commits and dirty worktrees
submodule status [--recursive]

# Smart commit with Pantheon integration
submodule smart commit "message" [--dry-run] [--recursive]
```

## Canonical Scripts

| Script | Purpose |
|--------|---------|
| `bin/align-submodules` | Align submodules based on parent branch (Rule 1/2) |
| `bin/fork-tax-submodules` | Capture current submodule state into parent repo |

## Submodule Workflows
- `cd orgs/riatzukiza/promethean && pnpm --filter @promethean-os/<pkg> <command>`
- `cd orgs/anomalyco/opencode && bun dev`

## Core Principles
- **Atomic operations**: Commit submodule changes before updating parent pointers.
- **Recursive awareness**: Use `--recursive` only when nested submodules are required.
- **Parallel execution**: Control job count with `SUBMODULE_JOBS=<n>` (default: 8).
- **Status awareness**: Check `bin/submodules-status` before committing workspace changes.
- **Branch-aware alignment**: Always check the parent branch before deciding how to align submodules.

## Strong Hints
- Avoid touching submodules you do not need.
- Keep edits localized to the intended repo.
- If a change spans multiple submodules, sequence the work and call it out.
- Use `submodule status` before and after large changes to confirm cleanliness.
- On feature branches, remember to pay the fork tax before switching away.

## Output
- Clear list of modified submodule paths and any follow-up commands.
- Confirmation of which alignment rule was applied.

## Troubleshooting References
- Nested submodule failures: `spec/submodules-update.md`
- Worktree conflicts: `docs/worktrees-and-submodules.md`
- Full protocol: `AGENTS.md` → "Git Submodule Operations Protocol"
