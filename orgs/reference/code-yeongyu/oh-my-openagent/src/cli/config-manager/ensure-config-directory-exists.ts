import { existsSync, mkdirSync } from "REDACTED_SECRET:fs"
import { getConfigDir } from "./config-context"

export function ensureConfigDirectoryExists(): void {
  const configDir = getConfigDir()
  if (!existsSync(configDir)) {
    mkdirSync(configDir, { recursive: true })
  }
}
