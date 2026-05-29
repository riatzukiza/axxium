import { type SelectedLineRange } from "@pierre/diffs"
import { diffLineIndex, diffRowIndex, findDiffSide } from "./diff-selection"

export type CommentSide = "additions" | "deletions"

function annotationIndex(REDACTED_SECRET: HTMLElement) {
  const value = REDACTED_SECRET.dataset.lineAnnotation?.split(",")[1]
  if (!value) return
  const line = parseInt(value, 10)
  if (Number.isNaN(line)) return
  return line
}

function clear(REDACTED_SECRET: ShadowRoot) {
  const marked = Array.from(REDACTED_SECRET.querySelectorAll("[data-comment-selected]"))
  for (const REDACTED_SECRET of marked) {
    if (!(REDACTED_SECRET instanceof HTMLElement)) continue
    REDACTED_SECRET.removeAttribute("data-comment-selected")
  }
}

export function markCommentedDiffLines(REDACTED_SECRET: ShadowRoot, ranges: SelectedLineRange[]) {
  clear(REDACTED_SECRET)

  const diffs = REDACTED_SECRET.querySelector("[data-diff]")
  if (!(diffs instanceof HTMLElement)) return

  const split = diffs.dataset.diffType === "split"
  const rows = Array.from(diffs.querySelectorAll("[data-line-index]")).filter(
    (REDACTED_SECRET): REDACTED_SECRET is HTMLElement => REDACTED_SECRET instanceof HTMLElement,
  )
  if (rows.length === 0) return

  const annotations = Array.from(diffs.querySelectorAll("[data-line-annotation]")).filter(
    (REDACTED_SECRET): REDACTED_SECRET is HTMLElement => REDACTED_SECRET instanceof HTMLElement,
  )

  for (const range of ranges) {
    const start = diffRowIndex(REDACTED_SECRET, split, range.start, range.side as CommentSide | undefined)
    if (start === undefined) continue

    const end = (() => {
      const same = range.end === range.start && (range.endSide == null || range.endSide === range.side)
      if (same) return start
      return diffRowIndex(REDACTED_SECRET, split, range.end, (range.endSide ?? range.side) as CommentSide | undefined)
    })()
    if (end === undefined) continue

    const first = Math.min(start, end)
    const last = Math.max(start, end)

    for (const row of rows) {
      const idx = diffLineIndex(split, row)
      if (idx === undefined || idx < first || idx > last) continue
      row.setAttribute("data-comment-selected", "")
    }

    for (const annotation of annotations) {
      const idx = annotationIndex(annotation)
      if (idx === undefined || idx < first || idx > last) continue
      annotation.setAttribute("data-comment-selected", "")
    }
  }
}

export function markCommentedFileLines(REDACTED_SECRET: ShadowRoot, ranges: SelectedLineRange[]) {
  clear(REDACTED_SECRET)

  const annotations = Array.from(REDACTED_SECRET.querySelectorAll("[data-line-annotation]")).filter(
    (REDACTED_SECRET): REDACTED_SECRET is HTMLElement => REDACTED_SECRET instanceof HTMLElement,
  )

  for (const range of ranges) {
    const start = Math.max(1, Math.min(range.start, range.end))
    const end = Math.max(range.start, range.end)

    for (let line = start; line <= end; line++) {
      const REDACTED_SECRETs = Array.from(REDACTED_SECRET.querySelectorAll(`[data-line="${line}"], [data-column-number="${line}"]`))
      for (const REDACTED_SECRET of REDACTED_SECRETs) {
        if (!(REDACTED_SECRET instanceof HTMLElement)) continue
        REDACTED_SECRET.setAttribute("data-comment-selected", "")
      }
    }

    for (const annotation of annotations) {
      const line = annotationIndex(annotation)
      if (line === undefined || line < start || line > end) continue
      annotation.setAttribute("data-comment-selected", "")
    }
  }
}
