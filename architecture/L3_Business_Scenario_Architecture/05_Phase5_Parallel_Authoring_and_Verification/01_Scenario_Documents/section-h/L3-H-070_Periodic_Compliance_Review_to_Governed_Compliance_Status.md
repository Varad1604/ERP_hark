# Periodic Compliance Review to Governed Compliance Status

## 1. Scenario Metadata

**Scenario ID:** L3-H-070  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H, VAR:Obligation-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-H09_Accreditation_and_Compliance_Workflows.md  
**Referenced By:** None yet

## 2. Scenario Purpose

Perform a governed periodic or material-change compliance review and record the authoritative H09 compliance status and required actions while preserving source obligation ownership.

## 3. Business Outcome

Requirement fulfillment, exceptions, risks, and evidence are assessed and an authoritative compliance status with actions is recorded

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A scheduled review or material obligation change requires enterprise compliance evaluation.

### 4.2 End Boundary

Requirement fulfillment, exceptions, risks, and evidence are assessed and an authoritative compliance status with actions is recorded.

### 4.3 In Scope

- H09-owned governance behaviour materially required to reach the frozen business outcome.
- Applicable source-domain references, evidence and governed hand-offs without ownership transfer.
- Only upstream-defined rules, lifecycle positions, events, approvals and evidence obligations relevant to this scenario.

### 4.4 Out of Scope

- Operational business-state ownership belonging to participating source domains.
- Adjacent downstream institutional outcomes after this scenario's End Boundary.
- Later architecture-level implementation concerns.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accreditation & Compliance (H) | Owns the H09 governance outcome and applicable H09 lifecycles. |
| VAR:Obligation-Owning Domain | Retains ownership of authoritative obligation/source facts where applicable while H09 governs the compliance requirement/review. |

Participation and evidence contribution do not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Compliance Cell | Owns the compliance register, periodic reviews, deviation tracking and action follow-up. | Performs / reviews |
| IQAC | Participates in compliance governance and escalated review. | Reviews |
| Department Compliance Officers | Coordinate requirement/evidence review for applicable units. | Performs |
| Business Owner | Provides accountable requirement ownership and action response. | Accountable |

## 7. Preconditions and Entry Criteria

- The reviewed Compliance Requirements are governed in the H09 compliance register.
- Updated evidence and current obligation context are available or requested.

## 8. Trigger

A scheduled review or material obligation change requires enterprise compliance evaluation

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Schedule/start the Compliance Review for the applicable requirement scope. | Compliance Cell | The Compliance Review moves from Scheduled to In Progress. |
| 2 | Collect updated evidence and obligation context from `VAR:Obligation-Owning Domain`. | Compliance Cell / VAR:Obligation-Owning Domain | Current evidence/reference context is available. |
| 3 | Assess fulfillment, deviations/exceptions, risks and required actions. | Compliance Cell / IQAC | The review reaches Reviewed with identified deviations/actions. |
| 4 | Update the authoritative Compliance Requirement status only where updated evidence and review confirmation support it. | Accreditation & Compliance | `Compliance Status Updated` records the governed status decision. |
| 5 | Assign required actions and close the governed review when its closure conditions are met. | Compliance Cell | The review records status and action obligations and reaches closure. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Compliance deviation requires remediation | Review identifies a gap requiring action | Assign the governed action and, where required, route remediation to CAPA. | Compliance status and action obligations remain explicit. | Accreditation & Compliance |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Updated evidence is insufficient | Compliance status cannot advance to compliant. | Keep the requirement/review in the appropriate governed review position and obtain updated evidence/action. | Accreditation & Compliance |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| BR-H09-034 — Periodic Compliance Reviews follow a governed schedule approved by the compliance governance chain. | Accreditation & Compliance | Controls review cadence. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |
| BR-H09-035 — compliance status cannot advance to compliant without updated evidence and review confirmation. | Accreditation & Compliance | Prevents unsupported compliant status. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Compliance Review | Scheduled | Review, action assignment and closure. | Reviewed → Action Assigned → Closed | Accreditation & Compliance | L2-H09_Accreditation_and_Compliance_Workflows.md §6 H09-A11 / §7 |
| Compliance Requirement | Active / Under Review | Compliance review may update the requirement status when evidence supports the decision. | Compliant where BR-H09-035 is satisfied; otherwise the authoritative status is not advanced | Accreditation & Compliance | L2-H09_Accreditation_and_Compliance_Workflows.md §6 H09-A03 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Compliance Review Scheduled | Compliance Review aggregate | CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Compliance Review Started | Compliance Review aggregate | CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Compliance Review Completed | Compliance Review aggregate | CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Compliance Review Assigned | Compliance Review aggregate | CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Compliance Review Closed | Compliance Review aggregate | CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Compliance Status Updated | Compliance Requirement aggregate | Compliance Review; CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| VAR:Obligation-Owning Domain | Accreditation & Compliance | Source Evidence Reference Contract | Provide current obligation/evidence context for the compliance review while retaining source ownership. | L2-H09_Accreditation_and_Compliance_Workflows.md §23.1 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Compliance review governance | Compliance Cell | Owns periodic review, deviation tracking and action follow-up. | Accreditation & Compliance |
| Compliance exception approval | Compliance governance chain | H09 places compliance exception approval within the compliance chain before institutional escalation. | Accreditation & Compliance |

### 15.2 Separation of Duties, Overrides and Exceptions

No additional scenario-specific separation-of-duty rule is defined for periodic Compliance Review.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Compliance review | Requirement scope, reviewed evidence, deviations/exceptions, status decision and assigned actions | Accreditation & Compliance |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Requirement fulfillment, exceptions/risks and evidence have been reviewed.
- The authoritative H09 compliance status is recorded.
- Required actions are recorded and governed through the review/CAPA path as applicable.

### 16.2 Postconditions

- The Compliance Review history remains H09-owned.
- `VAR:Obligation-Owning Domain` retains ownership of source obligation facts.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| H09 defines `Compliance Review Completed` while the Compliance Review lifecycle continues through `Action Assigned → Closed`, and it does not define a complete compliance-status value vocabulary beyond lifecycle positions such as `Compliant`. | The scenario can record the authoritative status/event without inventing a value, but exact completion-event sequencing/status vocabulary remains ambiguous. | Reconcile event timing and authoritative status vocabulary upstream before locking. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
