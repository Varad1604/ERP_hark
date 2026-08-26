# Compliance Obligation to Effective Requirement Lifecycle

## 1. Scenario Metadata

**Scenario ID:** L3-H-063  
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

Govern an institutional compliance obligation through classification, accountable ownership, monitoring, review and supported terminal/history-preserving positions while retaining the authority of the obligation-owning domain.

## 3. Business Outcome

The requirement is classified, assigned, monitored, revised, fulfilled, waived, expired, or retired with full history

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A legal, regulatory, accreditation, or institutional obligation is identified or changed.

### 4.2 End Boundary

The requirement is classified, assigned, monitored, revised, fulfilled, waived, expired, or retired with full history.

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
| Compliance Cell | Maintains the compliance register, review cadence, deviations and action follow-up. | Performs / reviews |
| IQAC | Participates in compliance governance and escalation. | Reviews |
| Registrar | Participates in compliance governance. | Participates |
| Department Compliance Officers | Coordinate obligation handling for applicable units. | Performs |
| Business Owner | Carries the accountable ownership required for each Compliance Requirement. | Accountable |

## 7. Preconditions and Entry Criteria

- The obligation is identified with an applicable institutional scope.

## 8. Trigger

A legal, regulatory, accreditation, or institutional obligation is identified or changed

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Register and classify the Compliance Requirement. | Compliance Cell | The requirement is Created. |
| 2 | Assign exactly one accountable Business Owner and define applicability, review frequency, evidence needs and due/renewal context. | Compliance Cell | The requirement becomes Assigned. |
| 3 | Activate and monitor the requirement. | Accreditation & Compliance | The requirement is Active and monitored. |
| 4 | Review compliance status using current evidence and obligation context. | Compliance Cell / VAR:Obligation-Owning Domain | The requirement enters Under Review and its status may be updated. |
| 5 | Where the supported lifecycle applies, record Compliant, Expired and Archived positions with history. | Accreditation & Compliance | The supported H09 lifecycle positions are preserved. |

> The authoritative flow cannot be extended to the frozen End Boundary without resolving the material issue in Section 17.1.

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Requirement renewal is required | The obligation continues beyond its current renewal point | Renew the requirement using the canonical `Compliance Requirement Renewed` event and continue governed monitoring. | The requirement remains governed with history. | Accreditation & Compliance |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| The frozen outcome requires waiver or retirement | No H09 lifecycle state, transition, or canonical event defines waiver or retirement for Compliance Requirement. | Do not substitute Archived/Expired for waived/retired; upstream resolution is required. | Accreditation & Compliance |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| BR-H09-005 — every Compliance Requirement has exactly one accountable Business Owner. | Accreditation & Compliance | Requires singular accountable ownership. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |
| BR-H09-006 — Compliance Requirements define mandatory review frequencies. | Accreditation & Compliance | Requires governed review cadence. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |
| BR-H09-008 — compliance records maintain complete audit history. | Accreditation & Compliance | Preserves requirement history. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Compliance Requirement | Created | Assignment, activation, review and supported lifecycle progression. | Assigned → Active → Under Review → Compliant → Expired → Archived | Accreditation & Compliance | L2-H09_Accreditation_and_Compliance_Workflows.md §6 H09-A03 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Compliance Requirement Created | Compliance Requirement aggregate | Compliance Review; CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Compliance Requirement Assigned | Compliance Requirement aggregate | Compliance Review; CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Compliance Review Due | Compliance Requirement aggregate | Compliance Review; CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Compliance Status Updated | Compliance Requirement aggregate | Compliance Review; CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Compliance Requirement Renewed | Compliance Requirement aggregate | Compliance Review; CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| VAR:Obligation-Owning Domain | Accreditation & Compliance | Governed obligation context / source evidence reference | Supply authoritative obligation context and supporting evidence without transferring operational ownership. | L2-H09_Accreditation_and_Compliance_Workflows.md §23.1 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Requirement accountability | Business Owner | Exactly one accountable Business Owner is mandatory. | Accreditation & Compliance |

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty requirement is defined for the Compliance Requirement lifecycle.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Compliance requirement history | Requirement ownership, review, status and renewal history | Accreditation & Compliance |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The frozen outcome requires support for classified, assigned, monitored, revised, fulfilled, waived, expired or retired requirement outcomes with full history.
- The current locked lifecycle does not establish waiver or retirement, so completion cannot be fully proven.

### 16.2 Postconditions

- The supported Compliance Requirement lifecycle and history remain H09-owned.
- `VAR:Obligation-Owning Domain` retains ownership of its underlying obligation facts where applicable.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Frozen Stage 3 semantics require `waived` and `retired` outcomes, but H09 defines Compliance Requirement only as Created → Assigned → Active → Under Review → Compliant → Expired → Archived and has no waiver/retirement event. | Two frozen final-outcome branches lack authoritative lifecycle semantics. | Upstream H09 must define or formally reconcile waiver and retirement before this scenario can reach its frozen outcome. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
