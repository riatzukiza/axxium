(ns axxium.extern.mailbox-integration-regression-test
  "Cross-repository regression contribution for riatzukiza/axxium#18.
   These tests specify missing guarantees; they are not implementation fixes.
   Only synthetic, newly created temporary files are used. Damaged fixtures
   are retained for diagnosis. No production configuration is read."
  (:require ["node:fs" :as fs]
            ["node:os" :as os]
            ["node:path" :as path]
            [axxium.domain.kernel-fixture :as kernel]
            [axxium.domain.mailbox-mcp :as mailbox]
            [axxium.domain.mailbox-mcp-test :as fixture]
            [axxium.extern.ledger-file :as ledger-file]
            [axxium.shape.ledger-edn :as edn-lines]
            [cljs.test :refer [deftest is testing]]
            [clojure.string :as str]))

(defn- successful
  "Require successful setup instead of letting missing values satisfy a test."
  [response]
  (is (nil? (:error response)) "Fixture operation must succeed")
  (when (:error response)
    (throw (ex-info "Mailbox fixture operation failed"
                    {:error (:error response)})))
  response)

(defn- send-to-bob
  "Send a synthetic body through the real mailbox domain."
  [ledger session key body]
  (successful
   (mailbox/call ledger (fixture/ctx fixture/alice session) :send_message
                 {:to (:auth/did fixture/bob)
                  :idempotency-key key
                  :body body})))

(deftest replay-compares-present-receipts
  (let [{:keys [lg a]} (fixture/world)
        first-send (send-to-bob lg a "receipt-control" {:text "control"})
        replay (send-to-bob (:ledger first-send) a "receipt-control" {:text "control"})
        receipt (get-in first-send [:result :message/receipt])]
    (is (map? receipt) "Never accept nil = nil as a receipt comparison")
    (is (= :accepted (:receipt/status receipt)))
    (is (= receipt (get-in replay [:result :message/receipt])))
    (is (= (:ledger first-send) (:ledger replay)))))

(deftest acknowledging-later-message-keeps-earlier-message-recoverable
  (let [{:keys [lg a b]} (fixture/world)
        first-send (send-to-bob lg a "gap-first" {:order 1})
        second-send (send-to-bob (:ledger first-send) a "gap-second" {:order 2})
        first-id (get-in first-send [:result :message/id])
        second-id (get-in second-send [:result :message/id])
        ack (successful
             (mailbox/call (:ledger second-send) (fixture/ctx fixture/bob b)
                           :acknowledge_message {:message-id second-id}))
        reopened (successful
                  (mailbox/call (:ledger ack)
                                {:principal fixture/bob
                                 :audience fixture/aud :now kernel/t0}
                                :open_session
                                {:audience fixture/aud :ttl-ms 3600000}))
        inbox (successful
               (mailbox/call (:ledger reopened)
                             (fixture/ctx fixture/bob
                                          (get-in reopened [:result :session/id]))
                             :read_inbox {}))]
    (is (string? first-id))
    (is (not= first-id second-id))
    (is (some #{first-id} (map :message/id (get-in inbox [:result :messages])))
        "Acknowledging B must not silently acknowledge unread A")))

(deftest arbitrary-json-body-keys-survive-the-real-rpc-roundtrip
  (let [{:keys [lg a b]} (fixture/world)
        body {"a/id" "left" "b/id" "right"
              "nested" {"x/value" 1 "y/value" 2}}
        sent (mailbox/handle-rpc
              lg (fixture/ctx fixture/alice a)
              {"jsonrpc" "2.0" "id" 1 "method" "tools/call"
               "params" {"name" "send_message"
                         "arguments" {"to" (:auth/did fixture/bob)
                                      "idempotency_key" "json-preservation"
                                      "body" body}}})
        read-back (mailbox/handle-rpc
                   (:ledger sent) (fixture/ctx fixture/bob b)
                   {"jsonrpc" "2.0" "id" 2 "method" "tools/call"
                    "params" {"name" "read_inbox" "arguments" {}}})
        messages (get-in read-back [:response "result" "structuredContent" "messages"])]
    (is (not (get-in sent [:response "result" "isError"])))
    (is (= 1 (count messages)))
    (is (= body (get (first messages) "message_body"))
        "Response DTO conversion must not rename or collapse payload keys")))

(deftest planning-push-does-not-record-external-delivery
  (let [{:keys [lg a]} (fixture/world)
        sent (send-to-bob lg a "plan-only" {:text "synthetic"})
        planned (mailbox/dispatch (:ledger sent) kernel/t0)
        delivery (successful
                  (mailbox/call (:ledger planned) (fixture/ctx fixture/alice a)
                                :get_delivery
                                {:message-id (get-in sent [:result :message/id])}))]
    (is (= 1 (count (:notifications planned))) "Positive control: push was planned")
    (is (not (#{:push :notified :delivered}
               (get-in delivery [:result :delivery/notification])))
        "No transport was invoked and no external receipt was supplied")
    (is (= :unknown (get-in delivery [:result :delivery/execution])))))

(deftest crash-after-dispatch-plan-does-not-lose-notification
  (let [{:keys [lg a]} (fixture/world)
        sent (send-to-bob lg a "plan-crash" {:text "synthetic"})
        planned (mailbox/dispatch (:ledger sent) kernel/t0)
        persisted-plan (edn-lines/import-lines
                        (edn-lines/export-lines (:ledger planned)))
        retried (mailbox/dispatch persisted-plan fixture/t-mid)
        message-id (get-in sent [:result :message/id])]
    (is (= 1 (count (:notifications planned))))
    (is (some #{message-id} (map :message/id (:notifications retried)))
        "Thirty minutes later, an unacknowledged plan must remain retryable")))

(defn- interrupted-read
  "Observe either a recovered ledger or a fail-closed reader refusal."
  [filename]
  (try
    {:ledger (ledger-file/load-ledger filename)}
    (catch :default _error
      {:refused? true})))

(deftest interrupted-batch-never-exposes-a-committed-prefix
  (let [{:keys [lg a]} (fixture/world)
        after (:ledger (send-to-bob lg a "batch-interruption" {:text "synthetic"}))
        directory (fs/mkdtempSync (path/join (os/tmpdir) "axxium-mailbox-batch-regression-"))
        complete-path (path/join directory "complete.edn")
        _ (ledger-file/append-batch! complete-path [] lg)
        prefix (fs/readFileSync complete-path "utf8")
        _ (ledger-file/append-batch! complete-path lg after)
        complete (fs/readFileSync complete-path "utf8")
        suffix (subs complete (count prefix))
        line-ends (keep-indexed (fn [i character]
                                 (when (= character \newline) (inc i))) suffix)
        cuts (->> (concat [1 (quot (count suffix) 2) (dec (count suffix))]
                          (mapcat (fn [end] [(dec end) end (inc end)]) line-ends))
                  (filter #(< 0 % (count suffix))) distinct sort)]
    (is (str/starts-with? complete prefix) "Previously committed bytes stay intact")
    (is (= after (ledger-file/load-ledger complete-path)) "Full batch is readable")
    (is (seq cuts) "Fault positions must actually be exercised")
    (doseq [cut cuts]
      (testing (str "interrupted after " cut " bytes of the synthetic ASCII suffix")
        (let [torn-path (path/join directory (str "torn-" cut ".edn"))
              _ (fs/writeFileSync torn-path (str prefix (subs suffix 0 cut)) "utf8")
              recovered (interrupted-read torn-path)]
          (is (or (:refused? recovered) (= lg (:ledger recovered)))
              "Recovery may refuse corruption or return the old commit, not a partial batch"))))))
