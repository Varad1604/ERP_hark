# L4-E-006 — Workforce Long-Leave Lifecycle and Operational Availability

**Scenario ID:** L4-E-006  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-006 — Workforce Long-Leave Lifecycle and Operational Availability  
**Primary Owner:** Workforce & Engagement  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-006. Workforce & Engagement owns operational availability; Leave Management remains authoritative for leave administration and accounting.

## 2. System Behavior Purpose
Specify observable behavior for initiating and completing governed long-duration workforce leave while preserving Workforce Engagement continuity.

## 3. System Behavior Outcome
Long-duration unavailability is established for an active workforce member, `LongLeaveStarted` is published, and on governed completion normal operational availability is restored with `LongLeaveEnded` published.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
An authorised long-duration leave decision becomes effective for an active Workforce Engagement.

### 4.2 End Boundary
The long leave is completed and normal operational availability is restored.

### 4.3 In Scope
- Identify the active Workforce Engagement.
- Verify the authorised long-leave outcome.
- Initiate long leave.
- Establish operational unavailability.
- Complete long leave.
- Restore operational availability.
- Publish canonical long-leave events.
- Preserve Workforce Engagement continuity and history.

### 4.4 Out of Scope
- Leave accounting, balances, encashment, or approval policy definition.
- Payroll.
- Workforce Engagement closure.
- Position Assignment or Responsibility Assignment ownership.

## 5. Ownership and Participating Domains
Workforce & Engagement owns workforce operational availability. Leave Management owns leave administration. E02 owns assignment/responsibility consequences.

## 6. Business Actors, Roles and Responsibilities
The applicable institutional authority establishes the governed leave decision. Workforce & Engagement executes the operational-availability consequence. Leave Management remains authoritative for leave administration. No technical role is introduced.

## 7. Preconditions and Entry Criteria
- Workforce Engagement is Active.
- An authorised long-duration leave decision exists.
- The applicable organisational/workforce context is authoritative.

## 8. Trigger
An authorised long-duration leave decision becomes effective.

## 9. Main System Behavior
1. Identify the active Workforce Engagement.
2. Verify the authorised long-leave outcome.
3. Execute **WF-011 — Initiate Long Leave**.
4. Establish the long-leave operational-availability condition.
5. Publish **`LongLeaveStarted`**.
6. Preserve Workforce Engagement continuity.
7. On governed completion, execute **WF-012 — Complete Long Leave**.
8. Restore normal operational availability.
9. Publish **`LongLeaveEnded`**.
10. Preserve the complete business history.

## 10. Alternate System Behaviors
- If leave is not authorised, no long-leave operational state is established.
- While the leave condition remains active, the workforce member remains unavailable.
- If completion is not yet effective, the existing long-leave position remains authoritative.
- Leave administration remains with Leave Management.

## 11. Failure, Interruption and Business Recovery
If the operational-availability transition cannot validly complete, the last authoritative workforce position remains unchanged. Leave-administration corrections remain with Leave Management. No technical retry or infrastructure behavior is specified.

## 12. Business Rules and Decision Constraints
- Long leave does not close the Workforce Engagement.
- Operational availability is distinct from engagement lifecycle.
- Supported long-leave categories are those established upstream.
- Successful workflows publish the canonical business events.
- Business history is preserved.

## 13. State and Lifecycle Impact
Operational Availability: normal availability → long-leave unavailability through WF-011 → normal availability through WF-012. Workforce Engagement remains continuous.

## 14. Business Events and Cross-Domain Collaboration
| Event | Canonical Publisher | Business Meaning |
|---|---|---|
| `LongLeaveStarted` | Workforce & Engagement | Long-duration workforce unavailability began |
| `LongLeaveEnded` | Workforce & Engagement | Long-duration workforce unavailability ended |

## 15. Business Authorization, Approval and Audit Obligations
The underlying leave decision follows institutional policy/workflow governance. Operational-availability changes are auditable and effective-dated. No unsupported approver or technical enforcement mechanism is introduced.

## 16. Completion Criteria and Postconditions
- Long leave is established and `LongLeaveStarted` is published.
- Workforce Engagement remains continuous.
- On completion, normal operational availability is restored and `LongLeaveEnded` is published.
- Leave administration remains Leave Management-owned.

## 17. Open Issues, Assumptions and Deferred Questions
No material source gap affecting the core long-leave operational lifecycle was identified in the locked L3 source.

**Authoring classification: PASS — source-grounded Draft.**
