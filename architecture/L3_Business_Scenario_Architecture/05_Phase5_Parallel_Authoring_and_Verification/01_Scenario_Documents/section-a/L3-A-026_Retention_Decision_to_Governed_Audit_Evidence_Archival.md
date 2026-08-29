# Retention Decision to Governed Audit Evidence Archival

# 1. Scenario Metadata

**Scenario ID:** L3-A-026  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Audit Infrastructure  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows; L3-A-025 — Audit Evidence Retention Review to Governed Retention Decision  
**Referenced By:** L3-A-027 — Archived Audit Evidence Request to Authorized Restored Access

---

# 2. Scenario Purpose

Apply a governed archival-eligibility decision to immutable Audit Evidence, changing only its storage and availability dimensions while preserving content, attribution, relationships and historical traceability.

---

# 3. Business Outcome

Immutable audit evidence is transitioned to governed archived storage with restricted availability while its content, relationships, attribution and historical traceability remain preserved.

L2-A07 defines the archival workflow, lifecycle transitions, constraints and `Evidence Archived` event. The scenario is classified `PASS`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when governed retention determines that eligible audit evidence should move from active to archival storage.

## 4.2 End Boundary

The scenario ends when Storage Status is `Archived Storage`, Availability Status is `Restricted Access`, evidence integrity and governance are unchanged, the archival outcome is recorded, and `Evidence Archived` is published.

## 4.3 In Scope

- Archival-decision and evidence-scope validation.
- Retention, legal preservation, Tenant and classification checks.
- Integrity preservation.
- Storage Status and Availability Status transition.
- Archival outcome, event and traceability.

## 4.4 Out of Scope

- Changing Audit Evidence content or Governance Status.
- Deleting or disposing of evidence.
- Selecting storage technology or migration mechanics.
- Restoring access; that belongs to A027.
- Re-evaluating the original business fact.

## 4.5 Related Scenarios

| Relationship | Scenario | Reason |
|---|---|---|
| Upstream | L3-A-025 — Audit Evidence Retention Review to Governed Retention Decision | Supplies archival eligibility. |
| Followed by | L3-A-027 — Archived Audit Evidence Request to Authorized Restored Access | Temporarily restores authorized availability. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Audit Infrastructure / Section A | Owns evidence, archival operation, Storage/Availability Status and archival event. |
| Authorization & Policy / Section A | Owns Retention, Archival and Access Policy decisions. |
| Platform & Tenant / Section A | Supplies Tenant/institution isolation. |
| Event Infrastructure / Section A | Distributes `Evidence Archived`. |

The frozen Participating Domains value remains exactly `A`.

---

# 6. Business Actors, Roles and Responsibilities

No universal human archiver or approver is defined. A026 applies an already governed Retention Decision through Audit Infrastructure; no placeholder actor is introduced.

---

# 7. Preconditions and Entry Criteria

- Immutable Audit Evidence exists in `Active Storage`.
- A025 produced an authoritative archival-eligibility Retention Governance Decision.
- Tenant, institution, classification and evidence scope are resolved.
- Archival Policy permits the transition and no preservation obligation blocks it.
- Evidence content and relationships satisfy required integrity expectations.

---

# 8. Trigger

Governed retention determines that eligible audit evidence should move from active to archival storage.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Receive the archival decision and immutable evidence references. | Audit Infrastructure | Governed archival begins. |
| 2 | Validate the decision, eligibility, Tenant, institution, classification and applicable policies. | Audit Infrastructure; Authorization & Policy | Archival authority and scope are confirmed. |
| 3 | Confirm that evidence content, actor/context, correlation, causation and historical relationships will remain unchanged. | Audit Infrastructure | Integrity-preservation conditions are established. |
| 4 | Complete the Archival Operation for the approved evidence scope. | Audit Infrastructure | Storage handling changes without evidence mutation. |
| 5 | Transition Storage Status from `Active Storage` to `Archived Storage`. | Audit Infrastructure | Evidence is governed as archived. |
| 6 | Transition Availability Status from `Available` to `Restricted Access`. | Audit Infrastructure | Direct retrieval ceases pending authorization/restoration. |
| 7 | Record the Archival Result and lifecycle evidence. | Audit Infrastructure | The transition is traceable. |
| 8 | Publish `Evidence Archived`. | Audit Infrastructure | Authorized consumers learn archival completion. |

