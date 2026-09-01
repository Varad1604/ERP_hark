# L4-E-007 — Workforce Engagement Closure and Institutional Exit

**Scenario ID:** L4-E-007  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-007 — Workforce Engagement Closure and Institutional Exit  
**Primary Owner:** Workforce & Engagement  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-007. Workforce & Engagement owns Workforce Engagement closure; Position & Assignment remains E02-owned.

## 2. System Behavior Purpose
Specify observable behavior for authoritatively closing an active Workforce Engagement after an authorised institutional separation condition takes effect.

## 3. System Behavior Outcome
The Workforce Engagement reaches Closed through WF-013, historical continuity is preserved, and `WorkforceEngagementClosed` is published.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
An authorised institutional separation condition exists for an active Workforce Engagement.

### 4.2 End Boundary
The Workforce Engagement is authoritatively Closed and the closure fact is published.

### 4.3 In Scope
- Validate supported separation condition.
- Execute Workforce Engagement closure.
- Preserve Workforce Profile and history.
- Coordinate E02 assignment consequences.
- Publish canonical closure event.

### 4.4 Out of Scope
Recruitment/re-engagement, payroll settlement, leave administration, Position Assignment ownership, Responsibility Assignment ownership, technical identity deactivation, physical deletion or archival implementation.

## 5. Ownership and Participating Domains
Workforce & Engagement owns closure. Position & Assignment owns Position Assignment and Responsibility Assignment consequences. Downstream domains consume the closure fact.

## 6. Business Actors, Roles and Responsibilities
The applicable institutional authority establishes the authorised separation decision. Workforce & Engagement executes closure. E02 executes assignment/responsibility consequences. No unsupported technical role is introduced.

## 7. Preconditions and Entry Criteria
- Workforce Engagement is Active.
- Supported separation condition exists.
- Required business authorization is complete.
- Required E02 consequences are governed through E02.

## 8. Trigger
An authorised workforce separation condition becomes effective.

## 9. Main System Behavior
1. Identify the affected Workforce Engagement.
2. Confirm the supported separation category and authorization.
3. Execute **WF-013 — Close Workforce Engagement**.
4. Authoritatively conclude the engagement.
5. Coordinate E02-owned assignment/responsibility consequences.
6. Preserve the Workforce Profile and historical continuity.
7. Publish **`WorkforceEngagementClosed`**.

## 10. Alternate System Behaviors
- If separation does not become effective, the Workforce Engagement remains in its authoritative state.
- If a required business precondition is not satisfied, closure does not occur.
- Different supported separation categories may produce the same Closed engagement outcome.

## 11. Failure, Interruption and Business Recovery
If closure cannot validly complete, the last authoritative Workforce Engagement state remains unchanged and no closure event is asserted. No technical rollback, retry, queue, or infrastructure behavior is specified.

## 12. Business Rules and Decision Constraints
- Closed Workforce Engagements are not reopened.
- A subsequent employment relationship is a new Workforce Engagement.
- Workforce Profile remains available for historical continuity.
- E02 retains assignment/responsibility ownership.

## 13. State and Lifecycle Impact
Workforce Engagement: **Active → Closed** through WF-013. Workforce Profile remains available for historical continuity.

## 14. Business Events and Cross-Domain Collaboration
| Event | Canonical Publisher | Business Meaning |
|---|---|---|
| `WorkforceEngagementClosed` | Workforce & Engagement | Workforce Engagement has been authoritatively concluded |

## 15. Business Authorization, Approval and Audit Obligations
Separation follows policy-driven workflow governance. Closure and effective timing remain auditable. No unsupported approver or technical control is introduced.

## 16. Completion Criteria and Postconditions
- Workforce Engagement is Closed.
- `WorkforceEngagementClosed` is published.
- Workforce Profile and historical workforce facts remain preserved.
- E02-owned consequences are handled through E02.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, outcome, lifecycle, event publisher, or boundary was identified in the locked L3 source.

**Authoring classification: PASS — source-grounded Draft.**
