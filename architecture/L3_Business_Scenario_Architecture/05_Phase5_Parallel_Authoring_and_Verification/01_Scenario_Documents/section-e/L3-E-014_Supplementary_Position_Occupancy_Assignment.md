# Supplementary Position Occupancy Assignment

**Scenario ID:** L3-E-014  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Position & Assignment  
**Participating Domains:** B, E  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E02 — Position & Assignment Workflows; Organisation/Position Definition architecture; Workforce Engagement architecture  
**Referenced By:** Future L4 System Behavior Specification

## 1. Scenario Metadata
Frozen Final L3 identity, owner, participating domains, type and priority are preserved from the authoritative Phase 4/5 sources.

## 2. Scenario Purpose
Establish a supplementary organisational appointment for an active Workforce Engagement while preserving the canonical Primary Assignment and clear assignment precedence.

## 3. Business Outcome
The supplementary Position Assignment is established as an additional organisational occupancy, while the existing Primary Assignment remains authoritative for enterprise default behaviour.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An active Workforce Engagement has an existing Primary Assignment and is authorised for a supplementary Position occupancy.

### 4.2 End Boundary
The supplementary Position Assignment reaches its applicable active outcome and coexists with the Primary Assignment without replacing it.

### 4.3 In Scope
- Validate the active Workforce Engagement and Primary Assignment.
- Validate the target Position and occupancy eligibility.
- Establish the supplementary Position Assignment through the canonical Additional Assignment workflow.
- Preserve assignment precedence and enterprise default behaviour.
- Publish the applicable additional-assignment business facts.

### 4.4 Out of Scope
Only adjacent outcomes owned by another canonical scenario/domain or implementation-level behaviour are excluded.

### 4.5 Related Scenarios
Where materially relevant, related canonical scenarios are identified without changing their ownership.

## 5. Ownership and Participating Domains
| Domain | Responsibility |
|---|---|
| Position & Assignment | Owns supplementary Position Assignment lifecycle and precedence. |
| Organisation / Position Definition owner | Supplies the authoritative target Position Definition. |
| Workforce & Engagement | Supplies the active Workforce Engagement. |

## 6. Business Actors, Roles and Responsibilities
- Workforce Engagement is Active.
- A Primary Assignment exists.
- Target Position is active and eligible.
- Institutional policy permits an additional assignment.
- Assignment precedence is defined and applicable governance is satisfied.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria
An authorised administrator establishes an additional organisational appointment for an already engaged workforce member.

## 8. Trigger
| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Validate the existing Primary Assignment and target Position. | Position & Assignment | Supplementary occupancy is eligible. |
| 2 | Establish the additional Position Assignment through **W-009 — Additional Position Assignment**. | Position & Assignment | Additional occupancy is created. |
| 3 | Preserve the Primary Assignment and establish assignment precedence. | Position & Assignment | Enterprise default assignment remains deterministic. |
| 4 | Publish **`AdditionalAssignmentCreated`** and applicable precedence facts. | Position & Assignment | Supplementary appointment is an authoritative business fact. |

## 9. Main Business Flow
| Case | Condition | Alternate Path | Result |
|---|---|---|---|
| Future supplementary assignment | Effective date is in the future | Schedule the additional assignment for its effective boundary | Primary Assignment remains current until effective date |
| Temporary supplementary assignment | Appointment is time-bound | Govern validity through the applicable effective-dated lifecycle | Supplementary occupancy remains bounded by its approved period |

The flow stops when the Business Outcome and End Boundary are reached.

## 10. Alternate Business Flows
| Situation | Business Consequence | Business Resolution | Responsible Domain |
|---|---|---|---|
| No Primary Assignment | Supplementary appointment cannot be established under W-009 | Reject until a Primary Assignment exists | Position & Assignment |
| Target Position capacity/policy invalid | Supplementary occupancy cannot be established | Reject and retain existing Primary Assignment | Position & Assignment |
| Assignment precedence is ambiguous | Enterprise default behaviour cannot be determined safely | Resolve under the authoritative assignment policy before activation | Position & Assignment |

## 11. Failure, Interruption and Business Recovery
| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Every Workforce Engagement has exactly one Primary Assignment. | Position & Assignment | Supplementary assignment cannot create a second Primary. | L2-E02 W-009 / BR-W009-001 |
| Additional Assignments do not replace the Primary Assignment. | Position & Assignment | Existing Primary remains canonical default. | L2-E02 W-009 / BR-W009-002 |
| Assignment precedence is resolved deterministically. | Position & Assignment | Supplementary occupancy has governed precedence. | L2-E02 W-009 / BR-W009-003–004 |
| Closing an Additional Assignment does not invalidate the Primary Assignment. | Position & Assignment | Supplementary lifecycle is independent. | L2-E02 W-009 / BR-W009-005 |

Only business-level resolution is described.

## 12. Business Rules and Decision Constraints
| Aggregate | Entry | Impact | Result | Owner |
|---|---|---|---|---|
| Position Assignment | Active Primary context | Establish supplementary occupancy | Additional Position Assignment reaches Active/Scheduled outcome according to effective date | Position & Assignment |

Rules remain owned by the authoritative upstream domain.

## 13. State and Lifecycle Impact
| Event | Publisher | Purpose |
|---|---|---|
| `AdditionalAssignmentCreated` | Position & Assignment | Records supplementary appointment creation. |
| `AdditionalAssignmentEnded` | Position & Assignment | Records later supplementary completion, where applicable. |
| `AssignmentPriorityChanged` | Position & Assignment | Records a governed precedence change, where applicable. |
| `EnterpriseDefaultAssignmentChanged` | Position & Assignment | Records a changed enterprise default, where applicable. |

No new lifecycle state or transition is created by this scenario.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events
Organisation supplies Position Definition information and Workforce & Engagement supplies the active Workforce Engagement; these dependencies do not transfer Position Assignment ownership.

### 14.2 Business Contracts and Cross-Domain Hand-offs
Additional Assignment approval follows the Position & Assignment governance model and applicable institutional policy. The scenario does not invent a fixed approver or technical permission mechanism.

## 15. Business Authorization, Approval and Audit Obligations
- Additional Position Assignment exists.
- Primary Assignment remains intact.
- Assignment precedence is deterministic.
- Supplementary occupancy is effective for its approved period.
- Canonical additional-assignment facts are published.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
No open issues.

### 16.2 Postconditions
The completed business outcome remains authoritative and available for subsequent institutional behaviour according to upstream architecture.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
No open issues.

### 17.2 Assumptions
No assumptions.

### 17.3 Deferred Questions
No scenario-specific deferred questions.

**Authoring classification:** PASS — source-grounded Draft.
