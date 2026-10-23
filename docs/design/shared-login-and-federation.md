# Shared login and federated inference across the droplet, Stealth and Yoga

## Authority and scope (operator extension, 2026-10-22)

The operator explicitly requested extending the existing Knoxx and Proxx
identity plans, then implementing and deploying shared login and federation on
all three machines. The DigitalOcean droplet replaces Big Ussy. This document
extends, rather than completes or weakens, both migration contracts and their
existing account-continuity, rollback, revocation and retirement requirements.

The same document is carried by the two planning branches so they compose
without divergent definitions. It is proposed scope until current-head
planning qualification and lawful Rheos admission; an operator instruction
does not manufacture test, review or deployment evidence.

## Foundation and retained work

Use merged open-hax/axxium#30, merge
99e6c725c64baed7d3ee93930b206724874e1486, as the identity foundation after
rechecking integration ancestry. Personal fork synchronization #11 remains a
separate dependency; do not discard it or manufacture a merge result.
Knoxx organization #381 / personal #2 implement password delegation and
independent product sessions, not redirect SSO. Organization Axxium #13 keeps
cross-host passwords and sessions local and is independent/offline transfer
work, not a replacement for online SSO. Services #92 represents an existing
live deployment with pending source qualification. Reconcile these lines
explicitly; no unrelated PR is closed, merged or overridden by this extension.

## Identity and session contract

Axxium owns the stable actor/entity identity, authentication, grant issuance
and status/revocation. Knoxx and Proxx consume issuer plus immutable subject,
never merge identities by mutable email/handle, and keep their own restrictive
resource and tenant policies. Google/AT bindings already in the authority and
legacy local accounts must be inventoried without exposing credentials.
Conflicts are explicit proof-bound linking/recovery decisions.

Use one operator-configured primary online Axxium authority with separately
registered clients for Knoxx and Proxx on the droplet, Stealth and Yoga.
Registration, callback/origin allowlists and capability grants are
operator-owned data, not caller-selected authority. Local independent Axxium
instances are preserved until their chosen continuity role is qualified.
A stopped/offline primary does not authorize stale local credentials or roles.

### Browser authorization

Provide an authorization-code flow with S256 PKCE, unpredictable state and
nonce, exact pre-registered redirect matching, short expiry, atomic one-use
redemption, audience/client/redirect/challenge binding, and authenticated
confidential-client exchange. Use standard OAuth/OIDC claims and discovery
where implemented, and never advertise unsupported protocol capabilities.
Axxium reuses its existing browser login; it alone handles password/Google/AT
authentication. Products must not receive human passwords.

Products maintain opaque HttpOnly sessions backed by Axxium authorization and
fresh status, with exact host/path, Secure outside loopback development,
SameSite and CSRF protections. Such cookies are product transport bindings,
not a second password/identity authority. Never share a parent-domain cookie,
copy laptop cookies or expose authority refresh/access tokens to localStorage.
Retained product session bindings must be explicitly reconciled with the
original no-standalone-auth-storage retirement criteria; this extension does
not silently waive them. Axxium-issued product-audience sessions or bounded
non-authoritative references must be chosen and qualified in owner review.

Already authenticated in the same browser: visiting either product reuses the
Axxium session through its redirect and returns the same actor without another
password prompt. Product logout and authority logout are distinct visible
operations; revocation closes active HTTP/WS/SSE/bridge grants according to the
qualified freshness bound. Distinguish 401 authentication refusal, 403 denied
capability, and 503 unavailable mandatory verification.

### New device approval

Implement an expiring device authorization request with unpredictable device
secret, separately entered short user code, bounded attempts, minimum polling
interval/backoff, explicit approval or denial by an authenticated human,
client/device identity display and a final confirmation. Approval binds the
actor, registered client, receiving device, requested scopes and audience.
Redemption is atomic and once-only; reused, denied, expired and mismatched
requests refuse. A new laptop does not inherit another browser's session.

Approval must not automatically grant system administration, copy provider
credentials, bypass migration/account-link conflicts or authorize unknown
applications. Codes/verification links must not carry access tokens, and
polling/approval must enforce rate and origin/CSRF controls. Never trust an
actor subject provided by the pending device.

## Federated inference contract

Proxx owns providers, provider OAuth refresh tokens, billing, request
attribution and policy. Axxium authorizes distinct service principals/node
grants under the verified human/organization identity. Same identity does not
mean automatic rights to another node or tenant.

Use workstation-initiated outbound relay connections to the droplet so laptops
need no new public inbound listener. Admit Stealth and Yoga independently;
register topology and explicit capabilities, scope/audience/expiry and revocable
relay-only grants. Keep credentials on their source machines; no warm import
or provider credential projection by default. Reject spoofed owner headers,
arbitrary DID strings, cross-tenant grants, wrong-node audiences and routing
loops. Audit each request's actor, tenant, source node and served-by provenance.
A reconnect revalidates current grants and cannot resurrect a revoked node.

