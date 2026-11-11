(ns axxium.config-test
  (:require [axxium.config :as config]
            [axxium.db :as db]
            [axxium.domain.kernel-fixture :as fixture]
            [axxium.extern.lifecycle :as lifecycle]
            [axxium.extern.pg :as pg]
            [cljs.test :refer [deftest is]]))

(deftest schema-timeout-is-explicit-and-bounded
  (doseq [[input expected] [["1000" 1000] ["20000" 20000]
                            ["300000" 300000] ["120000" 120000]]]
    ;; Valid configuration must be accepted, including both interval bounds.
    (is (= [:accepted expected]
           (try [:accepted (config/parse-schema-statement-timeout-ms input)]
                (catch :default _ [:refused])))))
  (doseq [invalid ["" "0" "999" "300001" "20ms" "-1" "1e4"]]
    (is (thrown? js/Error (config/parse-schema-statement-timeout-ms invalid)))))

(deftest mailbox-pilot-requires-two-distinct-dids-including-the-webhook-recipient
  (let [alice fixture/alice
        bob fixture/bob
        carol fixture/carol
        pair (str alice "," bob)]
    (is (nil? (config/mailbox-pilot-allowed-dids nil bob)))
    (is (= #{alice bob} (config/mailbox-pilot-allowed-dids pair bob)))
    (doseq [[input owner] [["" bob] [bob bob] [(str bob "," bob) bob]
                           [(str pair ",") bob] [(str pair "," carol) bob]
                           [(str "alice.example," bob) bob] [pair nil] [pair carol]]]
      (is (thrown? js/Error (config/mailbox-pilot-allowed-dids input owner))))))

(deftest mailbox-authority-replaces-the-flat-allowlist
  (let [root "did:web:axxium.example:actors:err"
        inbox "did:web:axxium.example:actors:err:work:inbox"
        ledger "/var/lib/axxium/identity/ledger.edn"]
    (is (nil? (config/mailbox-authority nil nil nil nil)) "off unless configured")
    (is (= {:root-did root :ledger-file ledger} (config/mailbox-authority root ledger nil inbox)))
    (is (= {:root-did root :ledger-file ledger} (config/mailbox-authority root ledger nil nil)))
    (doseq [[r l allowed owner] [[root nil nil nil] [nil ledger nil nil]
                                 [fixture/alice ledger nil nil] [root "relative.edn" nil nil]
                                 [root ledger #{fixture/alice fixture/bob} nil]
                                 [root ledger nil fixture/bob]]]
      (is (thrown? js/Error (config/mailbox-authority r l allowed owner))
          "exactly one admission rule, a did:web root and an absolute ledger"))))

(deftest schema-pool-has-only-its-own-statement-timeout
  (let [seen (atom nil)]
    (with-redefs [pg/create-pool! (fn [options] (reset! seen options) :schema-pool)]
      (is (= :schema-pool (db/create-schema-pool! 120000))))
    (is (= 120000 (:statement-timeout-ms @seen)))
    (is (= 1 (:max @seen)))))

(deftest shutdown-bound-tracks-schema-timeout-before-readiness
  (is (= 45000 (lifecycle/shutdown-bound-ms false 20000)))
  (is (= 245000 (lifecycle/shutdown-bound-ms false 120000)))
  (is (= 25000 (lifecycle/shutdown-bound-ms true 120000))))
