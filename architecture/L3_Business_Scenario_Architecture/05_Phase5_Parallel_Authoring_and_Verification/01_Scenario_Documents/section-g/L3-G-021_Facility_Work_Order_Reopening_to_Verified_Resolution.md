# Facility Work Order Reopening to Verified Resolution

## 1. Scenario Metadata

**Scenario ID:** L3-G-021  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Facilities Management  
**Participating Domains:** G  
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

Govern authorised reopening of a completed or closed Facility Work Order when its prior resolution is incomplete or unsatisfactory, while preserving all preceding evidence.

## 3. Business Outcome

The reopened Work Order receives corrective execution, is reverified, and reaches verified closure with complete prior and successor history.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An authorised supervisor identifies a supported reopening reason for a completed or closed Work Order.

### 4.2 End Boundary

Corrective work is completed, verification succeeds, the Work Order is closed again, and earlier history remains immutable.

### 4.3 In Scope

- reopening decision and reason
- preservation of prior history
- corrective assignment and execution
- reverification and closure

### 4.4 Out of Scope

- deletion or rewriting of prior completion evidence
- creation of an unrelated replacement Work Order without source support

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Facilities Management (G01) | Owns reopening, corrective execution, reverification, closure and Work Order history. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Maintenance Supervisor | Reviews the completed Work Order and reopens it for a supported reason. | Decides/initiates. |
| Technician / Technician Team | Performs the corrective work. | Performs. |
| Facilities Manager | Verifies the successor resolution and closure. | Verifies/decides. |

## 7. Preconditions and Entry Criteria

- A completed or closed Work Order exists with immutable history.
- A supported reopening reason exists: incomplete resolution, quality failure, repeat failure, or administrative review.

## 8. Trigger

An authorised supervisor determines that the prior Work Order resolution requires reopening.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Review the prior completion, verification and closure evidence. | Maintenance Supervisor | Reopening basis is established. |
| 2 | Record the authorised reopening reason and publish WorkOrderReopened. | Maintenance Supervisor / Facilities Management | Work Order is reopened without erasing history. |
| 3 | Assign and perform the required corrective work. | Maintenance Supervisor / Technician | Corrective execution is completed. |
| 4 | Review the successor completion evidence. | Facilities Manager | Reverification decision is available. |
| 5 | Verify and close the Work Order again. | Facilities Management | Verified resolution is achieved. |

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Reopening reason is unsupported | Lifecycle must not be reopened. | Reject reopening and preserve the review outcome. | Facilities Management |
| Successor verification fails | Work Order cannot close. | Continue supported corrective execution and preserve failed verification. | Facilities Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Authorised supervisors may reopen completed Work Orders only for supported reasons. | Facilities Management | Constrains reopening. | L2-G01 §18 |
| Previous audit history remains immutable. | Facilities Management | Prevents destructive correction. | L2-G01 §18 |
| Verification occurs before closure. | Facilities Management | Requires reverified outcome. | L2-G01 §29 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Work Order | Completed / Verified / Closed | Authorised reopening followed by corrective execution and reverification. | In Progress → Completed → Verified → Closed | Facilities Management | L2-G01 §§18, 33 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| WorkOrderReopened | Facilities Management | Facilities operational participants | Records authorised reopening. | L2-G01 §31 |
| WorkOrderCompleted | Facilities Management | Verifier | Records successor completion. | L2-G01 §31 |
| WorkOrderVerified | Facilities Management | Facilities operational participants | Records successful reverification. | L2-G01 §31 |
| WorkOrderClosed | Facilities Management | Facilities operational participants | Records final closure. | L2-G01 §31 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain hand-off materially participates in this scenario.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Reopen Work Order | Maintenance Supervisor | Supported reason and audit record are required. | Facilities Management |
| Verify and close successor resolution | Facilities Manager | Mandatory evidence and verification must succeed. | Facilities Management |

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty obligation is defined by the authoritative L2 architecture. Any override stated above requires the source-supported authority, business reason, and audit history.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Reopening | Original Work Order state/history, reopening actor, reason and time-ordered action. | Facilities Management |
| Successor resolution | Corrective work, new completion evidence, reverification and final closure. | Facilities Management |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The reopening was authorised for a supported reason.
- Required corrective work was completed.
- Reverification succeeded and the Work Order is Closed.

### 16.2 Postconditions

- Original and successor Work Order histories remain chronologically available.
- No prior completion or verification evidence was modified.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No open issues.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
