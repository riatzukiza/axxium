(ns axxium.extern.mailbox-authority
  "Mailbox admission adapter: turns verified proofs into plain proof data and
   admits them against the shared identity-tree ledger (L6), read fresh on
   every call so a suspend or revoke on the authority instance takes effect at
   the next request. Without AXXIUM_MAILBOX_AUTHORITY_ROOT_DID the deprecated
   flat pilot allowlist still applies, unchanged, for a safe roll-forward."
  (:require [axxium.config :as cfg]
            [axxium.domain.mailbox-authority :as authority]
            [axxium.domain.state :as state]
            [axxium.extern.github-identity-mcp :as identity-mcp]
            [axxium.extern.ledger-file :as ledger-file]))

(defn current
  "The admission rule in force, as data."
  []
  (if-let [{:keys [root-did ledger-file]} (cfg/get-in-config [:mailbox/authority])]
    {:mode :tree :root-did root-did :ledger-file ledger-file
     :root-actor-id (let [binding (cfg/get-in-config [:identity/root-operator-binding])]
                      (when (= root-did (:root-did binding)) (:actor-id binding)))
     :send-policy authority/default-send-policy}
    {:mode :legacy
     :owner (cfg/get-in-config [:mailbox/webhook-actor-did])
     :allowed (cfg/get-in-config [:mailbox/pilot-allowed-dids])}))

(defn with-owner
  "The legacy fence's recipient is the OIDC mount's configured mailbox actor."
  [rule owner]
  (if (= :legacy (:mode rule)) (assoc rule :owner owner) rule))

(defn tree-state
  "Fold the shared tree ledger without repairing or writing it."
  [{:keys [ledger-file]}]
  (state/fold (ledger-file/read-ledger ledger-file)))

(defn proof-data
  "Plain proof data for the pure authority. `client-node` is present only for
   OAuth grants, as the registered client's node (nil when unregistered)."
  [proof {:keys [oauth? client-node]}]
  (cond-> (if-let [node (:bootstrap/did proof)]
            {:proof/kind :bootstrap :bootstrap/did node :bootstrap/digest (:bootstrap/digest proof)}
            {:proof/kind :atproto :atproto/did (get-in proof [:principal :auth/did])
             :actor-id (:actor-id proof)})
    oauth? (assoc :oauth/client-node client-node)))

(defn admit
  "The admitted node (tree) or AT Protocol DID (legacy) for a verified proof,
   or nil. `oauth` is {:oauth? true :client-node <did|nil>} for OAuth grants."
  ([proof oauth] (admit (current) proof oauth))
  ([{:keys [mode owner allowed] :as rule} proof {:keys [oauth?] :as oauth}]
   (case mode
     :tree (authority/admit (tree-state rule) rule (proof-data proof oauth))
     :legacy (let [did (get-in proof [:principal :auth/did])]
               (when (and (not (:bootstrap/did proof))
                          (if oauth?
                            (= owner did)
                            (or (nil? allowed) (contains? allowed did))))
                 did)))))

(defn principal
  "π: the mailbox principal for an admitted proof."
  [{:keys [mode]} proof node]
  (if (= :tree mode) (authority/principal node) (:principal proof)))

(defn bootstrap-proof
  "Verify a host agent's bootstrap key against the shared tree (timing-safe).
   The proof keeps only the key digest, never the key."
  [rule did key]
  (when (and (= :tree (:mode rule)) (string? did) (string? key))
    (when (identity-mcp/authenticate (tree-state rule) did key)
      {:bootstrap/did did :bootstrap/digest (identity-mcp/digest key)
       :cred-id (str "bootstrap:" (identity-mcp/digest key))})))

(defn send-predicate
  "The selected send policy as (fn [sender recipient]) over the tree as it is
   now, or nil (explicit mailbox grants only) without tree admission."
  [{:keys [mode send-policy] :as rule}]
  (when (= :tree mode)
    (let [st (tree-state rule)]
      (fn [sender recipient]
        (authority/send-allowed? st rule send-policy sender recipient)))))
