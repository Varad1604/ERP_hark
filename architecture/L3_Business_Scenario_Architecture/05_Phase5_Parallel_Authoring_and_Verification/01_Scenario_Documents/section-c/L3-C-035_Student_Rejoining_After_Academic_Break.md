# Student Rejoining After Academic Break

**Scenario ID:** L3-C-035  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Progression  
**Participating Domains:** C  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-C04_Student_Academic_Progression_Workflows.md`; `L2-C03_Student_Academic_Registration_Workflows.md`; `L2-C02_Student_Lifecycle_and_Activation.md`; applicable Curriculum Framework and Academic Calendar architecture  
**Referenced By:** L3-C-034 Student Academic Break; L3-C-021 Student Academic Registration

## 1. Scenario Metadata

Frozen identity: **C035 — Student Rejoining After Academic Break**; Primary Owner **Academic Progression**; Participating Domain **C**; Type **Critical**; Priority **P1**; Derivation **KEEP**.

---

## 2. Scenario Purpose

To restore a Student's academic progression following an approved Academic Break by verifying rejoining eligibility, considering curriculum changes, determining academic placement and establishing the authoritative rejoining outcome.

Academic Progression owns the rejoining decision. Academic Registration remains the canonical owner of the subsequent registration transaction.

---

## 3. Business Outcome

An eligible Student is authoritatively restored through the **Rejoining** path and becomes eligible to resume Academic Registration and normal Academic Progression.

Where rejoining is not approved, the existing Academic Break position remains governed by the applicable policy.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins with an approved Academic Break record and a valid request/process to resume studies.

### 4.2 End Boundary

The scenario ends when rejoining is approved and the Student reaches the canonical **Rejoining** position with registration eligibility restored.

The subsequent Academic Registration transaction remains outside this scenario.

### 4.3 In Scope

- Rejoining eligibility verification.
- Review of curriculum changes during interruption.
- Academic credit mapping.
- Semester/academic-period assignment.
- Rejoining approval.
- Restoration of registration eligibility.
- Publication of `StudentRejoined`.

### 4.4 Out of Scope

- Academic Break approval itself.
- Academic Registration creation or approval.
- Curriculum ownership.
- New progression evaluation after registration.
- Technical restoration mechanics.

### 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-C-034 Student Academic Break | Establishes the approved interruption. |
| Followed by | L3-C-021 Student Academic Registration | Registration resumes after rejoining eligibility is restored. |

---

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Academic Progression | Owns rejoining eligibility, curriculum review, credit mapping, semester assignment and rejoining approval. |

The frozen scenario is C-only. Academic Registration is a downstream dependency/consumer, not an added participating domain.

L2-C04 explicitly states that successful rejoining restores eligibility for Academic Registration, while C03 owns all registration transactions.

---

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Academic Progression | Evaluates and approves Student Rejoining. | Canonical owner. |
| Academic Registration | Acts on restored registration eligibility in its own registration workflow. | Downstream owner; not rejoining decision authority. |

No new rejoining-specific approval role is invented.

---

## 7. Preconditions and Entry Criteria

- An approved Academic Break record exists.
- The Student is eligible to seek rejoining under institutional rules.
- Current curriculum information is available.
- Relevant historical credits are available.
- Academic-period/semester allocation information is available.
- Rejoining is permitted under institutional regulations.

---

## 8. Trigger

**A Student with an approved Academic Break enters the governed process to resume studies.**

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Verify that the Student has an approved Academic Break and is eligible to rejoin. | Academic Progression | Rejoining eligibility is established. |
| 2 | Review curriculum changes that occurred during the interruption. | Academic Progression / Academic Structure | Current curriculum position is understood. |
| 3 | Map preserved academic credits to the current curriculum. | Academic Progression | Applicable credit position is established. |
| 4 | Determine the appropriate academic semester/period allocation. | Academic Progression | Rejoining academic placement is established. |
| 5 | Approve Student Rejoining. | Academic Progression | Student enters the canonical Rejoining position. |
| 6 | Restore eligibility for Academic Registration and publish `StudentRejoined`. | Academic Progression | Registration may resume through C03's own workflow. |

The flow stops when rejoining eligibility is restored; registration itself is a separate business outcome.

---

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result | Responsible Domain |
|---|---|---|---|---|
| A1 | Curriculum changed materially during break | Apply the governed curriculum/credit mapping before rejoining approval. | Rejoining proceeds only after valid mapping. | Academic Progression |
| A2 | Eligibility conditions are not satisfied | Do not approve rejoining. | Academic Break remains the governing state. | Academic Progression |
| A3 | Academic period allocation cannot be established | Defer approval until a valid institutional placement is determined. | Rejoining remains unresolved. | Academic Progression |

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Current curriculum differs from the pre-break curriculum | Historical credits may not map directly. | Complete governed curriculum and credit mapping before approval. | Academic Progression / Academic Structure |
| Rejoining eligibility is not satisfied | Student cannot resume academic progression. | Retain Academic Break status and apply the permitted institutional resolution. | Academic Progression |
| Semester/period placement cannot be established | Rejoining cannot be finalized. | Resolve the applicable academic placement under institutional rules. | Academic Progression |
| Rejoining is approved but registration has not yet completed | Student is eligible but has not resumed registration. | Academic Registration executes its own registration workflow. | Academic Registration |

No technical retry or rollback behaviour is defined.

---

## 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| **BR-REJ-001** Only approved Academic Break records may initiate Student Rejoining. | Academic Progression | No rejoining without an approved break. | L2-C04 §6.12 |
| **BR-REJ-002** Rejoining validates curriculum changes occurring during interruption. | Academic Progression | Current curriculum must be considered. | L2-C04 §6.12 |
| **BR-REJ-003** Semester assignment complies with institutional regulations. | Academic Progression | Placement is policy governed. | L2-C04 §6.12 |
| **BR-REJ-004** Successful rejoining restores eligibility for Academic Registration. | Academic Progression / Registration boundary | Registration may resume only after rejoining. | L2-C04 §6.12 |
| C03 registration ownership | Academic Registration | Rejoining does not create or modify registration. | L2-C03 |

---

## 13. State and Lifecycle Impact

| Owned Concept / Aggregate | Entry State | Scenario Impact | Resulting State | Canonical Owner | Source |
|---|---|---|---|---|---|
| Academic Progression | Academic Break | Rejoining approved | Rejoining | Student Rejoining within Academic Progression | L2-C04 §4.4 |
| Student Rejoining | Rejoining | Registration later completed | Progressed | Academic Progression | L2-C04 §4.4 |
| Academic Registration | Registration not yet active for resumed period | Rejoining restores eligibility | Registration may be initiated through C03 | Academic Registration | L2-C03 |

The scenario ends at restored registration eligibility, not at completion of registration.

---

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| `StudentRejoined` | Student Rejoining aggregate within Academic Progression | Academic Registration, Student Lifecycle, Timetable | Communicates completion of the rejoining decision. | L2-C04 §7.4–7.7 |

### 14.2 Synchronous / Business Hand-off

Academic Registration consumes the restored eligibility and performs its own registration workflow. It does not determine whether the Student is eligible to rejoin.

---

## 15. Business Authorization, Approval and Audit Obligations

Student Rejoining is an institutional Academic Progression decision.

The approved break reference, eligibility determination, curriculum/credit mapping, academic placement and rejoining outcome must remain auditable.

No technical access control/IAM design is introduced.

---

## 16. Completion Criteria and Postconditions

- Approved Academic Break is confirmed.
- Rejoining eligibility is verified.
- Current curriculum changes are considered.
- Preserved credits are mapped.
- Semester/academic-period assignment is established.
- Rejoining is approved.
- Student reaches the canonical Rejoining position.
- Registration eligibility is restored.
- `StudentRejoined` is published.
- Academic Registration may then execute its own registration transaction.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

**OI-C035-01 — Exact rejoining eligibility criteria are not fully enumerated in C04.**

**OI-C035-02 — Exact semester/academic-period placement rules are institution-specific.**

**OI-C035-03 — Detailed registration hand-off semantics remain under C03 and are intentionally not duplicated here.**

### Assumptions

1. Only an approved Academic Break can initiate rejoining.
2. Academic Progression owns the rejoining decision.
3. Academic Registration owns the subsequent registration transaction.
4. Curriculum changes during the interruption must be considered.

### Deferred Questions

- Institution-specific rejoining eligibility.
- Exact semester-placement rules.
- Detailed downstream registration sequence.


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
