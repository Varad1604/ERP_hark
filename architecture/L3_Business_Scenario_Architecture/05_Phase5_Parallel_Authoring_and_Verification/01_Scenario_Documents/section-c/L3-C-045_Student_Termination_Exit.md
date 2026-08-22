# Student Termination Exit

**Scenario ID:** L3-C-045  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Lifecycle / Qualification Completion & Student Exit  
**Participating Domains:** C, F, G  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Locked L1 Business Domain Architecture; L2-C05 Qualification Completion & Student Exit Workflows; DEPENDENCY_MAP.md  
**Referenced By:** Related Section C Student Exit scenarios

## 1. Scenario Metadata

Exact Phase 5 filename: `L3-C-045_Student_Termination_Exit.md`. Frozen identity is preserved; no owner, participating-domain, Type or Priority reclassification is introduced.

## 2. Scenario Purpose

Govern the institutionally authorized termination of a Student's institutional relationship while preserving the final academic position and historical academic record.

## 3. Business Outcome

The Student is authoritatively recorded as **Terminated**, applicable institutional conditions are satisfied, and the Student Exit lifecycle is completed without creating a qualification award.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An authorized termination request/decision enters the C05 Student Exit workflow.

### 4.2 End Boundary
The termination exit is recorded, final academic status is preserved, and Student Exit reaches its completed terminal outcome.

### 4.3 In Scope
- Validate termination as the applicable exit pathway.
- Determine applicable institutional clearances.
- Coordinate authoritative clearance outcomes.
- Confirm required authorization.
- Record the termination exit type.
- Preserve final academic status and historical records.
- Complete Student Exit.

### 4.4 Out of Scope
Examination processing, qualification conferment, underlying Finance/Library/Hostel/Transport decisions, certificate/transcript generation, alumni engagement, and technical closure mechanics.

### 4.5 Related Scenarios
C040 Academic Record Finalization and Student Exit Completion; C046 Institutional Exit Clearance Completion; C047 Administrative Exit Closure; C048 Deceased Student Academic Record Closure.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Student Lifecycle / C05 | Owns the termination decision and Student Exit lifecycle. |
| Finance (F) | Owns financial clearance where applicable. |
| Library / Hostel / Transport and other applicable domains | Own their respective clearance decisions. |

Participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Student Lifecycle / C05 | Owns termination and Student Exit. |
| Institutional authorized authority | Provides required termination authorization under configured policy. |
| Participating operational domains | Determine their own clearance status. |

## 7. Preconditions and Entry Criteria

- Student Context exists.
- Termination is a permitted exit type.
- Required evidence is available.
- Applicable clearances can be evaluated.
- No conflicting completed terminal exit exists.

## 8. Trigger

An authorized termination request/decision enters the Student Exit workflow.

## 9. Main Business Flow

1. Establish termination as the applicable exit pathway.
2. Validate that termination is permitted for the current lifecycle state.
3. Determine applicable clearance obligations.
4. Receive authoritative clearance outcomes from owning domains.
5. Confirm required institutional authorization.
6. Record the exit type as Terminated.
7. Preserve final academic status and historical records.
8. Complete Student Exit.
9. Publish the canonical completed-exit event.

## 10. Alternate Business Flows

- Required clearance remains pending: termination remains unresolved.
- Required authorization is rejected/returned: termination does not complete.
- Another approved terminal pathway supersedes termination before completion: follow the governed replacement pathway without creating two terminal exits.

## 11. Failure, Interruption and Business Recovery

Do not complete termination while mandatory business conditions remain unresolved. Resolve outstanding obligations through canonical owners and reassess. Post-completion correction uses governed institutional correction procedures while preserving history.

## 12. Business Rules and Decision Constraints

- **BR-037:** Every student concludes through exactly one approved exit pathway.
- **BR-038:** Exit pathways include non-qualification outcomes.
- **BR-039:** Each exit preserves final academic status.
- **BR-040:** Non-qualification exits never produce degree conferment.
- **BR-041:** Student exit records remain permanently available.
- **BR-042:** Academic records finalize only after qualification completion or approved institutional exit.
- **BR-045:** Historical qualification completion information is never physically deleted.

## 13. State and Lifecycle Impact

| Lifecycle | Impact |
|---|---|
| Student Exit | Current lifecycle position → Terminated → Student Exit Completed. |
| Academic Record | Final academic status preserved/finalized under the C05 boundary. |
| Academic Credit Ledger | Preserved; ownership remains Student Lifecycle. |

## 14. Business Events and Cross-Domain Collaboration

| Event | Publisher | Purpose |
|---|---|---|
| `StudentExitCompleted` | C05 Qualification Completion Aggregate | Canonical L2 completed Student Exit event. |
| Clearance outcomes | Respective owning domains | Supply authoritative clearance facts. |

L1 publishes `StudentExited` while L2-C05 uses `StudentExitCompleted`; this is recorded as an upstream naming discrepancy rather than silently reconciled.

## 15. Business Authorization, Approval and Audit Obligations

Termination requires configured institutional authorization and auditable evidence. Clearance decisions remain attributable to their owning domains. No technical IAM/RBAC model is defined.

## 16. Completion Criteria and Postconditions

- Termination pathway validated.
- Applicable clearances resolved.
- Required authorization completed.
- Terminated exit type recorded.
- Final academic status preserved.
- Student Exit completed.
- Canonical completed-exit event published.
- No qualification award created.

## 17. Open Issues, Assumptions and Deferred Questions

**OI-C045-01 — Event naming:** L1 publishes `StudentExited`, while L2-C05 uses `StudentExitCompleted`. The L2 event is used because it directly governs this workflow; the discrepancy remains for architecture review.

**OI-C045-02 — Termination authority:** C05 makes approval authority institution-configurable. No universal fixed approver is invented.

**OI-C045-03 — Clearance set:** Mandatory clearance is policy-driven and is not hard-coded beyond applicable ownership.

**Classification:** PASS WITH ISSUES.

## Production Verification Record

- [x] Exact Final L3 ID and filename preserved.
- [x] Frozen owner, domains, Type and Priority preserved.
- [x] C05 ownership preserved.
- [x] No invented lifecycle state, event publisher or authority.
- [x] All 17 mandatory sections present.
- [x] Material gaps recorded in Section 17.
- [x] Status remains Draft per Phase 5 governance.
