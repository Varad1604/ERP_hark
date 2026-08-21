# Laboratory Inspection to Operational Closure

## 1. Scenario Metadata

**Scenario ID:** L3-G-067  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Laboratory Management  
**Participating Domains:** G, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Architecture Approved  
**Version:** 1.3  
**Owner:** Pallav  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-G02_Laboratory_Management_Workflows.md; L3_Business_Scenario_Template_v2.0.md  
**Referenced By:** None yet  
**Architecture Approval:** ACR-G-2026-08-21-002  
**Approved By:** Arjun Mehta, Chief Enterprise Architect  
**Approval Date:** 21 August 2026  
**Approval Decision:** Approved — No Conditions  

## 2. Scenario Purpose

Govern the complete business path from **Laboratory Inspection** to **Operational Closure** using the ownership, lifecycle, authority, evidence and collaboration boundaries defined by L2-G02.

## 3. Business Outcome

Operational Closure is achieved and evidenced under Laboratory Management ownership, while every participating domain retains authority over its own canonical information.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The source-supported initiating condition **Laboratory Inspection** is identified and submitted for governed action.

### 4.2 End Boundary

The authoritative business outcome **Operational Closure** is confirmed, recorded and available for subsequent authorised use.

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
| Laboratory Management (G02) | Owns the operational decision, lifecycle, evidence and audit history described by the scenario. |
| People & Workforce (E) | Provides or consumes only its source-supported authoritative business context; canonical ownership remains unchanged. |
| Institutional Services (H) | Provides or consumes only its source-supported authoritative business context; canonical ownership remains unchanged. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Laboratory Coordinator | Initiates or performs the source-supported operational activity. | Operational participation only. |
| Laboratory In-Charge | Performs, reviews or authorises the activity within configured authority. | Operational participation only. |

## 7. Preconditions and Entry Criteria

- The initiating condition Laboratory Inspection is valid and within Laboratory Management scope.
- Referenced infrastructure, schedule, assignment or operational context is authoritative and active.
- Applicable configuration, authorization and evidence policies are effective.
- Participating-domain information is consumed without changing its canonical owner.

## 8. Trigger

The institution identifies or receives **Laboratory Inspection**, requiring the governed operational response defined by this scenario.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive and validate the initiating condition: Laboratory Inspection. | Laboratory Coordinator | Laboratory Inspection is accepted for governed processing. |
| 2 | Validate authoritative references, eligibility, applicable policy and operational constraints. | Laboratory Management | The valid processing path is established. |
| 3 | Perform the source-supported operational assessment, assignment or decision. | Laboratory In-Charge | The scenario progresses toward Operational Closure. |
| 4 | Record required business evidence and publish only exact source-defined events. | Laboratory Management | The decision and history are auditable. |
| 5 | Confirm completion at the scenario boundary: Operational Closure. | Laboratory Management | Operational Closure is established without transferring canonical ownership. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Configured alternative | The primary path cannot proceed but a source-supported alternative is permitted. | Apply the configured alternative with authorised reason and immutable history. | The obligation remains governed toward Operational Closure. | Laboratory Management |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Invalid authoritative reference | The scenario cannot progress safely. | Correct or reject the reference under policy and preserve the validation evidence. | Laboratory Management |
| Authorization or policy failure | The requested decision cannot be performed. | Route for authorised review or terminate under configured policy without deleting history. | Laboratory Management |
| Required cross-domain context unavailable | The dependent decision is incomplete. | Pause the affected boundary, retain state and resume only when authoritative context is available. | Laboratory Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Laboratory Management owns the operational lifecycle and permanent history. | Laboratory Management | Prevents cross-domain ownership transfer. | L2-G02 §§5, 13 |
| Every significant transition and business action is auditable. | Laboratory Management | Requires actor, time, reason, prior position and resulting position. | L2-G02 Audit Responsibilities |
| Configuration determines institution-specific rules and permitted exceptions. | Laboratory Management | Prevents scenario-specific invention of local policy. | L2-G02 Configuration Management |
| External canonical information is referenced, not re-owned. | Laboratory Management | Preserves authoritative-domain boundaries. | L2-G02 §§4, 9 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Laboratory Inspection | Source-supported current position | Applies the governed transition required for Laboratory Inspection to Operational Closure. | Planned → Assigned → Conducted → Reviewed → Closed | Laboratory Management | L2-G02 State Transition Matrix |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Scenario Purpose | Source |
|---|---|---|---|---|
| LaboratoryInspectionScheduled | Laboratory Management | Configured interested domains | Records the exact source-defined transition relevant to Laboratory Inspection to Operational Closure. | L2-G02 §27 |
| LaboratoryInspectionStarted | Laboratory Management | Configured interested domains | Records the exact source-defined transition relevant to Laboratory Inspection to Operational Closure. | L2-G02 §27 |
| LaboratoryInspectionCompleted | Laboratory Management | Configured interested domains | Records the exact source-defined transition relevant to Laboratory Inspection to Operational Closure. | L2-G02 §27 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| People & Workforce | Laboratory Management | Authoritative business context reference | Supports validation or decision without ownership transfer. | DEPENDENCY_MAP.md |
| Institutional Services | Laboratory Management | Authoritative business context reference | Supports validation or decision without ownership transfer. | DEPENDENCY_MAP.md |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate Laboratory Inspection | Laboratory Coordinator | Must act within configured operational scope. | Laboratory Management |
| Confirm Operational Closure | Laboratory In-Charge | Requires source-supported completion evidence and authority. | Laboratory Management |

### 15.2 Separation of Duties, Overrides and Exceptions

No additional separation-of-duty rule is invented. Any permitted override requires an authorised actor, configured basis, business reason and immutable audit history.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Laboratory Inspection | Initiating reference, actor, time, affected scope and validation result. | Laboratory Management |
| Operational Closure | Decision, evidence, resulting lifecycle position, event references and closure actor. | Laboratory Management |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The initiating condition Laboratory Inspection was validated.
- All mandatory business decisions and source-supported transitions were recorded.
- The outcome Operational Closure was confirmed with required evidence.
- Audit continuity and canonical ownership boundaries were preserved.

### 16.2 Postconditions

- Laboratory Management retains authoritative operational history.
- Participating domains retain ownership of their canonical information.
- The outcome is available to authorised downstream consumers.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No open issues.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No predetermined questions. Any further question must be triggered by new source evidence or a validation failure.
