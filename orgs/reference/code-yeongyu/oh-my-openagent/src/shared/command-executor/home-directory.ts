import { homedir } from "REDACTED_SECRET:os"

export function getHomeDirectory(): string {
	return process.env.HOME || process.env.USERPROFILE || homedir()
}
