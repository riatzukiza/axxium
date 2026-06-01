# Proxx service runtime instructions

This directory is the workspace-local runtime/devops home for Proxx. The source checkout is `/home/err/devel/orgs/open-hax/proxx`.

## Development runner

When doing host development on `/home/err/devel/orgs/open-hax/proxx`, use this service-local PM2 ecosystem file:

```bash
cd /home/err/devel/services/proxx
docker compose -f docker-compose.dev-db.yml up -d proxx-dev-db
./scripts/seed-dev-db-from-prod.sh
pm2 start ecosystem.host.config.cjs --only proxx-host,proxx-host-web --no-autorestart
```

`ecosystem.host.config.cjs` is the canonical host dev runner. It points at the source checkout, uses service-owned runtime paths, binds host-dev ports (`18789` API, `15174` web by default), and sets `PROXX_CLJS_POLICY_MANIFEST` to `services/proxx/policies/runtime/00-manifest.edn`.

## CLJS policy source of truth

Proxx is being slowly rewritten in ClojureScript. Provider and model behavior must move to the policy runtime:

- Provider routes/capabilities: `policies/runtime/20-provider-capabilities.edn` and related provider contracts.
- Model families/routing: `policies/runtime/10-model-families.edn` and `policies/runtime/30-model-routing.edn`.
- Pricing overrides: `policies/runtime/15-model-pricing-overrides.edn`.
- Manifest order: `policies/runtime/00-manifest.edn`.

Do not make provider/model decisions by adding knobs to `.env`, Docker Compose env blocks, shell exports, or TypeScript config parsing. Environment variables in this service directory are for secrets, ports, database URLs, process wiring, and temporary legacy compatibility only. If behavior changes provider selection, model classification, routing, allow/deny policy, or pricing, add or change a policy EDN contract instead.
