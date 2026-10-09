# SutraERP backend P1-002 — First protected-read integration candidate

**Date:** 2026-10-09  
**Status:** DRAFT; SOURCE-BACKED CANDIDATE; **NO APPROVED OR RELEASED API**, no backend code.  
**Decision proposal:** Start with an **authenticated, Tenant/Institution-scoped read of B01's current Institution identity and lifecycle position**, shown in a real Administrator-facing frontend view. Exclude Institution activation, profile revision, policy editing and cross-owner events from this first demonstration.  
**Governance:** B01 owner, C01 contract owner, D01 identity/session, D02 authorization, A04 scope, B02 persistence, G01 frontend and Lead Architect must clear their respective gates before implementation.  
**Source snapshot:** Architecture main `933cf5d06f565cc52ef7e3b2a7f821606c2b9b6f`; Frontend main `372376d23ac6478fa4ac3f209cb97c71317d89d2`; new private Backend main `213c323303bf9fff4284c15b94f9d4735d68759a` (README only). These are planning evidence pins, not proof of release. All older backend code is user-designated obsolete and not an implementation baseline.

## 1. Verified business and interface evidence

- [Locked L1 Domain Architecture](../../architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md) §§3.1,24: Tenant and Institution are distinct; Institution owns Institution business identity/lifecycle; one Tenant can contain multiple Institutions.
- [L2-B01 Institution Workflow](../../architecture/L2_Domain_Workflow_Architecture/section-b/L2-B01_Institution_Workflow.md) §§2,4–7,12–13: Institution owns canonical aggregate, identity, code, lifecycle and configuration. Every Institution belongs to exactly one Tenant; Institution Code is unique **within** a Tenant, not globally. L2 lists Draft, Configured, Pending Approval, Returned, Rejected, Active, Suspended, Archived. It does **not** grant every logged-in person access to every Institution.
- [L2-B01 locked activation authority amendment](../../architecture/L2_Domain_Workflow_Architecture/section-b/L2-B01_Institution_Activation_Authority_Amendment_2026-08-24.md): Final Institution activation is reserved to governed Institution Activation Authority; Institution Administrator does not automatically have that authority. Our first read **does not** perform activation or verify approval workflow.
- [L4-B-001 Establishment/Activation](../../architecture/L4_System_Behavior_Specification/01_Production/section-b/L4-B-001_Institution_Establishment_to_Final_Activation_Decision.md) v1.0.1 is **Architecture Approved**; establishes identity/Tenant/activation evidence. It does **not** define a released read endpoint or response body.
- [L4-B-004 Institution Profile Revision](../../architecture/L4_System_Behavior_Specification/01_Production/section-b/L4-B-004_Institution_Profile_Revision.md) v1.0.0 has **Status: Review** on the file. Do NOT silently treat profile revision as approved, or choose it as first write. A read of current canonical identity/status requires separate exact permission/contract approval.
- Frontend `app/features/product-workspaces/j01-b01-institution.tsx` is explicitly **mock-only activation**, using `executeDemoMockOperation`. `app/features/institution-structure-batch5/institution-structure-batch5-proofs.tsx` renders a read-only **fixture-based** reference. The current `product-workspace.tsx` source inspected does not prove the J01B01 mock component has a live workspace entry point. **Exact target route and integration seam require verification by frontend owner**; do not connect an orphan/reviewer-only component and call the journey integrated.
- [C01 contract architecture](../../architecture/L5_Solution_Architecture/specifications/L5-C/L5-C-01-002_Synchronous_Service_Contract_Architecture.md) requires released contract, scope, authorization and owner-local result. [Locked ADR-L5-0034](../../architecture/L5_Solution_Architecture/adr/ADR-L5-0034_Synchronous_API_Wire_Versioning_Error_and_Idempotency_Profile.md): OpenAPI 3.2, camelCase JSON, major version routes, RFC 9457 errors. No B01 read route/DTO is invented by this memo.
- [L5-G-01-004 future mobile](../../architecture/L5_Solution_Architecture/specifications/L5-G/L5-G-01-004_Future_Mobile_Application_Architecture.md): mobile and web consume **same governed backend contracts**; native mobile framework and auth carrier are still deferred.

## 2. Minimal logical ownership model — not SQL

| Logical information | Authority | Constraint | Scope for this slice |
|---|---|---|---|
| Tenant | A01 Platform/Tenant | Mandatory trusted Tenant root | Resolve server-side; never infer from UI default |
| Institution identity | B01 Institution | Exactly one parent Tenant per Institution | Fetch current B01-owned identity only |
| Institution code | B01 Institution | Unique per Tenant | Display only if approved contract permits |
| Institution lifecycle position | B01 Institution | Controlled by B01 state machine | Read existing authoritative state only |
| Institution Profile | B01 Institution | Distinct from Policy; effective history matters | OUT OF FIRST SLICE |
| Institution Policy | B01 Institution | Distinct from D02 authorization policy | OUT OF FIRST SLICE |
| Session/principal | D01 | Authoritative authenticated context | Must be real and validated |
| Access decision | D02 | Exact operation/resource permission | Never presume all Administrators may read |
| Scope validation | A04 | Institution subordinate to Tenant | Reject mismatch, missing or switched context |
| Browser projection | G01 | Non-authoritative, scope-partitioned | Discard old-context responses |

