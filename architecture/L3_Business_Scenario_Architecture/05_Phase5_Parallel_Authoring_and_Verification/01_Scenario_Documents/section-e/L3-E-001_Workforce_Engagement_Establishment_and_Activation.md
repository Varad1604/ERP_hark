# Workforce Engagement Establishment and Activation

**Scenario ID:** L3-E-001  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Workforce & Engagement  
**Participating Domains:** A, B, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-22  
**Authoritative L2:** Locked L2-E01 — Workforce & Engagement Workflows  

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-E-001` |
| Canonical Title | Workforce Engagement Establishment and Activation |
| Primary Owner | Workforce & Engagement |
| Participating Domains | A, B, E |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

Frozen identity is preserved. The approved E01/E02 ownership clarification remains controlling: E01 owns Workforce Engagement; E02 exclusively owns Position Assignment, Responsibility Assignment, Reporting Relationship, and Assignment History.

## 2. Scenario Purpose

Establish and activate an approved institutional Workforce Engagement so the institution has an authoritative workforce relationship for the Person and dependent domains can consume the completed activation fact.

## 3. Business Outcome

An approved Workforce Engagement reaches the canonical `Active` state from `Pending`, effective dating and history are preserved, and Workforce & Engagement publishes `WorkforceEngagementActivated`.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An approved workforce appointment/engagement decision is available with authoritative Person and organisational context.

### 4.2 End Boundary
The Workforce Engagement is `Active`, historical/effective-dated evidence is preserved, and `WorkforceEngagementActivated` has been published.

### 4.3 In Scope
- Workforce Engagement establishment.
- Activation of the approved engagement.
- Authoritative Person and organisation references.
- Effective dating and historical continuity.
- Publication of the activation business fact.

### 4.4 Out of Scope
- Recruitment/candidate processing.
- Canonical Person ownership.
- Organisation-structure ownership.
- Position/Responsibility/Reporting Relationship/Assignment History lifecycle owned by E02.
- Payroll calculation or leave administration.
- IAM implementation, APIs, databases, queues, UI, or infrastructure.

### 4.5 Related Scenarios
E002 — Active Workforce Engagement Modification; E003 — Workforce Transfer Between Organisational Contexts; E007 — Workforce Engagement Closure and Institutional Exit.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| A — Platform Foundation & Security | Provides canonical Person/foundation context; does not own Workforce Engagement. |
| B — Institution & Academic Structure | Provides authoritative institutional/organisational context; does not own Workforce Engagement. |
| E — Workforce & Engagement | Canonical owner of Workforce Engagement, its lifecycle, and activation event. |

Cross-domain participation does not transfer ownership. Any assignment aggregate remains exclusively E02-owned under the approved precedence clarification.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority |
|---|---|---|
| Workforce & Engagement | Establishes and activates the Workforce Engagement. | Canonical owner. |
| Approved appointment/engagement authority | Supplies the governed appointment decision. | Exact role/approval chain remains policy/workflow configured; no fixed title is invented. |
| Platform / organisation owners | Supply authoritative Person and organisational context. | Own their source concepts only. |

## 7. Preconditions and Entry Criteria

- Approved appointment/engagement exists.
- Canonical Person reference is available.
- Applicable institution/organisation context is authoritative.
- Applicable workforce governance/approval is satisfied.
- No competing E02 assignment is created by E01.

## 8. Trigger

An approved workforce appointment/engagement becomes ready for establishment and activation.

## 9. Main Business Flow

| Step | Business Behaviour | Owner / Responsibility | Result |
|---|---|---|---|
| 1 | Validate the approved appointment and canonical Person reference. | Workforce & Engagement | Valid engagement input established. |
| 2 | Resolve authoritative institutional/organisational context. | Workforce & Engagement using upstream-owned context | Context validated. |
| 3 | Establish the Workforce Engagement in its governed pre-activation position. | Workforce & Engagement | Engagement exists under E01 ownership. |
| 4 | Apply effective dating and approved engagement conditions. | Workforce & Engagement | Activation criteria established. |
| 5 | Transition the engagement from `Pending` to `Active`. | Workforce & Engagement | Canonical active engagement established. |
| 6 | Preserve historical/effective-dated evidence. | Workforce & Engagement | Traceability preserved. |
| 7 | Publish `WorkforceEngagementActivated`. | Workforce & Engagement | Downstream consumers can react to the completed fact. |

Any Position/Responsibility/Reporting Relationship/Assignment History change is requested from E02 and completed/published by E02 rather than performed by E01.

## 10. Alternate Business Flows

| Condition | Business Path | Result |
|---|---|---|
| Appointment approval is absent | Do not activate the engagement. | Engagement activation does not complete. |
| Person or organisation context is invalid | Correct the upstream-owned context before activation. | No invalid active engagement created. |
| E02 assignment is required | Request the required assignment through E02 governance. | E02 owns and publishes the assignment outcome. |

## 11. Failure, Interruption and Business Recovery

An incomplete engagement remains in its applicable pre-activation position until the business prerequisite is resolved. Person/organisation corrections remain with their canonical owners. E01 does not create a compensating E02 assignment. No technical retry, queue, rollback, or infrastructure recovery is defined.

## 12. Business Rules and Decision Constraints

- Workforce Engagement is E01-owned.
- Approved engagements activate through the canonical E01 lifecycle.
- Significant workforce information is effective-dated and historically traceable.
- E02 exclusively owns Position Assignment, Responsibility Assignment, Reporting Relationship, and Assignment History.
- Dependent domains consume workforce facts without acquiring ownership.
- L3 applies locked E01 rules and does not create new policy definitions.

## 13. State and Lifecycle Impact

| Aggregate | Transition | Owner |
|---|---|---|
| Workforce Engagement | `Pending` → `Active` | Workforce & Engagement |

The locked E01 state model explicitly defines `Pending → Active`; no new state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

| Business Event | Canonical Publisher | Purpose |
|---|---|---|
| `WorkforceEngagementActivated` | Workforce & Engagement | Announces successful activation of the Workforce Engagement. |

Locked E01 lists this event as `WDE-102` / `WFE-002` and identifies it as the published fact after engagement activation. Downstream consumers do not republish or re-own it.

## 15. Business Authorization, Approval and Audit Obligations

Appointment approval and applicable policy/workflow governance are prerequisites. The exact approving actor/chain remains configuration-driven by locked E01 and is not hard-coded here. Establishment and activation must remain auditable and historically traceable. No technical IAM/RBAC design is introduced.

## 16. Completion Criteria and Postconditions

- Approved appointment represented as a Workforce Engagement.
- Workforce Engagement is `Active`.
- Effective dating and history are preserved.
- `WorkforceEngagementActivated` published by Workforce & Engagement.
- Upstream Person/organisation ownership is preserved.
- E02 assignment ownership remains untouched.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
No unresolved event-name or lifecycle-state vocabulary issue remains after direct reconciliation with locked L2-E01.

### 17.2 Assumptions
The exact appointment approving authority remains governed by institutional policy/workflow configuration and is intentionally not fixed at L3.

### 17.3 Deferred Questions
Implementation mechanics and technical integration remain outside L3.

## Production Verification Record

- Frozen L3-E-001 identity, A/B/E participants, Critical type, and P0 priority preserved.
- Direct locked L2-E01 verification completed.
- `Pending → Active` state transition restored.
- `WorkforceEngagementActivated` restored as the canonical activation event.
- E01/E02 assignment-ownership precedence preserved.
- All 17 mandatory sections present.
- Status remains **Draft** pending architecture review.
