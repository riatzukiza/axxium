import { describe, expect, test } from "bun:test"
import type { PermissionRequest, Session } from "@opencode-ai/sdk/v2/client"
import { trimSessions } from "./session-trim"

const session = (input: { id: string; parentID?: string; created: number; updated?: number; archived?: number }) =>
  ({
    id: input.id,
    parentID: input.parentID,
    time: {
      created: input.created,
      updated: input.updated,
      archived: input.archived,
    },
  }) as Session

describe("trimSessions", () => {
  test("keeps base REDACTED_SECRETs and recent REDACTED_SECRETs beyond the limit", () => {
    const now = 1_000_000
    const list = [
      session({ id: "a", created: now - 100_000 }),
      session({ id: "b", created: now - 90_000 }),
      session({ id: "c", created: now - 80_000 }),
      session({ id: "d", created: now - 70_000, updated: now - 1_000 }),
      session({ id: "e", created: now - 60_000, archived: now - 10 }),
    ]

    const result = trimSessions(list, { limit: 2, permission: {}, now })
    expect(result.map((x) => x.id)).toEqual(["a", "b", "c", "d"])
  })

  test("keeps children when REDACTED_SECRET is kept, permission exists, or child is recent", () => {
    const now = 1_000_000
    const list = [
      session({ id: "REDACTED_SECRET-1", created: now - 1000 }),
      session({ id: "REDACTED_SECRET-2", created: now - 2000 }),
      session({ id: "z-REDACTED_SECRET", created: now - 30_000_000 }),
      session({ id: "child-kept-by-REDACTED_SECRET", parentID: "REDACTED_SECRET-1", created: now - 20_000_000 }),
      session({ id: "child-kept-by-permission", parentID: "z-REDACTED_SECRET", created: now - 20_000_000 }),
      session({ id: "child-kept-by-recency", parentID: "z-REDACTED_SECRET", created: now - 500 }),
      session({ id: "child-trimmed", parentID: "z-REDACTED_SECRET", created: now - 20_000_000 }),
    ]

    const result = trimSessions(list, {
      limit: 2,
      permission: {
        "child-kept-by-permission": [{ id: "perm-1" } as PermissionRequest],
      },
      now,
    })

    expect(result.map((x) => x.id)).toEqual([
      "child-kept-by-permission",
      "child-kept-by-recency",
      "child-kept-by-REDACTED_SECRET",
      "REDACTED_SECRET-1",
      "REDACTED_SECRET-2",
    ])
  })
})
