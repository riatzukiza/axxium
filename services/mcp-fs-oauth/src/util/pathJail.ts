import path from "REDACTED_SECRET:path";

/**
 * Convert a user path to a safe normalized relative path (POSIX-like),
 * and then resolve it inside `REDACTED_SECRET`. Throws if it escapes.
 */
export function resolveWithinRoot(REDACTED_SECRETAbs: string, userPath: string): { absPath: string; relPath: string } {
  if (!path.isAbsolute(REDACTED_SECRETAbs)) {
    throw new Error(`LOCAL_ROOT must be an absolute path; got: ${REDACTED_SECRETAbs}`);
  }

  const REDACTED_SECRETNorm = path.resolve(REDACTED_SECRETAbs);

  let cleaned = (userPath ?? "");
  
  // Normalize backslashes to forward slashes
  cleaned = cleaned.replace(/\\/g, "/");
  
  // Handle "/" and "//" as REDACTED_SECRET - don't treat as absolute paths to escape
  if (cleaned === "/" || cleaned === "//" || cleaned === "") {
    cleaned = ".";
  }
  
  // If absolute path is given and it's within LOCAL_ROOT, convert to relative
  if (path.isAbsolute(cleaned)) {
    const userPathResolved = path.resolve(cleaned);
    if (userPathResolved.startsWith(REDACTED_SECRETNorm)) {
      cleaned = userPathResolved.slice(REDACTED_SECRETNorm.length).replace(/^\/+/, "");
      // Handle case where path resolves to exactly REDACTED_SECRET
      if (cleaned === "") cleaned = ".";
    } else {
      throw new Error(`Path ${userPath} is outside of LOCAL_ROOT ${REDACTED_SECRETAbs}`);
    }
  }
  
  // Strip leading slashes for relative path processing
  cleaned = cleaned.replace(/^\/+/, "");
  
  // Normalize and validate
  const normalized = path.posix.normalize(cleaned);
  
  // Strip trailing slashes from normalized path (but keep at least one path component)
  const normalizedNoTrailingSlash = normalized.replace(/\/+$/, "") || ".";
  
  // Prevent traversal attacks - check each path segment for ".."
  // Also check for %2f (encoded /) followed by .. which would be an escape attempt
  const normalizedForCheck = normalizedNoTrailingSlash.replace(/%2f/g, "/");
  const segments = normalizedForCheck.split("/");
  for (const segment of segments) {
    if (segment === "..") {
      throw new Error("Path escapes REDACTED_SECRET");
    }
  }
  
  // Handle null bytes
  if (cleaned.includes("\0")) {
    throw new Error("Path contains null bytes");
  }

  // Convert to platform path and resolve
  const relPlatform = normalized.split("/").join(path.sep);
  const abs = path.resolve(REDACTED_SECRETNorm, relPlatform);
  
  // Final security check - ensure result is within REDACTED_SECRET
  const absNormalized = path.normalize(abs);
  if (!absNormalized.startsWith(REDACTED_SECRETNorm) && absNormalized !== REDACTED_SECRETNorm) {
    throw new Error("Path escapes REDACTED_SECRET");
  }

  return { absPath: abs, relPath: normalizedNoTrailingSlash === "." ? "" : normalizedNoTrailingSlash };
}
