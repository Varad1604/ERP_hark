# SutraERP — Frontend ↔ Backend Integration Safety & Acceptance Plan

**Date:** 2026-10-08  
**Status:** DRAFT PLANNING GATES / NOT RELEASED / NO REAL INTEGRATION PERFORMED  
**Requested outcome:** Reduce the risk of frontend/backend mismatches when development starts; zero integration defects cannot be guaranteed.  
**Current authority:** Only `DhruvNaik26/SutraERP_Architecture` and `DhruvNaik26/SutraERP_frontend` have been inspected. A new **private** `DhruvNaik26/SutraERP_backend` code repository is approved in principle by the user, but **does not yet exist as a verified accessible repository**. It will be an implementation repository, not an independent source of business/architecture authority.  
**Evidence snapshot:** Architecture `main@933cf5d06f565cc52ef7e3b2a7f821606c2b9b6f`; Frontend `main@372376d23ac6478fa4ac3f209cb97c71317d89d2`.

## 1. Precise integration objective

For each real user journey the browser shall consume exactly the version of the **released owner-backed contract** that the backend implements; the backend must enforce source-authorized tenant/institution scope, permissions, business rules and durable state. Users must receive accurate success, rejection, pending, indeterminate and error feedback. Independent local/CI evidence must demonstrate the actual browser↔backend↔owner persistence chain; frontend fixtures never count as real integration.

## 2. Non-negotiable source decisions

