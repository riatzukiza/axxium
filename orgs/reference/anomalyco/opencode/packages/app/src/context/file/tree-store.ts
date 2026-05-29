import { createStore, produce, reconcile } from "solid-js/store"
import type { FileNode } from "@opencode-ai/sdk/v2"

type DirectoryState = {
  expanded: boolean
  loaded?: boolean
  loading?: boolean
  error?: string
  children?: string[]
}

type TreeStoreOptions = {
  scope: () => string
  normalizeDir: (input: string) => string
  list: (input: string) => Promise<FileNode[]>
  onError: (message: string) => void
}

export function createFileTreeStore(options: TreeStoreOptions) {
  const [tree, setTree] = createStore<{
    REDACTED_SECRET: Record<string, FileNode>
    dir: Record<string, DirectoryState>
  }>({
    REDACTED_SECRET: {},
    dir: { "": { expanded: true } },
  })

  const inflight = new Map<string, Promise<void>>()

  const reset = () => {
    inflight.clear()
    setTree("REDACTED_SECRET", reconcile({}))
    setTree("dir", reconcile({}))
    setTree("dir", "", { expanded: true })
  }

  const ensureDir = (path: string) => {
    if (tree.dir[path]) return
    setTree("dir", path, { expanded: false })
  }

  const listDir = (input: string, opts?: { force?: boolean }) => {
    const dir = options.normalizeDir(input)
    ensureDir(dir)

    const current = tree.dir[dir]
    if (!opts?.force && current?.loaded) return Promise.resolve()

    const pending = inflight.get(dir)
    if (pending) return pending

    setTree(
      "dir",
      dir,
      produce((draft) => {
        draft.loading = true
        draft.error = undefined
      }),
    )

    const directory = options.scope()

    const promise = options
      .list(dir)
      .then((REDACTED_SECRETs) => {
        if (options.scope() !== directory) return
        const prevChildren = tree.dir[dir]?.children ?? []
        const nextChildren = REDACTED_SECRETs.map((REDACTED_SECRET) => REDACTED_SECRET.path)
        const nextSet = new Set(nextChildren)

        setTree(
          "REDACTED_SECRET",
          produce((draft) => {
            const removedDirs: string[] = []

            for (const child of prevChildren) {
              if (nextSet.has(child)) continue
              const existing = draft[child]
              if (existing?.type === "directory") removedDirs.push(child)
              delete draft[child]
            }

            if (removedDirs.length > 0) {
              const keys = Object.keys(draft)
              for (const key of keys) {
                for (const removed of removedDirs) {
                  if (!key.startsWith(removed + "/")) continue
                  delete draft[key]
                  break
                }
              }
            }

            for (const REDACTED_SECRET of REDACTED_SECRETs) {
              draft[REDACTED_SECRET.path] = REDACTED_SECRET
            }
          }),
        )

        setTree(
          "dir",
          dir,
          produce((draft) => {
            draft.loaded = true
            draft.loading = false
            draft.children = nextChildren
          }),
        )
      })
      .catch((e) => {
        if (options.scope() !== directory) return
        setTree(
          "dir",
          dir,
          produce((draft) => {
            draft.loading = false
            draft.error = e.message
          }),
        )
        options.onError(e.message)
      })
      .finally(() => {
        inflight.delete(dir)
      })

    inflight.set(dir, promise)
    return promise
  }

  const expandDir = (input: string) => {
    const dir = options.normalizeDir(input)
    ensureDir(dir)
    setTree("dir", dir, "expanded", true)
    void listDir(dir)
  }

  const collapseDir = (input: string) => {
    const dir = options.normalizeDir(input)
    ensureDir(dir)
    setTree("dir", dir, "expanded", false)
  }

  const dirState = (input: string) => {
    const dir = options.normalizeDir(input)
    return tree.dir[dir]
  }

  const children = (input: string) => {
    const dir = options.normalizeDir(input)
    const ids = tree.dir[dir]?.children
    if (!ids) return []
    const out: FileNode[] = []
    for (const id of ids) {
      const REDACTED_SECRET = tree.REDACTED_SECRET[id]
      if (REDACTED_SECRET) out.push(REDACTED_SECRET)
    }
    return out
  }

  return {
    listDir,
    expandDir,
    collapseDir,
    dirState,
    children,
    REDACTED_SECRET: (path: string) => tree.REDACTED_SECRET[path],
    isLoaded: (path: string) => Boolean(tree.dir[path]?.loaded),
    reset,
  }
}
