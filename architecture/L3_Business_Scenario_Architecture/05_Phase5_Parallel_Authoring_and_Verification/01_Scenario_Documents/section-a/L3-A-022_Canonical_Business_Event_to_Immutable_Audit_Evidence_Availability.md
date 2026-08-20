# Canonical Business Event to Immutable Audit Evidence Availability

# 1. Scenario Metadata

**Scenario ID:** L3-A-022  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Audit Infrastructure  
**Participating Domains:** A, VAR:Audited Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows; L3-A-019 — Committed Business Fact to Reliable Consumer Delivery  
**Referenced By:** Investigations, integrity verification, retention governance, archival, restoration, compliance, reporting and analytics

---

# 2. Scenario Purpose

Convert an auditable canonical business event into complete immutable Audit Evidence while preserving the originating domain's business ownership and adding the governed context required for accountability and historical reconstruction.

---

# 3. Business Outcome

Immutable audit evidence is generated with required context, actor attribution, change attribution, correlation, causation, classification and traceability while preserving the originating domain's business ownership.

The locked L2-A07 workflow, aggregate model and produced event define this outcome completely. The scenario is classified `PASS`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a canonical business event representing an auditable completed activity is received from the Event Infrastructure.

## 4.2 End Boundary

The scenario ends when one complete immutable Audit Evidence aggregate exists, `Audit Evidence Generated` is published, the evidence is available for authorized investigation, and timeline projection is eligible for update.

## 4.3 In Scope

- Audit-eligibility validation.
- Tenant, institutional, workflow, session, source and security context resolution.
- Actor and delegation attribution by reference.
- Logical change attribution without business-object duplication.
- Correlation and direct causation association.
- Evidence classification and immutable creation.
- Publication of the canonical audit lifecycle event.

## 4.4 Out of Scope

- Changing or re-owning the originating business fact.
- Authenticating actors or deciding authorization policy.
- Copying complete business aggregates into audit storage.
- Investigation, verification, retention, archival or restoration outcomes.
- Technical persistence, database, API, UI or deployment design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Upstream | L3-A-019 — Committed Business Fact to Reliable Consumer Delivery | Supplies the canonical completed-fact event. |
| Followed by | L3-A-023 — Investigation Request to Authorized Historical Evidence Outcome | Uses immutable evidence for historical reconstruction. |
| Followed by | L3-A-024 — Audit Evidence Verification Request to Integrity Verification Outcome | Verifies evidence and its relationships without modification. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Audit Infrastructure / Section A | Primary owner. Owns Audit Evidence, Entry, Context, Actor references, Change Set, Correlation, Causation, classification and audit lifecycle event. |
| VAR:Audited Domain | Owns the original business fact, event semantics, business entity and business decision. |
| Event Infrastructure / Section A | Delivers the canonical event and distributes the resulting audit event. |
| Platform & Tenant / Section A | Supplies Tenant and institutional context. |
| Person & Identity / Section A | Supplies identity references. |
| Authentication / Section A | Supplies authentication/session references. |
| Authorization & Policy / Section A | Supplies authorization and policy references. |

The frozen Participating Domains value remains exactly `A, VAR:Audited Domain`.

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Boundary |
|---|---|---|
| Audit Actor | Canonical attribution of the human, delegated authority, automated process, scheduled service or integration accountable for the audited activity. | A logical reference only; identity, authentication and authorization remain owned by A02–A04. |

The scenario is event initiated; no separate human Audit Evidence creator is defined.

---

# 7. Preconditions and Entry Criteria

- The originating business transaction and auditable activity completed.
- A registered canonical business event was successfully published through Event Infrastructure.
- The event satisfies the applicable Evidence Generation Policy.
- Tenant context and required business references are available.
- Actor, change, correlation and causation information can be resolved from authoritative references.

---

# 8. Trigger

A canonical business event representing an auditable completed activity is received from the Event Infrastructure.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the immutable canonical event from Event Infrastructure. | Audit Infrastructure | Evidence generation begins from one completed fact. |
| 2 | Validate audit eligibility under the applicable policy and Tenant context. | Audit Evidence Generation Service; Authorization & Policy supplies policy decision | The event is eligible or excluded by governance. |
| 3 | Resolve Tenant, institution, organization, workspace, session, workflow, source, channel and security references. | Context Resolution Service; applicable Section A providers | Audit Context is complete without re-owning source data. |
| 4 | Resolve the accountable actor, actor type, delegation, identity, authentication and authorization references. | Actor Attribution Service | Immutable actor attribution is established. |
| 5 | Identify affected business-entity references, operation and permitted logical changes. | Change Attribution Service; VAR:Audited Domain semantics | A Change Set describes the activity without duplicating the aggregate. |
| 6 | Generate or reuse the Correlation Identifier for the logical process. | Correlation Service | Related evidence can be reconstructed. |
| 7 | Associate at most one direct Causation reference where applicable. | Causation Service | Immediate cause-and-effect traceability is preserved. |
| 8 | Apply the governed Audit Classification. | Audit Evidence Generation Service; Authorization & Policy supplies classification policy | Evidence sensitivity and handling reference are established. |
| 9 | Construct and commit one complete immutable Audit Evidence aggregate. | Audit Evidence Generation Service | Evidence is created once and becomes immutable. |
| 10 | Publish `Audit Evidence Generated`. | Audit Infrastructure | Authorized downstream consumers learn that evidence exists. |
| 11 | Make the evidence available for authorized investigation and eligible timeline projection. | Audit Infrastructure | The frozen outcome is complete. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Path | Outcome | Owner |
|---|---|---|---|---|
| No direct causation | No immediate predecessor exists. | Create evidence without a Causation reference. | Evidence remains valid; causation is optional and correlation remains available. | Audit Infrastructure |
| Existing correlation | Related evidence already has a logical process correlation. | Reuse the authoritative Correlation Identifier. | Multi-domain process history remains grouped. | Correlation Service |
| Automated actor | Activity was performed by a system, integration or scheduled process. | Attribute the supported actor type and authoritative references. | Accountability is preserved without inventing a human. | Actor Attribution Service |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Governed Resolution | Owner |
|---|---|---|---|
| Event is not canonical, completed or audit eligible. | Evidence cannot be generated under A07-W01. | Reject evidence generation and preserve the policy outcome. | Audit Infrastructure; Event Infrastructure |
| Tenant context is absent or invalid. | Isolation cannot be guaranteed. | Stop evidence creation until authoritative context is available. | Platform & Tenant; Audit Infrastructure |
| Required actor or context reference cannot be resolved. | Complete accountability is absent. | Do not partially commit evidence; resolve the authoritative reference first. | Audit Infrastructure; applicable provider |
| Change information attempts full business snapshot duplication. | Audit boundary would be violated. | Retain only permitted logical changes and entity references. | Change Attribution Service |
| Duplicate evidence would be generated for the same event. | Evidence uniqueness would be violated. | Prevent duplicate creation and retain the original evidence identity. | Audit Evidence Generation Service |
| Evidence cannot be completed atomically. | An incomplete record would be unreliable. | Do not treat partial evidence as created or publish `Audit Evidence Generated`. | Audit Infrastructure |

