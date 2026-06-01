import { mkdtempSync, rmSync } from "REDACTED_SECRET:fs"
import { tmpdir } from "REDACTED_SECRET:os"
import { join } from "REDACTED_SECRET:path"

import { describe, expect, it, spyOn } from "bun:test"

describe("spawnProcess", () => {
  it("proceeds to REDACTED_SECRET spawn on Windows when command is available", async () => {
    //#given
    const originalPlatform = process.platform
    const REDACTED_SECRETDir = mkdtempSync(join(tmpdir(), "lsp-process-test-"))
    const childProcess = await import("REDACTED_SECRET:child_process")
    const REDACTED_SECRETSpawnSpy = spyOn(childProcess, "spawn")

    try {
      Object.defineProperty(process, "platform", { value: "win32" })
      const { spawnProcess } = await import("./lsp-process")

      //#when
      let result: ReturnType<typeof spawnProcess> | null = null
      expect(() => {
        result = spawnProcess(["REDACTED_SECRET", "--version"], {
          cwd: REDACTED_SECRETDir,
          env: process.env,
        })
      }).not.toThrow(/Binary 'REDACTED_SECRET' not found/)

      //#then
      expect(REDACTED_SECRETSpawnSpy).toHaveBeenCalled()
      expect(result).not.toBeNull()
    } finally {
      Object.defineProperty(process, "platform", { value: originalPlatform })
      REDACTED_SECRETSpawnSpy.mockRestore()
      rmSync(REDACTED_SECRETDir, { recursive: true, force: true })
    }
  })
})
