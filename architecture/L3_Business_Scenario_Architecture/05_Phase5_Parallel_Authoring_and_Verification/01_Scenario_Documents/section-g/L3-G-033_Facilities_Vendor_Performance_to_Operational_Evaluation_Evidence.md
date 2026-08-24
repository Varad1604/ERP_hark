# Facilities Vendor Performance to Operational Evaluation Evidence

## 1. Scenario Metadata

**Scenario ID:** L3-G-033  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Facilities Management  
**Participating Domains:** G, J  
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

Govern the complete business path from **Facilities Vendor Performance** to **Operational Evaluation Evidence** using the ownership, lifecycle, authority, evidence and collaboration boundaries defined by L2-G01.

## 3. Business Outcome

Operational Evaluation Evidence is achieved and evidenced under Facilities Management ownership, while every participating domain retains authority over its own canonical information.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The source-supported initiating condition **Facilities Vendor Performance** is identified and submitted for governed action.

### 4.2 End Boundary

The authoritative business outcome **Operational Evaluation Evidence** is confirmed, recorded and available for subsequent authorised use.

### 4.3 In Scope

- validation of the initiating condition and authoritative references
- application of configured operational rules and decision authority
- recording of the source-supported outcome, evidence and audit history
- cross-domain hand-offs explicitly required by the frozen scenario metadata

### 4.4 Out of Scope

- ownership changes to infrastructure, workforce, academic, finance, inventory or notification information
- implementation design or delivery mechanics
- downstream execution beyond the stated end boundary
- invented states, events, roles or approval authorities

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Facilities Management (G01) | Owns the operational decision, lifecycle, evidence and audit history described by the scenario. |
| Enterprise Resource Management (J) | Provides or consumes only its source-supported authoritative business context; canonical ownership remains unchanged. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Maintenance Supervisor | Initiates or performs the source-supported operational activity. | Operational participation only. |
| Technician / Vendor Technician | Performs, reviews or authorises the activity within configured authority. | Operational participation only. |
| Facilities Manager | Performs, reviews or authorises the activity within configured authority. | Operational participation only. |

## 7. Preconditions and Entry Criteria

- The initiating condition Facilities Vendor Performance is valid and within Facilities Management scope.
- Referenced infrastructure, schedule, assignment or operational context is authoritative and active.
- Applicable configuration, authorization and evidence policies are effective.
- Participating-domain information is consumed without changing its canonical owner.

## 8. Trigger

The institution identifies or receives **Facilities Vendor Performance**, requiring the governed operational response defined by this scenario.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive and validate the initiating condition: Facilities Vendor Performance. | Maintenance Supervisor | Facilities Vendor Performance is accepted for governed processing. |
| 2 | Validate authoritative references, eligibility, applicable policy and operational constraints. | Facilities Management | The valid processing path is established. |
| 3 | Perform the source-supported operational assessment, assignment or decision. | Facilities Manager | The scenario progresses toward Operational Evaluation Evidence. |
| 4 | Record required business evidence and publish only exact source-defined events. | Facilities Management | The decision and history are auditable. |
| 5 | Confirm completion at the scenario boundary: Operational Evaluation Evidence. | Facilities Management | Operational Evaluation Evidence is established without transferring canonical ownership. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Configured alternative | The primary path cannot proceed but a source-supported alternative is permitted. | Apply the configured alternative with authorised reason and immutable history. | The obligation remains governed toward Operational Evaluation Evidence. | Facilities Management |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Invalid authoritative reference | The scenario cannot progress safely. | Correct or reject the reference under policy and preserve the validation evidence. | Facilities Management |
| Authorization or policy failure | The requested decision cannot be performed. | Route for authorised review or terminate under configured policy without deleting history. | Facilities Management |
| Required cross-domain context unavailable | The dependent decision is incomplete. | Pause the affected boundary, retain state and resume only when authoritative context is available. | Facilities Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Facilities Management owns the operational lifecycle and permanent history. | Facilities Management | Prevents cross-domain ownership transfer. | L2-G01 §§5, 13 |
| Every significant transition and business action is auditable. | Facilities Management | Requires actor, time, reason, prior position and resulting position. | L2-G01 Audit Responsibilities |
| Configuration determines institution-specific rules and permitted exceptions. | Facilities Management | Prevents scenario-specific invention of local policy. | L2-G01 Configuration Management |
| External canonical information is referenced, not re-owned. | Facilities Management | Preserves authoritative-domain boundaries. | L2-G01 §§4, 9 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Work Order | Source-supported current position | Applies the governed transition required for Facilities Vendor Performance to Operational Evaluation Evidence. | Created → Assigned → Accepted → In Progress → Completed → Verified → Closed | Facilities Management | L2-G01 State Transition Matrix |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Scenario Purpose | Source |
|---|---|---|---|---|
| VendorRated | Facilities Management | Configured interested domains | Records the exact source-defined transition relevant to Facilities Vendor Performance to Operational Evaluation Evidence. | L2-G01 §31 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Enterprise Resource Management | Facilities Management | Authoritative business context reference | Supports validation or decision without ownership transfer. | DEPENDENCY_MAP.md |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate Facilities Vendor Performance | Maintenance Supervisor | Must act within configured operational scope. | Facilities Management |
| Confirm Operational Evaluation Evidence | Facilities Manager | Requires source-supported completion evidence and authority. | Facilities Management |

### 15.2 Separation of Duties, Overrides and Exceptions

No additional separation-of-duty rule is invented. Any permitted override requires an authorised actor, configured basis, business reason and immutable audit history.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Facilities Vendor Performance | Initiating reference, actor, time, affected scope and validation result. | Facilities Management |
| Operational Evaluation Evidence | Decision, evidence, resulting lifecycle position, event references and closure actor. | Facilities Management |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The initiating condition Facilities Vendor Performance was validated.
- All mandatory business decisions and source-supported transitions were recorded.
- The outcome Operational Evaluation Evidence was confirmed with required evidence.
- Audit continuity and canonical ownership boundaries were preserved.

### 16.2 Postconditions

- Facilities Management retains authoritative operational history.
- Participating domains retain ownership of their canonical information.
- The outcome is available to authorised downstream consumers.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No open issues.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No predetermined questions. Any further question must be triggered by new source evidence or a validation failure.
