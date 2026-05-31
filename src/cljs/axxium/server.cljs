(ns axxium.server
  "Axxium HTTP server.
   Fastify-based, serving the identity provider API and portal."
  (:require [axxium.config :as cfg]
            [axxium.db :as db]
            [axxium.routes.auth :as auth-routes]
            [axxium.routes.actor :as actor-routes]
            [axxium.routes.health :as health-routes]))

(defn- create-app
  "Create and configure the Fastify application."
  []
  (let [fastify (js/require "fastify")
        app (fastify #js {:logger true})]
    ;; Register plugins
    (-> (.register app (js/require "@fastify/cors")
                   #js {:origin true
                        :credentials true
                        :methods #js ["GET" "POST" "PUT" "DELETE" "OPTIONS"]
                        :allowedHeaders #js ["Authorization" "Content-Type" "X-Requested-With"]})
         (.then
           (fn [_]
             (.register app (js/require "@fastify/cookie")))))
    app))

(defn- register-routes!
  "Register all API routes on the app."
  [app]
  (health-routes/register-health-routes! app)
  (auth-routes/register-auth-routes! app)
  (actor-routes/register-actor-routes! app))

(defn- register-static!
  "Register static file serving for the portal."
  [app]
  (-> (.register app (js/require "@fastify/static")
                  #js {:root (str (js/require "node:path")
                                 (.join (js/require "node:path")
                                        js/__dirname ".." "resources" "public"))
                       :prefix "/portal/"})))

(defn start!
  "Start the Axxium server.
   Initializes database schema and starts listening."
  []
  (println "Starting Axxium identity kernel...")
  (-> (db/init-schema!)
      (.then
        (fn [_]
          (println "Database schema initialized")
          (let [app (create-app)]
            (register-routes! app)
            (register-static! app)
            (.then
              (.listen app #js {:port (cfg/get-in-config [:axxium/port])
                                :host (cfg/get-in-config [:axxium/host])})
              (fn [address]
                (println (str "Axxium listening on " address))
                (println (str "Portal: " (cfg/get-in-config [:axxium/public-base-url]) "/portal/index.html"))
                app)))))
      (.catch
        (fn [err]
          (println (str "Failed to start Axxium: " (.-message err)))
          (js/process.exit 1)))))

;; Entry point for shadow-cljs :init-fn
(start!)