**Unapproved:** exact database schema, IDs/column types, read endpoint, JSON DTO, permission name, caching policy and response status mapping. These must be specified/released by the applicable owners.

## 3. Delivery gates: sequence and acceptance evidence

### R0 — Secure review environment and first-slice eligibility

- [ ] User verifies GitHub protection and review process (connected API currently reports `protected:false`; no trust in unverified private-repo rules). No autonomous coding/merging until confirmed.
- [ ] B01 owner/Lead Architect accepts **read-only current identity/status** as bounded first slice; L4-B-004 profile revision is explicitly excluded.
- [ ] Frontend owner locates live Administrator workspace entry and appropriate non-mock integration seam; preserves mock/reviewer evidence separately.
- [ ] Plan synthetic but **owner-consistent** local Tenant/Institution test data. Do not promote frontend mock fixtures to authoritative institutional outcomes or bypass B01 creation/lifecycle constraints.
- [ ] Agree test-only D01/Keycloak, D02 policy and A04 scope mechanism; no real student/employee data or secret in repository.

### R1 — Governed read API and identity/scope requirements

- [ ] Approve and formally **release** versioned C01 OpenAPI 3.2 read contract, bounded to B01 and a defined consumer-visible field whitelist. No endpoint or payload exists merely because the plan describes it.
- [ ] Decide exactly which Institution lifecycle states may be disclosed, to which authenticated principals and which Tenant/Institution scope. These are D02/B01 decisions, not a generic "administrator can read" assumption.
- [ ] Decide D01 session outcomes, browser cookie/CSRF deployment profile, D02 resource authorization and A04 failure handling.
- [ ] Identify contract catalogue/version/digest, approved errors, no-information leakage, cache and freshness obligations; JSON names/version/errors conform to locked C01 profile.
- [ ] Design owner-local B01 persisted identity/current-status and Tenant association under B02; no cross-owner direct SQL or global lookup.

### R2 — Minimal real backend read

- [ ] Once R0/R1 approved, implement narrow Rust/Axum/SQLx read of the **B01-owned** record with validated Tenant root, protected Institution association, trusted D01 session and D02 authorization.
- [ ] Verify local PostgreSQL 18.x migrations and B01 owner-scoped runtime access, no default Tenant and safe logging.
- [ ] Negative tests: absent/expired session, wrong Tenant, same Tenant but unauthorized Institution, missing Institution, missing/stale scope, permission denial, concurrency with scope switch, unavailable service, contract version mismatch.
- [ ] Confirm protected data is never leaked by error responses or logs. Actual test results to be recorded only after execution.

### R3 — Real frontend integration, not mock evidence

- [ ] Frontend integrates via version-pinned released contract adapter in real user-facing workspace. Mock mode and real mode explicitly distinguished.
- [ ] Render Loading / Authoritative result / Denied / Expired session / Technical failure / Unsupported version; never fabricate success.
- [ ] After Tenant, Institution or session switch, quarantine old cached records and late requests.
- [ ] Exercise actual browser → backend HTTP → B01 authoritative persistence → response → UI. Confirm navigation entry, accessibility and responsive behavior.
- [ ] Capture exact frontend/backend SHA and released contract version/digest with actual positive and negative test evidence.

### R4 — Admit the next incremental feature

- [ ] Declare only **this protected read** integrated, not 595 workflows, backend production readiness or native mobile implementation.
- [ ] Review a separate **source-approved owner-local write** candidate; do not choose B01 activation without its complete governance/approval and durable `InstitutionActivated` business contract.
- [ ] Add owner-local outbox/event publication, schema, retries and consumer checks only for an appropriate source-authorized event use case. No artificial event to show off messaging.
- [ ] Revisit mobile session/token mechanism and installed-app compatibility before native mobile coding; same core API remains reusable.

## 4. Critical issues to avoid before code

1. **No verified released B01 read API:** this planning file itself is not an API contract. Block actual backend/frontend integration until R1.
2. **L4-B-004 is Review:** do not use profile revision as a first business mutation.
3. **Auth bootstrap is substantial:** read-only does not mean unauthenticated; Keycloak/custom D01 session with D02/A04 isolation must be designed and tested in the real path.
4. **Frontend mock not guaranteed route-wired:** confirm the actual W-B01 user entry; no unmounted demonstration components.
5. **The first B01 read may require creating valid B01 test data:** a fake Active record inserted with raw test SQL cannot prove valid activation business behavior. Test truth must distinguish controlled setup from a source-earned business transition.
6. **Old backend source is obsolete:** no legacy code reuse under the new `SutraERP_backend` baseline without separate authorization.
7. **Mobile-compatible architecture is not native-mobile-tested:** no mobile login/storage/framework implementation is claimed.

## 5. Beginner debugging orientation

If the Institution page fails, check: **(1)** was a real API called, not a fixture? **(2)** is the login/session valid? **(3)** does the user have permission for this Institution? **(4)** was the requested Institution in the validated Tenant? **(5)** do frontend/backend agree on the released API version? **(6)** was an old request shown after switching scope?

**Execution:** read-only architecture and frontend evidence inspection, draft document change only. No Rust code, API, SQL migration, CI, test or merge was performed. P0 still has pending gates. Broad A–J logical data mapping may continue in parallel; no need to design every physical table before the first approved slice.
