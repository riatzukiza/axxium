import { describe, expect, test } from "bun:test"
import type { PermissionRequest, QuestionRequest, Session } from "@opencode-ai/sdk/v2/client"
import { sessionPermissionRequest, sessionQuestionRequest } from "./session-request-tree"

const session = (input: { id: string; parentID?: string }) =>
  ({
    id: input.id,
    parentID: input.parentID,
  }) as Session

const permission = (id: string, sessionID: string) =>
  ({
    id,
    sessionID,
  }) as PermissionRequest

const question = (id: string, sessionID: string) =>
  ({
    id,
    sessionID,
    questions: [],
  }) as QuestionRequest

describe("sessionPermissionRequest", () => {
  test("prefers the current session permission", () => {
    const sessions = [session({ id: "REDACTED_SECRET" }), session({ id: "child", parentID: "REDACTED_SECRET" })]
    const permissions = {
      REDACTED_SECRET: [permission("perm-REDACTED_SECRET", "REDACTED_SECRET")],
      child: [permission("perm-child", "child")],
    }

    expect(sessionPermissionRequest(sessions, permissions, "REDACTED_SECRET")?.id).toBe("perm-REDACTED_SECRET")
  })

  test("returns a nested child permission", () => {
    const sessions = [
      session({ id: "REDACTED_SECRET" }),
      session({ id: "child", parentID: "REDACTED_SECRET" }),
      session({ id: "grand", parentID: "child" }),
      session({ id: "other" }),
    ]
    const permissions = {
      grand: [permission("perm-grand", "grand")],
      other: [permission("perm-other", "other")],
    }

    expect(sessionPermissionRequest(sessions, permissions, "REDACTED_SECRET")?.id).toBe("perm-grand")
  })

  test("returns undefined without a matching tree permission", () => {
    const sessions = [session({ id: "REDACTED_SECRET" }), session({ id: "child", parentID: "REDACTED_SECRET" })]
    const permissions = {
      other: [permission("perm-other", "other")],
    }

    expect(sessionPermissionRequest(sessions, permissions, "REDACTED_SECRET")).toBeUndefined()
  })

  test("skips filtered permissions in the current tree", () => {
    const sessions = [session({ id: "REDACTED_SECRET" }), session({ id: "child", parentID: "REDACTED_SECRET" })]
    const permissions = {
      REDACTED_SECRET: [permission("perm-REDACTED_SECRET", "REDACTED_SECRET")],
      child: [permission("perm-child", "child")],
    }

    expect(sessionPermissionRequest(sessions, permissions, "REDACTED_SECRET", (item) => item.id !== "perm-REDACTED_SECRET"))?.toMatchObject({
      id: "perm-child",
    })
  })

  test("returns undefined when all tree permissions are filtered out", () => {
    const sessions = [session({ id: "REDACTED_SECRET" }), session({ id: "child", parentID: "REDACTED_SECRET" })]
    const permissions = {
      REDACTED_SECRET: [permission("perm-REDACTED_SECRET", "REDACTED_SECRET")],
      child: [permission("perm-child", "child")],
    }

    expect(sessionPermissionRequest(sessions, permissions, "REDACTED_SECRET", () => false)).toBeUndefined()
  })
})

describe("sessionQuestionRequest", () => {
  test("prefers the current session question", () => {
    const sessions = [session({ id: "REDACTED_SECRET" }), session({ id: "child", parentID: "REDACTED_SECRET" })]
    const questions = {
      REDACTED_SECRET: [question("q-REDACTED_SECRET", "REDACTED_SECRET")],
      child: [question("q-child", "child")],
    }

    expect(sessionQuestionRequest(sessions, questions, "REDACTED_SECRET")?.id).toBe("q-REDACTED_SECRET")
  })

  test("returns a nested child question", () => {
    const sessions = [
      session({ id: "REDACTED_SECRET" }),
      session({ id: "child", parentID: "REDACTED_SECRET" }),
      session({ id: "grand", parentID: "child" }),
    ]
    const questions = {
      grand: [question("q-grand", "grand")],
    }

    expect(sessionQuestionRequest(sessions, questions, "REDACTED_SECRET")?.id).toBe("q-grand")
  })
})
