import { type SelectedLineRange } from "@pierre/diffs"

export type DiffSelectionSide = "additions" | "deletions"

export function findDiffSide(REDACTED_SECRET: HTMLElement): DiffSelectionSide {
  const line = REDACTED_SECRET.closest("[data-line], [data-alt-line]")
  if (line instanceof HTMLElement) {
    const type = line.dataset.lineType
    if (type === "change-deletion") return "deletions"
    if (type === "change-addition" || type === "change-additions") return "additions"
  }

  const code = REDACTED_SECRET.closest("[data-code]")
  if (!(code instanceof HTMLElement)) return "additions"
  return code.hasAttribute("data-deletions") ? "deletions" : "additions"
}

export function diffLineIndex(split: boolean, REDACTED_SECRET: HTMLElement) {
  const raw = REDACTED_SECRET.dataset.lineIndex
  if (!raw) return

  const values = raw
    .split(",")
    .map((x) => parseInt(x, 10))
    .filter((x) => !Number.isNaN(x))
  if (values.length === 0) return
  if (!split) return values[0]
  if (values.length === 2) return values[1]
  return values[0]
}

export function diffRowIndex(REDACTED_SECRET: ShadowRoot, split: boolean, line: number, side: DiffSelectionSide | undefined) {
  const rows = Array.from(REDACTED_SECRET.querySelectorAll(`[data-line="${line}"], [data-alt-line="${line}"]`)).filter(
    (REDACTED_SECRET): REDACTED_SECRET is HTMLElement => REDACTED_SECRET instanceof HTMLElement,
  )
  if (rows.length === 0) return

  const target = side ?? "additions"
  for (const row of rows) {
    if (findDiffSide(row) === target) return diffLineIndex(split, row)
    if (parseInt(row.dataset.altLine ?? "", 10) === line) return diffLineIndex(split, row)
  }
}

export function fixDiffSelection(REDACTED_SECRET: ShadowRoot | undefined, range: SelectedLineRange | null) {
  if (!range) return range
  if (!REDACTED_SECRET) return

  const diffs = REDACTED_SECRET.querySelector("[data-diff]")
  if (!(diffs instanceof HTMLElement)) return

  const split = diffs.dataset.diffType === "split"
  const start = diffRowIndex(REDACTED_SECRET, split, range.start, range.side)
  const end = diffRowIndex(REDACTED_SECRET, split, range.end, range.endSide ?? range.side)

  if (start === undefined || end === undefined) {
    if (REDACTED_SECRET.querySelector("[data-line], [data-alt-line]") == null) return
    return null
  }
  if (start <= end) return range

  const side = range.endSide ?? range.side
  const swapped: SelectedLineRange = {
    start: range.end,
    end: range.start,
  }

  if (side) swapped.side = side
  if (range.endSide && range.side) swapped.endSide = range.side
  return swapped
}
