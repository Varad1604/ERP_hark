# Student Profile Update

**Scenario ID:** L3-C-014  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Lifecycle  
**Participating Domains:** C  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-22  
**Depends On:** Locked L2-C02 — Student Lifecycle & Activation Workflows, especially `WF-C02-12 — Student Profile Update Workflow`  

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-C-014` |
| Canonical Title | Student Profile Update |
| Primary Owner | Student Lifecycle |
| Participating Domains | C |
| Scenario Type | Simple |
| Priority | P1 |
| Status | Draft |
| Applicable L2 Workflow | `WF-C02-12 — Student Profile Update Workflow` |

The exact Phase 5 filename is `L3-C-014_Student_Profile_Update.md`. Frozen identity fields are preserved without reclassification.

## 2. Scenario Purpose

Govern the controlled update of Student-owned profile information after Student creation while preserving immutable Student identity, canonical Student Lifecycle ownership, institutional policy, approval where required, and auditability.

## 3. Business Outcome

Permitted Student profile information is updated in the authoritative Student Aggregate, restricted changes are approved where policy requires, immutable identity remains unchanged, the completed change is auditable, and `StudentProfileUpdated` is published by Student Lifecycle.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
A permitted request is made to modify Student-owned profile information for an existing Student.

### 4.2 End Boundary
The scenario ends when the permitted profile change is validated, any required approval is complete, the authoritative profile is updated, audit evidence is preserved, and `StudentProfileUpdated` is published.

### 4.3 In Scope
- Editable personal/profile information.
- Editable contact and address information.
- Other Student-owned profile attributes permitted by institutional policy.
- Validation of editable versus immutable attributes.
- Restricted-change approval where required.
- Auditability.
- Publication of the canonical profile-update event.

### 4.4 Out of Scope
- Student creation.
- Admission changes.
- Academic assignment or enrollment changes.
- Student lifecycle-status changes.
- Identity correction, which is a distinct governed outcome.
- Communication delivery.
- APIs, persistence design, read models, UI, infrastructure, or technical authorization design.

### 4.5 Related Scenarios
| Relationship | Scenario |
|---|---|
| Related | L3-C-015 — Student Communication Preference Update |
| Distinct boundary | L3-C-016 — Student Identity Correction |

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| C — Student Lifecycle | Canonical owner of the Student Aggregate and Student-owned profile information; validates and applies the permitted profile change and publishes the canonical outcome. |

Downstream consumption of updated Student information does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority |
|---|---|---|
| Student | Submits permitted profile updates. | May initiate permitted changes within institutional policy. |
| Registrar | Approves restricted profile modifications where required. | Approval authority where locked C02 policy requires it. |
| Student Lifecycle | Validates, applies, records, and publishes the profile-update outcome. | Canonical business owner. |

These actor roles are taken directly from locked L2-C02 `WF-C02-12`; technical/supporting participants are intentionally excluded from L3 actor modeling.

## 7. Preconditions and Entry Criteria

- Student exists.
- Current Student lifecycle position permits profile modification.
- Requested attributes are editable under institutional policy.
- Immutable identity attributes are not being changed through this scenario.
- Any required approval condition can be evaluated.

## 8. Trigger

A permitted Student Profile Update request is made for an existing Student.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Result |
|---|---|---|---|
| 1 | Receive the profile-update request. | Student Lifecycle | Request enters governed evaluation. |
| 2 | Validate the Student and requested profile scope. | Student Lifecycle | Request is within Student profile ownership. |
| 3 | Validate editability and institutional policy. | Student Lifecycle | Permitted attributes identified. |
| 4 | Confirm immutable Student identity is not being changed. | Student Lifecycle | Identity boundary preserved. |
| 5 | Determine whether approval is required. | Student Lifecycle | Approval path established. |
| 6 | Where required, obtain restricted-change approval. | Registrar | Approval satisfied or request rejected. |
| 7 | Apply the permitted profile change. | Student Lifecycle | Authoritative Student profile updated. |
| 8 | Preserve the completed change as auditable business history. | Student Lifecycle | Traceability preserved. |
| 9 | Publish `StudentProfileUpdated`. | Student Lifecycle | Canonical business outcome announced. |

## 10. Alternate Business Flows

| Condition | Business Path | Result |
|---|---|---|
| No approval required | Complete validation and update directly. | Profile update completed. |
| Restricted modification requires approval | Obtain Registrar approval before applying the change. | Approved update completes or rejected update stops. |
| Several permitted profile attributes change together | Validate them within one governed profile-update outcome. | Authoritative profile reflects the permitted combined change. |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| Student does not exist | Reject the request; no Student state changes. |
| Requested attribute is immutable | Do not change it through C014; use the governed identity-correction boundary where applicable. |
| Requested attribute is not editable under policy | Reject or correct the request. |
| Lifecycle position does not permit profile modification | Reject or defer according to policy. |
| Required approval is rejected | Preserve the existing profile; do not publish the success event. |
| Business validation fails | Correct the business information before resubmission. |

No technical retry, rollback, queue, timeout, or infrastructure recovery is introduced.

## 12. Business Rules and Decision Constraints

- Student Lifecycle owns the Student Aggregate and Student profile information.
- Only editable attributes permitted by institutional policy may be changed.
- Immutable Student identity attributes are not changed through ordinary profile update.
- Restricted changes require approval only where policy requires it.
- Completed profile changes remain auditable.
- L3 applies these locked C02 constraints and does not create new local rules.

## 13. State and Lifecycle Impact

| Concept | Scenario Impact | Owner |
|---|---|---|
| Student Aggregate | Permitted profile attributes are updated. | Student Lifecycle |
| Student lifecycle state | No lifecycle transition is inherently caused by this scenario. | Student Lifecycle |
| Student identity | Remains unchanged. | Student Lifecycle |

No new lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration

| Business Event | Canonical Publisher | Purpose |
|---|---|---|
| `StudentProfileUpdated` | Student Lifecycle | Announces successful governed modification of the authoritative Student profile. |

Locked L2-C02 `WF-C02-12` and its service/event catalogue explicitly define this event. Interested dependent domains may consume it without acquiring ownership of Student profile information.

## 15. Business Authorization, Approval and Audit Obligations

- Student may submit permitted profile changes.
- Registrar approves restricted profile modifications where policy requires.
- No universal approval requirement is invented.
- Every completed profile change must remain auditable.
- L3 does not define technical IAM/RBAC mechanisms.

## 16. Completion Criteria and Postconditions

C014 is complete when:

- the Student and requested profile scope are valid;
- only permitted editable attributes are changed;
- immutable Student identity remains unchanged;
- required approval, if any, is complete;
- authoritative profile information is updated;
- audit evidence is preserved; and
- `StudentProfileUpdated` is published by Student Lifecycle.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No unresolved source-access, filename, actor, workflow, ownership, or event issue remains after direct reconciliation with the live Phase 5 directory and locked L2-C02 `WF-C02-12`.

Any older candidate/Phase 2 participating-domain notation is non-authoritative for Phase 5; the frozen Phase 4/5 value `C` is preserved without reinterpretation.

### 17.2 Assumptions

Institution-specific editable attributes and approval conditions remain governed by upstream policy/configuration and are not hard-coded here.

### 17.3 Deferred Questions

Technical synchronization and implementation mechanics remain outside L3.

## Production Verification Record

- Exact Phase 5 filename verified in the live `review` branch.
- Frozen `L3-C-014`, title, owner, domains, type, and P1 priority preserved.
- Direct locked L2-C02 source verification completed.
- Student and Registrar actor roles aligned to `WF-C02-12`.
- `StudentProfileUpdated` restored as the exact canonical event.
- Transient session citation markers removed; stable repository/workflow references used instead.
- No implementation mechanics introduced.
- All 17 mandatory sections present.
- Status remains **Draft** pending architecture review.
