# Laboratory Readiness Assessment to Operational Readiness

## 1. Scenario Metadata

**Scenario ID:** L3-G-050  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Laboratory Management  
**Participating Domains:** G, D, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.4  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-G02_Laboratory_Management_Workflows.md; L3_Business_Scenario_Template_v2.0.md  
**Referenced By:** None yet

## 2. Scenario Purpose

Establish whether a Laboratory is operationally ready for governed academic use by evaluating the safety, equipment and utility conditions defined by Laboratory Management architecture.

## 3. Business Outcome

The Laboratory readiness assessment is completed and its governed readiness position is available for subsequent Laboratory operations without transferring ownership of infrastructure, workforce or academic context.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A Laboratory requires an operational-readiness assessment before or during governed use.

### 4.2 End Boundary

The readiness assessment is completed, the resulting readiness evidence is recorded, and any source-supported operational-status consequence is established.

### 4.3 In Scope

- validation that the Laboratory and its authoritative physical-space reference are eligible for assessment;
- evaluation of safety, equipment and utility readiness;
- classification of warnings and critical safety exceptions;
- readiness evidence and applicable Laboratory Management business events.

### 4.4 Out of Scope

- ownership changes to infrastructure, workforce or academic information;
- practical-session execution;
- equipment procurement or financial processing;
- implementation design or technical monitoring mechanics.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Laboratory Management (G02) | Owns Laboratory readiness assessment, operational-status consequences and Laboratory evidence. |
| Academic Delivery (D) | Provides authoritative academic/practical-session context where materially required. |
| People & Workforce (E) | Provides authoritative workforce/assignment context where materially required. |
| Institutional Services (H) | Consumes or supports only source-authorized institutional-service outcomes; it does not own Laboratory readiness. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Laboratory In-Charge | Prepares and validates Laboratory readiness evidence within the Laboratory Management responsibility defined by L2-G02. | Performs/reviews Laboratory readiness activity within source-supported authority. |
| Programme / Department Coordinator | Provides academic context where the readiness decision depends on the planned academic use. | Participates through authoritative academic context; does not own Laboratory readiness. |
| Senior Institutional Authority | Participates only where L2-G02 requires a decision on a critical safety exception. | Decides the critical-safety exception within the authority defined upstream. |

No `Laboratory Coordinator` or `Laboratory Assistant` authority is introduced because those roles are not required by the authoritative L2 readiness workflow.

## 7. Preconditions and Entry Criteria

- The Laboratory is identified by Laboratory Management and references an authoritative Infrastructure Space where applicable.
- The Laboratory is not in the terminal `DECOMMISSIONED` operational state.
- Safety, equipment and utility evidence required by the readiness assessment is available.
- Any academic or workforce context used by the assessment is obtained from its canonical owner.

## 8. Trigger

