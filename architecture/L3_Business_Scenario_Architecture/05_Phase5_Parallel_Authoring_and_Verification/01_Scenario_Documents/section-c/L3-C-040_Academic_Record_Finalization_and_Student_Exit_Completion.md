# Academic Record Finalization and Student Exit Completion

**Architecture Level:** L3 — Business Scenario Architecture  
**Final L3 ID:** L3-C-040  
**Canonical Title:** Academic Record Finalization and Student Exit Completion  
**Primary Owner:** Qualification Completion & Student Exit  
**Participating Domains:** C, I  
**Scenario Type:** Critical  
**Priority:** P0  
**Derivation:** KEEP  
**Status:** Draft  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-22  

## 1. Scenario Metadata

The frozen canonical identity is preserved for **L3-C-040 — Academic Record Finalization and Student Exit Completion**. No ownership, participating-domain, type, priority, title, or scenario-boundary reclassification is introduced.

## 2. Scenario Purpose

Finalize the Student's academic record after an authoritative qualification or approved institutional-exit outcome, preserve the final academic status, and complete the institutional exit.

## 3. Business Outcome

The final academic record is finalized and the institutional relationship is conclusively closed through `StudentExitCompleted`. Qualification and Examination decisions remain authoritative within their canonical owners and historical records remain preserved.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An approved qualification-completion/degree outcome or another approved institutional exit pathway exists and the C05 aggregate is eligible for finalization and exit.

### 4.2 End Boundary
The scenario ends when the academic record is finalized and Student Exit is completed. Alumni transition is downstream and is not absorbed into this scenario.

### 4.3 In Scope
- Validate the approved terminal pathway.
- Finalize the academic record.
- Preserve final academic status and history.
- Record the approved exit type.
- Complete Student Exit.
- Publish `AcademicRecordFinalized` and `StudentExitCompleted`.

### 4.4 Out of Scope
- Determining whether the Student qualifies for a qualification.
- Degree conferment.
- Underlying Finance, Library, Hostel, or Examination operations.
- Certificate or transcript generation.
- Alumni engagement.
- Technical record-locking implementation.

### 4.5 Related Scenarios
| Relationship | Scenario |
|---|---|
| Predecessor | C039 — Qualification Completion and Degree Conferment |
| Related | C042 — Student Withdrawal Exit |
| Related | C043 — Student Transfer Exit |
| Related | C044 — Student Discontinuation Exit |
| Related | C045 — Student Termination Exit |
| Related | C046 — Institutional Exit Clearance Completion |

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Qualification Completion & Student Exit (C05) | Owns academic-record finalization and the Student Exit lifecycle and publishes the resulting C05 business events. |
| Examination (I) | Remains authoritative for Examination-owned facts and evidence; it does not own final institutional exit or academic-record finalization. |

Cross-domain participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority |
|---|---|---|
| Qualification Completion & Student Exit | Validates the approved terminal pathway, finalizes the academic record, and completes Student Exit. | Canonical C05 owner. |
| Examination responsibility | Supplies or preserves authoritative Examination facts where required by the terminal academic record. | Evidence authority only; no C05 ownership. |
| Authorized institutional exit responsibility | Provides any required governed exit authorization. | Exact role follows upstream institutional governance and is not invented here. |

## 7. Preconditions and Entry Criteria

- A valid approved qualification-completion outcome or approved non-qualification exit exists.
- No conflicting terminal exit exists.
- Final academic status is known.
- Required governed exit authorization exists.

## 8. Trigger

An approved terminal qualification/exit outcome reaches the point where the Student's academic record and institutional relationship may be finalized.

## 9. Main Business Flow

| Step | Business Behaviour | Owner |
|---|---|---|
| 1 | Validate the approved terminal exit pathway and current C05 lifecycle state. | Qualification Completion & Student Exit |
| 2 | Confirm the final academic outcome that must be preserved. | Qualification Completion & Student Exit |
| 3 | Finalize the academic record. | Qualification Completion Aggregate |
| 4 | Record the approved exit type. | Qualification Completion Aggregate |
| 5 | Complete the Student Exit lifecycle. | Qualification Completion Aggregate |
| 6 | Publish `AcademicRecordFinalized` and `StudentExitCompleted`. | Qualification Completion Aggregate |

