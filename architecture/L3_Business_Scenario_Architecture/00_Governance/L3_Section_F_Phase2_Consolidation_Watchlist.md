# L3 Section F — Phase 2 Consolidation Watchlist

**Architecture Level:** L3 — Business Scenario Architecture
**Applies To:** Section F — Finance, current candidates `CAN-L3-F-001` … `CAN-L3-F-087`; former candidates `CAN-L3-F-0001` … `CAN-L3-F-1858`
**Status:** Audit consolidation applied — retained as traceability and L4-input register
**Created:** 2026-08-12
**Source:** independent adversarial audit of Section F, 2026-08-12 — `generated/reviews/L3_Phase1_Audit_Varad_Section_F.md`
**Conformance applied:** 2026-08-12, recorded in section 8 below

---

## 1. Purpose

This document holds the candidate relationships, boundary questions and deferred decisions identified during the independent Section F audit of 2026-08-12, and records for each one whether it is settled or still requires a Phase 2 decision.

It exists as a separate document because `L3_Phase1_Discovery_Rules.md` section 3 requires each Phase 1 inventory file to contain candidate scenario rows only, and section 16 requires the inventory to avoid long explanations.

The consolidation described here was applied on 2026-08-12 after the user accepted the independent all-assigned audit recommendations. Section F now contains 87 institutional-outcome candidates. The former ranges below are retained as the audit basis and as traceability for detailed L4 rules, guards, events and invariants.

This document is not a governance standard. It adds and amends no Phase 1 rule and resolves no architectural conflict.

It follows the structure established by `L3_Section_A_Phase2_Consolidation_Watchlist.md`, `L3_Section_C_Phase2_Consolidation_Watchlist.md` and `L3_Section_G_Phase2_Consolidation_Watchlist.md`.

---

## 2. Scope Warning

Section F is one of three sections assigned to `L3_Phase1_Inventory_Varad.md` under `L3_Phase1_Workload_Distribution_Decision.md` section 2.

Section H was corrected and completed on 2026-08-12 after this watchlist was first written. The former `F-FR-13` blocker is resolved: H07 owns dashboard composition; F13 owns financial facts, report logic, KPI definitions and dashboard-ready projections.

This supersedes the original statement in `L3_Section_G_Phase2_Consolidation_Watchlist.md` section 2 that *"Sections F and H remain entirely undiscovered"* — both sections are now present.

---

## 3. Status Values

| Status | Meaning | Phase 2 action |
| --- | --- | --- |
| `CLOSED` | Repository evidence establishes the candidates are distinct or the question is settled. | None. Do not re-raise. |
| `DETERMINED` | Repository evidence is one-sided. The decision is recorded; only its execution is deferred. | Execute the recorded decision. |
| `OPEN` | A genuine Phase 2 judgment is required. | Decide, then execute. |
| `BLOCKED` | Cannot be decided from current repository evidence. | Resolve the blocker first. |

---

## 4. Scale Statement

Before correction, Section F carried **1,858 candidates against an estimated 75–90 materially distinct institutional business outcomes** — a fragmentation ratio near 20:1. The corrected inventory contains **87** outcome-level candidates.

The estimate was audit inference from the L2 lifecycle, workflow and ownership sections. The current count falls within that evidence-backed range; final approval remains a Phase 2 governance action.

For comparison, Section G still carries 541 provisional candidates across eleven sub-domains; corrected Section F carries 87 across fifteen L2 sources.

---

## 5. Historical Fragmentation Clusters — Applied

Each cluster is the former candidate run for one L2 sub-domain. Its applied current boundary is recorded in section 11.

