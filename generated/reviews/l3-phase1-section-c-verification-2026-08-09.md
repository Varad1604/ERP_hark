# SutraERP L3 Phase 1 — Chetan Section C Verification (2026-08-09)


> **Numbering note (2026-08-17).** Candidate IDs in this historical report reflect the numbering in force when it was written. Repository-wide candidate numbering was normalized on 2026-08-17; for current IDs see `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Phase1_Candidate_ID_Migration_2026-08-17.md`.
**Audit type:** Independent, adversarial, evidence-based, repository-first verification
**Subject artifact:** `architecture/L3_Business_Scenario_Architecture/01_Phase1_Discovery/L3_Phase1_Inventory_Chetan.md`
**Discovery area audited:** Section C — Student Lifecycle (`L2-C00` … `L2-C08`)
**Candidates reviewed at audit time:** 71 (`CAN-L3-C-001` … `CAN-L3-C-071`, contiguous, no duplicate IDs)
**Date:** 2026-08-09
**Supersedes:** `l3-phase1-section-c-verification-2026-08-08.md`, which audited the pre-remediation 45-row / 36-ID version of the inventory

**Scope guard:** Phase 1 candidate discovery only. This audit created no L3 documents, assigned no final L3 numbering, locked no ownership, and modified no L1 or L2 content. No API, database, UI, schema, or deployment content appears in any proposed candidate.

---

## 1. Verdict at audit time

```text
FAIL — BOTH COVERAGE AND ARCHITECTURE PROBLEMS
```

```text
Existing candidates: 71
Credible missing candidates: 3
Candidates needing merge: 0
Candidates needing split: 1
Candidates needing rewording: 9
Ownership/boundary defects: 3
Potential duplicates: 4 (2 OVERLAP, 2 RELATED-BUT-INDEPENDENT — no true duplicates)
Blocked by repository conflict: 2 (2 further conflict-dependent)
```

All findings in sections 3–6 were applied to the inventory on 2026-08-09. See section 8.

---

## 2. Executive assessment

A near-miss, not a weak inventory. The submission is a substantially rebuilt one: contiguous unique IDs, all eight locked columns, only legal `Type` and `Priority` values, section headings for every L2 source, correct `Prepared By` and `Status`. No violation of the locked Phase 1 row format was found.

Coverage is strong where earlier discovery was weakest. C04 carries one candidate per APW workflow (024–035, twelve for twelve) plus a progression-revision candidate derived from `L2-C04` §4.4 `Detained → Evaluation Running : Appeal Approved`. C06 carries the full GW-01…GW-12 catalogue plus reinstatement. C07 carries DW-01…DW-10 plus the Finance hand-off. C05 covers all six §9.3 alternative exits including Administrative Exit and Deceased, plus `QualificationAwardRevoked`. Hard boundaries hold: Examination results are consumed and never owned; Course Offering stays in D; the Academic Credit Ledger stays with Academic Progression across the ABC hand-off; Person identity stays with A.

It nevertheless failed on three verifiable grounds.

**The front half of the Admission lifecycle had no candidate.** `CAN-L3-C-001`'s trigger read *"Admission is approved…"*. Under that trigger the span `Admission Initiated → Documents Submitted → Verification Completed → Admission Approved` (`L2-C01` §6.4, §6.9.1) fell outside every candidate. Two full workflows live there — WF-C01-02 Document Verification and WF-C01-03 Admission Approval, the latter with five approval configurations (AF-01…AF-05), an escalation path, and a published `AdmissionRejected` event.

**`Registration Hold` had full canonical ownership and zero coverage.** `L2-C03` §5.3 lists `Registration Hold | Full` among Owned Business Objects, §6.6 defines `RegistrationHold` as an owned child entity, §4.3 names Registration Hold Management a core capability, §7.14 lists it as an exceptional workflow path, §12.8 defines the governed `OverrideRegistrationHold()` service, and §13 records Finance as a collaborating source of holds.

**Canonical StudentContext transfer semantics were contradicted, not covered.** `CAN-L3-C-011` stated the outcome as *"Student Context reflects the completed intra-institution transfer"*. `L2-C02` §11.10.3 states the opposite for the Institute-crossing case: `Transferred (closed) + Pending Activation (new)` — the originating context becomes terminal and a new context opens at the receiving Institute, governed by SC-INV-004.

