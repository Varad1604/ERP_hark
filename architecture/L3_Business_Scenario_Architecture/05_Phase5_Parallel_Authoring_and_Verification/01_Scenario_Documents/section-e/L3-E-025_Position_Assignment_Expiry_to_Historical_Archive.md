# Position Assignment Expiry to Historical Archive

**Scenario ID:** L3-E-025  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Position & Assignment  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Author:** Varad  
**Version:** 1.0  
**Date:** 2026-08-20  
**Depends On:** L2-E02 — Position & Assignment Workflows  
**Referenced By:** Future L4 System Behavior Specification

## 1. Scenario Metadata
This scenario preserves the frozen canonical identity, title, owner, participating domains, Scenario Type and Priority. It applies the locked upstream architecture without transferring ownership or introducing new architecture.

## 2. Scenario Purpose
Complete the operational lifecycle of a Position Assignment through expiry and then transition the completed assignment into its immutable historical archive.

## 3. Business Outcome
The Position Assignment reaches Expired and subsequently Archived through the canonical lifecycle, with complete historical lineage preserved and the archived assignment excluded from operational processing.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An active Position Assignment has reached its authoritative expiry condition.

### 4.2 End Boundary
The Position Assignment is Archived and remains available as immutable historical information without participating in operational processing.

### 4.3 In Scope
- Evaluate the Position Assignment's authoritative expiry condition.
- Apply the canonical Assignment Expiry workflow.
- Preserve organisational occupancy history and assignment lineage.
- Once the assignment is Expired, apply the canonical Archive Position Assignment workflow.
- Preserve the archived assignment as queryable historical information.

### 4.4 Out of Scope
Implementation mechanics, APIs, databases, services, UI, infrastructure, technical access control/IAM, queues, technical retries and other L4+ concerns are outside L3.

### 4.5 Related Scenarios
Only canonical dependencies relevant to this outcome are referenced; no new scenario ownership is created.

## 5. Ownership and Participating Domains
**Canonical owner:** Position & Assignment.

Participating domains remain authoritative for their own concepts and provide business context only within those ownership boundaries. Participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities
- Position & Assignment lifecycle authority.
- Authorised institutional governance administrator where lifecycle governance requires administrative action.
- Downstream business participants consume the resulting business facts; they do not own the lifecycle.

No technical permission model is introduced.

## 7. Preconditions and Entry Criteria
- Position Assignment exists.
- The assignment is Active and eligible for expiry.
- The authoritative expiry condition/date has been reached.
- Archive eligibility is satisfied after expiry.

## 8. Trigger
An authoritative expiry condition is reached for the Position Assignment.

## 9. Main Business Flow
1. Identify the Position Assignment and its authoritative expiry condition.
2. Apply **W-015 — Assignment Expiry**.
3. Transition the Position Assignment from **Active** to **Expired**.
4. Publish **`PositionAssignmentExpired`**.
5. Confirm that the assignment is no longer operational.
6. When archive eligibility is satisfied, apply **W-021 — Archive Position Assignment**.
7. Transition the Position Assignment from **Expired** to **Archived**.
8. Publish **`PositionAssignmentArchived`**.
9. Preserve the complete historical lineage and auditability of the assignment.
10. Stop when the archived historical outcome is authoritative.

The flow stops when the Business Outcome and End Boundary are reached.

## 10. Alternate Business Flows
- If the assignment is extended before expiry, the canonical extension lifecycle applies rather than expiry.
- If the assignment is suspended, its suspension lifecycle remains distinct from expiry.
- An Expired assignment does not return to Active through the expiry workflow.
- Archival occurs only after the assignment has reached the canonical Expired state.

## 11. Failure, Interruption and Business Recovery
If the expiry condition is not satisfied, the Position Assignment remains in its authoritative current lifecycle state. If archive eligibility is not satisfied, the Expired assignment remains historical but is not yet Archived. No technical retry or rollback is described.

Only business-level recovery is described.

## 12. Business Rules and Decision Constraints
- W-015 is the canonical Assignment Expiry workflow.
- W-021 is the canonical Archive Position Assignment workflow.
- The lifecycle matrix defines **Active → Expired → Archived**.
- `PositionAssignmentExpired` is the canonical expiry event.
- `PositionAssignmentArchived` is the canonical archive event.
- Archived assignments remain queryable but never again participate in operational processing.
- Historical lineage remains immutable and auditable.

The rules remain owned by the authoritative upstream architecture.

## 13. State and Lifecycle Impact
**Position Assignment lifecycle:**
Active → Expired → Archived.

**Canonical workflows:**
- W-015 — Assignment Expiry
- W-021 — Archive Position Assignment

No new lifecycle state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events
| Event | Canonical Publisher | Business Meaning |
|---|---|---|
| `PositionAssignmentExpired` | Position & Assignment | Position Assignment expiry is complete. |
| `PositionAssignmentArchived` | Position & Assignment | Position Assignment archival is complete. |

### 14.2 Business Contracts / Hand-offs
Cross-domain consumers may use the completed business facts for their own responsibilities. No downstream domain acquires ownership of Position Assignment lifecycle.

Events remain distinct from synchronous business contracts and hand-offs.

## 15. Business Authorization, Approval and Audit Obligations
Expiry and archival follow the locked Position & Assignment governance model. Historical auditability and retention obligations remain preserved. No unsupported approver is invented.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Position Assignment is Expired.
- Operational participation ends.
- Position Assignment is subsequently Archived when eligible.
- Archived history remains queryable and immutable.
- Both canonical lifecycle events are published.

### 16.2 Postconditions
The completed business outcome remains authoritative for subsequent institutional behaviour and preserves required historical traceability.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
None identified.

### 17.2 Assumptions
None.

### 17.3 Deferred Questions
None.

**Authoring Classification:** PASS — source-grounded Draft.