| ID | Cluster | Candidates | Recommended consolidated boundary | Ratio | Status |
| --- | --- | ---: | --- | ---: | --- |
| `F-FR-00` | F00 Foundation | 25 — `F-0001`…`F-0025` | Retain as the **consolidation target set**. Already correctly bounded. | 1:1 | `APPLIED` — used as the consolidation map; see §11 |
| `F-FR-01` | F01 Student Charge & Fee | 31 — `F-0026`…`F-0056` | ~7: charge generation, publication, revision, cancellation, waiver, hold, per-source origination | 4:1 | `APPLIED` — see §11 |
| `F-FR-02` | F02 Payment | 70 — `F-0057`…`F-0126` | ~7 centred on `Student Payment to Receipt and Accounting Completion` (`F-0003`) | 10:1 | `APPLIED` — see §11 |
| `F-FR-03` | F03 Refund | 260 — `F-0127`…`F-0386` | ~6 centred on `Approved Refund to Refund Completion` (`F-0006`). Evidence: `L2-F03` §5 flow, line 2250 | 43:1 | `APPLIED` — see §11 |
| `F-FR-04` | F04 Financial Adjustment | 190 — `F-0387`…`F-0576` | ~5: credit note, debit note, write-off adjustment, reversal, correction | 38:1 | `APPLIED` — see §11 |
| `F-FR-05` | F05 Accounting & GL | 125 — `F-0577`…`F-0701` | ~8 centred on `Financial Transaction to Balanced Accounting Posting` (`F-0013`) | 16:1 | `APPLIED` — see §11 |
| `F-FR-06` | F06 Budget & Cost Center | 84 — `F-0702`…`F-0785` | ~6: budget proposal to active budget, revision, virement, allocation, utilization control, cost centre lifecycle. Evidence: `L2-F06` lines 485–562 | 14:1 | `APPLIED` — see §11 |
| `F-FR-07` | F07 Accounts Receivable | 94 — `F-0786`…`F-0879` | ~5: recognition, ageing, dunning, settlement, write-off | 19:1 | `APPLIED` — see §11 |
| `F-FR-08` | F08 Accounts Payable | 275 — `F-0880`…`F-1154` | ~7 centred on `Vendor Invoice to Vendor Financial Settlement` (`F-0010`) | 39:1 | `APPLIED` — see §11 |
| `F-FR-09` | F09 Period & Closing | 284 — `F-1155`…`F-1438` | ~5: period establishment to open period; period closing to locked period; financial year closing; controlled period reopening; closed period archival. Evidence: `L2-F09` lines 457–470 define exactly one state machine | **57:1** | `APPLIED` — see §11 |
| `F-FR-10` | F10 Treasury & Banking | 54 — `F-1439`…`F-1492` | ~6: bank account lifecycle, bank reconciliation, cash position, fund transfer | 9:1 | `APPLIED` — see §11 |
| `F-FR-11` | F11 Scholarship & Financial Aid | 82 — `F-1493`…`F-1574` | ~5: aid decision to financial allocation, disbursement, adjustment, recovery | 16:1 | `APPLIED` — see §11 |
| `F-FR-12` | F12 Asset Financial Accounting | 62 — `F-1575`…`F-1636` | ~6: capitalization, depreciation, revaluation, impairment, disposal, write-off accounting. Evidence: `L2-F12` §2, §4, §5 — a 210-line boundary document | 10:1 | `APPLIED` — see §11 |
| `F-FR-13` | F13 Reporting & Analytics | 125 — `F-1637`…`F-1761` | Retain financial report, KPI and projection outcomes in F13; consolidate dashboard composition onto H07. | 35:1 | `APPLIED` — F13/H07 boundary recorded in current candidates and §11 |
| `F-FR-14` | F14 Procurement Consumption | 97 — `F-1762`…`F-1858` | ~4: budget commitment, three-way match participation, vendor liability recognition, procurement accounting closure. Evidence: `L2-F14` — a 255-line boundary document | 24:1 | `APPLIED` — see §11 |

**Cluster count: 15.**

### 5.1 Cross-cutting sub-clusters

These cut across the sub-domain clusters above and should be decided once, not fifteen times.

| ID | Pattern | Approx. rows | Nature | Status |
| --- | --- | ---: | --- | --- |
| `F-FR-X1` | `Preserve …` / `Prevent …` | ~40 | Architectural invariants from L2 constraint sections rendered as candidates. No trigger, no journey, no outcome. Examples: `Preserve Tenant and Institution Context`, `Prevent Finance from Modifying Operational Asset State` | `APPLIED` — absorbed into parent scenarios as rules |
| `F-FR-X2` | `Publish … Fact` / `Publish … Event` | ~30 | Event emission as candidate. `L3_Phase1_Discovery_Rules.md` §8 forbids `Send Notification` | `APPLIED` — absorbed as published outcomes of parent scenarios |
| `F-FR-X3` | `Consume … Fact` | 13 | Maps 1:1 onto the 13 bullets of `L2-F14` §6 "Representative Consumed Events". Event consumption is an integration mechanic | `APPLIED` — absorbed as scenario triggers |
| `F-FR-X4` | `Reject …` guard clauses | ~120 | Individual validation-failure branches, e.g. `F09` rows `Reject period registration without Financial Year`, `Reject opening when predecessor period is incomplete` | `APPLIED` — merged into parents as alternate outcomes |
| `F-FR-X5` | Cross-cutting configuration change | ~15 | One `… Configuration Change to Governed Active Configuration` per sub-domain | `APPLIED` — consolidated into `F-084`/`F-085`; Finance owns policy and Section A supplies the mechanism |
| `F-FR-X6` | Cross-cutting audit history | ~20 | One immutable-audit-history candidate per sub-domain | `APPLIED` — audit capture retained in Finance parents; A07 owns the institutional audit trail |

