# Devel Workspace

Monorepo workspace containing 475+ git submodules organized under `orgs/`.

## Quick Setup

```bash
# Initialize all submodules
git submodule update --init --recursive

# Check status
git submodule status
```

## Submodule Operations Protocol

This workspace uses a **branch-based submodule alignment protocol**.

### On `main` / `staging`
Submodules track their `main` branch:
```bash
git submodule update --remote --recursive
```

### On Feature Branches (`feat/*`, `fix/*`, `wip/*`, etc.)
Submodules follow their current checkout:
```bash
# Each submodule stays on whatever branch it is currently on
# The parent repo records the exact commit
```

### Fork Tax
Before switching context or ending a session, capture submodule state:
```bash
# Run the fork tax script
bin/fork-tax-submodules
```

### Safety
- Never commit broken submodule refs
- Always verify no `-` (uninitialized) in `git submodule status`
- Remove orphaned refs (not in `.gitmodules`) promptly

## Documentation

- **Submodule Protocol**: See `AGENTS.md` → "Git Submodule Operations Protocol"
- **Detailed Guides**: See `docs/worktrees-and-submodules.md`
- **CLI Tools**: See `docs/SUBMODULE_CLI_MIGRATION.md`

## Canonical Scripts

| Script | Purpose |
|--------|---------|
| `bin/align-submodules` | Align submodules based on parent branch (Rule 1/2) |
| `bin/fork-tax-submodules` | Capture current submodule state into parent repo |

## Workspace Structure

```
.
├── orgs/          # Git submodules (475+ repos)
│   ├── open-hax/  # Core OpenHax projects
│   ├── octave-commons/  # Octave Commons projects
│   ├── shuv/      # Shuv's repos
│   └── ussyverse/ # USSYVerse repos
├── services/      # Runtime services (not submodules)
├── packages/      # Workspace packages
├── tools/         # Workspace tools
└── docs/          # Documentation
```
