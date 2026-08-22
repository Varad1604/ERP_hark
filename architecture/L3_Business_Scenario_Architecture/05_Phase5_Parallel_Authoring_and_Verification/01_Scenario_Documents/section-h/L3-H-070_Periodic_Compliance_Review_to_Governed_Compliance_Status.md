# Periodic Compliance Review to Governed Compliance Status

# 1. Scenario Metadata

**Scenario ID:** L3-H-070  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H, VAR:Obligation-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H09 — Accreditation & Compliance Workflows  
**Referenced By:** CAPA, audit, institutional governance, readiness, and compliance-monitoring scenarios

# 2. Scenario Purpose

Govern a scheduled or change-driven H09 Compliance Review so that current obligation fulfilment, exceptions, material risks, updated evidence, deviations, and required actions are evaluated and an authoritative compliance status is recorded without H09 taking ownership of the underlying operational obligation.

# 3. Business Outcome

Requirement fulfillment, exceptions, risks, and evidence are assessed and an authoritative compliance status with actions is recorded

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A scheduled review or material obligation change requires enterprise compliance evaluation.

## 4.2 End Boundary

The scenario ends when the governed review has evaluated current evidence/status, recorded deviations and approved exceptions where applicable, assigned required actions, refreshed the authoritative H09 compliance status, and closed the review cycle.

## 4.3 In Scope

- Governed review schedule and scope.
- Updated evidence and current requirement fulfilment.
- Deviation identification.
- Compliance exceptions through the source-defined compliance governance chain.
- Material risk consideration and institutional escalation where required.
- Action assignment.
- Compliance status refresh and review closure.

## 4.4 Out of Scope

- Performing the operational obligation for its owning domain.
- Changing source-domain facts to achieve compliance.
- Inventing local compliance statuses beyond H09's owned Compliance Requirement lifecycle/status semantics.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accreditation & Compliance / H | Owns the Compliance Review, Compliance Register, review governance, deviation/action follow-up, and authoritative compliance status within H09. |
| VAR:Obligation-Owning Domain | Owns the underlying business obligation/activity and supplies authoritative fulfilment evidence/status under its own governance. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Compliance Cell | Owns periodic compliance review, compliance status review, deviation tracking, and action follow-up. | Coordinates / reviews |
| IQAC | Participates in institutional compliance governance. | Reviews |
| Department Quality Committee | Reviews local compliance actions and evidence completeness. | Reviews local inputs |
| Institutional Leadership | Reviews material risk and final governance exceptions after escalation. | Escalated review / approval |
| Department Compliance Officers | Participate where the reviewed requirement belongs to a department. | Provides fulfilment evidence/status |

# 7. Preconditions and Entry Criteria

- The review follows a governed schedule approved by the compliance governance chain, or a material obligation change requires review.
- Applicable Compliance Requirements and their accountable Business Owners are identifiable.
- Updated evidence can be obtained from the authoritative source owner.

# 8. Trigger

A scheduled review or material obligation change requires enterprise compliance evaluation

# 9. Main Business Flow