## 10. Alternate Business Flows

| Condition | Business Path | Result |
|---|---|---|
| Exit authorization is incomplete | Do not finalize the exit. | Existing non-terminal position remains. |
| Final academic outcome is unresolved | Defer finalization. | No terminal closure. |
| A conflicting terminal exit already exists | Do not create a second terminal exit. | Existing terminal outcome remains authoritative. |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| Required exit authorization is missing | Obtain the governed authorization before finalization. |
| Academic record contains unresolved completion information | Resolve the issue through the canonical owning academic process before finalization. |
| Exit is already completed | Do not reopen through ordinary lifecycle processing; use governed correction/revocation procedures. |
| Qualification award is later corrected/revoked | Apply the existing C05 exceptional governance without silently rewriting historical finalization. |

No technical recovery mechanics are introduced.

## 12. Business Rules and Decision Constraints

| Rule | Effect |
|---|---|
| **BR-037** Every Student concludes through exactly one approved exit pathway. | Prevents competing terminal outcomes. |
| **BR-039** Every exit preserves final academic status. | Protects the final record. |
| **BR-041** Exit records remain permanently available. | Preserves institutional evidence. |
| **BR-042** Academic records finalize only after qualification completion or approved institutional exit. | Establishes the finalization boundary. |
| **BR-043** Finalized records become read-only. | Protects finalized history. |
| **BR-044** Corrections use governed procedures. | Prevents ordinary mutation. |
| **BR-045** Historical qualification information is never physically deleted. | Preserves history. |

These rules remain owned by locked L2-C05.

## 13. State and Lifecycle Impact

| Lifecycle | Transition | Result |
|---|---|---|
| Qualification Completion Aggregate | `QualificationAwarded` / approved alternative exit → `AcademicRecordFinalized` | Final academic record established. |
| Qualification Completion Aggregate | `AcademicRecordFinalized` → `StudentExitCompleted` | Institutional relationship concluded. |

No new lifecycle state or transition is created by this scenario.

## 14. Business Events and Cross-Domain Collaboration

| Event | Canonical Publisher | Purpose |
|---|---|---|
| `AcademicRecordFinalized` | Qualification Completion Aggregate | Announces that the academic record is finalized. |
| `StudentExitInitiated` | Qualification Completion Aggregate | Announces an approved Student Exit has begun where applicable. |
| `StudentExitCompleted` | Qualification Completion Aggregate | Announces terminal institutional exit. |

`QualificationAlumniTransitionInitiated` is downstream of completed exit where applicable and is not required to complete C040.

## 15. Business Authorization, Approval and Audit Obligations

Finalization and exit require institutionally governed authorization. Final academic status, exit authorization, finalization, and terminal outcome must remain auditable. No technical access-control design is introduced.

## 16. Completion Criteria and Postconditions

- Correct terminal pathway validated.
- Final academic status preserved.
- Academic record finalized.
- Exactly one approved exit recorded.
- `AcademicRecordFinalized` published.
- `StudentExitCompleted` published.
- Terminal lifecycle is not reopened through ordinary business operations.

## 17. Open Issues, Assumptions and Deferred Questions

**OI-C040-01:** C05 defines multiple approved exit pathways. This scenario treats the terminal exit generically and does not invent a single universal exit reason.

**OI-C040-02:** Correction/revocation remains exceptional institutional governance rather than ordinary finalization.

**Assumption:** Alumni transition is downstream of completed qualification/exit and does not alter C05 ownership of the terminal Student Exit record.

## Production Verification Record

- Frozen canonical identity expressed in required `L3-C-040` format.
- Frozen owner, participating domains, type, priority, and title preserved.
- Duplicate Section 5 table header removed.
- Locked L2-C05 lifecycle ordering and event publishers applied.
- Examination ownership preserved.
- No implementation mechanics introduced.
- All 17 mandatory sections present.
- Status remains **Draft** pending architecture review.
