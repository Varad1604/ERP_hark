# Workforce Long-Leave Lifecycle and Operational Availability

**Scenario ID:** L3-E-006  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Workforce & Engagement  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-20  

## 1. Scenario Metadata
Frozen Section E identity is preserved. The authoritative lifecycle is L2-E01 WF-011 / WF-012.

## 2. Scenario Purpose
Govern long-duration workforce unavailability as an operational-availability lifecycle while preserving the Workforce Engagement and returning the workforce member to normal availability when the leave ends.

## 3. Business Outcome
A governed long-duration leave is recorded and operational availability changes appropriately; when the leave is completed, normal operational availability is restored and the canonical leave lifecycle facts are published.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An authorised long-duration leave decision is established for an active workforce member.

### 4.2 End Boundary
The long leave is completed and operational availability returns to normal.

### 4.3 In Scope
- Initiation of long-duration leave.
- Operational availability management.
- Supported long-leave categories.
- Completion of long leave.
- Restoration of operational availability.
- Canonical long-leave events.
- Workforce continuity and historical traceability.

### 4.4 Out of Scope
- Leave administration and leave accounting.
- Leave balance calculation.
- Leave approval policy definition.
- Payroll.
- Closing the Workforce Engagement.
- Position Assignment ownership.
- Responsibility Assignment ownership.

L2-E01 explicitly keeps leave calculations, balances, approvals, encashment and leave accounting outside the Workforce & Engagement domain.

### 4.5 Related Scenarios
E004 Workforce Engagement Suspension and Operational Restoration; E005 Workforce Deputation Lifecycle; E007 Workforce Engagement Closure and Institutional Exit.

## 5. Ownership and Participating Domains
Workforce & Engagement owns workforce operational availability. Leave Management owns leave administration. The Workforce domain consumes/implements the business consequence of an authorised long-leave outcome without taking ownership of leave accounting.

## 6. Business Actors, Roles and Responsibilities
- Workforce & Engagement owns the workforce operational-availability lifecycle.
- Leave Management, where involved, remains authoritative for leave administration.
- The applicable institutional authority establishes the governed leave decision through Policy/Workflow governance.
- E02 owns any Position Assignment or Responsibility Assignment changes required as a consequence.

No technical role is invented.

## 7. Preconditions and Entry Criteria
- Workforce Engagement is Active.
- Authorised long-duration leave decision exists.
- Applicable leave category is one of the supported categories.
- Applicable organisational/workforce context is authoritative.

## 8. Trigger
An authorised long-duration leave decision becomes effective.

## 9. Main Business Flow
1. Identify the active Workforce Engagement.
2. Consume/verify the authorised long-leave outcome.
3. Execute **WF-011 — Initiate Long Leave**.
4. Record the operational availability condition.
5. Publish **`LongLeaveStarted`**.
6. Preserve Workforce Engagement continuity and historical context.
7. When the governed leave period is completed, execute **WF-012 — Complete Long Leave**.
8. Restore normal operational availability.
9. Restore responsibilities according to institutional policy where applicable, through the authoritative responsibility lifecycle.
10. Publish **`LongLeaveEnded`**.
11. Preserve the complete business history.

## 10. Alternate Business Flows
- If the leave decision is not authorised, no long-leave operational state is established.
- If the applicable leave condition remains active, the workforce member remains unavailable.
- If completion is not yet authorised/effective, the long-leave position remains authoritative.
- Leave balance/accounting matters remain with Leave Management.

## 11. Failure, Interruption and Business Recovery
If the operational-availability transition cannot validly complete, the existing authoritative workforce position remains unchanged. Leave-administration corrections remain with Leave Management. Assignment/responsibility corrections remain with E02. No technical retry or infrastructure behavior is described.

## 12. Business Rules and Decision Constraints
- Long leave is an operational-availability workflow, not Workforce Engagement closure.
- Supported long-leave categories include Study Leave, Sabbatical, Extraordinary Leave, Medical Leave, Maternity and Paternity.
- Operational availability is independent of the engagement lifecycle.
- Successful workflows publish authoritative business events.
- Leave calculations, balances, approvals, encashment and accounting remain outside E01.
- Business history is immutable and corrections use governed compensating workflows.

## 13. State and Lifecycle Impact
**Operational Availability:** normal availability → long-leave unavailability through WF-011; long-leave completion → normal availability through WF-012.

**Workforce Engagement:** remains continuous; long leave does not create a new engagement or close the existing engagement.

## 14. Business Events and Cross-Domain Collaboration
| Event | Publisher | Meaning |
|---|---|---|
| `LongLeaveStarted` | Workforce & Engagement | Long-duration workforce unavailability began |
| `LongLeaveEnded` | Workforce & Engagement | Long-duration workforce unavailability ended |

Both events are explicitly defined by L2-E01 WF-011 and WF-012.

Leave Management remains responsible for leave administration; downstream consumers use the Workforce event as an authoritative workforce-availability fact.

## 15. Business Authorization, Approval and Audit Obligations
The underlying leave decision follows institutional Policy/Workflow governance and Leave Management's authoritative leave process. Workforce operational-availability changes remain auditable and effective-dated. Technical approval enforcement is outside L3.

## 16. Completion Criteria and Postconditions
- Long leave initiation is established and `LongLeaveStarted` is published.
- Workforce Engagement remains continuous.
- On completion, normal operational availability is restored and `LongLeaveEnded` is published.
- Leave administration remains Leave Management-owned.
- Historical workforce availability remains reconstructable.

## 17. Open Issues, Assumptions and Deferred Questions
No material source gap identified for the core long-leave operational lifecycle in the retrieved locked L2-E01 material.

**Authoring classification: PASS — source-grounded.**
