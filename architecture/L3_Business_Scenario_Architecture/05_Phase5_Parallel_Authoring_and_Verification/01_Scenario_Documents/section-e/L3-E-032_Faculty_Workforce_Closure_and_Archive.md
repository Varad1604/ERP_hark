# Faculty Workforce Closure and Archive

**Scenario ID:** L3-E-032  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Faculty Workforce  
**Participating Domains:** D, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Author:** Varad  
**Version:** 1.0  
**Date:** 2026-08-20  
**Depends On:** L2-E03 — Faculty Workforce Workflows

## 1. Scenario Metadata

This document preserves the frozen Phase 4 / Stage 3 scenario identity, title, Primary Owner, Participating Domains, Scenario Type and Priority. It applies the locked upstream architecture and does not redesign ownership.

## 2. Scenario Purpose

Complete the Faculty Workforce lifecycle after participation ends and preserve the resulting faculty record as historical information according to retention policy.

## 3. Business Outcome

The Faculty Workforce reaches Inactive after participation ends and is subsequently Archived, preserving regulatory traceability, compliance history and historical workforce information.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

Faculty participation has ended and the Faculty Workforce is eligible for closure and archival.

### 4.2 End Boundary

The Faculty Workforce is Inactive and subsequently Archived according to the canonical lifecycle and retention policy.

### 4.3 In Scope

- Faculty participation end/closure.
- Transition to the canonical Inactive state.
- Historical preservation.
- Archival according to retention policy.
- Prevention of archived records participating in active workforce processes.

### 4.4 Out of Scope

Implementation mechanics, APIs, databases, services, Business Data Structures, code, UI, infrastructure, technical access control/IAM, queues, technical retries, rollback and deployment concerns are outside L3.

### 4.5 Related Scenarios

Only canonical upstream/downstream dependencies explicitly supported by the locked architecture are referenced.

## 5. Ownership and Participating Domains

**Canonical Owner:** Faculty Workforce

Participating domains remain authoritative for their own concepts. Participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

- Faculty Workforce.
- Authorised institutional governance participants where closure/retention decisions require governance.
- Relevant D/E participants as business context.

No technical role or unsupported authority is introduced.

## 7. Preconditions and Entry Criteria

- Faculty Workforce exists.
- Participation has ended.
- Closure conditions are satisfied.
- Retention/archival policy is applicable.

## 8. Trigger

An authoritative end-of-participation condition requires the Faculty Workforce to be closed and later archived.

## 9. Main Business Flow

1. Identify the Faculty Workforce whose participation has ended.
2. Validate that the participation-ending condition is authoritative.
3. Establish the Faculty Workforce in **Inactive** in accordance with the canonical lifecycle.
4. Preserve the historical faculty record and applicable compliance information.
5. When retention/archival conditions are satisfied, apply **WF-E03-010 Faculty Archival**.
6. Transition **Inactive → Archived**.
7. Publish `FacultyWorkforceArchived`.
8. Ensure the Archived record remains historical and does not re-enter active workforce processing.
9. Stop when archival is authoritative.

The Main Business Flow stops when the Business Outcome and End Boundary are reached.

## 10. Alternate Business Flows

- If participation has not actually ended, archival does not occur.
- If retention conditions have not been satisfied, the record remains Inactive and historically preserved until the archival condition is met.
- Archived records do not return to active workforce processing through the archival workflow.

## 11. Failure, Interruption and Business Recovery

If the participation-ending condition is not established, the Faculty Workforce does not progress to the closure outcome. If archival conditions are not satisfied, the record remains Inactive and historically preserved. No technical retry or rollback is defined.

Only business-level recovery is described.

## 12. Business Rules and Decision Constraints

- Faculty Archival is owned by E03.
- Archival preserves historical faculty workforce records.
- Archival supports regulatory traceability and compliance history.
- Lifecycle transition: **Active → Inactive → Archived**.
- Archived faculty records are historical and are not active workforce participants.

These rules remain owned by the authoritative upstream architecture.

## 13. State and Lifecycle Impact

**Faculty Workforce:** Active → Inactive → Archived.

Canonical workflow:
- WF-E03-010 — Faculty Archival

The source identifies `FacultyParticipationEnded` for the Active → Inactive transition and `FacultyWorkforceArchived` for Inactive → Archived.

No new lifecycle state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events

| Event | Canonical Publisher | Business Meaning |
|---|---|---|
| `FacultyParticipationEnded` | Faculty Workforce | Faculty participation has ended. |
| `FacultyWorkforceArchived` | Faculty Workforce | Faculty workforce record is archived according to retention policy. |

### 14.2 Business Contracts / Hand-offs

Downstream domains may consume the authoritative faculty lifecycle facts. No downstream domain acquires ownership of Faculty Workforce archival.

Events are distinct from synchronous business contracts and hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

Closure and archival follow the institutional lifecycle and retention policy. Historical and compliance evidence is preserved. No unsupported named approver is invented.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Participation has ended.
- Faculty Workforce reaches Inactive.
- Historical record is preserved.
- Retention/archival condition is satisfied.
- Faculty Workforce reaches Archived.
- `FacultyWorkforceArchived` is published.

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
