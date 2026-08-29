# Position Assignment Completion or Termination

**Scenario ID:** L3-E-013  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Position & Assignment  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E02 — Position & Assignment Workflows  
**Referenced By:** Future L4 System Behavior Specification

## 1. Scenario Metadata
Frozen Final L3 identity, owner, participating domains, type and priority are preserved from the authoritative Phase 4/5 sources.

## 2. Scenario Purpose
Bring an operational Position Assignment to its authoritative completed/terminated business outcome while preserving assignment identity, immutable history and organisational traceability.

## 3. Business Outcome
The Position Assignment has ceased to be operational through its authoritative completion/closure outcome, with its historical lineage preserved.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An active Position Assignment is subject to an authorised completion or termination decision.

### 4.2 End Boundary
The Position Assignment reaches the authoritative Closed outcome; subsequent archival remains a separate lifecycle outcome.

### 4.3 In Scope
- Validate the existing Position Assignment and completion condition.
- Apply the canonical close/termination lifecycle.
- End organisational occupancy and operational authority as governed by the lifecycle.
- Preserve immutable assignment history and lineage.
- Publish the completed closure fact.

### 4.4 Out of Scope
Only adjacent outcomes owned by another canonical scenario/domain or implementation-level behaviour are excluded.

### 4.5 Related Scenarios
Where materially relevant, related canonical scenarios are identified without changing their ownership.

## 5. Ownership and Participating Domains
| Domain | Responsibility |
|---|---|
| Position & Assignment | Owns Position Assignment completion/termination and lifecycle history |

## 6. Business Actors, Roles and Responsibilities
The relevant Position Assignment exists and is in an operational lifecycle state from which closure is permitted. Any required upstream business decision or approval is complete.

No technical access control/technical identity profiles is introduced.

## 7. Preconditions and Entry Criteria
An authorised business decision or lifecycle condition requires the Position Assignment to be completed or terminated.

## 8. Trigger
| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Identify the Position Assignment and validate the completion condition. | Position & Assignment | Assignment is eligible for completion. |
| 2 | Apply the canonical **CloseAssignment** lifecycle operation. | Position & Assignment | Assignment reaches its Closed lifecycle outcome. |
| 3 | End the assignment's operational applicability and preserve its historical lineage. | Position & Assignment | Organisational history remains reconstructable. |
| 4 | Publish **`PositionAssignmentClosed`** as the completed business fact. | Position & Assignment | Closure is available to interested domains. |

## 9. Main Business Flow
No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

The flow stops when the Business Outcome and End Boundary are reached.

## 10. Alternate Business Flows
| Situation | Business Consequence | Business Resolution | Responsible Domain |
|---|---|---|---|
| Completion condition is not satisfied | Assignment cannot be validly completed | Retain the current authoritative lifecycle position until the business condition is resolved | Position & Assignment |
| Required governance is not satisfied | Closure cannot proceed | Follow the applicable upstream governance path | Position & Assignment |

## 11. Failure, Interruption and Business Recovery
| Business Rule / Constraint | Owning Domain | Scenario Effect | Source |
|---|---|---|---|
| Position Assignment lifecycle transitions occur only through approved workflow operations. | Position & Assignment | Completion must use the canonical lifecycle. | L2-E02 §9.3 BR-006 |
| Historical assignment records remain immutable. | Position & Assignment | Completion cannot rewrite prior history. | L2-E02 §9.3 BR-008 |
| Archived assignments never re-enter operational processing. | Position & Assignment | Closure remains distinct from later archival. | L2-E02 §9.3 BR-010 |
| Only active Position Assignments establish organisational authority. | Position & Assignment | Completion removes the assignment from active authority. | L2-E02 §9.3 BR-009 |

Only business-level resolution is described.

## 12. Business Rules and Decision Constraints
| Aggregate | Relevant Entry State | Scenario Impact | Resulting State | Owner | Source |
|---|---|---|---|---|---|
| Position Assignment | Active | Canonical CloseAssignment operation completes the assignment | Closed | Position & Assignment | L2-E02 §6.5.5–6.5.7 |

Rules remain owned by the authoritative upstream domain.

## 13. State and Lifecycle Impact
| Business Event | Canonical Publisher | Purpose |
|---|---|---|
| `PositionAssignmentClosed` | Position & Assignment | Records completed Position Assignment closure. |

No new lifecycle state or transition is created by this scenario.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events
No material synchronous contract or governed cross-domain hand-off beyond the authoritative Position Assignment business fact is required to define the L3 outcome.

### 14.2 Business Contracts and Cross-Domain Hand-offs
The completion decision follows the Position & Assignment governance and approval obligations defined upstream. Exact approving roles are not invented where the locked L2 material does not define a scenario-specific actor for this operation.

## 15. Business Authorization, Approval and Audit Obligations
- The Position Assignment is Closed.
- Operational authority no longer derives from the closed assignment.
- Historical assignment lineage remains preserved.
- `PositionAssignmentClosed` is published.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
**Open issue:** The locked L2 aggregate explicitly defines `CloseAssignment`, the Closed state, and `PositionAssignmentClosed`, but the workflow catalogue does not provide a separate detailed Close Position Assignment workflow specification. This does not prevent the core L3 outcome from being grounded, but detailed closure sequencing should remain an upstream/L4 verification point.

### 16.2 Postconditions
The completed business outcome remains authoritative and available for subsequent institutional behaviour according to upstream architecture.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
Interpretation is limited to the explicit locked aggregate lifecycle semantics; no missing detailed workflow sequence is invented.

### 17.2 Assumptions
No scenario-specific deferred questions.

### 17.3 Deferred Questions
No scenario-specific deferred questions.

**Authoring classification:** PASS WITH ISSUES — source-grounded Draft.