---

## 6. Over-Broad Candidates — Split Review

| ID | Candidate | Issue | Status |
| --- | --- | --- | --- |
| `F-SP-01` | `CAN-L3-F-0001` Business Event to Financial Charge | Trigger is *any* financially consequential event from *any* upstream domain. Admission, hostel, transport, examination-fee and library-fine charges have different triggers, owners and approval paths. `CAN-L3-F-0028` already exists as one such child. | `APPLIED` — split across current charge outcomes `F-001`…`F-006` |
| `F-SP-02` | `CAN-L3-F-0023` Financial Transaction to Statutory Compliance Outcome | Spans tax treatment, statutory approval and compliance recording across all Finance sub-domains | `APPLIED` — distributed to the applicable current reporting, settlement, accounting and closure outcomes |

**Split count: 2.**

---

## 7. Duplication and Cross-Inventory Findings

| ID | Finding | Status |
| --- | --- | --- |
| `F-DU-01` | The 25 F00 candidates are semantic parents of the F01–F14 runs. `F-0003` is the parent of the entire F02 run, `F-0006` of F03, `F-0010` of F08, `F-0019` of F09, `F-0015` of F06. | `APPLIED` — top-down consolidation completed; see §11 |
| `F-DU-02` | Internal duplication: zero duplicate candidate IDs across all 1,858 rows. | `CLOSED` |
| `F-DU-03` | Cross-inventory ownership: no other team member claims a Finance outcome as Primary Owner. Checked against Chetan (74), Dhruv (192), Varad Section G (541). Pallav and Devang inventories are empty. | `CLOSED` |
| `F-DU-04` | 83 pre-consolidation candidates in other inventories declared F as a participating domain — Chetan 14, Dhruv 22, Varad Section G 47 — including `CAN-L3-C-001`, the repository's flagship P0 example scenario. The consolidated F inventory now provides stable owner-side outcomes; consumer inventories retain their Phase 1 handoff intent without claiming Finance ownership. | `APPLIED` — consolidation completed; exact final L3 scenario IDs are assigned by the ordinary Phase 2 selection process, not by mutating discovery IDs |
| `F-DU-05` | `CAN-L3-J-043`…`J-048` (Purchase Order lifecycle, owner J04) against `F-1762`…`F-1858` (procurement budget validation, vendor liability recognition). `L2-F14` §3 and §4 draw the boundary cleanly and the inventory respects it. | `CLOSED` — confirm at Phase 2, no correction required |

---

## 8. Conformance Corrections Applied 2026-08-12

**Historical pre-consolidation record:** this section describes the mechanical cleanup of the former 1,858-row inventory. Section 11 records the later outcome-level consolidation and supersedes the row counts and open ID-width status below.

These were mechanical governance violations with a single correct resolution under existing locked rules. They were applied to the inventory. They are recorded here for traceability. **No candidate was added, removed, merged or split.**

