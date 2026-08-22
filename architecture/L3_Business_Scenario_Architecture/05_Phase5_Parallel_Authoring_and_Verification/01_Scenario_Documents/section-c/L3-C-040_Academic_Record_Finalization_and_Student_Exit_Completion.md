# Academic Record Finalization and Student Exit Completion

**Architecture Level:** L3 — Business Scenario Architecture  
**Final L3 ID:** C040  
**Canonical Title:** Academic Record Finalization and Student Exit Completion  
**Primary Owner:** Qualification Completion & Student Exit  
**Participating Domains:** C, I  
**Scenario Type:** Critical  
**Priority:** P0  
**Derivation:** KEEP  
**Status:** Draft  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-20  

## 1. Scenario Metadata

Frozen Stage 3 identity is preserved exactly for **C040 — Academic Record Finalization and Student Exit Completion**. No ownership, participating-domain, type, priority, title, or scenario-boundary reclassification is introduced.

## 2. Scenario Purpose

Finalize the student's academic record after the authoritative qualification or approved institutional exit outcome, preserve the final academic status, and complete the institutional exit.

## 3. Business Outcome

The Student's final academic record is finalized and the institutional relationship is conclusively closed through `StudentExitCompleted`. Qualification completion/degree decisions remain authoritative and historical records are preserved.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
An approved qualification-completion/degree outcome or another approved institutional exit pathway exists and the aggregate is eligible for finalization and exit.

### 4.2 End Boundary
The scenario ends when the academic record is finalized and the Student Exit is completed. Alumni transition is a downstream integration outcome for qualification-completed students and is not expanded into this scenario.

### 4.3 In Scope
- Validate the approved terminal pathway.
- Finalize the academic record.
- Preserve the final academic status and history.
- Record the approved exit type.
- Complete Student Exit.
- Publish `AcademicRecordFinalized` and `StudentExitCompleted` as applicable.

### 4.4 Out of Scope
- Deciding whether the student qualifies for the qualification.
- Degree conferment.
- Underlying finance/library/hostel operations.
- Certificate or transcript generation.
- Alumni engagement.
- Technical record locking implementation.

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
| Qualification Completion & Student Exit (C05) | Canonical owner of the qualification-completion / exit outcome within this scenario. |
| Domain | Responsibility |
|---|---|
| Examination / Assessment (I) | Remains authoritative for examination facts; it does not own the final institutional exit or academic-record-finalization decision. |
| Qualification Completion & Student Exit (C05) | Owns academic record finalization and Student Exit lifecycle. |


Cross-domain participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority |
|---|---|---|
| Qualification Completion & Student Exit | Determines qualification completion, degree conferment and final institutional outcome. | Canonical qualification authority. |
| Examination / Assessment | Supplies authoritative examination completion evidence. | Evidence authority for examination-owned facts. |


No actor, role, approval authority, rule, lifecycle or event is invented where the upstream source does not define it.

## 7. Preconditions and Entry Criteria

- A valid approved qualification-completion outcome or approved non-qualification exit exists.
- No conflicting terminal exit exists.
- Final academic status is known.
- Required governed exit authorization exists.

## 8. Trigger

An approved terminal qualification/exit outcome reaches the point where the student's academic record and institutional relationship can be finalized.

## 9. Main Business Flow

| Step | Business Behaviour | Owner |
|---|---|---|
| 1 | Validate the approved terminal exit pathway and current aggregate state. | Qualification Completion |
| 2 | Confirm the final academic outcome that must be preserved. | Qualification Completion |
| 3 | Finalize the academic record. | Qualification Completion |
| 4 | Record the approved exit type. | Qualification Completion |
| 5 | Complete the Student Exit lifecycle. | Qualification Completion |
| 6 | Publish `AcademicRecordFinalized` and `StudentExitCompleted`. | Qualification Completion Aggregate |

The flow stops at the frozen institutional outcome. Downstream consumers do not become part of the scenario merely because they consume the resulting event.

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
| Academic record contains unresolved completion information | Resolve it through the canonical owning academic process before finalization. |
| Exit is already completed | No ordinary lifecycle reopening is permitted; use governed correction/revocation procedures. |
| Qualification award is later subject to correction/revocation | Apply the existing C05 exceptional governance without silently rewriting historical finalization. |

Recovery is expressed only as business resolution. No technical queues, retries, rollback, services, APIs or infrastructure mechanics are defined.

## 12. Business Rules and Decision Constraints

| Rule | Effect |
|---|---|
| **BR-037** Every student concludes through exactly one approved exit pathway. | Prevents competing terminal outcomes. |
| **BR-039** Every exit preserves final academic status. | Protects the final record. |
| **BR-041** Exit records remain permanently available. | Preserves compliance/reporting evidence. |
| **BR-042** Academic records finalize only after qualification completion or approved institutional exit. | Establishes finalization boundary. |
| **BR-043** Finalized records become read-only. | Protects finalized history. |
| **BR-044** Corrections use governed procedures. | Prevents ordinary mutation. |
| **BR-045** Historical qualification information is never physically deleted. | Preserves institutional history. |

These rules remain owned by the authoritative upstream L2 domain. L3 does not create local substitutes.

## 13. State and Lifecycle Impact

| Lifecycle | Transition | Result |
|---|---|---|
| Qualification Completion Aggregate | Degree Conferred / approved alternative exit → Academic Record Finalized | Final academic record established |
| Academic Record | Finalization initiated → finalized | Academic history authoritative |
| Student Exit | Approved exit → Student Exit Completed | Institutional relationship concluded |

No new lifecycle state or transition is created by this scenario.

## 14. Business Events and Cross-Domain Collaboration

| Event | Canonical Publisher | Purpose |
|---|---|---|
| `AcademicRecordFinalized` | Qualification Completion Aggregate | Announces that academic records are finalized. |
| `StudentExitInitiated` | Qualification Completion Aggregate | Indicates approved institutional exit has begun where applicable. |
| `StudentExitCompleted` | Qualification Completion Aggregate | Announces terminal institutional exit. |

For qualification-completed students, `QualificationAlumniTransitionInitiated` is a subsequent C05 event/integration outcome and is not required to complete the C040 business outcome.

Business events are distinct from synchronous business hand-offs. Canonical publishers remain unchanged.

## 15. Business Authorization, Approval and Audit Obligations

Finalization and exit require authorized institutional governance. The final academic status, exit authorization, finalization decision and terminal outcome must be auditable. No technical access-control design is introduced.

No technical access control/IAM model is introduced.

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

**OI-C040-02:** Exact correction/revocation procedure is institutionally governed and intentionally remains outside ordinary finalization.

**Assumption:** Alumni transition is downstream of successful qualification completion and does not alter the C05 ownership of the terminal student-exit record.

## Production Verification Record

- [x] Frozen Stage 3 identity preserved.
- [x] Owner / participating domains / type / priority preserved.
- [x] Locked L2-C05 ownership boundary applied.
- [x] No ownership transfer caused by cross-domain consumption.
- [x] Canonical C05 lifecycle states/events preserved.
- [x] No implementation mechanics introduced.
- [x] Business-level failure/recovery only.
- [x] All 17 mandatory sections present.
- [x] Material source gaps recorded in Section 17.
- [x] Status retained as **Draft** per Phase 5 lifecycle governance; this is production-quality authored content, not a claim of architectural approval/lock.
