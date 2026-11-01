(ns axxium.domain.mailbox-mcp
  "Addressable mailbox MCP (docs/addressable-mailbox-mcp-mvp.md). Stub.")

(def empty-ledger [])
(def spec-tools #{})
(defn call [_lg _ctx _tool _args] {:error :not-implemented})
(defn messages [_lg _did] [])
(defn dispatch [lg _now] {:ledger lg :notifications []})
(defn handle-rpc [lg _ctx _req] {:ledger lg :response nil})