Two candidates additionally redefined locked lifecycle rules: `CAN-L3-C-005` asserted that readmission restores the admission lifecycle, where `L2-C01` §6.13 states no terminal Admission Record shall ever be reopened and §7.6 rule 6 states readmission creates a **new** Admission Record; and `CAN-L3-C-034` assigned the ABC integration partner to Section A, where the repository places APAAR & ABC integration in `L2-H01`.

Outside these, the adversarial reconstruction (section 7) matched the inventory.

---

## 3. Missing candidates — added

| ID | Scenario Title | Primary Owner | Domains | Type | Priority | Source evidence | Why independent |
|---|---|---|---|---|---|---|---|
| `CAN-L3-C-072` | Admission Document Verification and Approval Decision | Admissions | A, B, C | Critical | P0 | `L2-C01` §6.4 Primary Lifecycle; §6.6 states *Documents Submitted*, *Verification Completed*, *Admission Approved*; §6.9.1 transition matrix; §7.3 catalogue WF-C01-02, WF-C01-03; WF-C01-03 §2.6 AF-01…AF-05; WF-C01-03 §3.2 events `AdmissionApproved`, `AdmissionRejected`; WF-C01-02 §2.7 EX-01…EX-06 | The approval decision is made by a distinct institutional authority under configurable single/multi-level/parallel/committee chains, gates every downstream admission state, and produces its own published institutional fact. `CAN-L3-C-001` begins after it, by its own stated trigger. **Repository Fact** for both workflows; **Logical L3 Composition** for combining verification and approval into one decision outcome. |
| `CAN-L3-C-073` | Registration Hold Placement and Release | Academic Registration | C, F | Cross-domain | P1 | `L2-C03` §4.3; §5.3 `Registration Hold — Full`; §6.4/§6.6 child entity `RegistrationHold`; §7.14 exceptional paths; §12.8 `OverrideRegistrationHold()`; §13 "Finance — Financial clearance and registration holds (where applicable)" | A hold is raised by a party outside the registration transaction, carries its own authorization, justification and audit obligation, persists across registration attempts, and its release is a separately authorized governed act. It exists whether or not a registration is attempted. **Repository Fact**. |
| `CAN-L3-C-074` | Inter-Institute Student Transfer with Student Context Re-establishment | Student Lifecycle | B, C | Critical | P1 | `L2-C02` §11.10.1 `Transferred` (Terminal = Yes); §11.10.3 mapping row *"the originating context becomes terminal; a new context opens at the receiving Institute"*; §11.10.2 SC-INV-004, SC-INV-005; §11.2/§11.5 | Ends in a terminal context **plus** a newly created context — materially different from `CAN-L3-C-011`, whose outcome updates the existing context, and from `CAN-L3-C-040` Transfer Exit, which ends the relationship with no receiving context. **Repository Fact** for the mapping and invariants; **Logical L3 Composition** for stating it as one end-to-end outcome. |

---

## 4. Split applied

| Existing candidate | Split | Reason |
|---|---|---|
| `CAN-L3-C-011` | (a) retitled **Intra-Institute Student Academic Reassignment**, restricted to same-Institute campus/department/batch/section change per `L2-C02` WF-C02-08 §1 and §11.5 `Transferred → Active`; (b) becomes `CAN-L3-C-074` | The single row asserted the context "reflects the completed transfer", true only for the same-Institute form. `L2-C02` §11.10.3 defines the Institute-crossing form as originating context closed plus a new context under SC-INV-004. Two different terminal outcomes cannot share one candidate. |

---

## 5. Rewording and boundary corrections applied

