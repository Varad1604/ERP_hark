# L4-E-009 — Duplicate Workforce Profiles to Canonical Workforce Resolution

**Scenario ID:** L4-E-009  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-009 — Duplicate Workforce Profiles to Canonical Workforce Resolution  
**Primary Owner:** Workforce & Engagement  
**Participating Domains:** A, E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-009. Workforce & Engagement owns Workforce Profile governance and controlled duplicate-profile resolution.

## 2. System Behavior Purpose
Specify observable behavior for resolving duplicate Workforce Profiles into a canonical workforce representation while preserving workforce history.

## 3. System Behavior Outcome
The governed merge resolves duplicate profiles to a canonical Workforce Profile, preserves engagement and assignment/responsibility history, and publishes `WorkforceProfilesMerged`.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
Duplicate Workforce Profiles are identified and a governed resolution decision is available.
### 4.2 End Boundary
The canonical profile resolution is complete and the merge fact is published.
### 4.3 In Scope
- Identify duplicate profiles.
- Determine canonical profile.
- Execute controlled profile merge.
- Preserve engagement, assignment, responsibility and audit history.
- Publish canonical merge event.
### 4.4 Out of Scope
Canonical Person ownership, authentication/identity accounts, Position Assignment creation/modification, Responsibility Assignment ownership, payroll, leave processing, and technical data-migration mechanics.

## 5. Ownership and Participating Domains
Workforce & Engagement owns the controlled merge workflow. Position & Assignment remains owner of assignment/responsibility history. No ownership is transferred by participation.

## 6. Business Actors, Roles and Responsibilities
The authorised institutional governance process determines the merge decision. Workforce & Engagement executes the canonical resolution. No unsupported technical role or approver is introduced.

## 7. Preconditions and Entry Criteria
- Duplicate Workforce Profiles are identified.
- Canonical profile can be determined.
- Merge is authorised.
- Associated workforce history is available for preservation.

## 8. Trigger
A governed duplicate-profile resolution decision authorises the merge.

## 9. Main System Behavior
1. Identify duplicate Workforce Profiles.
2. Determine the canonical profile.
3. Review history requiring preservation.
4. Execute **WF-016 — Merge Workforce Profiles**.
5. Consolidate the workforce representation under the canonical profile.
6. Preserve engagement history.
7. Preserve Position Assignment and Responsibility history.
8. Preserve audit history.
9. Publish **`WorkforceProfilesMerged`**.

## 10. Alternate System Behaviors
- If duplicate status is not confirmed, no merge occurs.
- If canonical profile cannot be determined, profiles remain authoritative pending resolution.
- If governance rejects the merge, no merge outcome is completed.

## 11. Failure, Interruption and Business Recovery
If the governed merge cannot validly complete, existing authoritative records and relationships remain unchanged. No technical deletion, rollback, migration or retry behavior is specified.

## 12. Business Rules and Decision Constraints
- Duplicate profiles for the same canonical person are resolved through controlled Workforce governance.
- Workforce Profiles are not deleted as a business lifecycle action.
- Engagement, assignment, responsibility and audit history must be preserved.
- Workforce & Engagement publishes the merge fact.

## 13. State and Lifecycle Impact
The outcome is canonical Workforce Profile resolution; no new Workforce Engagement lifecycle is created and no unsupported profile state transition is introduced.

## 14. Business Events and Cross-Domain Collaboration
| Event | Canonical Publisher | Business Meaning |
|---|---|---|
| `WorkforceProfilesMerged` | Workforce & Engagement | Duplicate Workforce Profiles have been resolved through the canonical merge workflow |

## 15. Business Authorization, Approval and Audit Obligations
Profile merging is governed and auditable. The resolution decision and preserved histories remain traceable. No unsupported approval mechanism is specified.

## 16. Completion Criteria and Postconditions
- Canonical Workforce Profile is established.
- Engagement, assignment, responsibility and audit history are preserved.
- `WorkforceProfilesMerged` is published.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, outcome, lifecycle, event publisher, or boundary was identified in the locked L3 source.

**Authoring classification: PASS — source-grounded Draft.**
