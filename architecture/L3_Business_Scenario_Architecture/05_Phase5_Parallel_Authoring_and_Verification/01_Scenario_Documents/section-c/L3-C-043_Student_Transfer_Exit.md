# Student Transfer Exit

**Scenario ID:** L3-C-043  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Lifecycle / Qualification Completion & Student Exit  
**Participating Domains:** C, F, G  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Locked L1 Student Lifecycle Architecture; L2-C05 Qualification Completion & Student Exit Workflows; DEPENDENCY_MAP.md  
**Referenced By:** Related Section C lifecycle scenarios and downstream institutional consumers  

## 1. Scenario Metadata

Final ID and exact Phase 5 filename are preserved from the locked authoring allocation: `L3-C-043_Student_Transfer_Exit.md`. Section C is the canonical owner section. The frozen Phase 4 scenario identity is not redefined by this document.

## 2. Scenario Purpose

Govern the institutionally authorized conclusion of the current institutional Student Context through approved transfer, while preserving the student's academic history.

## 3. Business Outcome

The student's transfer is authoritatively recorded, the current institutional Student Exit lifecycle is completed, and the academic history remains preserved.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
The scenario begins when an approved institutional withdrawal/transfer/discontinuation decision is eligible to enter the corresponding Student Exit pathway.

### 4.2 End Boundary
The scenario ends when the specified exit outcome is authoritatively recorded, the student's final academic position is preserved, and the Student Exit lifecycle reaches its applicable completed outcome.

### 4.3 In Scope
- Validation of the applicable exit pathway.
- Applicable institutional clearance coordination.
- Recording the exit type.
- Preservation of academic history and final academic status.
- Completion of the Student Exit lifecycle.
- Publication of the canonical C05 exit business fact.

### 4.4 Out of Scope
- Qualification conferment.
- Examination-result ownership.
- Finance, Library, Hostel or Transport operational decisions themselves.
- Certificate/transcript generation.
- Alumni engagement.
- Technical identity, access-control, persistence or integration implementation.

### 4.5 Related Scenarios
| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-C-040 — Academic Record Finalization and Student Exit Completion | Common terminal finalization/exit boundary. |
| Related | L3-C-046 — Institutional Exit Clearance Completion | Applicable clearance dependency. |
| Related | L3-C-043 | Canonical scenario itself; no duplicate scenario is introduced. |

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Student Lifecycle / C05 | Sole owner of the institutional Student Exit outcome, exit record, final academic status and lifecycle transition. |
| Finance / F | Supplies its own authoritative clearance outcome where applicable; does not own Student Exit. |
| Library / G | Supplies its own authoritative clearance outcome where applicable; does not own Student Exit. |
| Hostel / G | Supplies its own authoritative clearance outcome where applicable; does not own Student Exit. |
| Transport / G | Supplies its own authoritative clearance outcome where applicable; does not own Student Exit. |

Participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

- **Student:** May initiate or participate in a withdrawal/transfer pathway where institutional policy permits; no assumption is made about mandatory initiation authority.
- **Institutional academic/administrative authority:** Authorizes the applicable exit according to institutional policy.
- **Student Lifecycle / Qualification Completion & Student Exit:** Evaluates and records the authoritative exit outcome.
- **Participating clearance domains:** Determine their own clearance status under their own rules.

Exact institution-specific approval roles are not invented where the locked L2 does not define a fixed chain.

## 7. Preconditions and Entry Criteria

- The Student Context exists.
- The requested exit pathway is institutionally permitted.
- Required evidence for the exit decision is available.
- Applicable clearance requirements can be evaluated.
- No conflicting completed terminal exit already exists.
- Academic history remains available for preservation.

## 8. Trigger

An authorized withdrawal, transfer, or discontinuation request/decision enters the applicable Student Exit workflow.

## 9. Main Business Flow

1. Identify the applicable Student Exit pathway.
2. Validate that the pathway is permitted for the student's current lifecycle position.
3. Determine the institutional clearance obligations applicable to that exit.
4. Coordinate clearance validation with the owning operational domains.
5. Receive and consolidate the authoritative clearance outcomes without re-evaluating those domains' underlying business rules.
6. Confirm the exit authorization required by institutional governance.
7. Record the specific exit type.
8. Preserve the student's final academic status and historical academic records.
9. Complete the Student Exit lifecycle.
10. Publish the canonical `StudentExitCompleted` business fact where defined by C05.

The flow ends at the exit outcome; downstream consumers do not become part of this scenario.

## 10. Alternate Business Flows

- **Clearance not yet complete:** The exit remains pending until the applicable owning domain resolves its own clearance condition.
- **Exit authorization rejected or returned:** The exit remains unresolved and follows the institutionally governed review path.
- **Withdrawal/transfer/discontinuation pathway changed before completion:** The applicable governed pathway is re-evaluated rather than creating competing terminal outcomes.
- **Student remains eligible for a qualification outcome instead:** Qualification Completion is handled by the separate qualification pathway.

