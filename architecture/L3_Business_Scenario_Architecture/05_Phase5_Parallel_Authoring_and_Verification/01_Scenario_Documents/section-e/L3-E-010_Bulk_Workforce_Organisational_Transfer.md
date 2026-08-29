# Bulk Workforce Organisational Transfer

**Scenario ID:** L3-E-010
**Primary Owner:** Workforce & Engagement
**Participating Domains:** B, E
**Scenario Type:** Critical
**Priority:** P1
**Architecture Level:** L3 — Business Scenario Architecture
**Status:** Locked  
**Version:** 1.0
**Owner:** Varad
**Last Updated:** 2026-08-20

**Depends On:** L2-E01 — Workforce & Engagement Workflows; L2-E02 — Position & Assignment Workflows; Organisation Management
**Referenced By:** Future L4 System Behavior Specification

## 1. Scenario Metadata
Frozen Section E identity is preserved. This scenario applies the locked E01/E02 architecture without transferring ownership.

## 2. Scenario Purpose
Complete an authorised organisational movement affecting a defined workforce population while preserving Workforce Engagement continuity, canonical ownership, effective dating and history.

## 3. Business Outcome
All eligible workforce movements in the governed transfer set reach their authoritative target organisational context, corresponding Position Assignment outcomes remain E02-owned, and `BulkTransferCompleted` is published.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An authorised bulk organisational transfer and affected workforce population exist.

### 4.2 End Boundary
The governed transfer set has reached its authoritative outcome and `BulkTransferCompleted` has been published.

### 4.3 In Scope
- Authorised workforce population definition.
- Source/target organisation validation.
- Canonical bulk workforce transfer.
- E02-owned assignment consequences.
- Effective dating and history.
- Completed bulk-transfer event.

### 4.4 Out of Scope
Position Definition lifecycle, Workforce Engagement creation/closure, payroll, leave administration, and technical batch mechanics.

### 4.5 Related Scenarios
E003 — Workforce Transfer Between Organisational Contexts; E011 — Workforce Member Assignment to Active Institutional Position; E012 — Position Assignment Change or Reassignment.

## 5. Ownership and Participating Domains
Workforce & Engagement owns Workforce Engagement transfer. Organisation Management owns organisational structure. Position & Assignment owns Position Assignment lifecycle. Participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities
The authorised institutional business authority governs the transfer decision; Workforce & Engagement executes the workforce outcome; Position & Assignment executes assignment lifecycle consequences; Organisation Management supplies authoritative organisational context. No technical role is invented.

## 7. Preconditions and Entry Criteria
- Affected Workforce Engagements exist.
- Source and target organisational contexts are authoritative.
- Transfer authority exists.
- Applicable transfer conditions are satisfied.

## 8. Trigger
An authorised bulk organisational transfer becomes ready for execution.

## 9. Main Business Flow
1. Identify the authorised transfer population.
2. Validate source and target organisational contexts.
3. Validate each affected Workforce Engagement.
4. Execute **WF-017 — Bulk Workforce Transfer**.
5. Establish the target organisational outcome for each eligible engagement.
6. Coordinate required Position Assignment changes through E02.
7. Preserve effective dates and historical continuity.
8. Publish **WDE-502 — `BulkTransferCompleted`** after the completed bulk-transfer outcome.
9. Make the completed business fact available to dependent domains.

## 10. Alternate Business Flows
- An ineligible workforce member does not receive the transfer outcome.
- An invalid target organisation prevents the affected transfer.
- E01 does not perform E02-owned Position Assignment lifecycle changes.

## 11. Failure, Interruption and Business Recovery
If a transfer cannot validly complete, the affected member remains in the last authoritative organisational position. Historical information is preserved. Technical retry/rollback behaviour is outside L3.

## 12. Business Rules and Decision Constraints
- Workforce Engagement remains E01-owned.
- Organisation structure remains Organisation-owned.
- Position Assignment remains E02-owned.
- Transfer preserves Workforce Engagement continuity.
- Business-critical transfer requires business justification.
- Successful Workforce workflows publish completed business facts.

## 13. State and Lifecycle Impact
Workforce Engagement remains in its E01 lifecycle. Any Position Assignment lifecycle consequence is governed by E02.

## 14. Business Events and Cross-Domain Collaboration
| Event ID | Event | Publisher |
|---|---|---|
| WDE-502 | `BulkTransferCompleted` | Workforce & Engagement |

## 15. Business Authorization, Approval and Audit Obligations
Bulk transfer follows applicable institutional transfer governance. Approval and business justification remain governed upstream and auditable.

## 16. Completion Criteria and Postconditions
- Authorised population processed.
- Eligible workforce members have authoritative target context.
- E02 assignment outcomes are complete.
- History is preserved.
- `BulkTransferCompleted` is published.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, outcome, lifecycle, event publisher or boundary was identified in the authoritative sources read for this batch.

**Authoring classification: PASS — source-grounded production-quality Draft.**
