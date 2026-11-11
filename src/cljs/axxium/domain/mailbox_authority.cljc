(ns axxium.domain.mailbox-authority
  "Mailbox admission by identity-tree ancestry (pure).

   Objects. Nodes N of the identity tree (did:web:<authority>:actors:<path>),
   proofs Π (an AT Protocol sign-in, the bound portal root session, a verified
   bootstrap key, an OAuth grant for a registered mailbox client), and mailbox
   principals M = {:auth/did :auth/method}.

   Morphisms.
     parent : N ⇀ N          the tree edge; ≤ is its reflexive-transitive closure
     authn  : Π ⇀ N          `authn` below; every proof kind lands on one node
     π      : N → M          `principal`; the mailbox only ever sees π(authn p)

   Laws (numbered as in the PR).
     L1 ≤ is a forest order (gi/ancestors-of is cycle-bounded).
     L2 effective(n) ⇔ every a ≥ n is :active (gi/effective?).
     L3 a child's policy is within its parent's (gi/provision-actor).
     L4 manage(o, n) ⇔ n < o ∧ effective(o) (gi/may-manage?).
     L5 mailbox principals factor through authn: M ∋ m = π(authn p).
     L6 admit(p) ⇔ ∃n. authn(p) = n ∧ n ≤ root ∧ effective(n).
     L7 did(n) depends only on the authority host and the node path, so a
        mailbox instance on another origin reads the same names.

     L8 send(v,s) ⇔ see(v,s) ⇔ v, s ≤ root ∧ eff(v) ∧ eff(s) under the default
        :tree-ownership policy; explicit mailbox grants still add sends.

   Send permission is data: `send-policies` maps a policy key to its rule, so
   the default can be narrowed (e.g. to :explicit-grants) without a new path."
  (:require [axxium.domain.github-identity :as gi]
            [axxium.law.principal :as principal]))

(def send-policies
  "How send permission beyond explicit mailbox grants is decided, as data.
   Explicit grant_send/revoke_send grants always still apply; a policy only
   adds sends. Narrow by selecting another key, or add one with its rule."
  {:tree-ownership {:send/rule :same-root-effective
                    :send/doc "send(v,s) ⇔ see(v,s): same root ∧ eff(v) ∧ eff(s)"}
   :explicit-grants {:send/rule :grants-only
                     :send/doc "only grants recorded in the mailbox ledger"}})

(def default-send-policy :tree-ownership)

(defn admitted?
  "L6 for a node: at or below `root`, and effective with every ancestor."
  [state root node]
  (boolean (and (string? node) (string? root)
                (gi/effective? state node)
                (or (= node root) (gi/ancestor? state root node)))))

(defn may-act-for?
  "A signer may obtain a credential for `node` when it is that node or an
   effective strict ancestor of it (L4)."
  [state signer node]
  (boolean (and (string? signer) (string? node)
                (or (= signer node) (gi/may-manage? state signer node)))))

(defn signer
  "authn for a direct proof. Proofs are plain data made by verified adapters:
     {:proof/kind :atproto :atproto/did <did:plc> :actor-id <portal actor>}
     {:proof/kind :bootstrap :bootstrap/did <node> :bootstrap/digest <hex>}
   The bound portal root session wins for its exact actor; otherwise an AT
   Protocol DID authenticates only through a proof binding. A bootstrap proof
   holds only while its digest is the node's current usable one."
  [state {:keys [root-did root-actor-id]} {:proof/keys [kind] :as proof}]
  (case kind
    :atproto
    (if (and root-actor-id (= root-actor-id (:actor-id proof))
             (gi/active-human-root? state root-did))
      root-did
      (gi/bound-node state :atproto (:atproto/did proof)))

    :bootstrap
    (let [node (:bootstrap/did proof)
          digest (:bootstrap/digest proof)]
      (when (and (string? digest) (= digest (gi/bootstrap-digest state node)))
        node))

    nil))

(defn authn
  "Proof → node, or nil. An OAuth grant authenticates as its registered
   client's node, and only while its signer may still act for that node;
   anything else authenticates as its signer."
  [state authority proof]
  (when-let [s (signer state authority proof)]
    (if (contains? proof :oauth/client-node)
      (let [node (:oauth/client-node proof)]
        (when (may-act-for? state s node) node))
      s)))

(defn admit
  "L6: the admitted node for `proof`, or nil."
  [state {:keys [root-did] :as authority} proof]
  (let [node (authn state authority proof)]
    (when (admitted? state root-did node) node)))

(defn principal
  "π : N → M. The mailbox principal a node acts as."
  [node]
  (principal/from-actor-tree node))

(defn send-allowed?
  "Does `policy-key` let `sender` send to `recipient` without an explicit
   grant? Unknown policies and rules allow nothing beyond grants."
  [state {:keys [root-did]} policy-key sender recipient]
  (case (get-in send-policies [policy-key :send/rule])
    :same-root-effective (and (admitted? state root-did sender)
                              (admitted? state root-did recipient))
    false))