A Laboratory reaches a governed point at which operational readiness must be assessed or reassessed.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Confirm that the Laboratory is eligible for readiness assessment and that its authoritative physical-space reference is valid. | Laboratory Management | The Laboratory may enter the readiness assessment. |
| 2 | Evaluate the safety, equipment and utility conditions defined by L2-G02. | Laboratory In-Charge / Laboratory Management | Readiness evidence and any warning or exception are established. |
| 3 | Establish the readiness profile in the source-supported monitoring position and classify identified conditions. | Laboratory Management | The Laboratory readiness position is governed. |
| 4 | Where a critical safety exception exists, obtain the source-required senior institutional decision before treating the Laboratory as ready for the affected use. | Senior Institutional Authority / Laboratory Management | Critical safety exception is resolved by authorized business decision. |
| 5 | Complete the readiness assessment and publish `LaboratoryReadinessReviewed`; where the Laboratory operational status changes, publish `LaboratoryOperationalStatusChanged`. | Laboratory Management | The governed readiness outcome is available to authorized consumers. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Non-critical readiness warning | L2-G02 identifies a warning that is non-blocking under the governing policy. | Record the warning and continue the governed readiness path with monitoring obligations. | Readiness assessment completes with the warning preserved. | Laboratory Management |
| Critical safety exception | A critical safety condition requires the source-defined senior institutional decision. | Escalate the exception to the Senior Institutional Authority and apply the authorized decision. | The readiness path proceeds only if the critical exception is authoritatively resolved. | Laboratory Management |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Laboratory is `DECOMMISSIONED` | Readiness assessment cannot establish operational use. | Preserve the terminal Laboratory position; no readiness outcome authorizing use is created. | Laboratory Management |
| Required safety, equipment or utility evidence is unavailable or invalid | Readiness cannot be authoritatively established. | Preserve the last valid Laboratory position and correct the missing business evidence before reassessment. | Laboratory Management |
| Critical safety exception is unresolved | The affected Laboratory use cannot be treated as ready. | Preserve the restrictive business position until the source-required senior institutional decision is obtained. | Laboratory Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Laboratory Management owns the Laboratory operational lifecycle and readiness evidence. | Laboratory Management | Participating domains cannot modify the Laboratory lifecycle. | L2-G02 |
| Safety, equipment and utility readiness are assessed as part of Laboratory readiness. | Laboratory Management | Readiness cannot be asserted without the governing assessment. | L2-G02 readiness workflow |
| Non-critical warnings do not block readiness by default where L2-G02 permits continued monitoring. | Laboratory Management | Warning may lead to a governed monitored readiness outcome. | L2-G02 readiness workflow |
| A critical safety exception requires the source-defined senior institutional decision. | Laboratory Management | L3 cannot bypass or reassign the critical-safety authority. | L2-G02 readiness workflow |
| External canonical information is referenced rather than re-owned. | Respective canonical owners | Preserves domain boundaries. | L2-G02; DEPENDENCY_MAP.md |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Laboratory | `ACTIVE`, `RESTRICTED`, or `SUSPENDED` as applicable; `DECOMMISSIONED` is terminal | Readiness assessment may confirm the current operational position or support a source-defined status consequence; it does not invent a new Laboratory state. | Existing source-supported Laboratory operational position | Laboratory Management | L2-G02 Laboratory Operational Status lifecycle |
| Laboratory Readiness Profile | Source-supported readiness position | Readiness evidence is assessed and maintained for operational monitoring. | `MONITORING` where defined by the L2 readiness workflow | Laboratory Management | L2-G02 readiness workflow |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Scenario Purpose | Source |
|---|---|---|---|---|
| LaboratoryReadinessReviewed | Laboratory Management | Authorized interested domains | Communicates completion of the Laboratory readiness assessment. | L2-G02 |
| LaboratoryOperationalStatusChanged | Laboratory Management | Authorized interested domains | Communicates an operational-status change where the readiness outcome causes one. | L2-G02 |

No `LaboratoryPrepared` or `LaboratoryReady` event is retained because those event names are not the canonical readiness events established by the current L2-G02 source.

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Infrastructure Management / Section G physical-space owner | Laboratory Management | Authoritative Infrastructure Space reference | Establish the physical Laboratory context without duplicate ownership. | DEPENDENCY_MAP.md; L2-G02 |
| Academic Delivery | Laboratory Management | Authoritative academic/practical-use context where required | Relate readiness to planned academic use without transferring academic ownership. | L2-G02 |
| People & Workforce | Laboratory Management | Authoritative workforce/assignment context where required | Establish responsible workforce context without re-owning workforce data. | L2-G02 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Prepare and validate Laboratory readiness evidence | Laboratory In-Charge | Authority is limited to the Laboratory Management responsibility established by L2-G02. | Laboratory Management |
| Decide a critical safety exception | Senior Institutional Authority | Applies only where the critical-safety exception defined by L2-G02 is present. | Laboratory Management |

### 15.2 Separation of Duties, Overrides and Exceptions

No additional separation-of-duty or override rule is introduced at L3. The only exception authority retained here is the critical-safety authority explicitly defined by L2-G02.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Readiness assessment | Safety, equipment and utility readiness evidence and resulting readiness position. | Laboratory Management |
| Critical safety exception, where applicable | Exception evidence and the source-required senior institutional decision. | Laboratory Management |
| Operational-status consequence, where applicable | Prior and resulting source-supported Laboratory operational position. | Laboratory Management |

Section A retains ownership of platform Authorization and Audit infrastructure; Laboratory Management owns the underlying readiness decision and evidence.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Safety, equipment and utility readiness have been assessed under L2-G02.
- Any warning or critical safety exception has been handled using the source-defined business path.
- `LaboratoryReadinessReviewed` is published by Laboratory Management.
- Any operational-status change uses only the source-supported Laboratory lifecycle and event.

### 16.2 Postconditions

- The governed Laboratory readiness position is available for subsequent authorized Laboratory operations.
- Participating domains retain ownership of their canonical information.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The scenario previously claimed Architecture Approval under `ACR-G-2026-08-21-002`, but no independently traceable governance/ADR/approval artifact establishing that scenario approval has been located in the repository. | Source-supported L2-G02 actors, readiness rules, lifecycle and events remain usable, but the scenario cannot rely on the unsupported Architecture Approved claim. | Provide independently traceable approval provenance before promotion beyond Review. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Authoring Validation Classification:** REVIEW REQUIRED  
**Validation Note:** Generic readiness boilerplate, non-canonical readiness events and unsupported approval provenance were corrected against locked L2-G02; frozen ID, title, owner, participating domains, type and priority were preserved.
