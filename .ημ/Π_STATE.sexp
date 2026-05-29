(fork-tax-state
  (repo "devel")
  (timestamp "20260529T012348Z")
  (scope "Proxx bridge lease routing plus services/proxx runtime policy snapshot")
  (mode "bottom-up child submodule commit; path-scoped root staging; concurrent dirt preserved")
  (branches
    (root "current fork-tax root branch; full ref in Π_LAST.md")
    (proxx "recursive Proxx fork-tax branch; full ref in Π_LAST.md"))
  (tags
    (proxx "full Proxx tag recorded in Π_LAST.md")
    (root "full root tag recorded in Π_LAST.md"))
  (commits
    (proxx "recorded in Π_LAST.md and Π_MANIFEST.sha256")
    (root "pending"))
  (verification
    "child: npx tsx --test src/tests/bridge-helpers.test.ts passed 3/3"
    "child: git diff --check passed on Proxx bridge paths and receipts"
    "root: node -c services/proxx/ecosystem.host.config.cjs passed"
    "root: bash -n services/proxx/scripts/run-host-proxx.sh services/proxx/scripts/seed-dev-db-from-prod.sh passed"
    "root: git diff --check passed on staged-intended Proxx service/docs/policy paths and receipts"
    "secret heuristic scan over staged-intended diffs found 0 findings")
  (staged-root-paths
    "AGENTS.md"
    "services/proxx/.env.example"
    "services/proxx/README.md"
    "services/proxx/docker-compose.yml"
    "services/proxx/ecosystem.host.config.cjs"
    "services/proxx/scripts/run-host-proxx.sh"
    "services/proxx/scripts/seed-dev-db-from-prod.sh"
    "services/proxx/policies/runtime/05-provider-seed.edn"
    "services/proxx/policies/runtime/10-model-families.edn"
    "services/proxx/policies/runtime/20-provider-capabilities.edn"
    "services/proxx/policies/runtime/30-model-routing.edn"
    "orgs/open-hax/proxx"
    "receipts.edn"
    ".ημ/Π_LAST.md"
    ".ημ/Π_STATE.sexp"
    ".ημ/Π_MANIFEST.sha256"
    ".ημ/Π_MANIFEST_2026-05-29_012348.sha256")
  (residual
    (dirty-count 2967)
    (deleted-count 2941)
    (left-unstaged ".gitignore Voice/** orgs/open-hax/eta-mu orgs/open-hax/openplanner orgs/stakira/OpenUtau packages/kanban/package.json pnpm-lock.yaml receipts.log broad root deletions")
    (guardrail "no blanket add-all; no destructive cleanup")))
