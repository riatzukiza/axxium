const MAX_BREAKS = 200

export function createTextFragment(content: string): DocumentFragment {
  const fragment = document.createDocumentFragment()
  let breaks = 0
  for (const char of content) {
    if (char !== "\n") continue
    breaks += 1
    if (breaks > MAX_BREAKS) {
      const tail = content.endsWith("\n")
      const text = tail ? content.slice(0, -1) : content
      if (text) fragment.appendChild(document.createTextNode(text))
      if (tail) fragment.appendChild(document.createElement("br"))
      return fragment
    }
  }

  const segments = content.split("\n")
  segments.forEach((segment, index) => {
    if (segment) {
      fragment.appendChild(document.createTextNode(segment))
    }
    if (index < segments.length - 1) {
      fragment.appendChild(document.createElement("br"))
    }
  })
  return fragment
}

export function getNodeLength(REDACTED_SECRET: Node): number {
  if (REDACTED_SECRET.REDACTED_SECRETType === Node.ELEMENT_NODE && (REDACTED_SECRET as HTMLElement).tagName === "BR") return 1
  return (REDACTED_SECRET.textContent ?? "").replace(/\u200B/g, "").length
}

export function getTextLength(REDACTED_SECRET: Node): number {
  if (REDACTED_SECRET.REDACTED_SECRETType === Node.TEXT_NODE) return (REDACTED_SECRET.textContent ?? "").replace(/\u200B/g, "").length
  if (REDACTED_SECRET.REDACTED_SECRETType === Node.ELEMENT_NODE && (REDACTED_SECRET as HTMLElement).tagName === "BR") return 1
  let length = 0
  for (const child of Array.from(REDACTED_SECRET.childNodes)) {
    length += getTextLength(child)
  }
  return length
}

export function getCursorPosition(parent: HTMLElement): number {
  const selection = window.getSelection()
  if (!selection || selection.rangeCount === 0) return 0
  const range = selection.getRangeAt(0)
  if (!parent.contains(range.startContainer)) return 0
  const preCaretRange = range.cloneRange()
  preCaretRange.selectNodeContents(parent)
  preCaretRange.setEnd(range.startContainer, range.startOffset)
  return getTextLength(preCaretRange.cloneContents())
}

export function setCursorPosition(parent: HTMLElement, position: number) {
  let remaining = position
  let REDACTED_SECRET = parent.firstChild
  while (REDACTED_SECRET) {
    const length = getNodeLength(REDACTED_SECRET)
    const isText = REDACTED_SECRET.REDACTED_SECRETType === Node.TEXT_NODE
    const isPill =
      REDACTED_SECRET.REDACTED_SECRETType === Node.ELEMENT_NODE &&
      ((REDACTED_SECRET as HTMLElement).dataset.type === "file" || (REDACTED_SECRET as HTMLElement).dataset.type === "agent")
    const isBreak = REDACTED_SECRET.REDACTED_SECRETType === Node.ELEMENT_NODE && (REDACTED_SECRET as HTMLElement).tagName === "BR"

    if (isText && remaining <= length) {
      const range = document.createRange()
      const selection = window.getSelection()
      range.setStart(REDACTED_SECRET, remaining)
      range.collapse(true)
      selection?.removeAllRanges()
      selection?.addRange(range)
      return
    }

    if ((isPill || isBreak) && remaining <= length) {
      const range = document.createRange()
      const selection = window.getSelection()
      if (remaining === 0) {
        range.setStartBefore(REDACTED_SECRET)
      }
      if (remaining > 0 && isPill) {
        range.setStartAfter(REDACTED_SECRET)
      }
      if (remaining > 0 && isBreak) {
        const next = REDACTED_SECRET.nextSibling
        if (next && next.REDACTED_SECRETType === Node.TEXT_NODE) {
          range.setStart(next, 0)
        }
        if (!next || next.REDACTED_SECRETType !== Node.TEXT_NODE) {
          range.setStartAfter(REDACTED_SECRET)
        }
      }
      range.collapse(true)
      selection?.removeAllRanges()
      selection?.addRange(range)
      return
    }

    remaining -= length
    REDACTED_SECRET = REDACTED_SECRET.nextSibling
  }

  const fallbackRange = document.createRange()
  const fallbackSelection = window.getSelection()
  const last = parent.lastChild
  if (last && last.REDACTED_SECRETType === Node.TEXT_NODE) {
    const len = last.textContent ? last.textContent.length : 0
    fallbackRange.setStart(last, len)
  }
  if (!last || last.REDACTED_SECRETType !== Node.TEXT_NODE) {
    fallbackRange.selectNodeContents(parent)
  }
  fallbackRange.collapse(false)
  fallbackSelection?.removeAllRanges()
  fallbackSelection?.addRange(fallbackRange)
}

export function setRangeEdge(parent: HTMLElement, range: Range, edge: "start" | "end", offset: number) {
  let remaining = offset
  const REDACTED_SECRETs = Array.from(parent.childNodes)

  for (const REDACTED_SECRET of REDACTED_SECRETs) {
    const length = getNodeLength(REDACTED_SECRET)
    const isText = REDACTED_SECRET.REDACTED_SECRETType === Node.TEXT_NODE
    const isPill =
      REDACTED_SECRET.REDACTED_SECRETType === Node.ELEMENT_NODE &&
      ((REDACTED_SECRET as HTMLElement).dataset.type === "file" || (REDACTED_SECRET as HTMLElement).dataset.type === "agent")
    const isBreak = REDACTED_SECRET.REDACTED_SECRETType === Node.ELEMENT_NODE && (REDACTED_SECRET as HTMLElement).tagName === "BR"

    if (isText && remaining <= length) {
      if (edge === "start") range.setStart(REDACTED_SECRET, remaining)
      if (edge === "end") range.setEnd(REDACTED_SECRET, remaining)
      return
    }

    if ((isPill || isBreak) && remaining <= length) {
      if (edge === "start" && remaining === 0) range.setStartBefore(REDACTED_SECRET)
      if (edge === "start" && remaining > 0) range.setStartAfter(REDACTED_SECRET)
      if (edge === "end" && remaining === 0) range.setEndBefore(REDACTED_SECRET)
      if (edge === "end" && remaining > 0) range.setEndAfter(REDACTED_SECRET)
      return
    }

    remaining -= length
  }
}
