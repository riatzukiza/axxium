---
uuid: "orgs-open-hax-archived-museeks-kanban-orgs-open-hax-archived-museeks-spec-opencode-plugin-md"
title: "OpenCode Plugin Scaffold Spec"
status: incoming
priority: P3
labels: ["specs", "migrated-spec"]
created_at: "2026-05-29T04:01:18.616Z"
source: "orgs/open-hax/archived/museeks/spec/opencode-plugin.md"
category: "specs"
---

> Source: `orgs/open-hax/archived/museeks/spec/opencode-plugin.md`
> Migrated-to-kanban: `orgs/open-hax/archived/museeks/kanban/opencode-plugin.md`

# OpenCode Plugin Scaffold Spec

## Existing Context
- `package.json:1` – repo currently contains a placeholder npm package with no code.
- No other source files exist yet; repo is ready for scaffolding.

## Related Issues / PRs
- GitHub issues: none (`gh issue list -R open-hax/museeks`).
- GitHub PRs: none (`gh pr list -R open-hax/museeks`).

## Requirements
1. Scaffold a reusable TypeScript plugin that targets the OpenCode plugin API (`@opencode-ai/plugin`).
2. Provide build tooling (TypeScript config + npm scripts) so `pnpm install` and `pnpm build` succeed.
3. Demonstrate at least one useful hook (e.g., logging or safety guard) to show plugin behavior.
4. Document installation/usage so users can drop the build artifact into `.opencode/plugin/` or publish to npm.
5. Ensure repository metadata (package.json) reflects the plugin (name, description, keywords, author placeholder, license, etc.).
6. Code must build successfully (via `pnpm build`) and any available tests must pass after each phase.

## Definition of Done
- Clean TypeScript source under `src/` compiled to `dist/` using `pnpm build` without errors.
- Plugin exports a typed `Plugin` implementation with at least one hook.
- README outlines how to install, configure, and extend the plugin.
- Root `package.json` scripts cover `build`, `lint` (if applicable), and `test` (even if placeholder) while referencing pnpm.
- No lint/type errors; repository ready for publication.

## Implementation Plan (Phased)

### Phase 1 – Repo Setup & Tooling Baseline
- Replace placeholder `package.json` with initial metadata for the OpenCode plugin (name, version, scripts, dependencies placeholder).
- Add `pnpm-lock.yaml` by running `pnpm install` and ensure repo builds even with empty src (tsconfig + src stub) to satisfy "build passes after phase".
- Deliverable: minimal TypeScript project structure with working `pnpm build` (even if build is a no-op) to verify toolchain.

### Phase 2 – Plugin Source + Build Output
- Implement `src/index.ts` exporting a `Plugin` typed function; include representative hook (e.g., log session lifecycle, guard for `.env` reads) plus helper utilities.
- Configure TypeScript (`tsconfig.json`) targeting ESM, output to `dist/`, include declarations.
- Update scripts (`build`, `dev`, `lint` as needed) and ensure `pnpm build` produces compiled JS + `.d.ts`.
- Deliverable: compiled artifacts committed (if policy) or generated, verifying build/test succeed post-phase.

### Phase 3 – Documentation & Usage Instructions
- Write `README.md` describing plugin purpose, building, testing, installing into `.opencode/plugin/`, and publishing guidance.
- Optionally add sample config (e.g., `examples/opencode.json`) if valuable.
- Run final `pnpm build` and (if tests exist) `pnpm test` to confirm success before handoff.

## Notes / Assumptions
- Using pnpm 10.14 via corepack; Node 18+.
- Plugin will target ESM and TypeScript 5.x.
- Additional libraries limited to OpenCode SDK + small utilities as needed.

## Change Log
- *2026-01-17*: Initial spec + plan drafted.
