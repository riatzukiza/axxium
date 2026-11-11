(ns axxium.domain.mailbox-authority-test
  "L1–L8 of identity-tree mailbox admission, over a seeded tree. Pure."
  (:require [axxium.domain.github-identity :as gi]
            [axxium.domain.kernel-fixture :as fx]
            [axxium.domain.mailbox-authority :as authority]
            [axxium.domain.state :as state]
            [cljs.test :refer [deftest is testing]]))

(def host "axxium.example")
(defn- did [path] (str "did:web:" host ":actors:" path))

(def err (did "err"))
(def work (did "err:work"))
(def sender (did "err:work:sender"))
(def inbox (did "err:work:inbox"))
(def grokbots (did "err:grokbots"))
(def grok-bot (did "err:grokbots:grok-bot"))
(def mallory (did "mallory"))
(def mallory-bot (did "mallory:bot"))

(def sender-plc "did:plc:aaaaaaaaaaaaaaaaaaaaaaaa")
(def inbox-plc "did:plc:bbbbbbbbbbbbbbbbbbbbbbbb")
(def stray-plc "did:plc:cccccccccccccccccccccccc")
(def digest (apply str (repeat 64 "d")))

(def root-policy
  {:policy/repos #{"*"} :policy/permissions {"contents" "write"}
   :policy/actions #{:read :push} :policy/forbid #{:force-push}})

(def mailbox-only
  "A node with no GitHub reach at all: mailbox identity only."
  {:policy/repos #{} :policy/permissions {} :policy/actions #{} :policy/forbid #{:force-push}})

(def seed
  {:policies {:mailbox mailbox-only}
   :root {:id "err" :name "Err" :policy root-policy}
   :nodes [{:id "err:work" :parent "err" :kind :group :name "Work" :policy :mailbox}
           {:id "err:work:sender" :parent "err:work" :kind :agent :name "Sender"
            :policy :mailbox :proofs {:atproto sender-plc}}
           {:id "err:work:inbox" :parent "err:work" :kind :agent :name "Inbox"
            :policy :mailbox :proofs {:atproto inbox-plc}}
           {:id "err:grokbots" :parent "err" :kind :group :name "Grok Bots" :policy :mailbox}
           {:id "err:grokbots:grok-bot" :parent "err:grokbots" :kind :bot :name "Grok Bot"
            :policy :mailbox}]})

(defn- counter-ids []
  (let [n (atom 500)]
    (fn [] {:decision-id (fx/event-uuid (swap! n inc)) :effect-id (fx/event-uuid (swap! n inc)) :at fx/t1})))

(defn- world
  "Err's seeded tree, plus an unrelated root with a child, plus a bootstrap
   key for Grok Bot."
  []
  (let [ids (counter-ids)
        seeded (gi/seed-ledger (gi/resolve-seed seed host) ids)
        lg (:ledger seeded)
        lg (:ledger (gi/register-root lg (state/fold lg) mallory "Mallory" root-policy (ids)))
        lg (:ledger (gi/provision-actor lg (state/fold lg) mallory mallory-bot "Bot" mailbox-only (ids)))
        lg (:ledger (gi/issue-bootstrap lg (state/fold lg) err grok-bot digest (ids)))]
    {:ids ids :seeded seeded :lg lg :st (state/fold lg)}))

(def rule {:root-did err :root-actor-id "actor_root" :send-policy authority/default-send-policy})

(defn- atproto [plc] {:proof/kind :atproto :atproto/did plc :actor-id "actor_someone"})

(defn- change [{:keys [ids lg]} f & args]
  (let [out (apply f lg (state/fold lg) (concat args [(ids)]))]
    (is (some? (:effect out)) "the tree change is allowed")
    (state/fold (:ledger out))))

(deftest seed-binds-pilot-accounts-to-their-nodes
  (let [{:keys [seeded st]} (world)]
    (is (= #{[sender :atproto sender-plc] [inbox :atproto inbox-plc]} (set (:bound seeded))))
    (is (empty? (:refused-bindings seeded)))
    (is (= sender (gi/bound-node st :atproto sender-plc)))
    (is (= inbox (gi/bound-node st :atproto inbox-plc)))
    (is (nil? (gi/bound-node st :atproto stray-plc)))))

(deftest descendant-is-admitted-as-its-node
  (let [{:keys [st]} (world)]
    (is (authority/admitted? st err err) "the root itself (L6, n = root)")
    (doseq [node [work sender inbox grokbots grok-bot]]
      (is (authority/admitted? st err node) node))
    (testing "π ∘ authn: a bound did:plc acts as its node, not as itself (L5)"
      (is (= sender (authority/admit st rule (atproto sender-plc))))
      (is (= {:auth/did sender :auth/method :actor-tree}
             (authority/principal (authority/admit st rule (atproto sender-plc))))))
    (testing "the bound portal root session authenticates as the root"
      (is (= err (authority/admit st rule {:proof/kind :atproto :atproto/did stray-plc
                                           :actor-id "actor_root"}))))
    (testing "a verified bootstrap proof with the node's current digest"
      (is (= grok-bot (authority/admit st rule {:proof/kind :bootstrap :bootstrap/did grok-bot
                                                :bootstrap/digest digest})))
      (is (nil? (authority/admit st rule {:proof/kind :bootstrap :bootstrap/did grok-bot
                                          :bootstrap/digest (apply str (repeat 64 "e"))}))))))

(deftest unbound-or-unknown-proofs-are-refused
  (let [{:keys [st]} (world)]
    (is (nil? (authority/admit st rule (atproto stray-plc))) "no binding, no node")
    (is (nil? (authority/admit st rule {:proof/kind :password :did sender})))
    (is (nil? (authority/admit st (dissoc rule :root-actor-id)
                               {:proof/kind :atproto :atproto/did stray-plc :actor-id nil})))))

(deftest suspended-ancestor-is-refused-and-resume-restores
  (let [w (world)
        st (change w gi/suspend err work)]
    (is (not (authority/admitted? st err sender)) "L2: suspension inherits")
    (is (nil? (authority/admit st rule (atproto inbox-plc))))
    (is (authority/admitted? st err grok-bot) "a sibling subtree is unaffected")
    (let [st' (change (assoc w :lg (:ledger (gi/suspend (:lg w) (:st w) err work ((:ids w)))))
                      gi/resume err work)]
      (is (authority/admitted? st' err sender)))))

(deftest revoked-node-is-refused
  (let [st (change (world) gi/revoke-actor err inbox)]
    (is (not (authority/admitted? st err inbox)))
    (is (nil? (authority/admit st rule (atproto inbox-plc))))
    (is (authority/admitted? st err sender))))

(deftest outside-root-is-refused
  (let [{:keys [st]} (world)]
    (is (authority/admitted? st mallory mallory-bot) "admitted under its own root")
    (is (not (authority/admitted? st err mallory)))
    (is (not (authority/admitted? st err mallory-bot)))
    (is (not (authority/admitted? st err (did "nobody"))) "not a node at all")))

(deftest oauth-client-node-requires-a-signer-that-may-act-for-it
  (let [{:keys [st]} (world)
        oauth (fn [plc node] (assoc (atproto plc) :oauth/client-node node))]
    (is (= inbox (authority/admit st rule (oauth inbox-plc inbox))) "the node itself (ChatGPT)")
    (is (= grok-bot (authority/admit st rule (assoc (oauth stray-plc grok-bot) :actor-id "actor_root")))
        "L4: the root manages Grok Bot's node")
    (is (nil? (authority/admit st rule (oauth sender-plc grok-bot))) "a sibling may not")
    (is (nil? (authority/admit st rule (oauth inbox-plc nil))) "an unregistered client")
    (is (nil? (authority/admit st rule (oauth inbox-plc mallory-bot))) "outside the root")))

(deftest only-a-managing-ancestor-binds-each-subject-once
  (let [{:keys [ids lg st]} (world)
        try-bind (fn [operator node plc] (:effect (gi/bind-proof lg st operator node :atproto plc (ids))))]
    (is (nil? (try-bind grok-bot grok-bot stray-plc)) "no self-binding")
    (is (nil? (try-bind sender inbox stray-plc)) "no sibling binding")
    (is (nil? (try-bind err grok-bot sender-plc)) "a bound subject is never moved")
    (is (nil? (try-bind err grok-bot "did:web:example.com")) "only did:plc subjects")
    (is (some? (try-bind err grok-bot stray-plc)))
    (let [unbound (:ledger (gi/unbind-proof lg st err :atproto sender-plc (ids)))]
      (is (nil? (gi/bound-node (state/fold unbound) :atproto sender-plc)) "unbinding ends authn"))))

(deftest same-tree-send-needs-no-grant
  (let [{:keys [st]} (world)
        send? (fn [st from to] (authority/send-allowed? st rule :tree-ownership from to))]
    (is (send? st grok-bot inbox) "L8: in-tree pair, no grant")
    (is (send? st inbox grok-bot) "and back")
    (is (send? st sender inbox))
    (testing "a suspended or revoked recipient is refused"
      (is (not (send? (change (world) gi/suspend err inbox) grok-bot inbox)))
      (is (not (send? (change (world) gi/suspend err work) grok-bot inbox)) "inherited")
      (is (not (send? (change (world) gi/revoke-actor err inbox) grok-bot inbox))))
    (testing "a suspended sender is refused"
      (is (not (send? (change (world) gi/suspend err grokbots) grok-bot inbox))))
    (testing "across trees is refused"
      (is (not (send? st grok-bot mallory-bot)))
      (is (not (send? st mallory-bot inbox)))
      (is (not (send? st grok-bot sender-plc)) "a raw did:plc is not a tree node"))
    (testing "the policy is data: narrowing to explicit grants adds nothing"
      (is (not (authority/send-allowed? st rule :explicit-grants grok-bot inbox)))
      (is (not (authority/send-allowed? st rule :no-such-policy grok-bot inbox))))))
