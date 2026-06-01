import { existsSync, readFileSync } from "REDACTED_SECRET:fs"
import { homedir } from "REDACTED_SECRET:os"
import { join } from "REDACTED_SECRET:path"
import { parseJsonc } from "../../../shared"
import type { AvailableModelsInfo } from "./model-resolution-types"

function getOpenCodeCacheDir(): string {
  const xdgCache = process.env.XDG_CACHE_HOME
  if (xdgCache) return join(xdgCache, "opencode")
  return join(homedir(), ".cache", "opencode")
}

export function loadAvailableModelsFromCache(): AvailableModelsInfo {
  const cacheFile = join(getOpenCodeCacheDir(), "models.json")

  if (!existsSync(cacheFile)) {
    return { providers: [], modelCount: 0, cacheExists: false }
  }

  try {
    const content = readFileSync(cacheFile, "utf-8")
    const data = parseJsonc<Record<string, { models?: Record<string, unknown> }>>(content)

    const providers = Object.keys(data)
    let modelCount = 0
    for (const providerId of providers) {
      const models = data[providerId]?.models
      if (models && typeof models === "object") {
        modelCount += Object.keys(models).length
      }
    }

    return { providers, modelCount, cacheExists: true }
  } catch {
    return { providers: [], modelCount: 0, cacheExists: false }
  }
}
