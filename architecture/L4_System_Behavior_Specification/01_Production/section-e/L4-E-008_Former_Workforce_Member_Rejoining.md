# L4-E-008 — Former Workforce Member Rejoining

**Scenario ID:** L4-E-008  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-008 — Former Workforce Member Rejoining  
**Primary Owner:** Workforce & Engagement  
**Participating Domains:** A, B, E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-008. A new Workforce Engagement is created beneath the existing canonical Workforce Profile; a closed engagement is not reopened.

## 2. System Behavior Purpose
Specify observable behavior for rejoining a former workforce member while preserving the existing Workforce Profile and historical engagements.

## 3. System Behavior Outcome
A new Workforce Engagement is established beneath the canonical Workforce Profile, prior engagements remain unchanged, and `WorkforceRejoined` is published.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
An authorised former workforce member is eligible to rejoin and the existing Workforce Profile is identified.
### 4.2 End Boundary
A new Workforce Engagement is established and the rejoining fact is published.
### 4.3 In Scope
- Identify canonical Workforce Profile.
- Verify authorised rejoining outcome.
- Create new Workforce Engagement.
- Preserve historical engagements.
- Publish canonical rejoining event.
- Coordinate subsequent E02 placement where required.
### 4.4 Out of Scope
Recruitment pipeline management, reopening historical engagements, Position Assignment ownership, payroll, leave administration, and technical identity/account provisioning.

## 5. Ownership and Participating Domains
Workforce & Engagement owns the re-engagement lifecycle. Position & Assignment owns any subsequent Position Assignment lifecycle.

## 6. Business Actors, Roles and Responsibilities
The applicable institutional authority establishes the authorised rejoining/appointment outcome. Workforce & Engagement establishes the new engagement. E02 owns subsequent placement. No technical role is invented.

## 7. Preconditions and Entry Criteria
- Existing canonical Workforce Profile exists.
- Historical engagements are preserved.
- Authorised rejoining/appointment outcome exists.
- Applicable policy/workflow conditions are satisfied.

## 8. Trigger
An authorised former-workforce rejoining outcome becomes effective.

## 9. Main System Behavior
1. Identify the canonical Workforce Profile.
2. Confirm the authorised rejoining outcome.
3. Execute **WF-014 — Rejoin Workforce**.
4. Retain the existing Workforce Profile.
5. Create a new Workforce Engagement beneath it.
6. Preserve all prior engagements unchanged.
7. Establish the new engagement lifecycle.
8. Publish **`WorkforceRejoined`**.
9. Continue through E02 for any required organisational placement.

## 10. Alternate System Behaviors
- If rejoining is not authorised, no new engagement is established.
- If the canonical profile cannot be confidently identified, the outcome does not proceed authoritatively.
- Historical engagements are never reopened.

## 11. Failure, Interruption and Business Recovery
If the new engagement cannot validly complete, the canonical profile and historical engagements remain unchanged. No technical retry or rollback behavior is specified.

## 12. Business Rules and Decision Constraints
- Closed Workforce Engagements are not reopened.
- Subsequent employment creates a new Workforce Engagement.
- Workforce Profile continuity is preserved.
- Position Assignment remains E02-owned.

## 13. State and Lifecycle Impact
Historical engagements retain their existing states. A new Workforce Engagement begins its own lifecycle beneath the existing Workforce Profile.

## 14. Business Events and Cross-Domain Collaboration
| Event | Canonical Publisher | Business Meaning |
|---|---|---|
| `WorkforceRejoined` | Workforce & Engagement | Former workforce member has rejoined through a new Workforce Engagement |

## 15. Business Authorization, Approval and Audit Obligations
Rejoining follows applicable policy-driven appointment/workflow governance. The decision and new engagement remain auditable.

## 16. Completion Criteria and Postconditions
- Existing Workforce Profile remains canonical.
- New Workforce Engagement exists beneath it.
- Historical engagements remain preserved.
- `WorkforceRejoined` is published.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, outcome, lifecycle, event publisher, or boundary was identified in the locked L3 source.

**Authoring classification: PASS — source-grounded Draft.**