| Candidate | Problem | Correction |
|---|---|---|
| `CAN-L3-C-001` | Trigger *"Admission is approved…"* excluded the `Initiated → Approved` span from the inventory. | Trigger retained; span covered by new `CAN-L3-C-072`. The alternative remedy — widening 001's trigger — was rejected because the approval decision has its own authority chain. Only one remedy applied. |
| `CAN-L3-C-004` | Outcome did not reflect the policy-controlled `Student Created → Revoked` path in `L2-C01` §6.9.1, where a Student Context already exists. | Outcome extended to name the governed hand-off of the consequential Student Context disposition to Student Lifecycle. Conflict-dependent — see section 6, conflict #6. |
| `CAN-L3-C-005` | Outcome *"the admission lifecycle is restored"* contradicted `L2-C01` §6.13 and §7.6 rule 6. `Type = Cross-domain` with `Domains = C` only. | Outcome restated as "a new Admission Record is created and linked to the same canonical Person, with the previous terminal Admission Record preserved immutably and closed". `Domains` set to `A, C`. |
| `CAN-L3-C-011` | See section 4. | Retitled and scope-restricted. |
| `CAN-L3-C-022` | Omitted Improvement Registration, a named owned capability. | Retitled "Backlog, Repeat and Improvement Course Registration"; trigger and outcome extended per `L2-C03` §4.3 and §15. |
| `CAN-L3-C-029` | `Domains = C` only, but `L2-C04` APW-07 lists External University, ABC, NEP Transfer and Exchange Programme as sources. | `Domains` set to `B, C, H`. |
| `CAN-L3-C-034` | `Domains = A, C`. The repository places APAAR & ABC integration in `L2-H01` (`ARCHITECTURE_INDEX:221`; `L2-C04` §18.10). | `Domains` set to `C, H`. Credit Ledger ownership statement retained — correct per `L1` §4. |
| `CAN-L3-C-046` | Did not name qualification-completion cancellation before conferment. | Title and outcome extended per `L2-C05` §11.12. |
| `CAN-L3-C-059` | Outcome stopped at "published and active"; `L2-C07` §6.6 continues through `Suspended`, `Expired`, `Archived`. | Retitled "Scholarship Definition Lifecycle Governance"; outcome extended across the full definition lifecycle. |
| `CAN-L3-C-066` | `Type = Cross-domain` with `Domains = C` only — inconsistent with Discovery Rules §12. | `Type` set to `Simple`. `L2-C07` §2.2 places external portal business logic outside the domain, so no second domain genuinely participates. |

---

## 6. Repository conflicts — not resolved here

Recorded in `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Section_C_Phase2_Consolidation_Watchlist.md`. Summary:

| # | Conflict | Sources | Status |
|---|---|---|---|
| 1 | `Rejected` used as an admission state and published as `AdmissionRejected`, but present in no canonical state list. | `L2-C01` WF-C01-03 §2.7 EX-01, §3.2 vs §6.8, §6.9, §6.9.1, §6.13, §10.3 | Conflict-dependent. Negative decision carried inside `CAN-L3-C-072`; no standalone rejection candidate created. |
| 2 | Two incompatible admission state vocabularies. | `L2-C01` §6.4–§6.9 vs §10.3, §10.7 | Open. C01 candidates quote §6. |
| 3 | §6.13 forbids reopening a terminal admission; §10.3 defines a `Readmitted` state meaning "Admission restored". | `L2-C01` §6.13 / §7.6 rule 6 vs §10.3 | Determined. §6.13 applied to `CAN-L3-C-005`. |
| 4 | Guardian capability defined twice — `L2-C02` WF-C02-13.3 claims guardian registration, replacement, Primary Guardian designation and emergency contacts; `L2-C06` §5 declares the same capabilities exclusively owned. | `L2-C02` WF-C02-13 vs `L2-C06` §2, §5 | Open. Candidates sit under C06; the reconciliation is now explicit rather than silent. |
| 5 | Alumni Transition ownership stated three ways. | `L2-C04` §1.5, `L2-C05` §5/§11.10, `L2-C08` §5.2 | Open. `CAN-L3-C-069` follows `L2-C08`. |
| 6 | `Student Created → Revoked (Policy Controlled)` allowed, but WF-C01-08 §1.4 excludes academic record management and defines no consequence. | `L2-C01` §6.9.1 vs WF-C01-08 §1.4 | Conflict-dependent. `CAN-L3-C-004` names the hand-off without inventing the unwinding process. |
| 7 | ABC / APAAR integration partner ambiguous between Section A and Section H. | `L1` §4 vs `ARCHITECTURE_INDEX:221`, `L2-C04` §18.10 | Determined. H applied for credit synchronisation. |
| 8 | **Policy Management has no owner.** `L2-C08` (10 refs) and `L2-C04` (3 refs) depend on it; no publisher exists. | `DEPENDENCY_MAP` §7 (status **Open**); `L2-C04` §2.6 | **BLOCKED.** Progression-policy and alumni-transition-policy lifecycle candidates cannot be discovered. Chetan's omission is correct. |
| 9 | Internal inconsistency between the two C01 transition matrices. | `L2-C01` §6.9 vs §6.9.1 | Open, low impact. |

---

## 7. Candidates considered and deliberately rejected

Proving that exhaustive does not mean uncontrolled multiplication.

