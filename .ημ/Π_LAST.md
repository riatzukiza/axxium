# Π fork tax — Proxx bridge lease + service runtime snapshot — 20260529T012348Z

- Timestamp: 20260529T012348Z
- Root repo: `/home/err/devel`
- Root branch: `pi/fork-tax/20260526T204054Z-knoxx-host-services`
- Scope: path-scoped preservation of Proxx bridge/federation source changes plus `services/proxx` runtime, policy, and operator-documentation changes.
- Mode: bottom-up child submodule commit, non-destructive concurrent-agent staging.

## Preserved child refs

- Proxx source repo: `297e05150986856544c0794432b8f9f73fd11767` (`297e05150986`)
  - Branch: `pi/fork-tax/20260526T191143Z-recursive-orgs-open-hax-proxx`
  - Tag: `pi/fork-tax/20260529T012348Z/proxx-bridge-lease-routing`
  - Remote: `git@github.com:open-hax/proxx.git`
  - Commit message: `Repair Proxx bridge lease routing`

## Root paths staged for final snapshot

- `AGENTS.md` — Proxx workflow updated to treat `services/proxx` as service-owned runtime/devops home and policy EDN as provider/model authority.
- `services/proxx/.env.example` — event-store TTL/sweep defaults documented as process wiring.
- `services/proxx/README.md` — policy-first provider/model configuration guidance and host PM2 runner notes.
- `services/proxx/docker-compose.yml` — event-store TTL/sweep env passed to local prod/proxy containers.
- `services/proxx/ecosystem.host.config.cjs` — host PM2 TTL/sweep defaults.
- `services/proxx/scripts/run-host-proxx.sh` — host runner TTL/sweep exports.
- `services/proxx/scripts/seed-dev-db-from-prod.sh` — skip schema import when target credential tables already exist; fail on partial schema.
- `services/proxx/policies/runtime/05-provider-seed.edn` — local provider route path/auth metadata for Ollama/LAN/llamacpp embeddings.
- `services/proxx/policies/runtime/10-model-families.edn` — explicit `openai/`, `factory/`, and `ollama/` model-family clauses.
- `services/proxx/policies/runtime/20-provider-capabilities.edn` — explicit provider orders, embeddings provider set, and ollama-cloud response passthrough exclusion.
- `services/proxx/policies/runtime/30-model-routing.edn` — strict routes for explicit provider-prefixed model families and gemma strategy ordering.
- `orgs/open-hax/proxx` — submodule pointer advanced to the Proxx commit above.
- `receipts.edn` — receipt-river evidence for verification and fork-tax scope.
- `.ημ/Π_LAST.md`, `.ημ/Π_STATE.sexp`, `.ημ/Π_MANIFEST.sha256`, `.ημ/Π_MANIFEST_2026-05-29_012348.sha256` — handoff artifacts.

## Verification

- Child Proxx: `npx tsx --test src/tests/bridge-helpers.test.ts` — 3 tests passed.
- Child Proxx: `git diff --check -- src/app.ts src/lib/bridge-helpers.ts src/lib/federation/bridge-fallback.ts src/tests/bridge-helpers.test.ts receipts.edn` — passed.
- Root: `node -c services/proxx/ecosystem.host.config.cjs` — passed.
- Root: `bash -n services/proxx/scripts/run-host-proxx.sh services/proxx/scripts/seed-dev-db-from-prod.sh` — passed.
- Root: `git diff --check` on staged-intended Proxx service/docs/policy paths and receipts — passed.
- Secret heuristic scan over staged-intended diffs reported `secret_heuristic_findings=0`.
- Earlier receipt evidence records Proxx event-store tests/build/live health checks and bridge repair validation.

## Concurrent dirt intentionally left untouched

- Root status before final staging contained 2,967 dirty entries: 2,941 deletions and 26 modified paths.
- The 2,941 broad root deletions are pre-existing/historical workspace dirt and were not staged or cleaned.
- Unrelated modified paths left unstaged include `.gitignore`, `Voice/**` fork-tales assets, `orgs/open-hax/eta-mu`, `orgs/open-hax/openplanner`, `orgs/stakira/OpenUtau`, `packages/kanban/package.json`, `pnpm-lock.yaml`, and `receipts.log`.
- No repo-wide `git add -A`, reset, restore, clean, or destructive checkout was used.

## Root finalization

- Planned root tag: `pi/fork-tax/20260529T012348Z/devel-proxx-bridge-service-runtime`
- Root commit: pending at artifact render time; final tag points to the committed snapshot.
