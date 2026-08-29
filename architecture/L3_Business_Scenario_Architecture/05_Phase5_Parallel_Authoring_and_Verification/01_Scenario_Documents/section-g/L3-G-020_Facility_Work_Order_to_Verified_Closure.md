# Facility Work Order to Verified Closure

## 1. Scenario Metadata

**Scenario ID:** L3-G-020  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Facilities Management  
**Participating Domains:** G, E, D, I, H  
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

Govern a Facility Work Order from creation through assignment, execution, evidence-based verification and authoritative closure.

## 3. Business Outcome

The Work Order is completed, verified, closed, and retained with its full maintenance and audit history.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A valid operational source creates a Work Order for required facility work.

### 4.2 End Boundary

Mandatory work, checklist and evidence requirements are satisfied, verification succeeds, and the Work Order reaches Closed.

### 4.3 In Scope

- Work Order creation, assignment, acceptance, planning and execution
- completion evidence and checklist obligations
- verification and closure
- workforce context and supported operational-impact awareness

### 4.4 Out of Scope

- employee lifecycle management
- academic or examination rescheduling
- notification delivery mechanics
- physical infrastructure ownership

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Facilities Management (G01) | Owns the Work Order lifecycle, execution governance, verification, closure and history. |
| People & Workforce (E) | Provides authoritative technician identity, skills and availability. |
| Academic Delivery (D) | Owns academic responses to material facility impact. |
| Examination (I) | Owns examination responses to material facility impact. |
| Institutional Services (H) | Owns supported notification delivery. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Maintenance Supervisor | Creates and assigns the Work Order. | Initiates/assigns. |
| Technician / Technician Team | Accepts and performs assigned work and supplies completion evidence. | Performs. |
| Facilities Manager | Governs verification and closure under configured policy. | Verifies/decides. |

## 7. Preconditions and Entry Criteria

- A valid operational trigger and infrastructure reference exist.
- The applicable Work Order policy, checklist, SOP and verification rules are effective.
- Authoritative workforce context is available for assignment.

## 8. Trigger

A Complaint, Preventive Maintenance activity, Inspection, Emergency Maintenance need, Administrative Request, or Vendor Recommendation requires a Work Order.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create the Work Order against the valid trigger and infrastructure reference. | Maintenance Supervisor | Work Order is Created. |
| 2 | Select and assign an authorised technician, team or vendor using applicable policy. | Maintenance Supervisor / E | Work Order is Assigned. |
| 3 | Accept and plan the assigned work. | Technician / Team | Work Order is Accepted. |
| 4 | Execute the work and record required checklists, consumables and evidence. | Technician / Team | Work Order progresses In Progress to Completed. |
| 5 | Review completion evidence and quality requirements. | Facilities Manager | Work Order is Verified. |
| 6 | Close the verified Work Order and preserve its history. | Facilities Management | Work Order is Closed. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Multiple technicians or vendor team | Work requires a team or external operational participation. | Apply configured team/vendor assignment while Facilities retains governance. | Same verified Work Order outcome. | Facilities Management |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Assignment rejected or technician unavailable | Work cannot begin with the current assignment. | Use the governed reassignment scenario and preserve the exception. | Facilities Management |
| Verification fails | Work cannot close. | Return to corrective execution or authorised reopening and preserve failed evidence. | Facilities Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every Work Order references valid infrastructure and exactly one current lifecycle state. | Facilities Management | Required throughout the scenario. | L2-G01 §13 |
| Completed Work Orders require completion evidence. | Facilities Management | Completion alone is insufficient without evidence. | L2-G01 §13 |
| Verification occurs before operational closure. | Facilities Management | Enforces distinct Completed, Verified and Closed positions. | L2-G01 §§13, 29 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Work Order | Created | Assignment, acceptance, execution, completion, verification and closure. | Assigned → Accepted → In Progress → Completed → Verified → Closed | Facilities Management | L2-G01 §33 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| WorkOrderCreated | Facilities Management | E and operational participants | Establishes the Work Order. | L2-G01 §31 |
| WorkOrderAssigned | Facilities Management | Assigned technician/team; H where supported | Records assignment. | L2-G01 §31 |
| WorkOrderCompleted | Facilities Management | Verifier and interested domains | Records operational completion. | L2-G01 §31 |
| WorkOrderVerified | Facilities Management | D, I, H where materially interested | Records successful verification. | L2-G01 §31 |
| WorkOrderClosed | Facilities Management | D, I, H where materially interested | Publishes final closure. | L2-G01 §31 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| E | Facilities Management | Technician identity, skills, certification and availability context | Supports assignment without transferring personnel ownership. | L2-G01 §19 |
| Facilities Management | D / I | Material facility impact and verified closure evidence | Supports their owning operational decisions. | DEPENDENCY_MAP.md |
| Facilities Management | H | Exact supported Work Order event requiring awareness | Enables notification delivery. | L2-G01 §31 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Create and assign Work Order | Maintenance Supervisor | Must follow configured assignment policy. | Facilities Management |
| Perform assigned work | Technician / Technician Team | Must hold the operational assignment. | Facilities Management |
| Verify and close | Facilities Manager | Mandatory evidence and verification must succeed. | Facilities Management |

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty obligation is defined by the authoritative L2 architecture. Any override stated above requires the source-supported authority, business reason, and audit history.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Work Order lifecycle | Trigger, infrastructure, assigned party, previous/new state and business action. | Facilities Management |
| Completion, verification and closure | Checklists, mandatory evidence, verification outcome and closing decision. | Facilities Management |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Required facility work is completed.
- Mandatory checklist and evidence obligations are satisfied.
- Verification succeeds and the Work Order is Closed.

### 16.2 Postconditions

- The Work Order and maintenance history are immutable and searchable.
- Supported domains may consume verified closure evidence.
- Infrastructure and workforce ownership remain external.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No open issues.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
