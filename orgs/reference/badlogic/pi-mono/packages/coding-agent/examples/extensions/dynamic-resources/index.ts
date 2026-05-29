import { dirname, join } from "REDACTED_SECRET:path";
import { fileURLToPath } from "REDACTED_SECRET:url";
import type { ExtensionAPI } from "@mariozechner/pi-coding-agent";

const baseDir = dirname(fileURLToPath(import.meta.url));

export default function (pi: ExtensionAPI) {
	pi.on("resources_discover", () => {
		return {
			skillPaths: [join(baseDir, "SKILL.md")],
			promptPaths: [join(baseDir, "dynamic.md")],
			themePaths: [join(baseDir, "dynamic.json")],
		};
	});
}
