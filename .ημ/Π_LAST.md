# Π fork-tax snapshot — 20260610T042416Z

- Repository: riatzukiza/devel
- Source branch: `feat/ci-automation-1781026522`
- Base: `origin/staging`
- Snapshot commit: (pending)
- Π tag: `fork-tax/20260610-ci-automation`

## Scope (33 tracked files, submodule pointers + policy + config)

### Submodule pointer updates (20)
- `orgs/octave-commons/daimoi`, `eros-eris-field`, `eros-eris-field-app`, `eta-mu-sol`, `fork_tales`, `gates-of-aker`, `lineara_conversation_export`, `promethean`, `promethean-agent-system`, `shibboleth`, `simulacron`
- `orgs/open-hax/axxium`, `commanoxx`, `depenoxx`, `eta-mu`, `openplanner`, `privaxxy`, `proxx`, `uxx`, `vexx`
- `orgs/agustif/codex-linux`, `orgs/riatzukiza/TANF-app`, `orgs/shuv/mcporter`

### File changes
- `.factory/skills/submodule-ops/SKILL.md` — removed (66 lines)
- `.ημ/PRINCIPLE.edn` — minor edit
- `AGENTS.md` — removed (54 lines)
- `README.md` — removed (70 lines)
- `bin/align-submodules` — removed (31 lines)
- `bin/fork-tax-submodules` — removed (24 lines)
- `services/openplanner/compose/proxx.yml` — config update
- `services/proxx/policies/runtime/00-manifest.edn` — policy update
- `services/proxx/policies/runtime/10-model-families.edn` — policy update
- `spec.json` — schema update

## Excluded (secrets — NOT committed)

- `passwords.csv` — contains website credentials, explicitly excluded
- `services/proxx/cephalon-hive-accounts.json` — provider account data
- `services/proxx/cephalon-hive-providers.json` — provider config
- `services/proxx/proxx-federation-accounts.json` — federation accounts
- `services/proxx/proxx-federation-providers.json` — federation providers
- `services/openplanner/scripts/sync-runtime-secrets-env.sh` — secrets script
- `services/openplanner/scripts/unfragile-mongo-reset.sh` — DB reset script

## Concurrent dirt (intentionally untouched)

- **1478 untracked files** not staged:
  - ~452 audio files (Voice/TTS MP3s, WAVs)
  - ~200+ Lore/fork-tales events, world-states, plot-logs, world-building docs
  - ~100+ Graphics SVGs (Symmetry Council seals, emblems)
  - ~100+ kanban drafts, service scaffolds
  - Music/USTX files, narrative docs, notes
  - Services: `services/eta-mu/kanban-cljs/`, `services/eta-mu/kanban/.eta-mu/`
  - Orgs: `orgs/octave-commons/lyrical-engine/`, `orgs/octave-commons/markov_song_engine/`
- Per fork-tax guardrails, untracked creative artifacts and secrets are left as documented residual.

## Handoff artifacts updated

- `.ημ/Π_LAST.md` — this file
- `.ημ/Π_STATE.sexp` — machine-readable state
