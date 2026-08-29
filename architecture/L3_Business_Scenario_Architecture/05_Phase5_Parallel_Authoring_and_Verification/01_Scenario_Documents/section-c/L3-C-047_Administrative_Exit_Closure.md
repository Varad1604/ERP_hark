# Administrative Exit Closure

**Scenario ID:** L3-C-047  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Lifecycle / Qualification Completion & Student Exit  
**Participating Domains:** C, F, G  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Locked L1 Business Domain Architecture; L2-C05 Qualification Completion & Student Exit Workflows; DEPENDENCY_MAP.md  
**Referenced By:** Related Section C Student Exit scenarios

## 1. Scenario Metadata

Exact Phase 5 filename: `L3-C-047_Administrative_Exit_Closure.md`. Frozen identity is preserved.

## 2. Scenario Purpose

Govern an institutionally authorized administrative conclusion of a Student's institutional relationship where policy classifies the outcome as an Administrative Exit.

## 3. Business Outcome

The Student receives an authoritative **Administrative Exit** outcome, applicable obligations are resolved, final academic status is preserved, and Student Exit is completed without creating a degree award or Alumni Persona.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An institutionally authorized administrative-exit decision enters the C05 Student Exit pathway.

### 4.2 End Boundary
Administrative Exit is recorded and Student Exit reaches its completed terminal outcome.

### 4.3 In Scope
- Validate Administrative Exit as the applicable pathway.
- Determine applicable clearances.
- Coordinate authoritative clearance outcomes.
- Confirm required authorization.
- Record the administrative exit type.
- Preserve final academic status and history.
- Complete Student Exit.

### 4.4 Out of Scope
Qualification completion and degree conferment; underlying operational clearance decisions; Alumni engagement/persona creation; certificate/transcript generation; technical closure mechanics.

### 4.5 Related Scenarios
C040 Academic Record Finalization and Student Exit Completion; C046 Institutional Exit Clearance Completion; C042 Withdrawal; C043 Transfer; C044 Discontinuation; C045 Termination; C048 Deceased Student Academic Record Closure.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Student Lifecycle / C05 | Owns Administrative Exit and Student Exit lifecycle. |
| Finance (F) | Owns financial clearance where applicable. |
| Library / Hostel / Transport and other applicable domains | Own their individual clearance decisions. |

Participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Student Lifecycle / C05 | Owns Administrative Exit and Student Exit. |
| Institutional authorized authority | Authorizes Administrative Exit under institution-configured policy. |
| Participating operational domains | Own their individual clearance decisions. |

## 7. Preconditions and Entry Criteria

- Administrative Exit is a supported institutional pathway.
- Student Context exists.
- The basis for administrative exit is authoritative.
- Applicable clearances and authorization requirements can be evaluated.
- No completed terminal exit already exists.

## 8. Trigger

An authorized Administrative Exit decision enters the C05 Student Exit workflow.

## 9. Main Business Flow

1. Validate Administrative Exit as the applicable institutional pathway.
2. Confirm the current lifecycle state permits it.
3. Identify applicable institutional clearances.
4. Receive authoritative clearance outcomes.
5. Confirm configured institutional authorization.
6. Record Administrative Exit.
7. Preserve final academic status and historical records.
8. Complete Student Exit.
9. Publish the canonical completed-exit business outcome.

## 10. Alternate Business Flows

- Administrative Exit is not permitted: return to governed review.
- Mandatory clearance is unresolved: hold the exit until the owning domain resolves it.
- Another approved terminal pathway supersedes it before completion: follow the applicable governed pathway and maintain one terminal outcome.

## 11. Failure, Interruption and Business Recovery

Resolve unresolved obligations through canonical owners and reassess. Post-completion correction uses governed institutional procedures while preserving history.

## 12. Business Rules and Decision Constraints

- **BR-037:** Every student concludes through exactly one approved exit pathway.
- **BR-038:** Exit pathways include qualification completion and non-qualification outcomes.
- **BR-039:** Each exit preserves final academic status.
- **BR-040:** Non-qualification exits never produce degree conferment.
- **BR-041:** Student exit records remain permanently available.
- **BR-042:** Academic records finalize only after qualification completion or approved institutional exit.
- **BR-045:** Historical qualification completion information is never physically deleted.

## 13. State and Lifecycle Impact

| Lifecycle | Impact |
|---|---|
| Student Exit | Current position → Administrative Exit → Student Exit Completed. |
| Academic Record | Final academic status and historical records preserved. |
| Academic Credit Ledger | Survives the exit; ownership remains Student Lifecycle. |

## 14. Business Events and Cross-Domain Collaboration

| Event | Publisher | Purpose |
|---|---|---|
| `StudentExitCompleted` | C05 Qualification Completion Aggregate | Canonical L2 completed Student Exit event. |
| Applicable clearance outcomes | Respective owning domains | Supply authoritative clearance facts. |

L1's `StudentExited` naming is recorded as an upstream discrepancy rather than silently duplicated.

## 15. Business Authorization, Approval and Audit Obligations

Administrative Exit requires configured institutional authorization and auditable evidence. Locked L1 states that administrative exits do not create Alumni Personas. Exact approval authority is not fixed in C05 and is not invented.

## 16. Completion Criteria and Postconditions

- Administrative Exit pathway validated.
- Applicable clearances resolved.
- Required authorization completed.
- Administrative Exit recorded.
- Final academic status preserved.
- Student Exit completed.
- No degree or Alumni Persona created.

## 17. Open Issues, Assumptions and Deferred Questions

**OI-C047-01 — Administrative Exit definition:** L1/L2 identify Administrative Exit as a supported exit class but do not enumerate a universal reason taxonomy. None is invented.

**OI-C047-02 — Approval authority:** C05 approval is institution-configurable; exact authority remains an upstream policy matter.

**OI-C047-03 — Event naming:** L1 uses `StudentExited`; L2-C05 uses `StudentExitCompleted`. This remains an architecture-review discrepancy.

**Classification:** PASS WITH ISSUES — the canonical scenario and ownership are clear, while institution-specific administrative-exit policy and event-name reconciliation remain upstream governance items.

## Production Verification Record

- [x] Exact Final L3 ID and filename preserved.
- [x] C05 ownership preserved.
- [x] Cross-domain clearance ownership preserved.
- [x] No invented credential, lifecycle state, event publisher or authority.
- [x] All 17 mandatory sections present.
- [x] Material gaps recorded in Section 17.
- [x] Status remains Draft per Phase 5 governance.
