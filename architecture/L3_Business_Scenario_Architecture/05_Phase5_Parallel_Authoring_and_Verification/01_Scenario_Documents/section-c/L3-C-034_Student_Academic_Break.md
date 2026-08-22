# Student Academic Break

**Scenario ID:** L3-C-034  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Progression  
**Participating Domains:** C  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-C04_Student_Academic_Progression_Workflows.md`; `L2-C02_Student_Lifecycle_and_Activation.md`; applicable Academic Calendar/Institutional Policy architecture  
**Referenced By:** L3-C-035 Student Rejoining After Academic Break

## 1. Scenario Metadata

Frozen identity: **C034 — Student Academic Break**; Primary Owner **Academic Progression**; Participating Domain **C**; Type **Critical**; Priority **P1**; Derivation **KEEP**.

---

## 2. Scenario Purpose

To govern an approved temporary interruption of a Student's academic progression while preserving academic history, recognised credits and standing.

Academic Progression owns the Academic Break lifecycle and the decision to suspend progression under the approved institutional process.

---

## 3. Business Outcome

An eligible Student is placed into the canonical **Academic Break** state through an approved institutional process, with academic history, credits and standing preserved.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a Student requests an approved academic interruption or an institutionally valid break process is initiated.

### 4.2 End Boundary

The scenario ends when the Academic Break is approved and the Student's Academic Progression state becomes **Academic Break**.

### 4.3 In Scope

- Academic Break request.
- Eligibility/reason review.
- Institutional approval.
- Suspension of academic progression.
- Preservation of academic history, credits and standing.
- Publication of the approved break outcome.

### 4.4 Out of Scope

- Student Rejoining after the break.
- Modification of historical academic records.
- Academic Registration changes after break approval.
- Student identity lifecycle changes.
- Technical suspension mechanisms.

### 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Followed by | L3-C-035 Student Rejoining After Academic Break | Rejoining governs restoration after an approved interruption. |

---

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Academic Progression | Owns Academic Break approval, lifecycle and progression interruption. |

Because the frozen scenario is C-only, no additional participating domain is introduced merely because downstream operational domains may react to `AcademicBreakApproved`.

---

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Academic Progression | Evaluates and approves the Academic Break under institutional policy. | Canonical owner. |

The exact approving institutional role is not universally named by the source architecture and is therefore not invented.

---

## 7. Preconditions and Entry Criteria

- Student has an active Academic Progression context.
- Academic Break is permitted by institutional regulation.
- The request/reason satisfies the applicable institutional process.
- The required approval authority is available.

---

## 8. Trigger

**A Student submits or becomes subject to an institutionally permitted Academic Break request.**

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Receive the Academic Break request and establish its business basis. | Academic Progression | Break case is established. |
| 2 | Evaluate the request against applicable institutional Academic Break rules. | Academic Progression | Eligibility is determined. |
| 3 | Obtain the required institutional approval. | Academic Progression | Break is approved or not approved. |
| 4 | Place the Student in **Academic Break** when approved. | Academic Progression | Progression is temporarily interrupted. |
| 5 | Preserve academic history, credits and standing. | Academic Progression | Academic continuity is retained. |
| 6 | Publish `AcademicBreakApproved`. | Academic Progression | Downstream consumers receive the authoritative break fact. |

---

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result | Responsible Domain |
|---|---|---|---|---|
| A1 | Break request is not permitted | Reject the request. | Student remains in current progression state. | Academic Progression |
| A2 | Required approval is not obtained | Do not enter Academic Break. | Existing progression state remains authoritative. | Academic Progression |

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Break request lacks required evidence | Approval cannot be finalized. | Obtain the required business evidence and re-evaluate under policy. | Academic Progression |
| Break duration would violate institutional regulation | Requested interruption cannot be approved as submitted. | Apply the permitted institutional decision. | Academic Progression |
| Approved break exists | Normal progression is suspended. | Resume through C035 Student Rejoining after an approved rejoining decision. | Academic Progression |

No technical retry or rollback behaviour is defined.

---

## 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| **BR-BRK-001** Academic Break requires institutional approval. | Academic Progression | No break state without approval. | L2-C04 §6.11 |
| **BR-BRK-002** Academic history remains unchanged during approved interruption. | Academic Progression | Historical record is preserved. | L2-C04 §6.11 |
| **BR-BRK-003** Credits earned before interruption remain preserved. | Academic Progression | Credit continuity is retained. | L2-C04 §6.11 |
| **BR-BRK-004** Academic Break duration complies with institutional regulations. | Academic Progression | Duration is policy constrained. | L2-C04 §6.11 |

---

## 13. State and Lifecycle Impact

| Owned Concept / Aggregate | Entry State | Scenario Impact | Resulting State | Canonical Owner | Source |
|---|---|---|---|---|---|
| Academic Progression | Progressed / applicable active progression position | Approved interruption | Academic Break | Academic Break aggregate within Academic Progression | L2-C04 §4.4 |
| Academic history / Credit Ledger | Existing historical position | Preservation only | Unchanged | Canonical respective owners | L2-C04 §6.11 |

The scenario does not modify historical states or create a new break subtype.

---

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| `AcademicBreakApproved` | Academic Break aggregate within Academic Progression | Student Lifecycle, Registration, Hostel, Library, Finance | Communicates the approved interruption. | L2-C04 §7.4–7.7 |

The event publisher remains Academic Break within Academic Progression; participating consumers do not gain ownership.

### 14.2 Synchronous / Business Hand-off

No additional synchronous business hand-off is required to establish the Academic Break outcome at L3.

---

## 15. Business Authorization, Approval and Audit Obligations

Academic Break requires institutional approval.

The request, reason, approval, effective interruption and preserved academic context must remain auditable.

The exact approving role is left to upstream institutional governance where not explicitly named.

---

## 16. Completion Criteria and Postconditions

- Academic Break approval is established.
- Student enters the canonical Academic Break state.
- Academic history remains preserved.
- Credits earned before interruption remain preserved.
- Academic standing remains preserved.
- `AcademicBreakApproved` is published.
- Rejoining is not part of this scenario; it is C035.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

**OI-C034-01 — Exact Academic Break eligibility categories and maximum durations are institution-specific and are not fully enumerated in C04.**

**OI-C034-02 — Exact approving institutional role is not universally named.**

### Assumptions

1. Academic Progression owns the Academic Break lifecycle.
2. Approved break preserves historical academic information and credits.
3. C035 owns the subsequent rejoining outcome.

### Deferred Questions

- Institution-specific break categories.
- Maximum duration rules.
- Exact approval authority.


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
