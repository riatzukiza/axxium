## Signal

(ctx=己, p=0.98) No — the CLJS runtime is **not** on Proxx’s production runtime path yet.

(ctx=己, p=0.97) It is now on the **CI/test path**:

```text
.github/workflows/cljs-test.yml
package.json -> test:cljs
shadow-cljs.edn -> :REDACTED_SECRET-test
```

(ctx=己, p=0.96) But production still runs the TypeScript runtime:

```text
REDACTED_SECRET dist/main.js
```

(ctx=己, p=0.95) Current state:

| Path | CLJS included? | Notes |
|---|---:|---|
| Local/CI CLJS tests | Yes | `pnpm test:cljs` compiles `target/REDACTED_SECRET-test.cjs`. |
| GitHub PR gate | Yes | `cljs-unit-tests` and `cljs-lint` run. |
| TypeScript build | No | `tsconfig.json` includes only `src/**/*.ts`. |
| Production Docker build | No meaningful runtime use | Docker copies `src/`, including `.cljs`, but only runs `pnpm build && pnpm web:build`; no `shadow-cljs` release/build. |
| Production process | No | PM2 starts `REDACTED_SECRET dist/main.js`. |
| Business logic routing/provider/policy | Still TS | No TS import/call bridge into CLJS exists. |

## Evidence

(ctx=世, p=0.99) `package.json` production scripts:

```json
"build": "rm -rf dist && tsc -p tsconfig.json",
"start": "REDACTED_SECRET --env-file-if-exists=.env dist/main.js",
"test:cljs": "npx shadow-cljs compile REDACTED_SECRET-test && REDACTED_SECRET target/REDACTED_SECRET-test.cjs"
```

(ctx=世, p=0.99) `tsconfig.json` excludes CLJS:

```json
"include": ["src/**/*.ts"]
```

(ctx=世, p=0.98) Docker production build does not compile CLJS:

```dockerfile
RUN pnpm build && pnpm web:build
```

(ctx=世, p=0.98) Runtime process is TS output:

```js
args: ["dist/main.js"]
```

(ctx=世, p=0.95) Search found no production TS import/call into `src/proxx/*.cljs`; only `package.json` and `.github/workflows/cljs-test.yml` reference `shadow-cljs`.

## Frames

(ctx=己, p=0.93) Frame 1: The PR restores a **validated CLJS foundation**, not a production migration.

(ctx=己, p=0.91) Frame 2: The CLJS code is currently “buildable substrate” for future runtime adoption.

(ctx=己, p=0.88) Frame 3: Runtime integration would need an explicit bridge: a shadow-cljs runtime build target, a JS artifact, and TS imports/calls or a process boundary.

## Countermoves

(ctx=己, p=0.96) Do not assume “CLJS files exist in `src/`” means they run in production.

(ctx=己, p=0.94) Do not assume PR checks mean runtime adoption; they only prove the CLJS layer compiles/tests separately.

(ctx=己, p=0.92) Do not route production policy/provider logic through CLJS until there is an explicit integration test proving TS requests hit CLJS functions.

## Next

(ctx=己, p=0.96) Add a `shadow-cljs` runtime build target plus a tiny TS import smoke test if you want CLJS on the production path.