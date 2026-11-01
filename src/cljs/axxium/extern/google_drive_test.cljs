(ns axxium.extern.google-drive-test
  (:require [axxium.extern.google-drive :as gd]
            [axxium.extern.token-crypto :as tc]
            [axxium.law.google-drive :as law]
            [cljs.test :refer [deftest is async]]
            ["node:crypto" :as crypto]))

(deftest authorize-params
  (let [p (law/authorization-params {:client-id "cid" :redirect-uri "https://x/api/drive/callback"
                                     :state "s" :challenge "c"})]
    (is (= law/drive-file-scope (get p "scope")))
    (is (= "offline" (get p "access_type")))
    (is (= "consent" (get p "prompt"))))
  (is (law/granted-scopes-ok? law/drive-file-scope))
  (is (not (law/granted-scopes-ok? (str law/drive-file-scope " https://www.googleapis.com/auth/drive")))))

(deftest put-never-overwrites
  (is (= :refused (:drive/outcome (law/put-plan [{:drive/file-id "f1"}] {}))))
  (is (= :create (:drive/outcome (law/put-plan [] {}))))
  (is (= "name = 'a\\'b' and trashed = false and 'F' in parents"
         (law/same-name-query {:drive/name "a'b" :drive/folder-id "F"}))))

(deftest no-delete-surface
  (is (empty? (filter #(re-find #"(?i)delete|trash|update|patch" (str %))
                      (keys (ns-publics 'axxium.extern.google-drive))))))

(deftest refresh-token-sealing
  (let [k (crypto/randomBytes 32)
        sealed (tc/seal k "did:plc:a" "1//refresh")]
    (is (not (re-find #"refresh" sealed)))
    (is (= "1//refresh" (tc/open-sealed k "did:plc:a" sealed)))
    (is (nil? (tc/open-sealed k "did:plc:b" sealed)) "bound to the actor")
    (is (nil? (tc/open-sealed (crypto/randomBytes 32) "did:plc:a" sealed)))
    (is (nil? (tc/key-from-base64 "c2hvcnQ=")))))

(defn- fake-fetch [calls]
  (fn [url opts]
    (swap! calls conj [url (some-> opts .-method)])
    (js/Promise.resolve
     #js {:ok true :status 200
          :json (fn [] (js/Promise.resolve
                        (clj->js (cond (re-find #"upload" url) {:id "new" :name "n.txt" :parents ["F"]}
                                       (re-find #"q=" url) {:files []}
                                       :else {:id "new" :name "n.txt"}))))
          :text (fn [] (js/Promise.resolve "hello"))})))

(deftest fake-roundtrip
  (async done
    (let [calls (atom [])
          f (fake-fetch calls)]
      (-> (gd/list-files f "at" {:q (law/same-name-query {:drive/name "n.txt"})})
          (.then (fn [r] (is (= [] (:drive/files r)))
                   (gd/create-file f "at" {:drive/name "n.txt" :drive/folder-id "F" :drive/content "hello"})))
          (.then (fn [r] (is (= "new" (:drive/file-id r))) (gd/get-file f "at" "new")))
          (.then (fn [r] (is (= "hello" (:drive/content r)))
                   (is (every? #(contains? #{nil "POST"} (second %)) @calls))))
          (.catch (fn [e] (is false (pr-str e))))
          (.finally done)))))
