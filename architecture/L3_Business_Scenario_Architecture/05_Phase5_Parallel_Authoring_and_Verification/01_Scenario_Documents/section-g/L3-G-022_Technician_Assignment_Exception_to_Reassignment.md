# Technician Assignment Exception to Reassignment

## 1. Scenario Metadata

**Scenario ID:** L3-G-022  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Facilities Management  
**Participating Domains:** G, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Architecture Approved  
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

Resolve a technician-assignment exception through a governed alternative Facilities assignment while preserving workforce ownership and operational history.

## 3. Business Outcome

The Work Order has a valid alternative technician, team, or vendor-team assignment and can continue its existing lifecycle.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A current assignment cannot proceed because of unavailability, rejection with reason, skill mismatch, workload conflict, or assistance requirement.

### 4.2 End Boundary

A qualified and available alternative assignment is recorded, auditable, and communicated through supported notification ownership.

### 4.3 In Scope

- assignment-exception validation
- authoritative workforce context
- alternative operational assignment
- assignment audit and supported notification need

### 4.4 Out of Scope

- employee master-data or availability changes
- assignment implementation design
- Work Order execution after valid reassignment

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Facilities Management (G01) | Owns the Work Order and operational assignment decision. |
| People & Workforce (E) | Owns technician identity, skills, certification, availability and workforce state. |
| Institutional Services (H) | Owns supported notification delivery. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Technician | Rejects or requests reassignment with a supported reason where applicable. | Initiates exception. |
| Maintenance Supervisor | Validates the exception and selects an alternative under assignment policy. | Reviews/assigns. |
| Facilities Manager | Governs a manual override where supported. | Decides exception path. |

## 7. Preconditions and Entry Criteria

- A valid Work Order and current assignment exist.
- Authoritative workforce skills, certification, availability and workload context can be evaluated.
- Assignment and override policies are effective.

## 8. Trigger

A supported technician-assignment exception prevents the current assignment from progressing.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Record and validate the assignment exception and reason. | Technician / Maintenance Supervisor | Exception is confirmed. |
| 2 | Obtain authoritative workforce context for eligible alternatives. | E / Maintenance Supervisor | Qualified alternatives are known. |
| 3 | Apply configured assignment factors and any supported override. | Maintenance Supervisor / Facilities Manager | Alternative is selected. |
| 4 | Record the alternative operational assignment against the same Work Order. | Facilities Management | Work Order has a valid assignment. |
| 5 | Publish the supported assignment event and notification need where semantically exact. | Facilities Management / H | Assignment awareness is enabled. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Vendor or hybrid assignment | No suitable internal technician/team is available and policy permits external participation. | Assign an authorised vendor or hybrid team while Facilities retains governance. | Valid alternative assignment. | Facilities Management |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| No qualified alternative | Work Order cannot progress. | Escalate under configured Facilities policy and preserve the unresolved exception. | Facilities Management |
| Override lacks authority or reason | Override is invalid. | Reject override and retain the prior assignment exception. | Facilities Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Personnel ownership remains with Section E; Facilities owns operational assignment. | E / Facilities Management | Preserves ownership boundary. | L2-G01 §19 |
| Assignment decisions may consider skills, certification, availability, workload, campus, department, shift and emergency priority. | Facilities Management | Constrains alternative selection. | L2-G01 §19 |
| Manual override requires authorised supervision and recorded reason. | Facilities Management | Makes override auditable. | L2-G01 §19 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Work Order | Created / Assigned | Assignment exception is resolved without creating a new Work Order state. | Assigned | Facilities Management | L2-G01 §§19, 33 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| WorkOrderAssigned | Facilities Management | E, assigned technician/team, H where supported | Records the valid operational assignment; no distinct reassignment event is defined. | L2-G01 §31 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| E | Facilities Management | Authoritative technician skills, certification, availability and workload context | Supports reassignment without workforce ownership transfer. | L2-G01 §19 |
| Facilities Management | H | Supported WorkOrderAssigned notification need | Enables delivery to affected participants. | L2-G01 §§19, 31 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Request/reject assignment | Technician | Reason is required where applicable. | Facilities Management |
| Reassign Work Order | Maintenance Supervisor | Must apply configured assignment policy. | Facilities Management |
| Manual override | Facilities Manager | Authorised reason and audit record required. | Facilities Management |

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty obligation is defined by the authoritative L2 architecture. Any override stated above requires the source-supported authority, business reason, and audit history.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Assignment exception | Work Order, prior assignment, exception type, reason and actor. | Facilities Management |
| Alternative assignment | Selected assignee/team, authoritative decision actor, override reason where applicable. | Facilities Management |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The assignment exception was validated.
- A qualified and available alternative assignment was recorded.
- The same Work Order can continue under the new assignment.

### 16.2 Postconditions

- Facilities retains the operational assignment history.
- E retains workforce ownership.
- Affected participants may receive supported notification delivery.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No open issues.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No predetermined questions. Any further question must be triggered by new source evidence or a validation failure.