Remove retired Big Ussy endpoints from active desired configuration without
rewriting historical provenance. Preserve local operation and data; loss of a
laptop means capability becomes unavailable rather than cascading retries to
retired hosts. Qualify a healthy provider with a real completion before
advertising it. The droplet's Requesty 402 balance failure is distinct from
federation and cannot be repaired by restarting or resetting cooldown.

## Implementation slices and ownership

These proposed slices require review and native board admission; they are not
new status transitions or claims that the existing eight-point aggregate
fairly prices the expanded work.

| Slice | Owner | Deliverable and dependency |
| --- | --- | --- |
| Authority laws and clients | Axxium | Portable schemas/laws, operator-owned client registry, grants and authenticated versioned status/revocation; based on merged actor foundation. |
| Browser code flow | Axxium | Durable atomic authorization-code lifecycle, PKCE and consent/login return, audience keys/discovery, hostile and browser tests; depends on authority laws. |
| Device approval | Axxium | Device request/poll/approve/deny lifecycle and UI with abuse controls; depends on authority laws and status/revocation. |
| Knoxx consumer | Knoxx | Redirect/context/logout/device UI, restrictive policy and proof-bound legacy mapping; consumes qualified authority contract. |
| Proxx consumer | Proxx | All browser/API/HTTP/WS/SSE/bridge admission and provider-operation continuity; consumes the same qualified authority contract. |
| Node federation | Proxx | Relay-only node grant admission, reconnect/revocation, capacity and provenance; depends on verified identity and healthy provider. |
| Three-host rollout | Services | Immutable image/client/issuer configuration, state preservation, backups/rollback, exact-host TLS and operator browser acceptance; depends on owning product qualification. |

Do not create a second Rheos engine or hand-edit lifecycle status. Review
whether the original cards should become epics or link to separately sized
stories; preserve their original requirements and existing provenance.

## Acceptance and verification matrix

1. On a clean browser profile, sign in to the configured Axxium using an
   operator-controlled existing account. Enter both Knoxx and Proxx on the
   droplet and each laptop through their registered callbacks. Verify identical
   issuer/actor/entity binding and no second password prompt in that profile.
2. With a distinct Yoga browser profile, initiate device login, approve on the
   already-authenticated device and prove the receiving browser gets only the
   requested grant. Test denial, timeout, repeated polling, wrong code,
   approval-CSRF, malicious client label and cross-device replay.
3. Exercise code/PKCE/nonce/state/client/redirect/audience substitution,
   concurrent code redemption and verifier failure. Validate both portable
   laws and real database/HTTP adapters; one winner is permitted at most.
4. Revoke a session, capability, user and node while requests/streams are
   active; demonstrate the declared freshness bound and no stale allow path.
   Authority outage yields a visible retryable refusal without protected effects.
5. Run original migration fixtures for every account kind, attribution,
   membership, post-cutover user and rollback watermark. No automatic email
   merge, re-registration or revived local-password authority.
6. From the droplet request a model served by Stealth, then one served by Yoga;
   record matching request IDs, actor/tenant and served-by node. Verify source
   credentials never leave their host and no credential-import event occurred.
7. Disconnect/reconnect each laptop independently; verify healthy fallback or
   explicit unavailability, loop rejection, grant revalidation and node
   revocation. Big Ussy is absent from active route/relay configuration.
8. Complete a real Knoxx actor answer using the indexed creative corpus and the
   federated model path. Memory-source attachment, a healthy process, a model
   listing or a connected socket alone do not satisfy this outcome.
9. Retain exact source/image/config identities, red/green law evidence,
   nonempty zero-warning builds/tests, current-head review and mandatory
   Services rollout qualification. Browser account approval is performed by
   the operator; never fabricate it from API fixtures.

## Operational limits

No payment, top-up, public post, new provider subscription, private-key export,
automatic identity merge or destructive account retirement is authorized by
these tests. Ask for actual operator interaction when external login, MFA,
consent or device approval requires it; continue independent implementation and
verification while waiting. Preserve all existing state and unrelated runtimes.


## Protocol references

Implement and test against [OpenID Connect Core](https://openid.net/specs/openid-connect-core-1_0.html),
[PKCE (RFC 7636)](https://www.rfc-editor.org/rfc/rfc7636) and
[Device Authorization Grant (RFC 8628)](https://www.rfc-editor.org/rfc/rfc8628).
A browser that can run the code flow normally uses it; device approval is the
explicit optional cross-device onboarding path, not a replacement for ordinary
browser redirects. An OIDC ID token is login evidence, not an API access token.
