# Facility Complaint to Verified Closure

## 1. Scenario Metadata

**Scenario ID:** L3-G-017  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Facilities Management  
**Participating Domains:** G, H  
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

Govern a facility complaint from registration through investigation, supported operational resolution, verification, and authoritative closure.

## 3. Business Outcome

The Facility Complaint is resolved, verified, closed, and retained with complete operational evidence and history.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A facility-related operational problem is registered as a Complaint from a supported complaint source.

### 4.2 End Boundary

The resolution has been verified and the Complaint is in the canonical Closed state.

### 4.3 In Scope

- complaint validation, classification, assignment and investigation
- administrative resolution or Work Order determination
- verification and closure
- supported notification awareness

### 4.4 Out of Scope

- detailed execution of any generated Work Order
- notification delivery mechanics
- changes to physical infrastructure ownership

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Facilities Management (G01) | Owns the Complaint lifecycle, investigation, resolution, verification, closure and evidence. |
| Institutional Services (H) | Owns supported complaint notifications where required. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Service Requester / Department Representative | Registers the operational complaint and provides business context. | Initiates. |
| Maintenance Supervisor | Classifies, assigns, investigates and governs resolution. | Performs/reviews. |
| Facilities Manager | Governs resolution and closure under configured policy. | Decides. |

## 7. Preconditions and Entry Criteria

- The Complaint references at least one valid infrastructure location.
- Applicable complaint categories, priorities, severity and assignment policies are effective.

## 8. Trigger

A supported facility complaint is registered for investigation or corrective action.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate and register the Complaint. | Maintenance Supervisor | Complaint is Open. |
| 2 | Classify and assign the Complaint for investigation. | Maintenance Supervisor | Complaint is Assigned and progresses In Progress. |
| 3 | Determine whether a Work Order is required. | Maintenance Supervisor | Administrative or Work Order resolution route is selected. |
| 4 | Complete the supported resolution and record evidence. | Facilities Management | Complaint is Resolved. |
| 5 | Verify the resolution against applicable quality and evidence requirements. | Facilities Manager | Complaint is Verified. |
| 6 | Close the verified Complaint and preserve history. | Facilities Management | Complaint is Closed. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Administrative resolution | Investigation determines that no Work Order is required. | Record the administrative resolution and proceed to verification. | Verified complaint resolution. | Facilities Management |
| Work Order route | Operational maintenance work is required. | Create the governed Work Order and reconcile its verified outcome before Complaint closure. | Complaint can proceed to verification. | Facilities Management |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Verification failure | Resolution is not satisfactory. | Return the Complaint to supported corrective work or reopening; preserve failed evidence. | Facilities Management |
| Invalid infrastructure location | Complaint invariant is not satisfied. | Correct the reference before assignment or reject under policy. | Facilities Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every Complaint belongs to at least one infrastructure location. | Facilities Management | Required before processing. | L2-G01 §13 |
| Verification occurs before operational closure. | Facilities Management | Resolved is not equivalent to Closed. | L2-G01 §§13, 29 |
| Historical complaint and audit records remain immutable. | Facilities Management | Preserves reconstruction and investigation evidence. | L2-G01 §§13, 37 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Complaint | Open | Assignment, investigation, resolution, verification and closure. | Assigned → In Progress → Resolved → Verified → Closed | Facilities Management | L2-G01 §33 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| FacilityComplaintRegistered | Facilities Management | H where supported | Establishes the Complaint. | L2-G01 §31 |
| ComplaintAssigned | Facilities Management | Assigned operational participants; H where supported | Records operational assignment. | L2-G01 §31 |
| ComplaintClosed | Facilities Management | H and other interested domains where supported | Publishes verified final closure. | L2-G01 §31 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Facilities Management | H | Exact complaint events requiring supported awareness | Enables notification delivery without transferring Complaint ownership. | L2-G01 §31 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Register complaint | Service Requester / Department Representative | Must be a supported authorised source. | Facilities Management |
| Classify, assign and investigate | Maintenance Supervisor | Subject to configured Facilities policy. | Facilities Management |
| Verify and close | Facilities Manager | Requires successful verification and mandatory evidence. | Facilities Management |

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty obligation is defined by the authoritative L2 architecture. Any override stated above requires the source-supported authority, business reason, and audit history.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Complaint lifecycle | Complaint reference, infrastructure location, category, assignment, prior/new state and action. | Facilities Management |
| Verification and closure | Resolution evidence, verification outcome, business reason and closing authority. | Facilities Management |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The supported resolution is complete.
- Applicable evidence and quality verification succeeded.
- The Complaint is Closed and its history remains immutable.

### 16.2 Postconditions

- The verified complaint resolution is available as authoritative Facilities history.
- Any Work Order retains its independent lifecycle and evidence.
- Supported notifications may be delivered by H.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No open issues.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