No technical retry, transaction or storage recovery is defined.

---

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Audit Evidence is generated from canonical events. | Audit Infrastructure; Event Infrastructure | Business domains do not invoke evidence-generation workflows directly. | L2-A07 ADR-001; A07-W01 |
| One complete event produces one immutable Audit Evidence aggregate. | Audit Infrastructure | Evidence is created once, not partially or repeatedly. | L2-A07 §§12.1, 15.1 |
| Original events and business ownership remain unchanged. | VAR:Audited Domain | Evidence references facts; it does not become business ownership. | L2-A07 §§7.2–7.4 |
| Evidence is Tenant isolated, globally identifiable, never deleted or overwritten. | Audit Infrastructure | Historical traceability is permanent. | L2-A07 Audit Evidence constraints |
| Complete business aggregates are not duplicated. | Audit Infrastructure; VAR:Audited Domain | Only references and logical changes are retained. | L2-A07 Change Set constraints; ADR-005 |
| Actor, context, correlation and causation are immutable after creation. | Audit Infrastructure | Accountability relationships cannot be rewritten. | L2-A07 aggregate constraints |
| Policy evaluation remains owned by Authorization & Policy. | Authorization & Policy | A07 consumes immutable policy decisions and references. | L2-A07 §28 |

---

# 13. State and Lifecycle Impact

| Concept | Entry Condition | Impact | Result | Owner |
|---|---|---|---|---|
| Canonical business event | Published and immutable | Consumed without modification. | Unchanged. | VAR:Audited Domain owns fact; Event Infrastructure distributes |
| Audit Evidence | Does not exist | Create once with all subordinate evidence components. | Created Once; immutable. | Audit Evidence Generation Service |
| Storage Status | No evidence exists | Initial governed availability after creation. | `Active Storage` | Audit Infrastructure |
| Availability Status | No evidence exists | Make evidence retrievable under authorization. | `Available` | Audit Infrastructure |
| Governance Status | No evidence exists | Apply active preservation obligation. | `Under Retention` | Audit Infrastructure |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Business Events

| Event | Publisher | Condition | Effect |
|---|---|---|---|
| Registered canonical business event | VAR:Audited Domain; distributed by Event Infrastructure | Auditable completed activity was published. | Initiates A07-W01. |
| `Audit Evidence Generated` | Audit Infrastructure | Complete immutable Audit Evidence was successfully created. | Investigation, timeline, compliance and monitoring consumers learn of evidence availability. |

## 14.2 Contracts and Hand-offs

| Provider | Consumer | Contract / Information |
|---|---|---|
| Event Infrastructure | Audit Infrastructure | Canonical business event and metadata. |
| Platform & Tenant | Audit Infrastructure | Tenant and institutional context references. |
| Person & Identity / Authentication / Authorization | Audit Infrastructure | Identity, session, authentication, delegation and policy references. |
| VAR:Audited Domain | Audit Infrastructure | Business entity, operation and logical change semantics. |
| Audit Infrastructure | Authorized consumers | Immutable Audit Evidence identity and availability. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

Evidence generation follows Evidence Generation and Classification Policies. Later access is separately governed. Authentication and authorization references are preserved without re-owning them.

## 15.2 Approval and Separation of Duty

No separate human approval or scenario-specific SoD rule is defined for deterministic evidence generation from an eligible canonical event.

## 15.3 Audit Obligations

Evidence identity, Tenant, institutional and workflow context, actor/delegation, affected entities, operation, logical changes, correlation, causation, classification, source event and policy references must be immutable and traceable.

---

# 16. Completion Criteria and Postconditions

Completion requires an eligible canonical event, resolved context and actor, logical change attribution, correlation, optional direct causation, classification, complete immutable evidence creation and publication of `Audit Evidence Generated`.

Postconditions:

- Audit Evidence is available for authorized investigation.
- Timeline projection is eligible for update.
- The originating domain retains business ownership.
- No source event or business aggregate was modified or duplicated.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No scenario-specific open issues.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
