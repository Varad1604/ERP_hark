# Student Programme Migration

**Scenario ID:** L3-C-033  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Progression  
**Participating Domains:** B, C  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-C04_Student_Academic_Progression_Workflows.md`; `L2-C02_Student_Lifecycle_and_Activation.md`; applicable Curriculum Framework and Academic Registration architecture  
**Referenced By:** L3-C-009 Student Academic Assignment Change; L3-C-037 Academic Credit Ledger to ABC Synchronisation

## 1. Scenario Metadata

Frozen identity: **C033 — Student Programme Migration**; Primary Owner **Academic Progression**; Participating Domains **B, C**; Type **Critical**; Priority **P1**; Derivation **KEEP**.

---

## 2. Scenario Purpose

To govern approved movement of a Student from one academic Programme to another while preserving academic history and establishing an authoritative migration outcome.

Academic Progression APW-08 exclusively owns Programme-changing mobility, including eligibility, curriculum mapping, credit mapping, approval and the `ProgrammeMigrationApproved` outcome.

---

## 3. Business Outcome

A Programme Migration request is authoritatively **Approved** or not approved.

When approved, the Student has an authoritative programme-migration outcome with eligibility, curriculum mapping and credit mapping completed, and the approved result is made available for the Student Lifecycle domain to update its Programme association.

The migration decision remains owned by Academic Progression.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a valid Programme Migration request is submitted under an approved institutional process.

### 4.2 End Boundary

The scenario ends when the migration decision and required academic mappings are completed and `ProgrammeMigrationApproved` is published for downstream application.

### 4.3 In Scope

- Migration eligibility assessment.
- Seat validation where required by APW-08.
- Curriculum mapping.
- Credit mapping.
- Institutional migration approval.
- Publication of the authoritative migration outcome.

### 4.4 Out of Scope

- Direct modification of the C02-owned Student Context.
- Non-Programme transfers such as campus, department, section or batch changes.
- Academic Registration execution following migration.
- Curriculum ownership or curriculum redesign.
- Technical implementation of migration processing.

### 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-C-009 Student Academic Assignment Change | Non-Programme assignment changes remain distinct. |
| Related | L3-C-037 Academic Credit Ledger to ABC Synchronisation | Migration may affect the academic credit context but does not own ABC synchronization. |

---

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Academic Progression (C) | Owns Programme Migration eligibility, mapping, approval and authoritative outcome. |
| Academic Structure / Curriculum (B) | Supplies authoritative Programme/Curriculum structures used for validation and mapping. |

The Student Lifecycle C02 workflow explicitly consumes `ProgrammeMigrationApproved` only to update the C02-owned Programme association; it does not re-evaluate or republish the migration decision.

---

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Academic Progression | Evaluates and decides Programme Migration. | Canonical decision authority. |
| Academic Structure / Curriculum | Provides authoritative destination Programme and curriculum information. | Structural authority; does not decide migration. |

No new migration approver role is invented.

---

## 7. Preconditions and Entry Criteria

- Student has a valid existing Programme association.
- Requested destination Programme is an authoritative academic structure.
- Migration is permitted by institutional policy.
- Required eligibility information is available.
- Curriculum and credit mappings can be established.
- Where applicable, destination capacity/seat validation is available.

---

## 8. Trigger

**An institutionally valid Programme Migration request is submitted for academic evaluation.**

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Validate that Programme-changing mobility is permitted for the Student. | Academic Progression | Eligibility position established. |
| 2 | Validate destination Programme availability/seat conditions where required. | Academic Progression / Academic Structure | Destination is eligible for consideration. |
| 3 | Map the Student's current academic position to the destination curriculum. | Academic Progression / Academic Structure | Curriculum mapping completed. |
| 4 | Map recognised academic credits to the destination Programme. | Academic Progression / Academic Structure | Credit mapping completed. |
| 5 | Establish the institutional migration decision. | Academic Progression | Approved or not approved. |
| 6 | When approved, publish `ProgrammeMigrationApproved`. | Academic Progression | Authoritative migration outcome is available to downstream consumers. |

The flow stops at the approved migration outcome; C02 then applies the Programme association update within its own lifecycle boundary.

---

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result | Responsible Domain |
|---|---|---|---|---|
| A1 | Eligibility fails | Do not approve migration and preserve existing Programme association. | Migration not approved. | Academic Progression |
| A2 | Curriculum mapping cannot establish a valid destination mapping | Do not approve until mapping is valid. | Migration not approved/deferred according to policy. | Academic Progression / Academic Structure |
| A3 | Destination capacity is unavailable where seat validation applies | Migration cannot be approved. | Existing Programme remains authoritative. | Academic Progression |

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required Programme/curriculum information is unavailable | Migration decision cannot be completed. | Resolve authoritative academic structure information before decision. | Academic Structure / Academic Progression |
| Credit mapping is incomplete | Migration cannot be approved under BR-MIG-004. | Complete valid mapping before approval. | Academic Progression |
| Migration is rejected | Existing Programme remains authoritative. | Close the migration decision as not approved; preserve history. | Academic Progression |
| Approved migration requires downstream Programme-association update | Academic decision is complete but Student Context is not yet updated. | C02 applies the authoritative `ProgrammeMigrationApproved` result without making a second migration decision. | Student Lifecycle |

No technical recovery is defined.

---

## 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| **BR-MIG-001** All Programme-changing mobility is governed by APW-08 and approved institutional processes. | Academic Progression | No alternative migration path may bypass APW-08. | L2-C04 §6.10 |
| **BR-MIG-002** Migration preserves academic history. | Academic Progression | Historical academic record remains intact. | L2-C04 §6.10 |
| **BR-MIG-003** Migration preserves recognised academic credits unless policy specifies otherwise. | Academic Progression | Credit continuity is maintained subject to policy. | L2-C04 §6.10 |
| **BR-MIG-004** Curriculum mapping must complete before approval; `ProgrammeMigrationApproved` is the authoritative result for C02. | Academic Progression | Approval requires mapping completion and controls downstream hand-off. | L2-C04 §6.10 |
| C02 Programme-association update rule | Student Lifecycle | C02 applies the approved result without re-evaluation. | L2-C02 WF-C02-04 |

---

## 13. State and Lifecycle Impact

| Owned Concept / Aggregate | Entry Position | Scenario Impact | Result | Canonical Owner | Source |
|---|---|---|---|---|---|
| Programme Migration | Request under evaluation | Eligibility/mapping/approval | Approved or not approved | Programme Migration within Academic Progression | L2-C04 APW-08 |
| Student Context | Existing Programme association | Approved migration is consumed | Programme association updated | Student Lifecycle | L2-C02 |

The L3 scenario does not create a second Programme lifecycle or transfer C02 ownership.

---

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| `ProgrammeMigrationApproved` | Programme Migration aggregate within Academic Progression | Student Lifecycle, Academic Registration, Curriculum | Communicates the authoritative approved migration outcome. | L2-C04 §7.4–7.7 |
| Resulting Student Context update event | Student Lifecycle | Relevant downstream consumers | Communicates C02's own lifecycle update after consuming the approved result. | L2-C02 WF-C02-04 |

Only the first event is part of the migration decision itself.

### 14.2 Synchronous / Business Hand-off

Academic Structure supplies Programme and curriculum facts. Student Lifecycle consumes the approved migration outcome to update its own Student Context. Academic Registration subsequently owns registration consequences.

---

## 15. Business Authorization, Approval and Audit Obligations

Programme Migration is an institutional academic decision.

Eligibility, mapping, approval authority and the final decision must remain auditable. The migration outcome must preserve academic history.

No technical access control/IAM structure is defined.

---

## 16. Completion Criteria and Postconditions

- Eligibility has been evaluated.
- Curriculum mapping is complete.
- Credit mapping is complete.
- Institutional migration decision is established.
- If approved, `ProgrammeMigrationApproved` is published.
- Student Lifecycle may then apply the authoritative Programme association update within C02 ownership.
- Historical academic records remain preserved.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

**OI-C033-01 — Exact institutional migration eligibility and seat-allocation criteria are not enumerated in the available C04 excerpt.**

**OI-C033-02 — Exact migration approval role is not universally named.** No new role is invented.

**OI-C033-03 — Detailed downstream registration consequence is governed by Academic Registration and is not expanded into this L3 outcome.**

### Assumptions

1. APW-08 is the sole canonical Programme Migration decision workflow.
2. Academic Structure/Curriculum remains authoritative for destination programme structure.
3. C02 only applies the approved migration result.

### Deferred Questions

- Institution-specific migration eligibility criteria.
- Exact approval authority.
- Detailed post-migration registration sequence.


## Production Verification

- [x] Frozen Stage 3 title, owner, participating domains, type and priority preserved.
- [x] Final L3 identity remains in Section C and filename is derived from canonical title.
- [x] L2-C04 Academic Progression is the primary governing workflow source.
- [x] Cross-domain dependencies preserve canonical ownership.
- [x] No new actors, authorities, rules, states, events or publishers have been invented.
- [x] Business-level recovery only; no technical retry/queue/rollback mechanics.
- [x] L3 remains implementation-independent.
- [x] All 17 locked template sections are present.
- [x] Material source gaps are recorded in Section 17.
