# Infrastructure Compliance Requirement to Compliance Record Establishment

# 1. Scenario Metadata

**Scenario ID:** L3-G-300  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Establish a G10-owned Compliance Record against authoritative physical Infrastructure under configured compliance policy, authorization and historical-governance rules.

# 3. Business Outcome

A Compliance Record is established against the applicable infrastructure.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a physical infrastructure asset requires a compliance record.

## 4.2 End Boundary

The scenario ends when authorization, Infrastructure validation and policy validation succeed, the Compliance Record is established against the canonical Infrastructure, and `ComplianceRegistered` is published.

## 4.3 In Scope

- Identification of the applicable physical Infrastructure.
- Application of configured compliance category/policy.
- Compliance registration validation.
- Establishment of the Compliance Record.
- Canonical event publication and audit evidence.

## 4.4 Out of Scope

- Compliance status change, expiry, renewal or failure consequence.
- Regulatory interpretation beyond configured G10 compliance governance.
- Infrastructure inspection execution.
- Any transfer of compliance ownership to Section A.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G10 | Owns the physical Infrastructure, Compliance Record, compliance lifecycle and registration outcome. |
| Platform Foundation / A | Owns authorization, event-governance and audit capabilities without owning compliance semantics. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers Compliance Record registration against Infrastructure. | Acts only within authorized scope. |
| Compliance Officer | Participates in source-supported Infrastructure compliance governance. | No universal approval authority is inferred. |

# 7. Preconditions and Entry Criteria

- The physical Infrastructure exists under G10 ownership.
- A compliance requirement or configured compliance category applies.
- Required compliance-policy information is available.
- The registration business operation is authorized.

# 8. Trigger

A physical infrastructure asset requires a compliance record.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the authoritative physical Infrastructure and applicable compliance requirement. | Infrastructure Management | Compliance scope is established. |
| 2 | Confirm authorization for Compliance Record registration. | Platform Foundation / Infrastructure Management | Registration may proceed. |
| 3 | Validate the Infrastructure reference. | Infrastructure Management | The record will be attached only to canonical Infrastructure. |
| 4 | Validate the proposed registration against configured compliance policy. | Infrastructure Management | The compliance requirement is eligible for registration. |
| 5 | Establish the Compliance Record under the source-defined Compliance lifecycle. | Infrastructure Management | The frozen business outcome is achieved. |
| 6 | Publish `ComplianceRegistered`. | Infrastructure Management | The completed registration fact is canonical. |
| 7 | Generate immutable audit evidence from the canonical event. | Platform Foundation — Audit | Registration evidence is preserved. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| Institution-specific category | A configured category applies beyond the examples in G10. | Use the configured category under the same G10 compliance ownership. | Compliance remains canonical in G10. |
| Policy validation fails | Required information or policy condition is not satisfied. | Do not establish the record; correct the source condition first. | No unsupported Compliance Record is created. |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Authorization fails. | Compliance Record is not established. | A later authorized registration may proceed. | Platform Foundation / Infrastructure Management |
| Infrastructure validation fails. | The record cannot be linked to authoritative Infrastructure. | Resolve the G10 Infrastructure issue before a later registration. | Infrastructure Management |
| Compliance policy validation fails. | Registration does not complete. | Resolve the applicable business/policy deficiency. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| G10 owns Compliance Records and Compliance Status for physical Infrastructure. | Infrastructure Management | Compliance ownership remains in G10. | L2-G10 §§1, 3, 8 |
| Compliance supports versioning, effective dating, expiry management, auditability, workflow-driven approvals and configurable policies. | Infrastructure Management | The record enters governed compliance history. | L2-G10 §31 Compliance Principles |
| Compliance Registration follows Authorization Validation → Infrastructure Validation → Policy Validation → Compliance Created → Publish Domain Event. | Infrastructure Management | Registration cannot skip required checks. | L2-G10 §31 |
| The Compliance lifecycle is `Draft → Active → Expiring → Expired → Renewed`. | Infrastructure Management | No additional compliance state is invented. | L2-G10 canonical lifecycle summary |
| `ComplianceRegistered` is the canonical event for compliance creation. | Infrastructure Management | Exact event name/publisher is preserved. | L2-G10 Compliance Events |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Compliance Record | Not yet established | A new Compliance Record is created and enters the source-defined lifecycle. | `Draft` as the first defined Compliance lifecycle position. | Infrastructure Management | L2-G10 canonical lifecycle; §31 |
| Infrastructure | Existing source-defined state | Referenced by the Compliance Record. | Unchanged solely by record establishment. | Infrastructure Management | L2-G10 §§8, 31 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Scenario Purpose | Authoritative Source |
|---|---|---|---|
| `ComplianceRegistered` | Infrastructure Management | Publishes the completed creation of the Compliance Record. | L2-G10 Compliance Events |

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section A provides authorization, event governance and audit evidence. It does not own the Compliance Record or compliance lifecycle.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Register Compliance Record | Infrastructure Administrator or source-supported Compliance Officer within authorized scope | Must pass authorization, Infrastructure validation and policy validation. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

G10 supports workflow-driven approvals and configurable compliance policies but does not define one universal approver or separation-of-duty pattern for every compliance category.

## 15.3 Business Audit and Evidence Obligations

- The G10 Compliance Record and its history remain authoritative.
- Immutable audit evidence is generated from the canonical `ComplianceRegistered` event.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Canonical Infrastructure is identified.
- Required authorization and validations succeed.
- The Compliance Record is established.
- `ComplianceRegistered` is published.
- Audit evidence is preserved.

## 16.2 Postconditions

- Infrastructure Management owns the Compliance Record and subsequent lifecycle.
- Later status changes remain separate scenarios.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No blocking source conflict was identified.

**Source Assessment:** **PASS**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No deferred questions.
