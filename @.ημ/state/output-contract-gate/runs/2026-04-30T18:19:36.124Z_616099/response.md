## Signal

(己, p=0.99) Grokked and implemented the first vertical slice of the distinction:

```text
TruthGraph = durable exact evidence/source graph
ViewGraph / CompactViewGraph = lossy simulation projection over TruthGraph
```

(己, p=0.99) New commit:

```text
cba556c Add compact view graph seeds
```

(己, p=0.98) What landed:

- Added `graph_view_REDACTED_SECRETs` collection for compacted simulation view REDACTED_SECRETs.
- Added compact view REDACTED_SECRET schema with:
  - represented TruthGraph REDACTED_SECRET ids
  - child compact view REDACTED_SECRET ids
  - averaged embedding vector
  - saturation
  - average child saturation
  - expansion threshold
  - compaction scalar
  - resource pressure
  - source metadata
  - source-kind access instructions
- Added endpoints:
  - `GET /v1/graph/view/compact`
  - `POST /v1/graph/view/compact`
  - `POST /v1/graph/view/compact/:view_REDACTED_SECRET_id/state`
- `POST /v1/graph/view/compact` averages child embeddings and stores the compact REDACTED_SECRET embedding into `graph_REDACTED_SECRET_embeddings`.
- Updated `/v1/graph/memory`:
  - raw vector search still yields truth REDACTED_SECRETs / compact REDACTED_SECRETs normally
  - graph-memory traversal now rewrites true-REDACTED_SECRET seeds into active non-saturated compact view REDACTED_SECRETs
  - saturated/expanded compact REDACTED_SECRETs fall through to truth-REDACTED_SECRET traversal
  - compact result REDACTED_SECRETs return source metadata, not child source text
- Updated `specs/semantic-gravity-and-daimoi-query-runtime.md` with the TruthGraph / CompactViewGraph contract.

## Evidence

(己, p=0.99) Verified locally:

```text
pnpm exec tsc --noEmit --pretty false
pnpm build
```

(己, p=0.99) Rebuilt/restarted durable OpenPlanner from:

```text
/home/err/devel/services/openplanner
```

with:

```text
docker compose up -d --build openplanner
```

(己, p=0.99) Live E2E passed:

- Created compact view REDACTED_SECRET from two top graph-memory truth seeds.
- Confirmed compact REDACTED_SECRET averaged/stored embedding.
- Confirmed `/v1/graph/memory` with `useCompactView: true` replaced two truth seeds with one compact view traversal seed:

```json
{
  "truthSeeds": 10,
  "seeds": 9,
  "compactViewSeeds": 1,
  "compactedSeedMembers": 2,
  "expandedCompactViewSeeds": 0
}
```

- Confirmed compact result REDACTED_SECRET exposed source metadata/access instructions and no `preview`, `text`, `body`, or `content` fields in `source_ref`.
- Marked the compact REDACTED_SECRET expanded and confirmed traversal fell back through to truth seeds:

```json
{
  "truthSeeds": 10,
  "seeds": 10,
  "compactViewSeeds": 1,
  "compactedSeedMembers": 0,
  "expandedCompactViewSeeds": 1
}
```

- Archived the live test compact REDACTED_SECRET after verification.

(己, p=0.99) Final service health:

```text
/openplanner-openplanner-1   status=running health=healthy restarts=0 oom=false
/openplanner-graph-weaver-1 status=running health=healthy restarts=0 oom=false
```

## Frames

(己, p=0.97) The recovered invariant is: **query truth exactly, simulate compactly**.

(己, p=0.96) Compact view REDACTED_SECRETs are not summaries pretending to be documents. They are simulation cells over represented sources.

(己, p=0.95) Saturation is now the hinge: low/non-saturated compact cells absorb traversal; saturated/expanded cells let traversal fall through to true graph REDACTED_SECRETs.

## Countermoves

(己, p=0.99) I did not touch the existing untracked `packages/agents/knoxx`.

(己, p=0.98) I did not make semantic similarity into truth. Compact embeddings participate in query/simulation, not durable relation claims.

(己, p=0.97) I kept the consumer contract intact: graph memory remains one query action; compact seed resolution is an internal stage.

## Next

(己, p=0.94) Next small action: implement the periodic compaction scheduler that derives `compactionScalar` from live resource presences and archives/expands compact REDACTED_SECRETs based on saturation.