# Π Fork Tax Snapshot: devel-services-proxx-openplanner

- timestamp: 2026-05-15T06:01:45Z
- repo: /home/err/devel
- branch: pi/fork-tax/2026-04-15-170411
- head-before: 049622b
- origin: git@github.com:riatzukiza/devel.git
- scope: services/proxx, services/openplanner, orgs/open-hax/proxx, orgs/open-hax/openplanner, orgs/shuv/openplanner
- note: Root service snapshot for services/proxx, services/openplanner, and updated open-hax submodule pointers.

## Dirty summary before commit

```text
M services/proxx/.env.example
 M services/proxx/docker-compose.yml
 M services/proxx/policies/runtime/05-provider-seed.edn
 M services/proxx/policies/runtime/10-model-families.edn
 M services/proxx/policies/runtime/20-provider-capabilities.edn
 M services/proxx/policies/runtime/30-model-routing.edn
m orgs/open-hax/proxx
m orgs/open-hax/openplanner
m orgs/shuv/openplanner
```

## Verification plan

- git diff --cached --check after staging
- push branch and tag
- create or update GitHub PR

## Concurrent/residual dirt policy

Unrelated dirty paths outside the scope are intentionally left untouched. Nested submodules with local-only dirt that are not part of the requested scope are recorded as residual rather than cleaned.
