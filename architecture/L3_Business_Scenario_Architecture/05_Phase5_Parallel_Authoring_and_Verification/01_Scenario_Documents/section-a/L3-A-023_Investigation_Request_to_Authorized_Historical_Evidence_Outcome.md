# Investigation Request to Authorized Historical Evidence Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-A-023  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Audit Infrastructure  
**Participating Domains:** A, VAR:Investigated Domain  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows; L3-A-022 — Canonical Business Event to Immutable Audit Evidence Availability  
**Referenced By:** Authorized institutional investigation, governance, compliance and legal-review activities

---

# 2. Scenario Purpose

Reconstruct authorized historical business activity from references to immutable Audit Evidence, producing an investigation-scoped package and derived views without changing, duplicating or re-owning canonical evidence.

---

# 3. Business Outcome

Relevant audit evidence is assembled by reference into the governed investigation scope, derived timeline and correlation/causation views are produced as needed, and the authorized investigation outcome is made available without modifying or duplicating canonical evidence.

The locked L2 workflow and aggregate model support the outcome. This Draft is classified `PASS WITH ISSUES` because the universal investigation requestor/approver and a canonical investigation-completion event are not defined.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorized institutional investigation requires reconstruction of historical business activity.

## 4.2 End Boundary

The scenario ends when the authorized investigation scope has an immutable Evidence Package of references, required timeline/correlation/causation views, and a read-only Investigation View available to the authorized requester.

## 4.3 In Scope

- Request, identity, Tenant, purpose, authorization and scope validation.
- Retrieval of canonical Audit Evidence by reference.
- Evidence Package assembly without duplication.
- Derived timeline, correlation and causation views.
- Authorized Investigation View availability and investigation traceability.

## 4.4 Out of Scope

- Editing or deleting Audit Evidence.
- Reproducing complete evidence records inside the package.
- Deciding the investigated domain's business outcome.
- Integrity verification under A024 unless separately requested.
- Archived-evidence restoration under A027 unless required and authorized.
- UI, search implementation, database or reporting design.

## 4.5 Related Scenarios

| Relationship | Scenario | Reason |
|---|---|---|
| Upstream | L3-A-022 — Canonical Business Event to Immutable Audit Evidence Availability | Supplies immutable canonical evidence. |
| Related | L3-A-024 — Audit Evidence Verification Request to Integrity Verification Outcome | May verify selected evidence relationships. |
| Related | L3-A-027 — Archived Audit Evidence Request to Authorized Restored Access | Restores authorized access where evidence is archived. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Audit Infrastructure / Section A | Owns evidence retrieval, Evidence Package, Timeline Projection, Investigation View and investigation traceability. |
| VAR:Investigated Domain | Owns the business facts and supplies business meaning; does not own or modify Audit Evidence. |
| Platform & Tenant / Section A | Supplies Tenant and institutional scope. |
| Person & Identity / Authentication | Supply requester identity and authenticated context. |
| Authorization & Policy / Section A | Owns investigation-access and scope decisions. |
| Event Infrastructure / Section A | Distributes `Evidence Package Created` and `Timeline Projection Read Model Generated` where published. |

The frozen Participating Domains value remains exactly `A, VAR:Investigated Domain`.

---

# 6. Business Actors, Roles and Responsibilities

L2-A07 requires an authorized investigation and records requester attribution but does not define a universal Investigation Requestor or Investigation Approver role. Domain-specific actors may initiate through governed policy, but none is invented here.

---

# 7. Preconditions and Entry Criteria

- A legitimate institutional investigation purpose and Tenant scope exist.
- The requester is authenticated and the investigation is authorized under applicable policy.
- Relevant immutable Audit Evidence exists or can be located through authorized query.
- Requested scope, classification and access boundaries are explicit.
- Archived evidence is restored separately under A027 where required.

---

# 8. Trigger

An authorized institutional investigation requires reconstruction of historical business activity.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the investigation request, purpose and requested scope. | Authorized actor undefined; Audit Infrastructure | A governed investigation begins. |
| 2 | Validate requester identity, authorization, Tenant, institution, classification and investigation scope. | Authorization & Policy; Platform & Tenant; Audit Infrastructure | Authorized retrieval boundary is fixed. |
| 3 | Locate relevant immutable evidence using entity, actor, workflow, time, correlation and causation references permitted by scope. | Audit Query Service | Canonical evidence references are selected. |
| 4 | Retrieve the applicable Timeline Projection Read Model or generate one from canonical evidence where required. | Timeline Projection Service | A derived chronological view is available. |
| 5 | Resolve correlation and direct causation chains required for reconstruction. | Audit Query Service; Correlation/Causation services | Cross-domain progression can be understood. |
| 6 | Assemble an immutable investigation-scoped Evidence Package containing references rather than duplicated evidence. | Evidence Packaging Service | The governed evidence collection is established. |
| 7 | Publish `Evidence Package Created` and, when generated, `Timeline Projection Read Model Generated`. | Audit Infrastructure | Package/view availability becomes traceable. |
| 8 | Construct the read-only Investigation View from the package and derived views. | Audit Infrastructure | An authorized historical reconstruction exists. |
| 9 | Return the Investigation View to the authorized requester. | Audit Infrastructure | The frozen outcome is available. |
| 10 | Preserve request, scope, authorization, retrieval and outcome history. | Audit Infrastructure | Investigation accountability is complete. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Path | Outcome | Owner |
|---|---|---|---|---|
| Existing package | A governed reusable Evidence Package already matches the authorized scope. | Validate its scope and retrieve it without reassembly. | Canonical evidence remains referenced once. | Audit Query Service |
| Timeline not required | The authorized question can be resolved without chronology. | Omit timeline generation and use the permitted evidence relationships. | Same authorized investigation outcome. | Audit Infrastructure |
| Multi-domain scope | Correlated activity spans several business domains. | Follow canonical correlation/causation references within authorization boundaries. | Ownership of every domain fact remains unchanged. | Audit Infrastructure; investigated domains |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Resolution | Owner |
|---|---|---|---|
| Request or scope is not authorized. | Evidence cannot be disclosed. | Deny retrieval and preserve the decision. | Authorization & Policy; Audit Infrastructure |
| Scope crosses an unauthorized Tenant or institution. | Isolation would be violated. | Reject or require a newly governed scope. | Platform & Tenant; Authorization & Policy |
| Evidence is archived and unavailable. | Investigation cannot yet access the reference. | Obtain governed restored access under A027. | Audit Infrastructure |
| Required evidence cannot be located. | Reconstruction may be incomplete. | Return a governed incomplete outcome; do not fabricate or duplicate evidence. | Audit Query Service |
| Package integrity validation fails. | Investigation collection is unreliable. | Do not publish package availability; correct the scope/reference issue. | Evidence Packaging Service |
| Derived view conflicts with canonical evidence. | The view cannot be authoritative. | Discard/regenerate the derived view; canonical evidence remains unchanged. | Timeline Projection Service |