| # | Defect | Rows affected | Rule | Resolution applied |
| --- | --- | ---: | --- | --- |
| 1 | Candidate IDs used the L2 sub-section as `<AREA>` (`CAN-L3-F01-001`), ran to four digits globally, and 574 rows under the F03/F04/F05 headings carried an `CAN-L3-F02-` prefix contradicting both their heading and their declared owner | 1,858 | `L3_Phase1_Discovery_Rules.md` §6; `L3_Naming_Numbering_Standard.md` §4, §6, §7 | Renumbered sequentially in document order to `CAN-L3-F-0001` … `CAN-L3-F-1858`. Crosswalk retained in the audit working set. |
| 2 | `Type` column held 39 distinct values, of which 36 are not permitted — `Core` (240), `Control` (117), `Integration` (102), `Event` (86), `Configuration` (81), `Governance` (72), `Exception` (66), `Boundary` (47), `Closing` (43) and others. 23 rows held a Priority value (`P0`/`P1`) in the Type column. | 1,544 | `L3_Scenario_Type_and_Priority_Rules.md` §3, §16 | Re-derived from the §7 selection order: high institutional risk → `Critical`; else ≥2 participating sections → `Cross-domain`; else `Simple`. `Critical` was matched only against the §6 enumerated risk triggers, respecting the §14 Mistake 2 warning against typing all Finance scenarios `Critical`. |
| 3 | 20 rows had 7 columns; `Priority` absent | 20 | `L3_Phase1_Discovery_Rules.md` §4, §18 | Column restored, `P1` assigned per §12 step 2. **These 20 warrant a priority review at Phase 2** — the value is a rule-derived default, not an author judgment. |
| 4 | `Domains` held 379 distinct free-text values including unresolved placeholders (`F + originating domain`), external parties (`Banking / Payment Provider`, `Sponsor`, `Government funding context`), and Finance-internal capabilities listed as if they were sections (`Finance; GL; AR; AP; Treasury; Budget`) | 1,797 | `L3_Phase1_Discovery_Rules.md` §11 | Normalised to section letters. Finance-internal capabilities collapse to `F` — they are one section, and listing them separately inflated apparent cross-domain reach. External parties removed: they are not SutraERP sections. Unresolved placeholders dropped rather than guessed. |
| 5 | `Primary Owner` held 46 formats across four incompatible styles, including nine-domain chains such as `Finance; GL; AR; AP; Treasury; Budget; Reporting; Audit; External Systems`, which record no ownership decision | 1,157 | `L3_Phase1_Discovery_Rules.md` §10 | Normalised to a single canonical `F00`…`F14` code, matching the Section G `G00`…`G10` convention. Resulting owner distribution matches section membership exactly. |
| 6 | Stray `.` line inside the F07 block; F09 sub-heading read `Finance Domain Model & Foundation — Financial Period & Closing`; F02 sub-heading omitted `Architecture` | 3 | Document hygiene | Corrected to match the L2 source document titles. |
| 7 | Inventory file had no document header, unlike `L3_Phase1_Inventory_Chetan.md` and `L3_Phase1_Inventory_Dhruv.md` | — | Repository convention | Header and scope table added. |

### 8.1 Known deviation — candidate ID width

`L3_Naming_Numbering_Standard.md` §4 specifies `<NNN>` as *a three-digit running number*. Section F carries 1,858 candidates, which cannot be expressed in three digits, so four-digit IDs are in use.

**Current status:** `CLOSED` by the consolidation in section 11; the active Finance inventory now uses `CAN-L3-F-001` through `CAN-L3-F-087`.

```text
Status: CLOSED — consolidation reduced the active range to `F-001`…`F-087`
```

This deviation is a **symptom of the fragmentation in section 5, not an independent defect**. Consolidating Section F to its estimated ~80 material outcomes brings the count back under 999 and the three-digit rule becomes satisfiable. Renumbering to three digits should therefore happen once, as part of Phase 2 consolidation, and not before. Section G, at 541 candidates, complies today.

### 8.2 What was deliberately not done

The bullets below describe the earlier conformance-only pass, not the current consolidated inventory.

- **No merging or splitting.** `L3_Phase1_Discovery_Rules.md` §17 places this in the post-Phase-1 review.
- **No candidate removed**, including the `Preserve …`, `Prevent …`, `Publish … Fact` and `Consume … Fact` rows that are not scenario-shaped. They are recorded in section 5.1 for Phase 2.
- **No Section G row modified.** Verified byte-identical to the pre-change state.
- **No L2, L1 or governance file modified.**

### 8.3 Type and Priority remain provisional

`L3_Scenario_Type_and_Priority_Rules.md` §15 states that Phase 1 classification is provisional and that Lead Architect review may correct Type and Priority during Phase 2. The corrections in row 2 above replaced an unusable vocabulary with a rule-derived one; they are not a substitute for that review.

Two distributions warrant attention at Phase 2:

| Observation | Detail |
| --- | --- |
| `P0` inflation | 977 of 1,858 candidates (52.6%) are `P0`. `L3_Scenario_Type_and_Priority_Rules.md` §9 warns against marking a scenario `P0` because the domain is important. This is largely an artifact of fragmentation — when one P0 scenario is split into fifty rows, all fifty inherit P0 — and should substantially resolve on consolidation. |
| `Critical` share | 489 of 1,858 (26.3%) after correction, against 39 (2.1%) before. The prior figure was implausible for a section whose worked governance examples (`Student Fee Payment and Receipt`, `Financial Period Closing`, `Payroll Finalization`, `Refund Processing`) are all named `Critical` in §6, §7 and §13. Final distribution: `Simple` 771, `Cross-domain` 598, `Critical` 489. |

---

## 9. Applied Consolidation Sequence

1. Consolidated the one-sided and cross-cutting clusters against the F00 outcome map.
2. Resolved the former open sub-domain boundaries from their locked L2 lifecycles under the user's audit authorization.
3. Applied `H-DEC-01`: F13 owns financial information and H07 owns dashboard composition.
4. Renumbered once to the conformant three-digit range `CAN-L3-F-001` … `CAN-L3-F-087`, closing §8.1.
5. Bound owner-side Finance outcomes while leaving final approved cross-section scenario IDs to the ordinary Phase 2 selection process.

