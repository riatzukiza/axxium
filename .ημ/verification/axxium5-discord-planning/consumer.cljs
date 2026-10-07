(ns axxium-discord-planning-receipt
  (:require [clojure.string :as str]
            [eta-mu.receipt-river.api :as api]
            [eta-mu.receipt-river.domain.receipt :as receipt]
            [eta-mu.receipt-river.shape.edn :as edn]
            ["node:fs" :as fs]
            ["node:child_process" :as child]))
(let [[mode target] *command-line-args*]
  (if (= mode "append")
    (let [now (.toISOString (js/Date.))
          payload (receipt/build-payload
                   {:kind :decision :owner "root/issues"
                    :origin "open-hax/axxium issue5 complete Discord OAuth planning"
                    :dod "Preserve all six requirements/four acceptance criteria on existing Todo/P0/5 card; review portable decisions, provider/state/link/credential/guild-role/UI/Axxium Bearer contracts and full sizing before implementation"
                    :pi "pr-sprint-planning+receipt-river" :host "independent-private-persistent-planning-worktree"
                    :manifest "kanban/axxium-discord-oauth.md docs/design/discord-oauth-planning.md docs/verification/discord-oauth-planning.md .ημ/verification/axxium5-discord-planning"
                    :refs "issue:open-hax/axxium5 base:2439d4d6b8e546cda276f09f5c96db59226ecad6 personalPR1:539409b169302793a0b67f97d75ddd6265cf9d12 originPR13:0e0a67299b61ae8ac309385a6400ca603ac3855f originPR26:18f92db5ea106b4f9dd3eb4d84b9da0fc6ee9f04 RR:154440f3c997aa9208194bba59b5edbef3654f78"
                    :tests "Actual installed owning Rheos reads unchanged configured original/proposed fixtures full card Todo/P0/5; seven inputs unchanged after each read. Independent complete Git fsck/source/fork/head guards and actual RR known-kind consumer. No backend implementation/compiler/test, real OAuth/provider/DB/browser/login/shared service or state transition. Future full gates and live acceptance NOT RUN."
                    :note "LOCAL proposal only. Existing auth/identity/portal branches overlap future source paths and remain separate unqualified ownership. Published automation eager floating squash hold is not fixed here. No approval, ready transition, production proof, closure, native mutation or branch adoption."
                    :decisions "Prefer portable CLJC shapes/laws, named extern effects; stable subject/link uniqueness and rollback, separate protected provider credentials/public allowlists, actual member-role evidence/trusted mapping/freshness/removal, real session/Bearer integration and portal. PKCE/storage/session/revocation/integration/full5point sizing require review; missing app/credentials/live proof are holds, not scope reduction."}
                   "open-hax/axxium" now :decision)
          event (api/build-event {:event-id (str (random-uuid)) :recorded-at now
                                 :component-manifest {:eta-mu/version "1.1.1"}
                                 :command "local complete Discord OAuth planning refinement"
                                 :producer {:actor "root/issues"} :subject {:repo "open-hax/axxium"}}
                                payload)
          line (edn/format-line event) result (api/validate-line line 1)]
      (when-not (:ok result) (throw (ex-info "owning receipt refused" (dissoc result :line :event))))
      (.appendFileSync fs target (str line "\n"))
      (prn (select-keys result [:ok :line-number :source/schema :errors])))
    (let [text (if (= mode "git")
                 (.execFileSync child "git" #js ["show" (str target ":.ημ/receipts.edn")] #js {:encoding "utf8"})
                 (.readFileSync fs target "utf8"))
          lines (str/split-lines text)
          results (mapv (fn [i line] (select-keys (api/validate-line line (inc i)) [:ok :line-number :source/schema :errors])) (range) lines)]
      (prn {:total-lines (count lines) :owned-addition (first results) :results results
            :historical-rows 0 :historical-not-qualified 0})
      (when-not (and (= 1 (count lines)) (every? :ok results)
                     (= :declared (get-in results [0 :source/schema :status])))
        (set! (.-exitCode js/process) 1)))))
