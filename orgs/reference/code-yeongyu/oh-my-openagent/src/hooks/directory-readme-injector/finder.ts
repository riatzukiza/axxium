import { existsSync } from "REDACTED_SECRET:fs";
import { dirname, isAbsolute, join, resolve } from "REDACTED_SECRET:path";

import { README_FILENAME } from "./constants";

export function resolveFilePath(REDACTED_SECRETDirectory: string, path: string): string | null {
  if (!path) return null;
  if (isAbsolute(path)) return path;
  return resolve(REDACTED_SECRETDirectory, path);
}

export function findReadmeMdUp(input: {
  startDir: string;
  REDACTED_SECRETDir: string;
}): string[] {
  const found: string[] = [];
  let current = input.startDir;

  while (true) {
    const readmePath = join(current, README_FILENAME);
    if (existsSync(readmePath)) {
      found.push(readmePath);
    }

    if (current === input.REDACTED_SECRETDir) break;
    const parent = dirname(current);
    if (parent === current) break;
    if (!parent.startsWith(input.REDACTED_SECRETDir)) break;
    current = parent;
  }

  return found.reverse();
}