## 11. Failure, Interruption and Business Recovery

- **Unresolved institutional obligation:** Do not complete the exit; preserve the current business position and await authoritative clearance resolution.
- **Insufficient exit authority:** Do not complete the exit until the required business authorization exists.
- **Conflicting terminal outcome:** Do not create a second terminal exit; resolve the conflict through the governed institutional correction/review process.
- **Post-completion correction requirement:** Apply the institutionally governed correction procedure while preserving historical records.

No technical retry, rollback, queue, server-failure or infrastructure recovery is defined.

## 12. Business Rules and Decision Constraints

- Student history is immutable.
- Qualification never deletes academic records.
- Administrative exits preserve historical academic records.
- Each Student Exit follows an approved institutional pathway.
- Non-qualification exits do not produce a degree conferment.
- Clearance is policy-driven and only applicable operational modules participate.
- The Academic Credit Ledger survives every qualification and exit.
- C05 remains the canonical owner of the Student Exit decision.
- Participating domains remain authoritative for their own clearance decisions.

## 13. State and Lifecycle Impact

| Lifecycle | Scenario Impact |
|---|---|
| Student Context / Student Exit | Moves through the applicable authorized exit pathway to `StudentExitCompleted`. |
| Academic Record | Final academic position is preserved. |
| Academic Credit Ledger | Remains preserved; ownership remains Student Lifecycle / Academic Progression. |

No new lifecycle state or transition is introduced.

## 14. Business Events and Cross-Domain Collaboration

| Event / Collaboration | Publisher / Owner | Role |
|---|---|---|
| `StudentExitCompleted` | Qualification Completion Aggregate / C05 | Authoritative completed Student Exit fact. |
| Clearance status from Finance | Finance | Finance-owned clearance fact. |
| Clearance status from Library | Library / G | Library-owned clearance fact. |
| Clearance status from Hostel | Hostel / G | Hostel-owned clearance fact. |
| Clearance status from Transport | Transport / G | Transport-owned clearance fact. |

C05 consumes clearance outcomes; consumption does not transfer clearance ownership.

The L1 architecture also identifies `StudentExited` as a Section C publication. The locked L2-C05 document uses `StudentExitCompleted` as its canonical lifecycle event. This scenario therefore treats `StudentExitCompleted` as the L2-governed event for the completed lifecycle transition and records the naming difference in Section 17 rather than inventing a second publisher.

## 15. Business Authorization, Approval and Audit Obligations

- The applicable withdrawal, transfer or discontinuation outcome must be institutionally authorized according to governing policy.
- Clearance decisions must remain attributable to their owning domains.
- Exit decisions, authorization, clearance outcomes and final academic status must remain auditable.
- Historical academic records must be preserved.
- No technical access control/IAM model is defined at L3.

## 16. Completion Criteria and Postconditions

The scenario is complete only when:

- the correct exit pathway is established;
- required applicable clearances are resolved;
- required institutional authorization is complete;
- the exit type is recorded;
- final academic status is preserved;
- the Student Exit lifecycle is completed;
- the authoritative C05 exit event is published.

Postconditions:
- The Student's institutional relationship is conclusively closed through the selected exit pathway.
- Academic history remains preserved.
- The Academic Credit Ledger remains preserved.
- No qualification award is created by a non-qualification exit.

## 17. Open Issues, Assumptions and Deferred Questions

**OI-EXIT-01 — Event naming reconciliation:** Locked L1 identifies `StudentExited`, while locked L2-C05 identifies `StudentExitCompleted`. This is a source-level naming inconsistency. L3 does not create a new event or silently rename either source. The L2-C05 event definition is used for the scenario lifecycle, while the L1 discrepancy remains an architecture-review item.

**OI-EXIT-02 — Exact approval authority:** C05 states that exit governance is institution-specific/configurable. This document does not invent a universal approval chain.

**OI-EXIT-03 — Exact clearance set:** C05 lists Finance, Library, Hostel, Department, Laboratory, Scholarship and institution-specific clearances, while L1 additionally shows Transport. Only applicable clearances are included; the actual set remains policy-governed.

**Architecture classification:** **PASS WITH ISSUES** — source-grounded and usable, but the event-name discrepancy and institution-specific approval/clearance configuration should be explicitly confirmed before Architecture Approved/Locked status.

## Production Verification Record

- Exact ID preserved.
- Exact filename preserved.
- Section C ownership preserved.
- C05 remains sole Student Exit authority.
- Participating clearance domains retain their own ownership.
- No invented lifecycle state.
- No invented event publisher.
- No implementation leakage.
- All 17 mandatory sections present.
- Material source discrepancies recorded in Section 17.