1. H09 establishes the Compliance Review scope and applicable requirements.
2. The Compliance Cell obtains current requirement fulfilment status and updated evidence from the authoritative obligation owners.
3. H09 evaluates evidence completeness and current compliance status.
4. H09 identifies deviations, unresolved compliance gaps, and relevant material risks.
5. Where an exception is required, it is reviewed through the source-defined compliance governance chain before institutional escalation.
6. Material risks or final governance exceptions are escalated to Institutional Leadership where required.
7. H09 records the reviewed outcome and identifies required actions.
8. Required actions are assigned to accountable owners without transferring the underlying operational obligation into H09.
9. H09 updates the authoritative Compliance Requirement status only when supported by the reviewed evidence and applicable decision.
10. The Compliance Review closes with status, deviations, exceptions, actions, and review evidence preserved.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Evidence does not support compliance | Updated evidence is incomplete or indicates non-compliance | Do not advance the requirement to `Compliant`; record deviation/action | Governed non-compliant review outcome | Accreditation & Compliance / obligation owner |
| Exception requested | Requirement cannot be met in the ordinary manner | Route the exception through the compliance governance chain and escalate where required | Approved/rejected exception evidence retained | Accreditation & Compliance |
| Action required | Review finds a gap requiring correction | Assign action or route to CAPA/remediation as supported | Follow-up obligation established | Accreditation & Compliance / obligation owner |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Review schedule is not governed | BR-H09-034 is not satisfied | Obtain schedule approval before treating the review as a periodic governed review | Accreditation & Compliance |
| Updated evidence is absent | Requirement cannot validly advance to `Compliant` | Obtain current evidence and review confirmation | Obligation-owning domain / Accreditation & Compliance |
| Accountable action owner cannot be established | Review closure would lack follow-up accountability | Resolve ownership before final closure | Accreditation & Compliance / obligation owner |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Periodic Compliance Reviews follow a governed schedule approved by the compliance governance chain | H09 | Review cadence requires governance | L2-H09 BR-H09-034 |
| Compliance status cannot advance to compliant without updated evidence and review confirmation | H09 | Prevents unsupported compliance status | L2-H09 BR-H09-035 |
| Every Compliance Requirement has exactly one accountable Business Owner | H09 | Action/status ownership remains identifiable | L2-H09 BR-H09-005 |
| Compliance records maintain complete audit history | H09 | Review and status history remain traceable | L2-H09 BR-H09-008 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Compliance Review | Scheduled | Begin governed evaluation | In Progress | H09 | L2-H09 H09-A11 |
| Compliance Review | In Progress | Complete evidence/status review | Reviewed | H09 | L2-H09 H09-A11 |
| Compliance Review | Reviewed | Assign required follow-up | Action Assigned | H09 | L2-H09 H09-A11 |
| Compliance Review | Action Assigned | Preserve final review result | Closed | H09 | L2-H09 H09-A11 |
| Compliance Requirement | Active / Under Review / other supported current position | Refresh source-supported compliance status | Supported H09 status only | H09 | L2-H09 H09-A03; BR-H09-035 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Compliance Review Scheduled | Compliance Review aggregate / H09 | Compliance governance participants | Records governed review scheduling | L2-H09 §11 |
| Compliance Review Started | Compliance Review aggregate / H09 | Compliance governance participants | Records review start | L2-H09 §11 |
| Compliance Review Completed | Compliance Review aggregate / H09 | Compliance governance participants | Records completed review evaluation | L2-H09 §11 |
| Compliance Review Assigned | Compliance Review aggregate / H09 | Action owners / governance participants | Records action assignment | L2-H09 §11 |
| Compliance Review Closed | Compliance Review aggregate / H09 | Compliance governance participants | Records review closure | L2-H09 §11 |
| Compliance Status Updated | Compliance Requirement aggregate / H09 | Compliance governance consumers | Records authoritative requirement-status refresh | L2-H09 §11 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

The obligation-owning domain supplies current fulfilment facts and evidence under its own business ownership. H09 consumes those inputs for compliance governance and returns/assigns governed follow-up actions without taking ownership of the underlying obligation lifecycle.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The Compliance Cell owns periodic compliance review and deviation/action follow-up. Compliance exception approvals follow the compliance governance chain before institutional escalation; Institutional Leadership reviews final governance exceptions and material risk after escalation.

## 15.2 Separation of Duties, Overrides and Exceptions

An obligation-owning domain cannot make H09's compliance review authoritative merely by asserting fulfilment; H09 requires reviewed evidence. H09 cannot alter the source fact to produce a desired status.

## 15.3 Business Audit and Evidence Obligations

Preserve review schedule/scope, requirement set, Business Owners, updated evidence, fulfilment findings, deviations, risks, exception decisions, actions, status changes, and review closure history.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Current requirements and updated evidence are evaluated.
- Deviations, risks, and governed exceptions are recorded where applicable.
- Required actions are assigned.
- Compliance status is refreshed only when evidence and review support it.
- The Compliance Review reaches `Closed`.

## 16.2 Postconditions

The authoritative H09 compliance status and follow-up actions are available for governance monitoring while source-domain obligation ownership remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

H09 defines the H09-11 capability, Compliance Review aggregate/lifecycle, rules, governance responsibilities, and events but does not provide a dedicated stepwise Periodic Compliance Review workflow. Also, the event name `Compliance Review Completed` appears alongside a lifecycle that still permits `Action Assigned` before `Closed`; this Draft treats the event as completion of the review evaluation, not terminal lifecycle closure.

These are non-blocking because the frozen status/action outcome is otherwise explicitly supported.

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream H09 may add a dedicated workflow sequence and clarify the semantic boundary between `Compliance Review Completed` and `Compliance Review Closed`.
