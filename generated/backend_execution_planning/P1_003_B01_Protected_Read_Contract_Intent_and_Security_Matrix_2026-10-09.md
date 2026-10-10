# SutraERP Backend P1-003 — B01 Protected Institution Read: Contract Intent & Security Matrix

**Date:** 2026-10-09  
**Status:** DRAFT FOR DOMAIN/CONTRACT/SECURITY REVIEW — **NOT** APPROVED, Released, or Active API; **no endpoint, JSON Schema or database field names are defined**.  
**Target:** One B01-owned authenticated, authorized, Tenant/Institution-scoped **read-only** representation of current canonical Institution identity and lifecycle for the first real frontend–backend demonstration. Mobile will later consume the same C01 contract through its own governed client adapter.  
**Related source-backed candidate:** [P1-002](P1_002_First_Protected_Read_Slice_Candidate_and_Dependencies_2026-10-09.md).  
**Authority:** Architecture [main](https://github.com/DhruvNaik26/SutraERP_Architecture/tree/main) at `933cf5d06f565cc52ef7e3b2a7f821606c2b9b6f`. Frontend `main@372376d23ac6478fa4ac3f209cb97c71317d89d2`. New backend `main@213c323303bf9fff4284c15b94f9d4735d68759a` (README-only at inspection). These are historical review pins; refresh at implementation. Other historical backend code is expressly obsolete as an implementation baseline.

## 1. Read boundary and explicit exclusion

B01 owns Institution Aggregate identity, configuration, lifecycle and status under [L2-B01](../../architecture/L2_Domain_Workflow_Architecture/section-b/L2-B01_Institution_Workflow.md) §§2–7, 11–13, and its [locked activation-authority amendment](../../architecture/L2_Domain_Workflow_Architecture/section-b/L2-B01_Institution_Activation_Authority_Amendment_2026-08-24.md). Every Institution belongs to exactly **one** Tenant; its Code is unique **within the Tenant**. The L2 source names Institution Name, Institution Code, Short Name and Institution Type as master information, and outputs immutable Institution Business Identity (UUID + code) plus Institution lifecycle state. This source **does not** itself release an HTTP read operation or authorize general disclosure.

The first slice is a read of the **current** B01 state only. It **does not** establish an institution, change its profile, alter policy, approve/activate/suspend/archive it, declare readiness, or publish any business event. [L4-B-001](../../architecture/L4_System_Behavior_Specification/01_Production/section-b/L4-B-001_Institution_Establishment_to_Final_Activation_Decision.md) is Architecture Approved and constrains identity and activation semantics, but does not release this proposed read contract. [L4-B-004](../../architecture/L4_System_Behavior_Specification/01_Production/section-b/L4-B-004_Institution_Profile_Revision.md) has metadata `Status: Review` and does **not** authorize profile editing here. Do not convert read into a mock activation success, or infer that an Active Institution is automatically operationally ready under A10/L1-06.

## 2. Candidate minimum consumer-visible business fields (NOT RELEASED)

Fields below are **logical proposed disclosures, not actual JSON keys, Rust struct fields, SQL columns, route parameters or guaranteed permission grants**. Exact schema must be approved and released by owning C01/B01/D02 architecture before coding.

| Candidate business information | Canonical source basis | Proposed purpose | Provisional disclosure rule / decision needed |
|---|---|---|---|
| Canonical Institution identifier | B01 §12 “Institution Business Identity (Immutable UUID and code)” | Unambiguous identity of record viewed, avoiding display-name collisions | Owner and D02 confirm which B01 identifier is safe to disclose and whether stable cross-client reference is supported. Do not disclose implementation-internal SQL row keys by assumption. |
| Institution Name | B01 §7 Identity; locked L3-B-004 governs historically traceable revision | Human-readable heading for a subsequent enriched view | **DEFER from initial identity/status read** until B01 can supply one demonstrably current authoritative name under already-locked effective-profile requirements. This is a physical/source-resolution gate, not a new business rule. |
| Institution Code | B01 §7 Identity and §13 unique-within-Tenant rule | Distinguish institutions that may share names | Approve exposure to this consumer and ensure no claim of global uniqueness. |
| Current Institution lifecycle position | B01 §6 states + §12 lifecycle output | Show authoritative current status, not mock status | Decide whether every lifecycle state is readable to this caller. Preserve exact source-defined states; no inferred permissions or readiness from a state. |

**Suggested initial whitelist:** THREE business concepts—canonical Institution identifier, Institution Code, and current lifecycle state. The Institution Name is **deferred**, not rejected as a business concept: include it only in a later approved contract backed by an authoritative current-profile read. Do not implement a profile-revision workflow merely to demonstrate the first read. If B01/D02 review narrows the initial three-field set, implement only the approved subset; never fill missing facts from frontend fixtures.

**Not in the first response by default:** Institution Name (until an approved current-profile lookup exists), Tenant record/profile, Tenant administrator, Institution administrator identity, addresses, email/phone, regulatory registration data, university affiliations, external integrations, policy contents/versions, authorization rules, permission/role grants, activation approval/audit evidence, organisation/department/campus, curriculum, finance/student/employee data, unapproved readiness flag, historical profile versions, cross-institution directory listings or hidden server IDs. None is needed for the proposed minimal view. Future features require their own contract/disclosure review.

**Technical metadata:** version/correlation/freshness and validated scope references may be necessary for C01/G01 correctness, but exact representation and whether to expose Tenant scope in response are **undecided**. The backend must not treat a browser-supplied Tenant ID, Institution ID or client-selected persona as proof of authority. A canonical B01 identity is a *resource selector* only after applicable server-side validation.

## 3. Read admission and protection sequence (source-defined architecture)

1. **C01 released contract:** accept only a registered active/compatible operation, after owner-approved request/response/error semantics are issued. Current operation is *not defined or released*.
2. **D01:** establish a real authenticated Session/principal via approved D01/Keycloak mechanism; no fixture persona or demo `administrator` string grants access.
3. **A04:** resolve one unambiguous **Tenant root** and verify Institution belongs to that Tenant; missing/changed/widened/mismatched scope **fails closed**. Institution selection can narrow scope only, never switch Tenant implicitly.
4. **D02:** evaluate exactly the specified *read* permission/capability and current resource scope/state under applicable authoritative policies and effective human authority, including explicit deny and decision freshness; a workspace/role label alone is insufficient. D02 controls the policy outcome, **not** B01 business status.
5. **B01:** retrieve only B01-owned authoritative current record via B02-compliant owner-local persistence; no cross-owner direct joins into Tenant source or another Institution's state. Do not authorize via result filtering in the React app.
6. **C01/G01:** return the precisely whitelisted *authorized* representation; frontend may show it only in current validated context epoch/partition. Late results from a previous Institution/Tenant/session must not overwrite current data. Mobile later uses same canonical contract with a separately authorized native session adapter.

**Primary sources:**
- [L5-A-04-001](../../architecture/L5_Solution_Architecture/specifications/L5-A/L5-A-04-001_Tenant_and_Institutional_Isolation_Architecture.md), [A04-002](../../architecture/L5_Solution_Architecture/specifications/L5-A/L5-A-04-002_Tenant_and_Institution_Context_Propagation_and_Data_Isolation_Architecture.md): Tenant-rooted fail-closed isolation.
- [L5-D-02-003](../../architecture/L5_Solution_Architecture/specifications/L5-D/L5-D-02-003_Hierarchical_Scope_and_Resource_Context_Architecture.md): Role/permission scope, resource context and workspace selection are not authorization.
- [L5-D-02-004](../../architecture/L5_Solution_Architecture/specifications/L5-D/L5-D-02-004_Policy_Decision_and_Enforcement_Architecture.md): deny precedence; decision context and enforcement before protected data access.
- [ADR-L5-0038](../../architecture/L5_Solution_Architecture/adr/ADR-L5-0038_Custom_Credential_API_Server_Session_and_MFA_Profile.md): web custom authentication API, Keycloak credential authority and secure server cookie; this is not yet a built/login-tested integration.
- [ADR-L5-0034](../../architecture/L5_Solution_Architecture/adr/ADR-L5-0034_Synchronous_API_Wire_Versioning_Error_and_Idempotency_Profile.md): contract-first OpenAPI 3.2, camelCase member names, major API version routing, RFC 9457 technical errors; read does not implicitly need a mutation idempotency key.
- [L5-G-01-003](../../architecture/L5_Solution_Architecture/specifications/L5-G/L5-G-01-003_Client_Security_Session_State_and_Contract_Consumption_Architecture.md), [G01-004](../../architecture/L5_Solution_Architecture/specifications/L5-G/L5-G-01-004_Future_Mobile_Application_Architecture.md): client-held state cannot become authority; old-context results barred; same backend contract reused by future mobile without forcing browser-session technology on mobile.

### User business-direction decision, 2026-10-10 — historical visibility (requires D02 policy approval)

- **Accepted user requirement:** When the **Tenant is Active**, an **appropriately authorized Administrator** may read the current identity/status of **Suspended and Archived Institutions for historical reference**. Archived Institutions remain permanently non-operational and cannot be modified. Suspended Institutions do not permit normal new operational activity. The word "Administrator" describes an intended user population, **not** a blanket cross-Institution/cross-Tenant permission grant.
- **Not accepted by implication:** unconditional visibility of any Institution, access when the **Tenant itself** is Suspended/Archived, record editing, restoration, historical audit export, sensitive Profile/Policy disclosure, universal Administrator grants, or Tenant-inactive bypass.
- **Architecture conflict/gap to escalate:** L2-A01 context resolution rejects inactive Tenants, while B01/L4-B-002/B-003 preserve historical reference for Suspended/Archived Institutions. A special inactive-Tenant retrieval capability requires **A01/D01/D02/A04/D03** upstream definition. Normal B01 API shall **fail closed** until then; do not invent a support-user bypass.
- **Next action:** B01 + D02 formalize specific read permissions per Institution status and resource scope; C01 releases an exact contract after review. The user's preference is an accepted **product requirement**, not a formally activated security policy or released endpoint.

See [P1-004 draft API contract](P1_004_B01_Protected_Institution_Summary_API_Draft_2026-10-10.md) and [P1-005 draft physical columns](P1_005_A01_B01_Column_Level_Persistence_Blueprint_2026-10-10.md).

## 4. Exact access/disclosure questions requiring owner decisions

| Question | Why it matters | Current status / required reviewer |
|---|---|---|
| Which authenticated principal/role assignments may view this one Institution? | Administrator is a UI persona, not a universal D02 grant. | **UNDECIDED — D02 + B01**, no permission key invented |
| Can Draft/Configured/Pending Approval/Returned/Rejected/Suspended/Archived Institutions be viewed, and by whom? | Lifecycle status alone cannot grant access; historical/reporting and risk policy differ. | **UNDECIDED — B01/D02**, no blanket visibility |
| May Institution Code and canonical UUID be exposed together? | Display utility vs ID/scope disclosure; UUID is identity not proof of permission. | **PROPOSED — B01 + D02/C01 review** |
| What is the authoritative current-source read (including version/freshness) under lifecycle changes? | Avoid stale or fake snapshots; support concurrency and audit traceability. | **UNDECIDED — B01/B02/C01** |
| How does request identify which Institution without granting cross-Institution enumeration? | Resource selector and Tenant context must be validated independently. | **UNDECIDED — C01/A04/D02** |
| Should unauthorized vs nonexistent Institution have indistinguishable external outcomes? | Avoid institution-existence discovery; exact API error policy must be approved. | **UNDECIDED — D02/C01/D03** |
| Who provides source-valid test Institution records while the B01 creation API is not yet built? | A raw inserted `Active` record cannot prove a governed activation decision. | **UNDECIDED — B01 + B02 test authority** |
| Is the first frontend entry the ordinary Administrator workspace, or a reviewer-only fixture route? | An unmounted/demo-only component is not a valid live journey. | **UNVERIFIED — G01/L6 + frontend owner** |

**Security stop rule:** Do not approve any response field solely because it appears in `L2-B01`. Source-defined data existence is **not** equivalent to consent, disclosure classification, caller capability or permission. The actual D02 policy and released C01 contract must authorize every visible field.

## 5. Contract outcome / failure matrix — semantics, not numeric response guesses

| Technical/business condition | Required outcome class | Additional review needed |
|---|---|---|
| Compatible contract + valid D01 session + valid A04 Tenant/Institution + D02 Allow + B01 record exists | **Authorized current B01 read** of the approved subset only | Approve exact C01 response, freshness and cache controls |
| No login / session expired or revoked | **Not authenticated / protected read unavailable**; no stale authenticated UI projection | D01/C01 approved technical error/status mappings |
| Authenticated, but D02 Deny / out-of-scope Institution | **Denied / protected data not returned** | External distinction from nonexistent resource needs D02/D03/C01 decision |
| Missing / mismatched Tenant context or Institution association | **Fail closed**, no fallback to other Tenant or demo/current default | A04/context technical mapping and negative tests |
| Nonexistent resource / stale resource selector | **No unauthorized disclosure**; technical outcome governed by contract | B01/C01/D02 non-enumeration policy |
| Backend/DB unavailable, network failure or timed-out read | **Technical failure or uncertainty**, not a fake business result | C01/G01 failure and bounded retry policy |
| API version unsupported/retired | **Explicit compatibility failure**, no silent fallback route | C01 compatibility lifecycle |
| User changes Tenant/Institution/session while request in flight | **Old response quarantined/discarded**; no prior-scope data displayed | G01 client epoch and D01/A04 revalidation |
| Source Institution status changes during/after read | Display only an attributable current/freshness-bound response; never claim permanent correctness | B01/B02 source/current-version strategy |

**No hard-coded public HTTP route, status integer, payload name, permission label or error-code string is established by this table.** RFC9457 error representation and owner-specific business rejection semantics remain governed, not improvised.

## 6. Acceptance tests to design BEFORE implementation

Minimum evidence after future code is authorized:

- **Correct Tenant:** authenticated, explicitly authorized user gets exactly their approved Institution's whitelisted B01 state, read directly from approved B01 test source, no frontend fixture.
- **Wrong Tenant:** an Institution from another Tenant cannot be retrieved even if its identifier is known; backend denies; no data/log leaks.
- **Same Tenant, wrong Institution/permission:** correct Tenant alone does not entitle the caller to all Institutions; D02 explicit access evaluation denies.
- **Unauthenticated/expired session:** backend denies; browser clears protected views and does not keep an old B01 result.
- **All lifecycle states:** authorized/unauthorized visibility for every source state is tested only after B01/D02 decide disclosure; Suspended/Archived does not automatically mean invisible nor freely visible.
- **Contract drift:** frontend rejects incompatible OpenAPI release; backend rejects retired/incompatible consumer version under source policy.
- **Context race:** start Institution A read, switch to Institution B or another Tenant, receive late A response; no A data appears in B screen.
- **Confidentiality:** response excludes unapproved policy, administrator, contact, regulatory, audit and downstream records.
- **Negative source:** missing B01 record or source unavailable never falls back to demo data.
- **Mobile-readiness:** selected API is client-neutral; actual native mobile authentication and app-version compatibility are future test gates, not passed today.

All tests above are **DESIGN PROPOSALS / NOT EXECUTED**. A successful demo or mock React test does not count as real backend integration.

## 7. Approval path and precise next engineering work

1. **B01 Institution owner:** approve the read's business meaning, field disclosure and treatment of lifecycle statuses; identify real authoritative projection/current-version semantics.
2. **D02/A04/D01/D03 security owners:** approve permission mapping and scope enforcement, session/trust profile and non-enumerating external outcomes. **No implicit Administrator privilege**.
3. **C01 contract owner:** create/review formally versioned OpenAPI 3.2 operation (exact URL, method, selector, JSON fields, error/status mappings) under canonical contract catalogue; mark **Released** only after proper approval. Verify client-neutral web/mobile contract.
4. **G01/L6 frontend:** confirm the real user-facing Institution screen/route and its authorized source consumption boundary; no reviewer-only/orphan component.
5. **B02/runtime:** approve isolated PostgreSQL 18.x owner-local source/migration, realistic test-only Tenant association and session integration. Document source-valid test-data establishment procedure.
6. **Lead Architect + GitHub protection:** verify appropriate approvals and base-branch controls, then authorize smallest implementation package. Do **not** merge draft planning PRs or start coding/CI merely because this memo exists.

### Current checkpoint

- This is a **draft disclosure-and-security decision packet**, not a released API, implementation specification, DDL or authorization policy.
- Exact request/response keys, route, API identifier, role/permission name, retention/cache TTL, visibility table and test fixtures remain **unverified/unapproved**.
- No code, migrations, database, Keycloak adapter, event, frontend API integration, tests or CI were created/executed.
