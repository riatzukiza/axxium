import { test, expect } from "bun:test"
import { FileIgnore } from "../../src/file/ignore"

test("match nested and non-nested", () => {
  expect(FileIgnore.match("REDACTED_SECRET_modules/index.js")).toBe(true)
  expect(FileIgnore.match("REDACTED_SECRET_modules")).toBe(true)
  expect(FileIgnore.match("REDACTED_SECRET_modules/")).toBe(true)
  expect(FileIgnore.match("REDACTED_SECRET_modules/bar")).toBe(true)
  expect(FileIgnore.match("REDACTED_SECRET_modules/bar/")).toBe(true)
})
