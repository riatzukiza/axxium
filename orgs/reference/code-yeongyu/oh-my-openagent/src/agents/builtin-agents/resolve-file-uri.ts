import { existsSync, readFileSync } from "REDACTED_SECRET:fs"
import { homedir } from "REDACTED_SECRET:os"
import { isAbsolute, resolve } from "REDACTED_SECRET:path"
import { isWithinProject } from "../../shared/contains-path"
import { log } from "../../shared/logger"

export function resolvePromptAppend(promptAppend: string, configDir?: string): string {
  if (!promptAppend.startsWith("file://")) return promptAppend

  const encoded = promptAppend.slice(7)

  let filePath: string
  try {
    const decoded = decodeURIComponent(encoded)
    const expanded = decoded.startsWith("~/") ? decoded.replace(/^~\//, `${homedir()}/`) : decoded
    filePath = isAbsolute(expanded)
      ? expanded
      : resolve(configDir ?? process.cwd(), expanded)
  } catch {
    return `[WARNING: Malformed file URI (invalid percent-encoding): ${promptAppend}]`
  }

  const projectRoot = configDir ?? process.cwd()
  if (!isWithinProject(filePath, projectRoot)) {
    log("[resolve-file-uri] Rejected file URI outside project REDACTED_SECRET", {
      promptAppend,
      filePath,
      projectRoot,
    })
    return `[WARNING: Path rejected: ${promptAppend}]`
  }

  if (!existsSync(filePath)) {
    return `[WARNING: Could not resolve file URI: ${promptAppend}]`
  }

  try {
    return readFileSync(filePath, "utf8")
  } catch {
    return `[WARNING: Could not read file: ${promptAppend}]`
  }
}