| Contract dimension | Authority | Implementation implication |
|---|---|---|
| API representation | [ADR-L5-0034](../../architecture/L5_Solution_Architecture/adr/ADR-L5-0034_Synchronous_API_Wire_Versioning_Error_and_Idempotency_Profile.md) | Released OpenAPI 3.2 first; camelCase JSON, major-version URLs (e.g. /api/v1), RFC 9457 technical errors, selective Idempotency-Key; no invented domain endpoints |
| Event representation | [ADR-L5-0035](../../architecture/L5_Solution_Architecture/adr/ADR-L5-0035_Event_Wire_Envelope_and_Schema_Serialization_Profile.md) | Governed versioned JSON Schema 2020-12, CloudEvents 1.0-compatible envelope, explicit source owner, tenant/Institution correlation; JetStream routing not business identity |
| Session and identity | [ADR-L5-0038](../../architecture/L5_Solution_Architecture/adr/ADR-L5-0038_Custom_Credential_API_Server_Session_and_MFA_Profile.md) | D01 custom credential API delegates verification to Keycloak-side adapter (no OAuth password grant), server-side opaque Secure/HttpOnly cookies, CSRF and session rotation; D02 authorizes protected operations |
| Validated scope | [L5-A-04-002](../../architecture/L5_Solution_Architecture/specifications/L5-A/L5-A-04-002_Tenant_and_Institution_Context_Propagation_and_Data_Isolation_Architecture.md) | Exactly one validated Tenant root per protected operation; Institution/lower scope may narrow but never widen; reject stale/missing/wrong scope; never trust only browser-selected values |
| Client authority | [L6 frontend/backend guardrails](../../architecture/L6_UI_UX_Architecture/00_Governance/L6_Frontend_Backend_Integration_Guardrails.md) | No client business truth, permission authority or direct DB calls; separate mock vs integrated behavior; success only after authoritative confirmation; old-context data/late responses quarantined |
| Actual integration evidence | [L5-F-02-003](../../architecture/L5_Solution_Architecture/specifications/L5-F/L5-F-02-003_Integration_Contract_and_End-to-End_Verification_Architecture.md) | Exercise real caller/provider or publisher/consumer boundaries; verify versioning, retries, duplicate requests, failed/unknown outcomes and same-subject cross-owner data as applicable |
| Client implementation status | [Frontend README](https://github.com/DhruvNaik26/SutraERP_frontend/blob/main/README.md) | Current Stage 5 is deterministic mock-operational only (595 workflows); mock/browser tests do not prove real backend integration |

## 3. Recommended cross-repository delivery protocol (PROPOSED, not a released implementation)

### Gate I0: Repository and access before any backend code

- User creates the approved **private** backend repository under the approved GitHub owner; use empty repo with README and a normal `main` branch.
- Give the existing connected GitHub App/ChatGPT access to that specific new repository, subject to user review. Never copy production secrets into the chat.
- Require protected `main` / human review, defined AI write access to task branches only, distinct backend/test credentials, no force-push or auto-merge. Verify rules actually enforced (current existing-repo protected flags alone do not establish rule evaluation).
- Confirm branch workflows, self-hosted runners, isolation and local test environment before any build/run authorization. No third-party legacy source is grandfathered into authority.
- Architecture `main` remains canonical governance; backend, frontend and approved contracts must reference exact relevant source revisions, never reinterpret upstream decisions.

### Gate I1: First owner-approved **contract** before server or client implementation

For each candidate workflow, one release packet must name:

1. canonical owning domain / authoritative aggregate / allowed state change and L2/L3/L4 source;
2. exact Tenant/Institution and lower scope, principal authorization, input and output semantics;
3. a **released**, versioned OpenAPI request/response schema with error and unknown-outcome semantics; optional relevant released event schema and declared consumers;
4. idempotency/retry and concurrent-command semantics, plus compatible upgrade/retirement rules;
5. owner-store or governed projection evidence needed by the UI; mock fixtures clearly separate;
6. exact contract catalog identity/version/hash and approval/release evidence, only when actually verifiable.

**STOP if no released contract exists.** A draft OpenAPI file, generated Rust handler, or frontend `fetch` implementation does not make a contract released. No invented API fields/URLs, event names or schema-registry records.

### Gate I2: Contract compatibility checks BEFORE UI connection

- Backend verifies real handler against the released OpenAPI schema (wire names, status classes, typed response bodies and RFC 9457 Problem Details); frontend uses a version-pinned contract client/adapter (generated if toolchain compatibility is verified).
- Compare exposed routes, payloads and schema versions; changing incompatible/removed fields must fail the change-impact check unless an approved version/transition plan exists.
- Frontend and backend both record the **same** approved contract version/digest; contract snapshots are **consumable copies**, not a new architecture source.
- Verify Keycloak-backed session → opaque cookie → authorized API path using an appropriate approved local HTTPS/same-site deployment profile; **same-origin reverse-proxy deployment is a proposed simplification, not an approved universal topology**.
- If legitimately cross-origin, explicitly test allowed-origin/credentials/CORS, cookie SameSite/Secure and CSRF behavior. A working development proxy does not prove deploy-time cookies.
- No browser-stored access/refresh secrets or sensitive fixture-to-production carry-over.

### Gate I3: Real vertical-slice integration tests

Minimum source-backed tests for each applicable operation:

| Case | Required proof |
|---|---|
| Valid operation | Real logged-in, authorized user, correct Tenant/Institution; backend authoritative state changes and frontend displays confirmed outcome |
| Wrong Tenant / Institution | Protected action/read fails closed, **no** cross-Tenant leaks or fallback |
| Absent / expired / changed session | Reject/re-authenticate as authorized; no leftover access to old protected data |
| Switch context during request | Browser invalidates/quarantines stale context, discards late old-context response |
| Duplicate click / network retry | Owner-scoped idempotency and no duplicate business outcome where specified |
| Business rejection | UI displays **owner-defined** rejection, not generic technical 500 or fabricated success |
| API technical failure or timeout | UI shows failure/unknown/pending as appropriate and does not falsely mark authoritative completion |
| Wrong API/event version | Fails compatibility check; no silent alternate endpoint or event-alias guessing |
| Cross-owner event delivery when applicable | Actual approved publisher → durable publication intent → routed event → authorized consumer, duplicate/recovery checked |
| Browser journey | Real UI with backend connected; form/navigation/state changes, accessible feedback and no mock truth used as proof |

### Gate I4: Merge and release evidence

Each slice's review packet records **actual** frontend and backend commit SHAs, exact architecture source revision, contract/version/digest, owner review, test logs and explicit passed/failed/skipped/unexecuted gates. Backend release artifact and frontend deploy must be tested together as an exact compatible version pair. No PR merge, deployment, CI dispatch, contract activation or architecture promotion without independently approved authorization.

## 4. Integration risks requiring separate resolution

| Risk | Consequence | Status |
|---|---|---|
| Backend repository not created / GitHub App lacks access | Cannot initialize and verify backend source/branch control | BLOCKED |
| No active released owner API/event catalogs for some journeys | Client/backend could agree on invented contract but still violate architecture | BLOCKED FOR AFFECTED SLICES |
| Cookie deployment origin/CSRF configuration untested | Logins fail or protected requests get denied even if handlers work | PROPOSED TEST |
| Difference between mock operation and actual backend workflow | UI may falsely show success or call an invalid transition | NOT INTEGRATED |
| Cross-owner semantics and live-source evidence (e.g. C05/I13 #136, Examination W3 #57, J05/A06 #152) | Real owner chain not ready even if screens exist | OPEN DOMAIN GATES |
| Shared host runners have no strong isolation | Races, data/secret leakage, flaky CI | NOT APPROVED FOR PARALLEL RUN |

## 5. Minimum approval decisions next

- **Repository creation:** Private `DhruvNaik26/SutraERP_backend`, user authorized in principle on 2026-10-08; **creation pending** because the available GitHub connector has no create-repository action.
- **Governed contract catalog location and mechanics:** Must be agreed before physical API schemas are promoted; do not assign authority to a code generator.
- **First slice choice:** Pick a simple source-owned, protected read + valid source-owned mutation once applicable L2–L5 semantics and contract authority are verified.
- **API cookie/deployment profile:** Approve concrete tested browser/backend URL and HTTPS local/dev profile before coding authentication integration.
- **CI and scope isolation checks:** Define genuine negative tests on real PostgreSQL and real browsers, then isolate runners.

## 6. Simple debugging vocabulary

- **API** = agreed way the screen talks to the backend.
- **Contract** = exact agreed message format and meaning.
- **Mismatch** = frontend asks for a field/operation the backend version does not support.
- **401** generally means login/session problem; **403** generally means access denied; **5xx** generally means a server-side failure; **business rejection** can have its own approved response. Exact meaning must come from the released contract, not guesses.
- **Integration test** = exercises real frontend + backend rather than a simulated response.
- **Commit/version** = exact software snapshot. Debugging starts by recording frontend commit, backend commit and contract version.

**Integrity:** Planning document only; no new repository, no runtime code, no released API/event, no CI run, no integration test and no merge have occurred.
