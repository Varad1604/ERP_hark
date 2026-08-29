# Facility Complaint to Multi-Work-Order Resolution

## 1. Scenario Metadata

**Scenario ID:** L3-G-019  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Facilities Management  
**Participating Domains:** G, D, I, E  
**Scenario Type:** Cross-domain  
**Priority:** P1  
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

Govern a Facility Complaint whose resolution requires multiple independent Work Orders while preserving the Complaint as the coordinating operational outcome.

## 3. Business Outcome

The Complaint is split into the required independent Work Orders, their verified outcomes are reconciled, and the parent Complaint is resolved and closed.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

Complaint investigation determines that more than one operational work discipline or team is required.

### 4.2 End Boundary

All Work Orders required for the Complaint have reached their governed verified outcomes and the Complaint is verified and closed.

### 4.3 In Scope

- split decision and Complaint lineage
- creation and independent execution of multiple Work Orders
- workforce context for assignment
- operational-impact evidence for Academic Delivery and Examination where materially affected
- reconciliation into Complaint closure

### 4.4 Out of Scope

- academic or examination rescheduling decisions
- workforce master-data changes
- combining independent Work Order lifecycles

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Facilities Management (G01) | Owns the Complaint split, Work Orders, operational resolution and closure. |
| Academic Delivery (D) | Owns academic scheduling decisions informed by material facility impact. |
| Examination (I) | Owns examination decisions informed by material facility impact. |
| People & Workforce (E) | Provides authoritative workforce identity, skills and availability for assignment. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Maintenance Supervisor | Determines the required work disciplines and creates governed Work Orders. | Reviews and performs. |
| Facilities Manager | Governs the split and reconciled resolution. | Decides. |
| Technician / Technician Team | Executes an assigned Work Order. | Performs. |

## 7. Preconditions and Entry Criteria

- A valid Complaint exists and is under investigation.
- The Complaint references valid infrastructure.
- Required technicians or teams can be evaluated from authoritative workforce context.

## 8. Trigger

Complaint investigation identifies multiple independent operational work requirements.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Classify the distinct operational work requirements. | Maintenance Supervisor | Required Work Order set is defined. |
| 2 | Split the Complaint into independently governed Work Orders. | Facilities Manager / Facilities Management | ComplaintSplit is established. |
| 3 | Assign each Work Order using authoritative workforce context. | Maintenance Supervisor / E | Each Work Order is Assigned. |
| 4 | Execute, complete and verify each Work Order independently. | Technician Teams / Facilities Management | Each required work outcome is verified. |
| 5 | Reconcile all required outcomes against the parent Complaint. | Facilities Manager | Complaint resolution is complete. |
| 6 | Verify and close the Complaint. | Facilities Management | Multi-Work-Order resolution is closed. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Partial completion with remaining work | Some Work Orders complete while another remains open. | Keep the Complaint open and preserve completed Work Order evidence until all required outcomes reconcile. | Complaint remains In Progress. | Facilities Management |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required Work Order fails verification | Parent Complaint cannot close. | Return that Work Order to supported correction or reopening. | Facilities Management |
| Operational impact affects academic/examination activity | Facilities cannot decide the downstream schedule. | Provide impact evidence to D or I for its owning decision. | D / I |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| A Complaint may generate multiple Work Orders and each has an independent lifecycle. | Facilities Management | Prevents combined state ownership. | L2-G01 §17 |
| Every Work Order references valid infrastructure and one current lifecycle state. | Facilities Management | Required for each generated Work Order. | L2-G01 §13 |
| Verification occurs before closure. | Facilities Management | All required work outcomes must be verified. | L2-G01 §29 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Complaint | Assigned / In Progress | Split and reconciliation of required work outcomes. | Resolved → Verified → Closed | Facilities Management | L2-G01 §§17, 33 |
| Work Order | Created | Independent assignment, execution, completion and verification. | Assigned → Accepted → In Progress → Completed → Verified → Closed | Facilities Management | L2-G01 §§18, 33 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| ComplaintSplit | Facilities Management | D, I where materially affected; Facilities participants | Records creation of multiple Work Orders. | L2-G01 §31 |
| WorkOrderCreated | Facilities Management | E and assigned operational participants | Establishes each independent Work Order. | L2-G01 §31 |
| WorkOrderVerified | Facilities Management | Parent Complaint coordination; D/I where materially interested | Confirms each verified work outcome. | L2-G01 §31 |
| ComplaintClosed | Facilities Management | D, I where materially affected | Publishes final verified Complaint closure. | L2-G01 §31 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| E | Facilities Management | Authoritative skills and availability context | Supports operational assignment without transferring workforce ownership. | L2-G01 §19; DEPENDENCY_MAP.md |
| Facilities Management | D / I | Facility impact and verified-resolution evidence | Supports owning academic or examination decisions. | DEPENDENCY_MAP.md; L2-G01 §§17–18 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Split Complaint and create Work Orders | Maintenance Supervisor / Facilities Manager | Must follow configured split and assignment policies. | Facilities Management |
| Verify Work Orders and Complaint | Facilities Manager | All mandatory evidence and required work outcomes must be satisfactory. | Facilities Management |

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty obligation is defined by the authoritative L2 architecture. Any override stated above requires the source-supported authority, business reason, and audit history.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Complaint split | Parent Complaint, generated Work Orders, work discipline, actor and reason. | Facilities Management |
| Independent execution and reconciliation | Each Work Order lifecycle, completion/verification evidence and parent reconciliation. | Facilities Management |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- All required independent Work Orders exist and retain separate lifecycles.
- Every required Work Order outcome is verified.
- The parent Complaint is verified and closed.

### 16.2 Postconditions

- The complete parent/Work Order lineage is preserved.
- D and I may use operational evidence for their own decisions.
- Workforce ownership remains with E.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No open issues.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
