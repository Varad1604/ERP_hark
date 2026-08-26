# Internal Audit Plan to Closed Audit Report

## 1. Scenario Metadata

**Scenario ID:** L3-H-065  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H, VAR:Audited Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-H09_Accreditation_and_Compliance_Workflows.md  
**Referenced By:** None yet

## 2. Scenario Purpose

Execute a governed internal audit from an authorized risk-based plan through evidence review, observations/findings, management acknowledgement and formal audit closure.

## 3. Business Outcome

Scope, evidence, testing, observations, findings, response, and approval produce a formally closed audit report

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A risk-based internal audit is authorized.

### 4.2 End Boundary

Scope, evidence, testing, observations, findings, response, and approval produce a formally closed audit report.

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
| VAR:Audited Domain | Retains ownership of audited operational facts and supplies governed audit evidence. |

Participation and evidence contribution do not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Internal Auditor | Conducts the audit and records audit evidence, observations and findings. | Performs |
| IQAC | Participates in audit governance and review. | Reviews |
| Compliance Cell | Participates in compliance-focused audit governance. | Reviews |
| Department Coordinator | Coordinates audited-unit participation and evidence. | Participates |
| Registrar | Participates in institutional audit governance. | Participates |
| Management | Provides the acknowledgement required before audit closure. | Acknowledges |

## 7. Preconditions and Entry Criteria

- The Audit Plan is approved.
- Audit scope and audit team are defined.
- Required evidence is available.

## 8. Trigger

A risk-based internal audit is authorized

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create and schedule the internal Audit from the authorized plan. | Accreditation & Compliance | The Audit progresses from Planned to Scheduled. |
| 2 | Review available evidence and conduct the defined audit scope without the auditor auditing a unit for which they have operational ownership. | Internal Auditor / VAR:Audited Domain | Audit testing is performed under the governed scope. |
| 3 | Record observations, findings and non-conformities and prepare the Audit Report. | Internal Auditor | The Audit enters Reporting with governed issue records. |
| 4 | Conduct the closing review and obtain the required management acknowledgement. | Accreditation & Compliance / Management | Management acknowledgement exists for closure. |
| 5 | Create required CAPA items and verify no unresolved Critical finding prevents closure. | Accreditation & Compliance | Closure prerequisites are satisfied. |
| 6 | Close the Audit. | Accreditation & Compliance | The formally closed audit report and governed audit history are established. |

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Unresolved Critical finding exists | Audit closure is prohibited. | Maintain the audit in its governed pre-closure position while required finding/CAPA action proceeds. | Accreditation & Compliance |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| BR-H09-015 — audit schedules require approval before publication. | Accreditation & Compliance | Controls audit scheduling. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |
| BR-H09-016 — auditors must not audit units for which they have operational ownership. | Accreditation & Compliance | Enforces audit independence. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |
| BR-H09-017 — audit reports require management acknowledgement before closure. | Accreditation & Compliance | Makes acknowledgement a closure prerequisite. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |
| BR-H09-018 — audit closure is prohibited while unresolved Critical findings exist. | Accreditation & Compliance | Blocks premature closure. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Audit | Planned | Scheduling, execution, reporting and management review. | Closed | Accreditation & Compliance | L2-H09_Accreditation_and_Compliance_Workflows.md §6 H09-A05 / §7 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Audit Planned | Audit aggregate | Observation; Finding; CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Audit Scheduled | Audit aggregate | Observation; Finding; CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Audit Started | Audit aggregate | Observation; Finding; CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Audit Completed | Audit aggregate | Observation; Finding; CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Audit Report Generated | Audit aggregate | Observation; Finding; CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Audit Closed | Audit aggregate | Observation; Finding; CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| VAR:Audited Domain | Accreditation & Compliance | Source Evidence Reference Contract | Provide authoritative audited-domain evidence/reference without transferring source ownership. | L2-H09_Accreditation_and_Compliance_Workflows.md §23.1 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Audit schedule approval | Accreditation & Compliance governance | Approval is required before schedule publication. | Accreditation & Compliance |
| Audit report acknowledgement | Management | Acknowledgement is mandatory before closure. | Accreditation & Compliance |

### 15.2 Separation of Duties, Overrides and Exceptions

The auditor must not audit an organisational unit for which the auditor has operational ownership (BR-H09-016).

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Internal audit engagement | Audit scope, evidence, observations, findings and Audit Report | Accreditation & Compliance |
| Closure authority evidence | Management acknowledgement and Critical-finding closure condition | Accreditation & Compliance |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The audit scope has been executed and documented.
- The Audit Report has management acknowledgement.
- No unresolved Critical finding prevents closure.
- The Audit lifecycle is Closed.

### 16.2 Postconditions

- The closed Audit Report and its observations/findings remain part of H09 governance history.
- Any CAPA created from findings continues under the H09 CAPA lifecycle.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No open issues.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
