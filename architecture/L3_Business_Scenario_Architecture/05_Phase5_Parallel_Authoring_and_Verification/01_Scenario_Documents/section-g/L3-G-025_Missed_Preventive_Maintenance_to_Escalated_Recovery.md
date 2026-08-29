# Missed Preventive Maintenance to Escalated Recovery

## 1. Scenario Metadata

**Scenario ID:** L3-G-025  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Facilities Management  
**Participating Domains:** G, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.3  
**Owner:** Pallav  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-G01_Facilities_Management_Workflows.md; L3_Business_Scenario_Template_v2.0.md  
**Referenced By:** None yet  
**Architecture Approval:** ACR-G-2026-08-21-002  
**Approved By:** Arjun Mehta, Chief Enterprise Architect  
**Approval Date:** 21 August 2026  
**Approval Decision:** Approved — No Conditions  

## 2. Scenario Purpose

Recover a missed preventive-maintenance occurrence through visible, auditable SLA escalation and a governed path back to maintenance execution.

## 3. Business Outcome

The missed occurrence remains preserved, an authorised escalated recovery plan is established, and the maintenance obligation is returned to controlled execution.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A governed Preventive Maintenance schedule reaches or passes its required execution boundary without completion.

### 4.2 End Boundary

The missed occurrence, reason, escalation and recovery action are recorded and a valid execution path is established.

### 4.3 In Scope

- missed-occurrence determination
- preservation of schedule history
- SLA/operational escalation
- reassignment or controlled recovery scheduling
- supported notifications

### 4.4 Out of Scope

- silent schedule deletion
- inventing a Missed lifecycle state
- recovery implementation design

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Facilities Management (G01) | Owns the maintenance obligation, missed-schedule evidence, escalation and recovery. |
| People & Workforce (E) | Provides authoritative assignment and availability context. |
| Institutional Services (H) | Owns supported notification delivery. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Maintenance Supervisor | Confirms the missed occurrence and initiates escalation. | Initiates/reviews. |
| Facilities Manager | Governs recovery action and resolution. | Decides. |
| Technician / Technician Team | Performs recovered maintenance once assigned. | Performs subsequent work. |

## 7. Preconditions and Entry Criteria

- A valid Preventive Maintenance plan and schedule occurrence exist.
- The required execution boundary and applicable SLA/escalation policy are known.

## 8. Trigger

A scheduled preventive-maintenance occurrence is not completed within its governed execution boundary.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Confirm the missed occurrence against the authoritative schedule. | Maintenance Supervisor | Missed maintenance is established without deleting the schedule. |
| 2 | Record the reason, operational impact and affected infrastructure. | Facilities Management | Recovery evidence is available. |
| 3 | Evaluate applicable SLA and escalation policy. | Maintenance Supervisor | Required escalation level is determined. |
| 4 | Publish exact SLA breach/escalation events only when their conditions are satisfied. | Facilities Management | Escalated recovery is initiated. |
| 5 | Select reassignment, controlled rescheduling or other supported recovery action. | Facilities Manager / E | Valid recovery path is established. |
| 6 | Preserve the missed occurrence and recovery plan for subsequent execution. | Facilities Management / H | Maintenance obligation remains visible and communicated. |

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| No authorised recovery path | Maintenance remains unaddressed. | Escalate to the next configured business authority and keep the occurrence visible. | Facilities Management |
| Replacement assignment unavailable | Recovered work cannot begin. | Apply governed reassignment/escalation using authoritative workforce context. | Facilities Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Missed schedules generate notifications, escalate according to SLA, remain visible, and preserve history. | Facilities Management | Defines required recovery governance. | L2-G01 §20 |
| Missed maintenance is never silently discarded. | Facilities Management | Preserves the original obligation. | L2-G01 §20 |
| Escalation requires a supported trigger and audit history. | Facilities Management | Constrains escalation. | L2-G01 §28 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Preventive Maintenance | Scheduled | Missed occurrence is recorded as business evidence; no new Missed state is introduced, and recovery returns the obligation to governed execution. | Scheduled / Generated when recovery Work Order is established | Facilities Management | L2-G01 §§20, 33 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| SLABreached | Facilities Management | E, H and configured operational authorities | Records an actual SLA breach when threshold conditions are satisfied. | L2-G01 §31 |
| EscalationTriggered | Facilities Management | Configured management authorities; H | Records initiation of governed escalation. | L2-G01 §31 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| E | Facilities Management | Authoritative technician/team availability context | Supports recovery assignment. | L2-G01 §19 |
| Facilities Management | H | Exact SLA/escalation event requiring supported awareness | Enables notification delivery. | L2-G01 §§20, 28, 31 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Confirm and escalate missed maintenance | Maintenance Supervisor | Must apply authoritative schedule and escalation policy. | Facilities Management |
| Approve recovery action | Facilities Manager | Must record business reason and selected recovery. | Facilities Management |

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty obligation is defined by the authoritative L2 architecture. Any override stated above requires the source-supported authority, business reason, and audit history.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Missed occurrence | Plan, schedule, required boundary, reason, infrastructure and impact. | Facilities Management |
| Escalation and recovery | Trigger, level, responsible actor, action, resolution plan and history. | Facilities Management |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The missed occurrence and reason are preserved.
- Applicable escalation has been initiated and recorded.
- A valid recovery path back to maintenance execution exists.

### 16.2 Postconditions

- The missed schedule remains historically visible.
- The maintenance obligation is not silently discarded.
- Affected authorities may receive supported notification delivery.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No open issues.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No predetermined questions. Any further question must be triggered by new source evidence or a validation failure.
