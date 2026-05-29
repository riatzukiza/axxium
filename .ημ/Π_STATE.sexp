(fork-tax-state
  (repo "devel")
  (timestamp "20260529T022118Z")
  (scope "whole-root all-dirt snapshot flattened onto origin/main")
  (mode "user-requested soft reset onto main, then git add -A")
  (base "origin/main")
  (branches "full branch names recorded in Π_LAST.md")
  (tags "full tag names recorded in Π_LAST.md")
  (child-commits
    (knoxx "recorded in Π_LAST.md and manifest")
    (openplanner "recorded in Π_LAST.md and manifest")
    (eta-mu "recorded in Π_LAST.md and manifest")
    (openutau "local-only; push blocked; recorded in Π_LAST.md and manifest")
    (proxx "recorded in Π_LAST.md and manifest")
    (bitch-tracker "recorded in Π_LAST.md and manifest")
    (eta-mu-sol "local-only; no origin remote; recorded in Π_LAST.md and manifest"))
  (verification
    "child branches created by soft reset onto each default main/master"
    "knoxx/openplanner/eta-mu child branches and tags pushed"
    "openutau commit local-only because upstream https auth is unavailable"
    "root pre-commit secret hook timed out on the 14k-file staged tree"
    "targeted staged high-risk secret scan passed after provider-key fixture redaction"
    "full test suite skipped because all-dirt snapshot is intentionally mixed")
  (root-finalization
    (commit "commit containing this artifact")
    (tag "recorded in Π_LAST.md")
    (push "branch and tag pushed; final receipt amendment force-with-lease pushed"))
  (guardrail "this intentionally supersedes prior long-history root branch to avoid replaying secret-bearing history"))

(pi-state
  (timestamp "20260529T040216Z")
  (batch-manifest "/tmp/eta-mu-kanban-batches/agent_riatzukiza.json")
  (action "eta-mu kanban migration batch agent_riatzukiza")
  (root-tracked-paths ("orgs/riatzukiza/kronos" "orgs/riatzukiza/openplanner"))
  (submodules ("orgs/riatzukiza/ollama-benchmarks" "orgs/riatzukiza/openhax" "orgs/riatzukiza/riatzukiza.github.io"))
  (verification "migration script and eta-mu-beta kanban count spot checks")
  (concurrency "path-scoped staging only; unrelated dirt untouched"))
