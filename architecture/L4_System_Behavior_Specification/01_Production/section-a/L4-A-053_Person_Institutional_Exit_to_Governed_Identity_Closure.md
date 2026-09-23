# Person Institutional Exit to Governed Identity Closure

# 1. Specification Metadata

**Specification ID:** L4-A-053  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Person & Identity  
**Realizes Scenario:** L3-A-053 — Person Institutional Exit to Governed Identity Closure  
**Primary Business Owner:** Person & Identity  
**Version:** 1.0.1  
**Status:** Architecture Approved  
**Owner:** Dhruv  
**Last Updated:** 2026-09-23  
**Depends On:** SutraERP L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-A01; L2-A02; L2-A03; L2-A06; L2-A07; L3-A-053  
**Referenced By:** None yet

---

# 2. Behavioral Objective & Boundary

Require a Tenant-scoped canonical Person whose institutional relationship has ended to leave operational participation through governed Person/Identity closure, preserve stable historical references and audit history, and allow anonymization or deletion only through their separately approved source-defined conditions.

This specification defines required observable system behavior only. It does not define APIs, database schemas, software classes, user interfaces, infrastructure, deployment, transport mechanisms, or technical orchestration.

---

# 3. Required Business Context

- Exactly one Tenant and canonical Person are resolved, with associated Identity/User Account references.
- VAR:Relationship-Owning Domain supplies a governed relationship-end or closure basis without acquiring Person/Identity ownership.
- Dependencies, retention obligations, downstream references and Authentication sessions can be evaluated.
- Person & Identity owns Person, Identity and User Account lifecycle; Authentication owns session termination.

---

# 4. System Behavioral Obligations

| Behavior ID | Condition | Required System Behavior | Governed Result | Provenance |
|---|---|---|---|---|
| L4-A-053-BH-001 | A governed institutional-exit or closure basis is received. | Validate Tenant, canonical Person, relationship-end evidence, closure authority basis, associated Identity/User Account and downstream references before state change. | One canonical closure subject and boundary are established. | L3-A-053 §§7–9; L2-A02 |
| L4-A-053-BH-002 | The Person is operationally Active and closure is eligible. | Apply the source-defined `Active → Inactive` Person transition, disable login capability under owned identity/account rules, and request termination of active Authentication sessions. | Operational participation stops without transferring ownership. | L3-A-053 §§9,13; L2-A02; L2-A03 |
| L4-A-053-BH-003 | Person inactivation succeeds. | Publish `PersonInactivated` from the Person aggregate only after the transition completes. | Consumers receive the canonical inactivation fact. | L3-A-053 §14; L2-A02 |
| L4-A-053-BH-004 | Archival eligibility is established after dependency and retention validation. | Archive the Person and associated Identity consistently, preserve stable identifiers and downstream references, and prevent authentication. | Canonical identity becomes historical/read-only while references remain resolvable. | L3-A-053 §§9,12–13; L2-A02 |
| L4-A-053-BH-005 | Person archival succeeds. | Publish `PersonArchived` from the Person aggregate and publish only the corresponding successful Identity-owned lifecycle fact(s) defined upstream. | Aggregate-owned closure facts become observable without invented coordination order. | L3-A-053 §§14,17; L2-A02 |
| L4-A-053-BH-006 | An approved privacy decision permits anonymization after archival. | Mask only policy-permitted personal data and publish `PersonAnonymized` after the Person reaches the source-defined `Anonymized` state. | Privacy treatment preserves business/audit references. | L3-A-053 §§9–14; L2-A02 |
| L4-A-053-BH-007 | Exceptional permanent deletion is requested. | Require source-defined administrative approval, dependency validation, legal-retention validation and security confirmation before deletion. | Deletion cannot bypass retention or governance. | L3-A-053 §§9,12,15; L2-A02 |
| L4-A-053-BH-008 | Exceptional deletion succeeds. | Publish `PersonDeleted` from the Person aggregate while preserving retained business and audit references required by policy. | Deletion is observable without erasing governed history. | L3-A-053 §§9,14,16; L2-A02 |
| L4-A-053-BH-009 | Session termination remains incomplete or a blocking dependency exists. | Do not assert completed closure; retain completed governed transitions and continue only after the blocking condition is resolved. | Closure cannot falsely appear complete. | L3-A-053 §11 |

---

# 5. Cross-Domain & Event Behavior

- VAR:Relationship-Owning Domain owns the relationship-end fact; the exact hand-off/event is undefined and is not invented.
- Authentication owns active-session termination and does not own Person/Identity state.
- `PersonInactivated`, `PersonArchived`, `PersonAnonymized`, `PersonDeleted` are Person-aggregate events; `IdentityDisabled`/`IdentityArchived` belong to Identity when their corresponding transitions succeed.
- Event Infrastructure distributes canonical events and Audit Infrastructure preserves closure evidence.

---

# 6. Failure / Rejection / Continuation

- Invalid closure authority or unresolved Tenant/Person → reject state change.
- Blocking dependency or retention requirement → stop the affected archival/anonymization/deletion transition.
- Incomplete session termination → do not assert completed operational closure.
- Attempt to break stable historical references or directly erase retained history → reject.

---

# 7. Audit & Evidence Behavior

Preserve Tenant, canonical Person/Identity references, relationship-end basis, authority evidence, dependency/retention decisions, prior/resulting states, session-termination outcome, anonymization/deletion approvals where applicable, canonical events, actors and correlation while protecting personal data.

---

# 8. Observable Completion Conditions

- Operational login/session use is unavailable for the closure subject.
- Person and Identity reach their source-governed archival outcome with stable historical references preserved.
- Optional anonymization/deletion occurs only when separately authorized and eligible.
- Only successful aggregate transitions publish their canonical events.
- Unresolved hand-off/actor/order questions remain explicit.

---

# 9. Traceability & Provenance

- **Primary parent:** `L3-A-053` — Person Institutional Exit to Governed Identity Closure.
- **Canonical owner:** Person & Identity.
- Higher-level ownership, business rules, lifecycle states and canonical event publishers remain governed by the cited L1/L2/L3 sources.
- This L4 specification refines observable system obligations; it does not revise or silently repair upstream architecture.

---

# 10. Open Issues / Deferred Design

| ID | Deferred Design / Upstream Gap | L4 Treatment |
|---|---|---|
| A053-L4-OPEN-01 | Exact contract/event for the relationship-owning domain to supply institutional exit and the exact privacy/deletion approval actors are undefined. | Preserve the gap explicitly; do not invent missing architecture at L4. |
| A053-L4-OPEN-02 | Exact coordination ordering among `PersonArchived`, `IdentityDisabled` and `IdentityArchived` is not specified upstream. | Preserve the gap explicitly; do not invent missing architecture at L4. |

No implementation design is introduced by these deferred items.


## Canonical Validation Note

This specification passed the scoped R3A hard-fail validation against the Locked L4 Governance and Architecture Standard and L4 Validation Standard and is `Architecture Approved` under `L4_R3A_IAM_Safe_Authority_Architecture_Approval_Decision_2026-09-23.md`. Existing non-blocking upstream gaps remain bounded exactly as recorded; this approval does not invent missing actors, lifecycle states, events, publishers, recovery contracts, provider mechanics, or implementation design.
