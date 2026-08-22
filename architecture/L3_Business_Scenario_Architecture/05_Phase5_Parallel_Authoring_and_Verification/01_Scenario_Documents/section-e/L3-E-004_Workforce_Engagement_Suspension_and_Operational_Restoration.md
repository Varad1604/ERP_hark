# Workforce Engagement Suspension and Operational Restoration

**Scenario ID:** L3-E-004  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Workforce & Engagement  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-20  

## 1. Scenario Metadata
Frozen identity is preserved from the Section E canonical register. This scenario applies locked L2-E01. Workforce Engagement remains the canonical aggregate; operational availability is a separate Workforce-owned lifecycle.

## 2. Scenario Purpose
Govern a temporary suspension of workforce operational availability while preserving the underlying Workforce Engagement and its historical continuity, and subsequently restore normal operational availability when the suspension condition is resolved.

## 3. Business Outcome
The workforce member's operational availability is authoritatively changed to **Suspended**, without closing the Workforce Engagement, and later restored to **Available** through the canonical resume workflow. The applicable suspension and restoration business facts are published.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An Active workforce member becomes subject to an authorised suspension condition.

### 4.2 End Boundary
The suspension is resolved and operational availability is restored to Available.

### 4.3 In Scope
- Suspension of workforce operational availability.
- Preservation of Workforce Engagement continuity.
- Suspension-related business record/history.
- Restoration of operational availability.
- Canonical suspension/resumption events.
- Coordination of E02 assignment/responsibility consequences where required.

### 4.4 Out of Scope
- Termination/closure of Workforce Engagement.
- Recruitment.
- Leave administration and leave accounting.
- Position Assignment ownership.
- Responsibility Assignment ownership.
- Payroll processing.
- Identity implementation.

### 4.5 Related Scenarios
E005 Workforce Deputation Lifecycle; E006 Workforce Long-Leave Lifecycle and Operational Availability; E007 Workforce Engagement Closure and Institutional Exit.

## 5. Ownership and Participating Domains
Workforce & Engagement owns Workforce Engagement and operational availability. L2-E01 explicitly states that suspension changes operational availability while preserving employment continuity. Position Assignment and Responsibility Assignment lifecycle changes remain L2-E02-owned. fileciteturn228file0L2-L2

## 6. Business Actors, Roles and Responsibilities
- Workforce & Engagement owns the suspension/restoration lifecycle.
- Applicable institutional authority supplies the authorised business decision under Policy/Workflow governance.
- E02 performs any Position Assignment or Responsibility Assignment state change that is required by the suspension; E01 coordinates/consumes the result.
- No technical IAM/RBAC role is introduced.

## 7. Preconditions and Entry Criteria
- Workforce Engagement is Active.
- A valid suspension business condition exists.
- Applicable policy/workflow authorization is complete.
- Required organisational/workforce context is authoritative.

## 8. Trigger
An authorised suspension decision is completed for an active workforce member.

## 9. Main Business Flow
1. Identify the Active Workforce Engagement.
2. Validate the suspension condition and applicable governance.
3. Execute **WF-007 — Suspend Workforce**.
4. Set operational availability to **Suspended**.
5. Create the suspension business record.
6. Coordinate required E02 assignment/responsibility consequences without taking E02 ownership.
7. Publish **`WorkforceSuspended`**.
8. When the suspension condition is resolved, execute **WF-008 — Resume Workforce**.
9. Restore operational availability to **Available**.
10. Publish **`WorkforceResumed`**.
11. Preserve complete business history.

## 10. Alternate Business Flows
- If suspension authorization is rejected, operational availability remains unchanged.
- If the suspension condition remains unresolved, the Workforce Engagement remains active but operational availability remains Suspended.
- If restoration is not yet authorized, the Suspended position remains authoritative.
- A separation condition is handled by E007, not by treating suspension as closure.

## 11. Failure, Interruption and Business Recovery
A business suspension/restoration that cannot validly complete leaves the last authoritative operational-availability position unchanged. Workforce Engagement continuity is preserved. No technical retry, queue or rollback behavior is defined at L3.

## 12. Business Rules and Decision Constraints
- Suspension changes operational availability while preserving Workforce Engagement continuity.
- Workforce operational availability is independent of the engagement lifecycle.
- Completed business history is immutable.
- Successful workflows publish authoritative business events.
- E02 owns Position Assignment and Responsibility Assignment state changes.
- Business approvals are policy-driven.

## 13. State and Lifecycle Impact
**Operational Availability:** Active/Available → **Suspended** through WF-007; **Suspended → Available** through WF-008.

The Workforce Engagement itself remains active; suspension does not close the engagement.

## 14. Business Events and Cross-Domain Collaboration
| Event | Canonical Publisher | Business Meaning |
|---|---|---|
| `WorkforceSuspended` | Workforce & Engagement | Operational availability became Suspended |
| `WorkforceResumed` | Workforce & Engagement | Operational availability was restored |

L2-E01 explicitly defines both events and their resulting availability positions. fileciteturn228file0L2-L2

## 15. Business Authorization, Approval and Audit Obligations
Suspension/restoration follows institutional policy and Workflow Engine governance. The workflow must remain fully auditable; business-critical operations retain the applicable justification and approval evidence. Technical authorization implementation is outside L3.

## 16. Completion Criteria and Postconditions
- Suspension: operational availability is Suspended and `WorkforceSuspended` is published.
- Restoration: operational availability is Available and `WorkforceResumed` is published.
- Workforce Engagement remains historically continuous.
- Any E02 assignment changes remain E02-owned.

## 17. Open Issues, Assumptions and Deferred Questions
No material source gap identified for the core suspension/restoration lifecycle in the retrieved locked L2-E01 material.

**Authoring classification: PASS — source-grounded.**
