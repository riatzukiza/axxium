(ns axxium.law.principal
  "An authenticated principal: who is acting, and how that was proven.
   Proof methods: an AT Protocol OAuth sign-in bound to the DID, an Axxium
   service token signed with the instance Ed25519 key, or :actor-tree, a node
   of the identity tree reached through a verified proof (π ∘ authn in
   axxium.domain.mailbox-authority). Verification happens in extern/auth
   adapters; this contract only admits their output."
  (:require [axxium.law.did :as did]
            [malli.core :as m]))

(def auth-methods #{:atproto :service-token :actor-tree})

(def PrincipalContract
  [:map {:closed true}
   [:auth/did did/Did]
   [:auth/method (into [:enum] (sort auth-methods))]])

(defn authenticated? [principal]
  (m/validate PrincipalContract principal))

(defn from-service-claims
  "Principal for verified service-token claims carrying a `did` claim."
  [claims]
  (let [p {:auth/did (get claims "did" (:did claims)) :auth/method :service-token}]
    (when (authenticated? p) p)))

(defn from-actor-tree
  "Principal for an identity-tree node already admitted by the tree authority."
  [node-did]
  (let [p {:auth/did node-did :auth/method :actor-tree}]
    (when (authenticated? p) p)))

(defn from-atproto-session
  "Principal for a completed AT Protocol OAuth session's DID."
  [session-did]
  (let [p {:auth/did session-did :auth/method :atproto}]
    (when (authenticated? p) p)))
