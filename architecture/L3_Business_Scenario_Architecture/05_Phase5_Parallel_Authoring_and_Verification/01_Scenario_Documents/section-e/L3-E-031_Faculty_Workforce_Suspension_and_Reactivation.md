# Faculty Workforce Suspension and Reactivation

**Scenario ID:** L3-E-031  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Faculty Workforce  
**Participating Domains:** D, E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Author:** Varad  
**Version:** 1.0  
**Date:** 2026-08-20  
**Depends On:** L2-E03 — Faculty Workforce Workflows

## 1. Scenario Metadata

This document preserves the frozen Phase 4 / Stage 3 scenario identity, title, Primary Owner, Participating Domains, Scenario Type and Priority. It applies the locked upstream architecture and does not redesign ownership.

## 2. Scenario Purpose

Temporarily restrict Faculty Workforce participation and subsequently restore participation when the suspension conditions are resolved, while preserving the faculty lifecycle history.

## 3. Business Outcome

The Faculty Workforce reaches Suspended when suspension is authorised and later returns to Active through the canonical reactivation workflow when restoration conditions are satisfied.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An Active Faculty Workforce requires a governed suspension, or an existing Suspended Faculty Workforce becomes eligible for restoration.

### 4.2 End Boundary

The Faculty Workforce is either authoritatively Suspended or authoritatively reactivated to Active, with the applicable lifecycle event published.

### 4.3 In Scope

- Faculty Workforce suspension.
- Preservation of faculty participation history and audit information.
- Reactivation after suspension conditions are resolved.
- Consequences for academic participation while Suspended.

### 4.4 Out of Scope

Implementation mechanics, APIs, databases, services, Business Data Structures, code, UI, infrastructure, technical access control/IAM, queues, technical retries, rollback and deployment concerns are outside L3.

### 4.5 Related Scenarios

Only canonical upstream/downstream dependencies explicitly supported by the locked architecture are referenced.

## 5. Ownership and Participating Domains

**Canonical Owner:** Faculty Workforce

Participating domains remain authoritative for their own concepts. Participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

- Faculty Workforce / authorised institutional governance participants.
- Relevant faculty workforce participant.
- D/E participating context where required by institutional governance.

No technical role or unsupported authority is introduced.

## 7. Preconditions and Entry Criteria

- Faculty Workforce exists.
- Suspension is permitted by the canonical lifecycle.
- For reactivation, the Faculty Workforce is Suspended and the suspension conditions have been resolved.

## 8. Trigger

An authorised business decision requires faculty participation to be suspended or restored.

## 9. Main Business Flow

1. Identify the Faculty Workforce and current lifecycle state.
2. For suspension, validate the applicable suspension condition and governance authority.
3. Transition **Active → Suspended** through **WF-E03-008 Faculty Suspension**.
4. Publish `FacultySuspended`.
5. Ensure the Suspended Faculty Workforce does not participate in academic activities.
6. When suspension conditions are resolved, evaluate the Faculty Workforce for restoration.
7. Transition **Suspended → Active** through **WF-E03-009 Faculty Reactivation**.
8. Publish `FacultyReactivated`.
9. Stop when the resulting lifecycle state is authoritative.

The Main Business Flow stops when the Business Outcome and End Boundary are reached.

## 10. Alternate Business Flows

- A suspension request that does not satisfy the applicable governance conditions does not make the Faculty Workforce Suspended.
- A Faculty Workforce remains Suspended until the canonical reactivation conditions are resolved.
- Reactivation restores participation; it does not create a new Faculty Workforce record.

## 11. Failure, Interruption and Business Recovery

If suspension cannot validly be established, the current authoritative faculty lifecycle remains unchanged. If restoration conditions are not resolved, the Faculty Workforce remains Suspended. No technical retry or rollback is defined.

Only business-level recovery is described.

## 12. Business Rules and Decision Constraints

- `WF-E03-008` temporarily restricts faculty participation.
- `WF-E03-009` restores faculty participation after suspension conditions are resolved.
- Lifecycle transition: **Active → Suspended → Active**.
- A non-Active Faculty Workforce SHALL NOT participate in academic activities.
- Historical records and audit information are preserved.

These rules remain owned by the authoritative upstream architecture.

## 13. State and Lifecycle Impact

**Faculty Workforce:** Active → Suspended → Active.

Canonical workflows:
- WF-E03-008 — Faculty Suspension
- WF-E03-009 — Faculty Reactivation

No new lifecycle state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events

| Event | Canonical Publisher | Business Meaning |
|---|---|---|
| `FacultySuspended` | Faculty Workforce | Faculty participation is suspended. |
| `FacultyReactivated` | Faculty Workforce | Suspended faculty participation is restored. |

### 14.2 Business Contracts / Hand-offs

Other domains consume the resulting faculty lifecycle facts according to their own responsibilities. Faculty Workforce retains lifecycle ownership.

Events are distinct from synchronous business contracts and hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

Suspension and reactivation follow the institutional governance and approval model defined by L2-E03. Suspension history, effective period and audit information are preserved. No unsupported named approver is invented.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Suspension is authoritative when the Faculty Workforce reaches Suspended.
- Academic participation is restricted while Suspended.
- Reactivation is authoritative only after restoration conditions are resolved.
- Active participation is restored through the canonical reactivation workflow.
- Both canonical lifecycle events are published.

### 16.2 Postconditions

The completed business outcome remains authoritative for subsequent institutional behaviour and required historical information is preserved.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

None identified from the authoritative sources reviewed for this scenario.

### 17.2 Assumptions

None.

### 17.3 Deferred Questions

None.

**Authoring Classification:** PASS — source-grounded Draft.
