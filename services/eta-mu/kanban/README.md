# eta-mu-kanban

PM2 service for the Knoxx kanban board.

## What it does

Runs `@openhax/kanban` web UI pointing at the Knoxx kanban tasks directory.

## Start

```bash
cd services/eta-mu/kanban
pm2 start ecosystem.config.cjs
```

## Stop

```bash
pm2 stop eta-mu-kanban
```

## Access

http://127.0.0.1:8787

## Config

- Tasks live in `orgs/open-hax/openplanner/packages/agents/knoxx/kanban/`
- Config: `orgs/open-hax/openplanner/packages/agents/knoxx/kanban/openhax.kanban.json`
- Import script: `orgs/open-hax/openplanner/packages/agents/knoxx/scripts/import-kanban-specs.mjs`

## Regenerate tasks from specs

```bash
cd orgs/open-hax/openplanner/packages/agents/knoxx
node scripts/import-kanban-specs.mjs
```

## Environment

| Variable | Default | Description |
|----------|---------|-------------|
| `KANBAN_HOST` | `127.0.0.1` | Bind address |
| `KANBAN_PORT` | `8787` | Bind port |