No technical query or storage recovery is defined.

---

# 12. Business Rules and Decision Constraints

| Rule | Owner | Effect | Source |
|---|---|---|---|
| Investigation access is policy governed and read-only. | Authorization & Policy; Audit Infrastructure | Unauthorized or mutating investigation is prohibited. | L2-A07 §§7.7, 19.9, 28 |
| Evidence Packages contain references, never duplicate evidence. | Audit Infrastructure | Canonical evidence remains the sole authority. | L2-A07 §12.8; A07-W07 |
| Packages are immutable and investigation scoped after assembly. | Audit Infrastructure | Scope cannot silently expand. | L2-A07 Evidence Package constraints |
| Timelines are derived views with no lifecycle or authority over evidence. | Audit Infrastructure | A view cannot replace historical evidence. | L2-A07 §12.9; ADR-004 |
| Correlation groups related activities; causation identifies direct predecessors. | Audit Infrastructure | Relationships retain distinct meanings. | L2-A07 §§12.6–12.7 |
| Investigation never changes the investigated domain's facts. | VAR:Investigated Domain | Business ownership remains unchanged. | L2-A07 domain boundaries |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result | Owner |
|---|---|---|---|---|
| Audit Evidence | Immutable and available/restored | Retrieved by reference only. | Unchanged. | Audit Infrastructure |
| Evidence Package | Does not exist or reusable package exists | Assemble/validate references for scope. | Immutable investigation-scoped package. | Evidence Packaging Service |
| Timeline Projection | Absent or existing | Generate/retrieve derived chronology where needed. | Derived read model; no lifecycle. | Timeline Projection Service |
| Investigation View | Does not exist | Assemble authorized read-only result. | Available to authorized requester. | Audit Infrastructure |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Events

| Event | Publisher | Condition | Effect |
|---|---|---|---|
| `Evidence Package Created` | Audit Infrastructure | Immutable package assembly succeeds. | Authorized investigation consumers learn package availability. |
| `Timeline Projection Read Model Generated` | Audit Infrastructure | Required derived timeline is generated. | Authorized consumers may retrieve the chronological view. |

L2-A07 defines no canonical Investigation View completion event.

## 14.2 Contracts

| Provider | Consumer | Hand-off |
|---|---|---|
| VAR:Investigated Domain | Audit Infrastructure | Business meaning and entity references. |
| Authorization & Policy | Audit Infrastructure | Investigation-access and scope decision. |
| Audit Query Service | Packaging/Timeline services | Authorized evidence references and relationship results. |
| Audit Infrastructure | Authorized requester | Evidence Package, derived views and Investigation View. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

Every investigation query, package and view is limited by requester identity, Tenant, institution, classification, purpose and approved scope.

## 15.2 Approval and Separation of Duty

Applicable Investigation Policy supplies approval. L2-A07 defines no universal approver, SoD or override role; none is introduced.

## 15.3 Audit Obligations

Requester, purpose, authorization, Tenant/institution, scope, evidence references, generated package/view identities, access and returned outcome must remain traceable.

---

# 16. Completion Criteria and Postconditions

Completion requires validated authorization and scope, relevant canonical evidence references, required timeline/correlation/causation views, an immutable Evidence Package, an authorized Investigation View, and preserved investigation history.

Postconditions: original evidence and domain facts remain unchanged; packages contain references only; derived views remain non-authoritative; access remains subject to policy.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Impact | Resolution |
|---|---|---|
| **A023-ISSUE-01:** L2-A07 does not define the universal investigation requestor or approving role. | Exact L4/RBAC responsibility cannot be mapped. | Define role-to-request/approval authority through Investigation Policy. |
| **A023-ISSUE-02:** No canonical event indicates Investigation View completion. | The result is available synchronously, but event-driven consumers lack an exact completion fact. | Define an event and publisher if enterprise notification is required. |
| **A023-ISSUE-03:** L2-A07 says business domains never invoke audit workflows directly, while A07-W09 receives an investigation request. | The initiating contract boundary is ambiguous. | Define the canonical request contract or initiating event between the requester and Audit Infrastructure. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