| Considered | Source | Rejected because |
|---|---|---|
| Admission Audit | `L2-C01` WF-C01-10 §1.1, §1.4 | "Rather than performing business operations" — cross-cutting obligation, not an outcome. |
| Admission Financial Clearance | `L2-C01` WF-C01-04, §6.6 | Lifecycle state and precondition of confirmation; a step inside `CAN-L3-C-001`, which already lists F. |
| Student Status Change | `L2-C02` WF-C02-05 | Generic transition mechanism; Discovery Rules §8 names "Update Student Status" a non-candidate. |
| Student Eligibility Validation (registration) | `L2-C03` WS-C03-02 | Validation gate; Discovery Rules §8 names "Check Eligibility" a non-candidate. |
| Late Registration / Administrative Override | `L2-C03` §7.14, BR-C03-009/010, §15.10 | Classified by the source as exceptional workflow paths and policy variants — ALTERNATIVE-FLOW of `CAN-L3-C-019`. |
| Registration Rejection | `L2-C03` §4.4, WS-C03-06 | Negative branch of the approval workflow inside `CAN-L3-C-019`. |
| Bulk Registration / Bulk Approval | `L2-C03` §4.4 | Execution mode, not a distinct outcome. |
| Registration Archival | `L2-C03` §6.9 `Frozen → Archived` | Retention disposition of a record already terminal under `CAN-L3-C-023`. |
| Progression Deferred | `L2-C04` §2.7 | No corresponding state in §4.3/§4.4; contained in 027 and 028. |
| Qualification Completion Cohort / Cycle setup | `L2-C05` §6, §11.1 trigger | Vocabulary and trigger variant only; no cycle-management workflow exists. |
| Degree Audit; Programme Completion Verification | `L2-C05` §11.2, §11.4 | Internal gates of `CAN-L3-C-036`; failure paths covered by `CAN-L3-C-046`. |
| Guardian Assignment (GW-03) | `L2-C06` §8.4 | Single transition `Verified → Active` inside `CAN-L3-C-048`. |
| Primary Guardian Designation | `L2-C06` §8.5 GW-04 "Contact priority"; BR-010/BR-012 | Explicitly an attribute maintained by GW-04 — contained in `CAN-L3-C-049`. |
| Guardian Duplicate Resolution | `L2-C06` GF-003, GV-003, GV-011 | Expressed only as a rejection rule, not a resolution process. Canonical duplicate resolution is `L2-A02` and `CAN-L3-C-015`. |
| Scholarship Recommendation Generation (DW-04) | `L2-C07` WS-04 | Composed into `CAN-L3-C-061`, whose outcome already names recommendations. |
| Scholarship Eligibility Suspension / Revocation | `L2-C07` §6.7 | No such states exist. Adding them would fabricate lifecycle states. |
| Student Reactivation from Archived | `L2-C02` §11.6 | The source prohibits it; the conditional carve-out names no defined process. |
| Alumni Transition Rejection / Failure | `L2-C08` §6.9.3, §6.11 | No rejected or failed status; the lifecycle is forward-only. |
| Admission Rejection (standalone) | `L2-C01` WF-C01-03 EX-01, §3.2 | Conflict #1 — `Rejected` is not a canonical state. Absorbed into `CAN-L3-C-072`. |
| Progression / Alumni Transition policy lifecycle | `L2-C04` §2.6, `L2-C08` policy refs | Conflict #8 — Policy Management has no owner. **Blocked.** |

---

## 8. Post-application state

All corrections in sections 3–5 were applied to `L3_Phase1_Inventory_Chetan.md` on 2026-08-09.

```text
Candidates before: 71   (CAN-L3-C-001 … CAN-L3-C-071)
Candidates added:   3   (CAN-L3-C-072, CAN-L3-C-073, CAN-L3-C-074)
Candidates after:  74
Merges executed:    0
Splits executed:    1   (CAN-L3-C-011 → CAN-L3-C-011 + CAN-L3-C-074)
Rows reworded:      9
Rows removed:       0
```

Post-application verdict:

```text
PASS — EXHAUSTIVE ENOUGH FOR PHASE 1
```

subject to the nine repository conflicts in section 6, which are architecture decisions rather than discovery defects and are carried to Phase 2 on the Section C watchlist. Two of them (#1, #6) constrain candidate wording; one (#8) blocks two candidates that must not be discovered until an owner is assigned.
