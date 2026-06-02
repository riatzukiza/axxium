# Π fork-tax snapshot — 20260602T172342Z

- Repository: riatzukiza/devel
- Source branch: `feat/axxium-restore-and-track`
- Base: `origin/staging` (dcb261bcc5edd2f328a5bff73dfabf89f0b9ed7c)
- Snapshot commit: 3305c3f104ffd828d113fed534332ae94957da07
- Π tag: `fork-tax/20260602-axxium-restore-and-track`

## Scope (path-scoped, 25 files, +2176/-0)

- `docs/morning_update_2026-04-06.md` — Hormuz Risk Clock morning update (carried forward from working tree)
- `docs/notes/2026.06.02.10.46.20.md` — Axxium design note (today's session)
- `orgs/open-hax/axxium/` — full ClojureScript identity/auth kernel project:
  - `README.md`, `deps.edn`, `shadow-cljs.edn`, `package.json`
  - `src/cljs/axxium/{auth,routes,db,config,schema,server}.cljs` and friends
  - `docs/axxium-kernel-spec.md`, `docs/axxium-kernel-spec-v2.md`, `docs/axium-synthesis.md`, `docs/open-hax-octave-commons-axxium.md`
  - `.github/workflows/deploy.yml`, `.env.example`
  - `.gitignore` (extended with `session-*.md` and standard caches)

## Excluded (per `.gitignore` rules, intentionally not tracked)

- `orgs/open-hax/axxium/node_modules/` (150 MB)
- `.shadow-cljs/`, `.cpcache/`, `.clj-kondo/`, `.lsp/`, `dist/`
- `session-ses_1802.md` (171 KB Kimi K2.6 session dump; new `session-*.md` rule in axxium `.gitignore`)

## Path-scoped staging verification

- `git status -uall` shows no other untracked files
- The `.git` submodule pointer was removed from the copied axxium directory before staging; project is added as plain tracked content under `orgs/open-hax/axxium/`
- `.gitmodules` on `origin/staging` does not register axxium; the new commit does not reintroduce a gitlink

## Concurrent dirt (intentionally untouched)

- Local main worktree (at `/home/err/devel`) still has branch `staging` at `db1586e03a`, which is stale relative to `origin/staging` `dcb261bcc5`.
- The stale tip carries a `scrub secrets, re-add axxium` commit (`db1586e03a`) that is functionally redundant with `db99be843e` already on `origin/staging`; another agent's Π promotion (`pi/fork-tax/20260602T164500Z/devel-main-staging-scrubbed-promotion`) is recorded in the tag namespace.
- Per fork-tax guardrails, no destructive reset of the main worktree was performed. The fork-tax work itself was carried out in the new worktree `.worktrees/feat-axxium-restore-and-track`.

## PR

- Target: `staging`
- Source: `feat/axxium-restore-and-track`
- Diff is the snapshot above (25 files, +2176 lines), clean against the latest `origin/staging`.
