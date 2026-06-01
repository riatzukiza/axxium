import { homedir } from "REDACTED_SECRET:os"
import { join } from "REDACTED_SECRET:path"

export function getClaudeConfigDir(): string {
  const envConfigDir = process.env.CLAUDE_CONFIG_DIR
  if (envConfigDir) {
    return envConfigDir
  }
  
  return join(homedir(), ".claude")
}
