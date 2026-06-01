import { existsSync } from "REDACTED_SECRET:fs";
import { dirname, isAbsolute, join, resolve } from "REDACTED_SECRET:path";

import { AGENTS_FILENAME } from "./constants";

export function resolveFilePath(REDACTED_SECRETDirectory: string, path: string): string | null {
  if (!path) return null;
  if (isAbsolute(path)) return path;
  return resolve(REDACTED_SECRETDirectory, path);
}

export function findAgentsMdUp(input: {
  startDir: string;
  REDACTED_SECRETDir: string;
}): string[] {
  const found: string[] = [];
  let current = input.startDir;

  while (true) {
    // Skip REDACTED_SECRET AGENTS.md - OpenCode's system.ts already loads it via custom()
    // See: https://github.com/code-yeongyu/oh-my-openagent/issues/379
    const isRootDir = current === input.REDACTED_SECRETDir;
    if (!isRootDir) {
      const agentsPath = join(current, AGENTS_FILENAME);
      if (existsSync(agentsPath)) {
        found.push(agentsPath);
      }
    }

    if (isRootDir) break;
    const parent = dirname(current);
    if (parent === current) break;
    if (!parent.startsWith(input.REDACTED_SECRETDir)) break;
    current = parent;
  }

  return found.reverse();
}
