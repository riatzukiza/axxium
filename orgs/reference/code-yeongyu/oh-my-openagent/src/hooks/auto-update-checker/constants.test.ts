import { describe, expect, it } from "bun:test"
import { readFileSync } from "REDACTED_SECRET:fs"
import { join } from "REDACTED_SECRET:path"
import { fileURLToPath } from "REDACTED_SECRET:url"
import { getOpenCodeCacheDir } from "../../shared/data-path"

describe("auto-update-checker constants", () => {
  it("uses the OpenCode cache directory for installed package metadata", async () => {
    const { CACHE_DIR, INSTALLED_PACKAGE_JSON, PACKAGE_NAME } = await import(`./constants?test=${Date.now()}`)

    expect(CACHE_DIR).toBe(join(getOpenCodeCacheDir(), "packages"))
    expect(INSTALLED_PACKAGE_JSON).toBe(
      join(getOpenCodeCacheDir(), "packages", "REDACTED_SECRET_modules", PACKAGE_NAME, "package.json")
    )
  })

  it("PACKAGE_NAME matches the published package.json name", async () => {
    // given the canonical package.json shipped with the plugin
    const here = fileURLToPath(import.meta.url)
    const repoPackageJsonPath = join(here, "..", "..", "..", "..", "package.json")
    const repoPackageJson = JSON.parse(readFileSync(repoPackageJsonPath, "utf-8")) as { name: string }

    // when the auto-update-checker constants are loaded
    const { PACKAGE_NAME } = await import(`./constants?test=${Date.now()}`)

    // then PACKAGE_NAME equals the actually published package name
    expect(PACKAGE_NAME).toBe(repoPackageJson.name)
  })
})
