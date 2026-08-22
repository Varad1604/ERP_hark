# Preventive Maintenance Plan to Completed Maintenance Cycle

## 1. Scenario Metadata

**Scenario ID:** L3-G-023  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Facilities Management  
**Participating Domains:** G, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P0  
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

Execute a governed Preventive Maintenance Plan through its scheduled Work Order, verification, completion, and next-cycle continuity.

## 3. Business Outcome

The scheduled preventive-maintenance activity is completed and verified, its history is preserved, and the next governed schedule is available.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A valid Preventive Maintenance Plan reaches its governed schedule occurrence.

### 4.2 End Boundary

The generated Work Order and preventive-maintenance activity are verified and completed, with the next schedule generated or preserved.

### 4.3 In Scope

- schedule occurrence and Work Order generation
- technician assignment and execution
- checklist and completion evidence
- verification, completion and next schedule
- supported notification need

### 4.4 Out of Scope

- physical infrastructure lifecycle ownership
- workforce master-data changes
- inventory valuation or procurement
- scheduling implementation design

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Facilities Management (G01) | Owns the Preventive Maintenance Plan, schedule occurrence, Work Order execution, verification and history. |
| People & Workforce (E) | Provides authoritative technician context. |
| Institutional Services (H) | Owns supported notification delivery. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Facilities Manager | Establishes or governs the preventive plan under policy. | Plans/decides. |
| Maintenance Supervisor | Generates/assigns the scheduled Work Order. | Assigns and monitors. |
| Technician / Technician Team | Executes the maintenance checklist and supplies evidence. | Performs. |
| Facilities Manager | Verifies completion. | Verifies. |

## 7. Preconditions and Entry Criteria

- A valid Preventive Maintenance Plan and current schedule exist.
- The referenced infrastructure and applicable checklist are valid.
- Authoritative workforce context is available for assignment.

## 8. Trigger

The Preventive Maintenance Plan reaches a scheduled, usage-based, seasonal, or institution-configured maintenance occurrence.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Recognise the governed schedule occurrence. | Facilities Management | Preventive Maintenance is Scheduled. |
| 2 | Generate the Work Order against the plan and infrastructure. | Maintenance Supervisor | Preventive Maintenance is Generated and Work Order Created. |
| 3 | Assign the Work Order using authoritative workforce context. | Maintenance Supervisor / E | Work Order is Assigned. |
| 4 | Execute the checklist and record completion evidence. | Technician / Team | Maintenance is Executed and Work Order Completed. |
| 5 | Verify the work and maintenance outcome. | Facilities Manager | Maintenance is Verified. |
| 6 | Complete the maintenance cycle and generate/preserve the next schedule. | Facilities Management | Preventive Maintenance is Completed. |

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Checklist or mandatory evidence incomplete | Cycle cannot be verified or completed. | Return the Work Order to supported corrective completion. | Facilities Management |
| Technician unavailable | Scheduled work cannot proceed with current assignment. | Use governed reassignment without changing workforce ownership. | Facilities Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Preventive Maintenance is schedule-driven and executes through Work Orders. | Facilities Management | Defines the operational path. | L2-G01 §20 |
| Checklist templates and schedules remain configuration-driven/version controlled. | Facilities Management | Applies the effective plan requirements. | L2-G01 §20 |
| Completed Work Orders require evidence and verification before closure. | Facilities Management | Prevents incomplete cycle completion. | L2-G01 §§13, 29 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Preventive Maintenance | Planned | Schedule, generation, execution, verification and completion. | Scheduled → Generated → Executed → Verified → Completed | Facilities Management | L2-G01 §§20, 33 |
| Work Order | Created | Assignment, execution, completion, verification and closure. | Assigned → Accepted → In Progress → Completed → Verified → Closed | Facilities Management | L2-G01 §33 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| PreventiveMaintenanceScheduled | Facilities Management | E, H where supported | Establishes scheduled maintenance occurrence. | L2-G01 §31 |
| WorkOrderCreated | Facilities Management | E and assigned participants | Establishes execution unit. | L2-G01 §31 |
| PreventiveMaintenanceCompleted | Facilities Management | H and operational consumers where supported | Records completed maintenance cycle. | L2-G01 §31 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| E | Facilities Management | Technician skills and availability context | Supports assignment. | L2-G01 §19 |
| Facilities Management | H | Supported maintenance/Work Order notification need | Enables awareness without ownership transfer. | L2-G01 §31 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Govern preventive plan and completion | Facilities Manager | Must follow configured maintenance and verification policy. | Facilities Management |
| Assign scheduled work | Maintenance Supervisor | Must use eligible authoritative workforce context. | Facilities Management |
| Execute checklist | Technician / Team | Must hold the Work Order assignment. | Facilities Management |

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty obligation is defined by the authoritative L2 architecture. Any override stated above requires the source-supported authority, business reason, and audit history.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Maintenance cycle | Plan, schedule occurrence, Work Order, infrastructure and lifecycle actions. | Facilities Management |
| Verification and completion | Checklist, mandatory evidence, verification result and next schedule. | Facilities Management |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The scheduled maintenance Work Order is completed and verified.
- PreventiveMaintenanceCompleted is supported by required evidence.
- The next governed schedule is generated or preserved.

### 16.2 Postconditions

- Preventive-maintenance and Work Order history remain immutable.
- The infrastructure remains governed by its canonical external owner.
- The next maintenance cycle can proceed under the plan.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No open issues.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
