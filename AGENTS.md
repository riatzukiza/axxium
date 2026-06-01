# Agent Skills Context

## RELEVANT SKILLS
These skills are configured for this directory's technology stack and workflow.

### testing-general
Apply testing best practices, choose appropriate test types, and establish reliable test coverage across the codebase

### workspace-code-standards
Apply workspace TypeScript and ESLint standards, including functional style and strict typing rules

### workspace-lint
Lint all TypeScript and markdown files across the entire workspace, including all submodules under orgs/**

### workspace-typecheck
Type check all TypeScript files across the entire workspace, including all submodules under orgs/**, using strict TypeScript settings

## Creative production skills

Use this suite when the user asks for creative development or production assets:

- `creative-storycraft`: story writing, lore, characters, outlines, scenes, scripts, and prose revision.
- `music-composition-arrangement`: songs, cues, loops, motifs, chord progressions, arrangements, and music handoffs.
- `openutau-synthetic-vocals`: OpenUtau/UTAU lyric timing, USTX drafting, phonemizers, voicebank checks, and vocal render handoff.
- `audio-reconstruction-audit`: local-only WAV/MP3 reconstruction audits with STT, f0/pitch, spectrogram, Gemma, evidence JSON, and receipts before claiming success.
- `visual-concept-art-direction`: concept art briefs, palettes, shape language, prompt packs, and visual critique.
- `graphics-asset-production`: logos, icons, posters, covers, UI graphics, SVG/raster exports, and sprite sheets.
- `blender-3d-modeling`: Blender scenes, procedural meshes, materials, model exports, and render previews.
- `animation-production`: storyboards, animatics, keyframes, motion graphics, renders, and ffmpeg assembly.
- `multimedia-creative-pipeline`: orchestration across story, music, visuals, graphics, 3D, animation, and vocals.

## Eta-mu runtime shorthand

When the user says things like:

- “eta-mu runtime”
- “fix the eta-mu extension X”

Interpret that as working in the `orgs/open-hax/eta-mu/packages/eta-mu-extensions` build/deploy system.

Reference: `docs/reference/eta-mu-runtime.md` (paths, build steps, and what “extension X” maps to).
EOFEOF
EOFEOF

## Docker compose preferences (non-ephemeral ops)
- Prefer versioned compose files and checked-in scripts over one-off CLI flags.
- Avoid requiring `docker compose ... --scale ...` as the only way to express desired replica counts; encode replica intent in compose (even if that means explicit shard services or preset compose overlays).
- Avoid fixed host-port publishing on services that may run multiple replicas; publish a single ingress port on a proxy (nginx) and use internal networking for replicas.

## Proxx Development Workflow (Updated 2026-05-20)
- Proxx source lives in `orgs/open-hax/proxx`; runtime/devops state lives in `services/proxx`.
- For host development on `orgs/open-hax/proxx`, use the service-owned PM2 ecosystem file:
  ```bash
  cd services/proxx
  docker compose -f docker-compose.dev-db.yml up -d proxx-dev-db
  ./scripts/seed-dev-db-from-prod.sh
  pm2 start ecosystem.host.config.cjs --only proxx-host,proxx-host-web --no-autorestart
  ```
- `services/proxx/ecosystem.host.config.cjs` is the canonical host dev runner; it points at the source checkout, the dev DB, and the active policy manifest.
- Proxx is being slowly rewritten in CLJS. Provider/model routes, capabilities, model families, routing, allow/deny behavior, and pricing must come from policy EDN files under `services/proxx/policies/runtime/` (and the source defaults in `orgs/open-hax/proxx/resources/policies/runtime/`), not from `.env`, Compose env blocks, shell exports, or TypeScript conditionals.
- Env vars in `services/proxx/.env` are for secrets, ports, database URLs, process wiring, and temporary legacy compatibility only; never add provider/model decision knobs there.

## Knoxx
Knoxx is located at `orgs/open-hax/openplanner/packages/agents/knoxx`. It is a central and highly important part of the workspace.

## Active Output Contract
For every assistant response, you MUST satisfy this output contract even for simple questions:
- Return Markdown with these exact level-2 headings in order: Signal, Evidence, Frames, Countermoves, Next
- Use `## Heading` level-2 markdown headers for each section. Do NOT use bold (`**Heading**`), emphasis, or deeper headings (`###`, `####`) in place of section headers.
- Next must contain exactly 1 concrete next action.
- Frames must contain 2-3 plausible interpretations.

## OpenCode eta-mu tool disclosure
When asked what tools you have, include the eta-mu OpenCode extension tools explicitly: `apply_patch`, `chronos`, `contract_fulfillment`, `graph-memory-search`, `graph-memory-recall`, `graph-memory-ingest`, `context-hydrate`, `graph-memory-status`, `render_image`, `opmf_parse`, `skill_graph`, `receipt_river`, `session_mycology`, and `websearch`.
