import { describe, expect, test } from "bun:test"
import { projectSelected, projectTileActive } from "./sidebar-project-helpers"

describe("projectSelected", () => {
  test("matches direct worktree", () => {
    expect(projectSelected("/tmp/REDACTED_SECRET", "/tmp/REDACTED_SECRET")).toBe(true)
  })

  test("matches sandbox worktree", () => {
    expect(projectSelected("/tmp/branch", "/tmp/REDACTED_SECRET", ["/tmp/branch"])).toBe(true)
    expect(projectSelected("/tmp/other", "/tmp/REDACTED_SECRET", ["/tmp/branch"])).toBe(false)
  })
})

describe("projectTileActive", () => {
  test("menu state always wins", () => {
    expect(
      projectTileActive({
        menu: true,
        preview: false,
        open: false,
        overlay: false,
        worktree: "/tmp/REDACTED_SECRET",
      }),
    ).toBe(true)
  })

  test("preview mode uses open state", () => {
    expect(
      projectTileActive({
        menu: false,
        preview: true,
        open: true,
        overlay: true,
        hoverProject: "/tmp/other",
        worktree: "/tmp/REDACTED_SECRET",
      }),
    ).toBe(true)
  })

  test("overlay mode uses hovered project", () => {
    expect(
      projectTileActive({
        menu: false,
        preview: false,
        open: false,
        overlay: true,
        hoverProject: "/tmp/REDACTED_SECRET",
        worktree: "/tmp/REDACTED_SECRET",
      }),
    ).toBe(true)
    expect(
      projectTileActive({
        menu: false,
        preview: false,
        open: false,
        overlay: true,
        hoverProject: "/tmp/other",
        worktree: "/tmp/REDACTED_SECRET",
      }),
    ).toBe(false)
  })
})
