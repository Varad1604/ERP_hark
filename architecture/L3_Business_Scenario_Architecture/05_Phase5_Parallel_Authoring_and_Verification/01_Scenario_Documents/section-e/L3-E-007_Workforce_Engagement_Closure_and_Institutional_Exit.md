# Workforce Engagement Closure and Institutional Exit

**Scenario ID:** L3-E-007  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Workforce & Engagement  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E01 — Workforce & Engagement Workflows; L2-E02 — Position & Assignment Workflows where assignment consequences are involved  
**Referenced By:** Future L4 System Behavior Specifications and materially participating downstream workforce consumers

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-E-007 |
| Canonical Title | Workforce Engagement Closure and Institutional Exit |
| Primary Owner | Workforce & Engagement |
| Participating Domains | E |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose
Authoritatively conclude a Workforce Engagement when an approved institutional exit condition has taken effect, while preserving the Workforce Profile and complete historical workforce continuity.

## 3. Business Outcome
The active Workforce Engagement is authoritatively closed for the applicable separation reason, associated workforce history is preserved, and the canonical closure fact is published for dependent domains.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An authorised institutional separation condition exists for an active Workforce Engagement.

### 4.2 End Boundary
The Workforce Engagement has reached its authoritative closed state and the closure fact has been published.

### 4.3 In Scope
- Workforce Engagement closure.
- Institutional exit/separation categories defined by L2-E01.
- Closure of associated workforce lifecycle activity.
- Preservation of Workforce Profile and historical continuity.
- Publication of the canonical closure event.
- Business collaboration with downstream workforce consumers.
- Coordination with L2-E02 for Position Assignment/Responsibility Assignment consequences.

### 4.4 Out of Scope
- Recruitment or future re-engagement.
- Payroll calculation or final payroll settlement.
- Leave administration/accounting.
- Position Assignment ownership.
- Responsibility Assignment ownership.
- Identity implementation or technical account deactivation.
- Physical deletion or archival implementation.

### 4.5 Related Scenarios
E004 Workforce Engagement Suspension and Operational Restoration; E005 Workforce Deputation Lifecycle; E006 Workforce Long-Leave Lifecycle and Operational Availability; E008 Former Workforce Member Rejoining.

## 5. Ownership and Participating Domains
Workforce & Engagement owns Workforce Engagement closure and the workforce lifecycle. L2-E01 explicitly defines closure as a Workforce-owned lifecycle operation. Position Assignment and Responsibility Assignment remain L2-E02-owned; E01 coordinates their consequences and consumes their authoritative outcomes rather than owning those aggregates.

## 6. Business Actors, Roles and Responsibilities
- Workforce & Engagement owns the Workforce Engagement closure outcome.
- The applicable institutional authority establishes the authorised separation decision under the governed policy/workflow process.
- Position & Assignment owns any Position Assignment or Responsibility Assignment closure/state changes required as part of the exit.
- Downstream domains consume the authoritative workforce closure fact without assuming workforce ownership.
- No unsupported technical role, technical identity profiles, or approval authority is introduced.

## 7. Preconditions and Entry Criteria
- A Workforce Engagement exists and is active.
- A supported separation condition has been established.
- Applicable policy/workflow authorization is complete.
- Required upstream person and organisational references remain valid.
- Any required E02 assignment consequences are governed through E02.

## 8. Trigger
An authorised workforce separation condition becomes effective.

## 9. Main Business Flow
1. Identify the Workforce Engagement subject to institutional exit.
2. Confirm the supported separation category.
3. Confirm that the applicable business authorization is complete.
4. Execute **WF-013 — Close Workforce Engagement**.
5. Authoritatively conclude the Workforce Engagement.
6. Coordinate closure of Position Assignments and Responsibilities through their canonical E02 lifecycle where applicable.
7. Preserve the Workforce Profile for historical continuity.
8. Publish **`WorkforceEngagementClosed`**.
9. Downstream workforce consumers use the completed closure fact according to their own responsibilities.

## 10. Alternate Business Flows
- Different supported separation categories may reach the same closed Workforce Engagement outcome.
- If the separation decision is rejected, withdrawn, or otherwise does not become effective, the Workforce Engagement remains in its existing authoritative state.
- If a required business precondition is not satisfied, closure does not occur and the existing workforce state remains authoritative.

## 11. Failure, Interruption and Business Recovery
If the business closure cannot validly complete, the Workforce Engagement remains at its last authoritative state and no closure event is asserted. A correction to the separation decision follows the applicable governed business workflow. No technical rollback, retry, queue, database, or infrastructure behavior is specified at L3.

## 12. Business Rules and Decision Constraints
- Closed Workforce Engagements shall not be reopened; a subsequent employment relationship is represented by a new Workforce Engagement.
- A Workforce Profile remains available for historical continuity.
- Supported separation categories include resignation, retirement, contract completion, termination, dismissal, medical separation, death in service, and administrative separation.
- Historical records are preserved.
- Position Assignment and Responsibility Assignment ownership remains with L2-E02.
- Successful Workforce workflows publish authoritative business facts.

## 13. State and Lifecycle Impact
**Workforce Engagement:** Active → Closed through WF-013.

The Workforce Profile remains available for historical continuity. Any Position Assignment/Responsibility Assignment lifecycle effects are governed by E02 and are not re-owned by E01.

## 14. Business Events and Cross-Domain Collaboration
| Event | Canonical Publisher | Business Meaning |
|---|---|---|
| `WorkforceEngagementClosed` | Workforce & Engagement | The Workforce Engagement has been authoritatively concluded |

L2-E01 defines `WorkforceEngagementClosed` as the event for WF-013. Downstream domains consume the completed workforce fact according to their own responsibilities.

## 15. Business Authorization, Approval and Audit Obligations
Separation follows policy-driven workflow authorization. The closure must remain fully auditable, with the applicable business decision and effective timing preserved. The scenario does not invent a specific approver, separation-of-duty rule, or evidence artefact not defined upstream.

## 16. Completion Criteria and Postconditions
- Workforce Engagement is Closed.
- `WorkforceEngagementClosed` has been published.
- Workforce Profile remains available for historical continuity.
- Associated E02-owned assignment/responsibility consequences are handled by E02.
- Historical workforce facts remain preserved.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, final outcome, lifecycle, event publisher, or scenario boundary was identified in the locked L2-E01 material used for this scenario.

**Authoring classification: PASS — source-grounded.**