**Preserved for L4.** The per-guard and per-invariant rows removed under `F-FR-X1` and `F-FR-X4` are well-researched L4 System Behavior Specification input — acceptance criteria and invariants traced to locked L2 sources. `L2-F03` line 2256 confirms that detailed L4 business rules are out of scope for L2. Carry this material into the L4 backlog attached to each consolidated scenario rather than discarding it.

---

## 10. Open Question for the Lead Architect

Section F discovery produced 1,858 candidates where Section G produced 541 for a comparable number of sub-domains. Both were produced by the same author under the same rules.

The difference is altitude: Section G candidates are outcome-shaped, Section F candidates are frequently rule-shaped. The Section H correction now provides the missing cross-domain comparison. It remains worth deciding whether the Phase 1 guidance in `L3_Phase1_Discovery_Rules.md` §7–§9 needs a worked Finance-style example showing where a lifecycle guard belongs inside a scenario rather than beside it.

```text
Status: DEFERRED — optional governance-example enhancement; no inventory or Phase 2 blocker
```

---

## 11. Consolidation Applied 2026-08-12

The user accepted the all-assigned audit recommendations and authorized repository correction. The former inventory was consolidated top-down against the material L2 lifecycle outcomes. This is the authoritative old-range to current-range crosswalk:

| Former source block | Former rows | Current candidates | Outcome boundary |
| --- | ---: | --- | --- |
| F00 `F-0001`…`F-0025` | 25 | Distributed across `F-001`…`F-087` | Former parent scenarios were used as the consolidation map; rule/configuration/notification/dashboard concerns became `F-084`…`F-087`. |
| F01 `F-0026`…`F-0056` | 31 | `F-001`…`F-006` | Charge creation, correction, state exception, dispute, installment schedule, closure. |
| F02 `F-0057`…`F-0126` | 70 | `F-007`…`F-013` | Payment and receipt, allocation, unidentified receipt, failure recovery, reversal, cancellation, bulk collection. |
| F03 `F-0127`…`F-0386` | 260 | `F-014`…`F-019` | Refund decision, execution, cancellation/withdrawal, recovery, reversal, closure. |
| F04 `F-0387`…`F-0576` | 190 | `F-020`…`F-024` | Credit note, debit note, write-off adjustment, reversal, correction. |
| F05 `F-0577`…`F-0701` | 125 | `F-025`…`F-032` | Ledger posting, manual journal, batch, reversal, reconciliation, trial balance, close, reconstruction. |
| F06 `F-0702`…`F-0785` | 84 | `F-033`…`F-038` | Budget approval, revision/virement, allocation, control, cost center, closure. |
| F07 `F-0786`…`F-0879` | 94 | `F-039`…`F-043` | Receivable recognition, dunning, settlement, write-off, reopening. |
| F08 `F-0880`…`F-1154` | 275 | `F-044`…`F-050` | Payable recognition, settlement, dispute/hold, adjustment, suspension/reopening/write-off, reconciliation, closure. |
| F09 `F-1155`…`F-1438` | 284 | `F-051`…`F-055` | Period establishment, period close, year close, controlled reopening/reclose, archival. |
| F10 `F-1439`…`F-1492` | 54 | `F-056`…`F-061` | Bank account, payment execution, reconciliation, cash position, transfer, recovery. |
| F11 `F-1493`…`F-1574` | 82 | `F-062`…`F-066` | Aid allocation, financial execution, renewal consequence, revocation/recovery, reconciliation. |
| F12 `F-1575`…`F-1636` | 62 | `F-067`…`F-072` | Capitalization, depreciation, revaluation, impairment, disposal, write-off accounting. |
| F13 `F-1637`…`F-1761` | 125 | `F-073`…`F-079`, `F-087` | Statements, statutory/management reports, KPI definitions and projections, correction/archive, reconstruction; H07 composes dashboards. |
| F14 `F-1762`…`F-1858` | 97 | `F-080`…`F-083` | Budget commitment, matching, liability recognition, procurement accounting closure. |

Former Section F total: **1,858 rows**. Current Section F total: **87 candidates**.

All former `Preserve…`, `Prevent…`, `Publish…`, `Consume…`, individual guard-rejection, audit-write and configuration-step rows were absorbed as rules, triggers, alternate outcomes, events or evidence inside the current scenario boundaries. They remain recoverable from this register and the audit reports for L4 authoring.