---

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Resolution | Owner |
|---|---|---|---|
| Decision is absent, invalid or no longer eligible. | Archival lacks governance authority. | Do not transition either status; return to retention review. | Audit Infrastructure; Authorization & Policy |
| Tenant/institution scope is inconsistent. | Isolation could be violated. | Stop archival until canonical scope is resolved. | Platform & Tenant; Audit Infrastructure |
| Integrity preservation cannot be confirmed. | Historical trust could be lost. | Do not archive; obtain an A024 verification outcome where policy requires. | Audit Infrastructure |
| Active legal/institutional preservation blocks the transition. | Evidence must remain in its current governed availability. | Retain existing statuses. | Authorization & Policy |
| Only one lifecycle dimension changes. | Archival outcome is incomplete. | Do not publish `Evidence Archived`; complete both authorized transitions or preserve prior statuses. | Audit Infrastructure |

No technical storage transfer, rollback or infrastructure recovery is defined.

---

# 12. Business Rules and Decision Constraints

| Rule | Owner | Effect | Source |
|---|---|---|---|
| Archival requires a governed retention decision and eligibility. | Authorization & Policy; Audit Infrastructure | Direct uncontrolled archival is prohibited. | L2-A07 A07-W12; §31 |
| Archival changes Storage and Availability only. | Audit Infrastructure | Evidence and Governance Status remain unchanged. | L2-A07 §§21.7, 22.1 |
| Archival never modifies or deletes evidence. | Audit Infrastructure | Content and relationships remain immutable. | L2-A07 A07-W12; §22.2 |
| Archival produces `Archived Storage` and `Restricted Access`. | Audit Infrastructure | Archived evidence requires authorized restoration for access. | L2-A07 lifecycle matrix |
| Every transition creates additional audit evidence. | Audit Infrastructure | Archival itself is historically accountable. | L2-A07 §22.2 |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result | Owner |
|---|---|---|---|---|
| Audit Evidence | Created Once; immutable | No content/relationship change. | Unchanged. | Audit Infrastructure |
| Storage Status | `Active Storage` | Archival Operation. | `Archived Storage` | Audit Infrastructure |
| Availability Status | `Available` | Archival Operation. | `Restricted Access` | Audit Infrastructure |
| Governance Status | `Under Retention` or policy-valid value | No transition. | Unchanged. | Audit Infrastructure |
| Archival Result | Does not exist | Record completed operation. | Immutable result. | Audit Infrastructure |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Events

| Event | Publisher | Condition | Effect |
|---|---|---|---|
| `Evidence Archived` | Audit Infrastructure | Both archival lifecycle transitions and Archival Result complete successfully. | Authorized consumers learn evidence now has restricted archived availability. |

## 14.2 Contracts

| Provider | Consumer | Hand-off |
|---|---|---|
| A025 Retention Governance | Audit Infrastructure | Archival-eligibility decision and evidence scope. |
| Authorization & Policy | Audit Infrastructure | Archival/preservation policy decision. |
| Audit Infrastructure | Authorized governance consumers | Archival Result and archived/restricted statuses. |
| Audit Infrastructure | A027 Restoration | Immutable archived evidence reference and restricted availability. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

The operation applies only to the scope authorized by the Retention/Archival Policy and Tenant/institution boundary.

## 15.2 Approval and Separation of Duty

Approval occurs in A025 or applicable A04 policy. L2-A07 defines no additional A026 approval or SoD rule.

## 15.3 Audit Obligations

Decision reference, evidence scope, prior/new statuses, Tenant/institution, classification, policy, integrity confirmation, actor/reference where applicable, outcome, time and `Evidence Archived` publication must be traceable.

---

# 16. Completion Criteria and Postconditions

Completion requires a valid archival decision, confirmed eligibility and integrity preservation, `Active Storage → Archived Storage`, `Available → Restricted Access`, immutable Archival Result and publication of `Evidence Archived`.

Postconditions: evidence and relationships remain unchanged; Governance Status is unchanged; direct access is restricted; authorized temporary restoration may occur under A027.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No scenario-specific open issues.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
