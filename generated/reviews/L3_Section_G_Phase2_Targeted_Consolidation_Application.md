# L3 Section G — Phase 2 Targeted Consolidation Application

**Architecture Level:** L3 — Business Scenario Architecture
**Role:** Senior Enterprise Architecture Remediation Engineer
**Date:** 2026-08-17
**Type:** Controlled remediation — application of the 12 evidence-backed clusters from
`generated/reviews/L3_Section_G_350_Candidate_Independent_Boundary_Audit.md` to Phase 2 artifacts only.
**Phase 1 inventory modified:** No.

---

# 1. Executive Summary

All 12 requested clusters (SG-01…SG-12) were reviewed against the current repository state before
application. 11 clusters applied exactly as specified. **SG-01 applied partially**: 4 of its 5 targeted
absorptions (`CAN-L3-G-003`, `G-004`, `G-005`, `G-540`) were applied as VARIANT; the fifth
(`CAN-L3-G-002`) was withheld and marked `REVALIDATION REQUIRED` because it carries a pre-existing
`BLOCKED` status under cross-inventory issue `I-XI-03` (Section I boundary conflict) that outranks the
Section G-only audit in the authority order and was not visible to that audit. No other deviation from
the 12 specified clusters occurred; no additional merges were discovered or applied.

`CAN-L3-G-202` was left `REVIEW-REQUIRED` as instructed — not merged. All eight explicitly protected
candidates (`G-006`, `G-199`, `G-201`, `G-275`, `G-492`, `G-493`, `G-494`, `G-516`) remain untouched and
independent. Domains-field corrections were applied exactly as specified, including leaving `G-058`
unchanged and flagged `REVIEW-REQUIRED` rather than mechanically edited.

**Result:** 350 Phase 1 candidates (unchanged) → **322** independently proposed Phase 2 Section G scenarios
(not the pre-computed 321, for the reason above) + 28 candidates absorbed (20 MERGE, 6 VARIANT, 2
DUPLICATE) + 1 candidate left REVIEW-REQUIRED (`G-202`), all accounted for inside the unchanged 350.

---

# 2. Files Modified

| File | Type of change |
| --- | --- |
| `architecture/L3_Business_Scenario_Architecture/02_Phase2_Consolidation/L3_Phase2_Proposed_Canonical_Scenario_Register.md` | 12 surviving-parent rows rewritten/annotated; 22 absorbed rows re-dispositioned (MERGE/VARIANT/DUPLICATE) with evidence; `G-202` re-dispositioned REVIEW-REQUIRED; 7 Domains-field rows corrected/annotated; Section G header summary updated. |
| `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Section_G_Phase2_Consolidation_Watchlist.md` | New dated §11 "2026-08-17 Targeted Phase 2 Remediation" appended. All prior history (§1–§10, `G-FR-*`/`G-SP-*`/`G-OV-*`/`G-XI-*`/`MG-*`) preserved unchanged. |
| `generated/reviews/L3_Phase2_Repository_Wide_Consolidation_Review.md` | Disposition-count table, §10 final scenario count, and the Section G entry in Pass 2 updated to reflect the 2026-08-17 remediation; `I-XI-03` collision note extended with a confirmation that it is still live. |
| `generated/reviews/L3_Section_G_Phase2_Targeted_Consolidation_Application.md` | New — this report. |

**Not modified:** `architecture/L3_Business_Scenario_Architecture/01_Phase1_Discovery/L3_Phase1_Inventory_Varad.md` (verified via `git diff` — zero changes). No L0, L1, or L2 file was touched. No other section (A–F, H–J) row was touched — verified by section-header line positions and per-section disposition counts before/after.

---

# 3. Pre-Remediation Counts

Verified directly against the repository before any edit (per §4 of the remediation brief):

- Section G Phase 1 row count: **350** (`grep -c '^| CAN-L3-G-'` against the inventory) — matches the audit's stated count.
- `git status`/`git diff` on the inventory and watchlist showed **no changes** since the audit was written — wording of every cited candidate is identical to what the audit read.
- No newer approved repository decision was found superseding the audit's SG-01…SG-12 findings, **except** the pre-existing `I-XI-03` block on `CAN-L3-G-002`, which the audit itself had no visibility into (it audited Section G only) and which the repository-wide Phase 2 review had already recorded before this remediation began.
- Phase 2 register Section G header before this pass: `**Total:** 350 | **KEEP:** 349 | **BLOCKED:** 1`.

---

# 4. Applied Clusters

| Cluster | Absorbed (targeted) | Absorbed (applied) | Parent | Disposition | Status |
| --- | --- | --- | --- | --- | --- |
| SG-01 | `G-002, G-003, G-004, G-005, G-540` (5) | `G-003, G-004, G-005, G-540` (4) | `CAN-L3-G-001` | VARIANT | **PARTIAL** — `G-002` REVALIDATION REQUIRED (see below) |
| SG-02 | `G-047` (1) | `G-047` (1) | `CAN-L3-G-025` | MERGE | APPLIED |
| SG-03 | `G-270, G-271, G-272, G-273, G-274` (5) | same (5) | `CAN-L3-G-269` | MERGE (270-272) / VARIANT (273-274) | APPLIED |
| SG-04 | `G-311` (1) | same (1) | `CAN-L3-G-310` | MERGE | APPLIED |
| SG-05 | `G-313, G-315` (2) | same (2) | `CAN-L3-G-312` | MERGE | APPLIED |
| SG-06 | `G-323, G-324, G-326` (3) | same (3) | `CAN-L3-G-321` | MERGE | APPLIED |
| SG-07 | `G-331, G-332, G-333` (3) | same (3) | `CAN-L3-G-330` | MERGE | APPLIED |
| SG-08 | `G-334` (1) | same (1) | `CAN-L3-G-330` | DUPLICATE | APPLIED |
| SG-09 | `G-400, G-401, G-402` (3) | same (3) | `CAN-L3-G-390` | MERGE | APPLIED |
| SG-10 | `G-514, G-515` (2) | same (2) | `CAN-L3-G-513` | MERGE | APPLIED |
| SG-11 | `G-496, G-497` (2) | same (2) | `CAN-L3-G-495` | MERGE | APPLIED |
| SG-12 | `G-520` (1) | same (1) | `CAN-L3-G-518` | DUPLICATE | APPLIED |
| **Total** | **29** | **28** | | | 11 fully applied, 1 partial |

`CAN-L3-G-275` and `CAN-L3-G-516` were explicitly not absorbed into SG-03/SG-10, exactly as instructed.

---

# 5. Absorbed Candidate Crosswalk

| Absorbed Candidate | Disposition | Surviving Candidate | Evidence | Cluster |
| --- | --- | --- | --- | --- |
| `CAN-L3-G-003` | VARIANT | `CAN-L3-G-001` | L2-G00 "Campus Readiness Workflow" — configurable Readiness Type | SG-01 |
| `CAN-L3-G-004` | VARIANT | `CAN-L3-G-001` | same | SG-01 |
| `CAN-L3-G-005` | VARIANT | `CAN-L3-G-001` | same | SG-01 |
| `CAN-L3-G-540` | VARIANT | `CAN-L3-G-001` | same | SG-01 |
| `CAN-L3-G-047` | MERGE | `CAN-L3-G-025` | L2-G01 Inspection Workflow -> Work Order Generated branch; extends `G-FR-13` | SG-02 |
| `CAN-L3-G-270` | MERGE | `CAN-L3-G-269` | L2-G07 single "Incident Workflow" diagram | SG-03 |
| `CAN-L3-G-271` | MERGE | `CAN-L3-G-269` | same | SG-03 |
| `CAN-L3-G-272` | MERGE | `CAN-L3-G-269` | same | SG-03 |
| `CAN-L3-G-273` | VARIANT | `CAN-L3-G-269` | same, Severity Levels configurable within one workflow | SG-03 |
| `CAN-L3-G-274` | VARIANT | `CAN-L3-G-269` | same; distinct from `G-275` which is not absorbed | SG-03 |
| `CAN-L3-G-311` | MERGE | `CAN-L3-G-310` | L2-G08 Cleaning Schedule aggregate lifecycle table | SG-04 |
| `CAN-L3-G-313` | MERGE | `CAN-L3-G-312` | L2-G08 Cleaning Task aggregate lifecycle table | SG-05 |
| `CAN-L3-G-315` | MERGE | `CAN-L3-G-312` | same | SG-05 |
| `CAN-L3-G-323` | MERGE | `CAN-L3-G-321` | L2-G08 "Complaint Workflow" diagram | SG-06 |
| `CAN-L3-G-324` | MERGE | `CAN-L3-G-321` | same | SG-06 |
| `CAN-L3-G-326` | MERGE | `CAN-L3-G-321` | same | SG-06 |
| `CAN-L3-G-331` | MERGE | `CAN-L3-G-330` | L2-G08 Hygiene Inspection aggregate lifecycle table | SG-07 |
| `CAN-L3-G-332` | MERGE | `CAN-L3-G-330` | same | SG-07 |
| `CAN-L3-G-333` | MERGE | `CAN-L3-G-330` | same | SG-07 |
| `CAN-L3-G-334` | DUPLICATE | `CAN-L3-G-330` | `G-330`'s own Trigger already enumerates "washroom" | SG-08 |
| `CAN-L3-G-400` | MERGE | `CAN-L3-G-390` | L2-G09 §15 "Referral Workflow" diagram | SG-09 |
| `CAN-L3-G-401` | MERGE | `CAN-L3-G-390` | same | SG-09 |
| `CAN-L3-G-402` | MERGE | `CAN-L3-G-390` | same | SG-09 |
| `CAN-L3-G-514` | MERGE | `CAN-L3-G-513` | L2-G10 `WF-INF-013` single workflow | SG-10 |
| `CAN-L3-G-515` | MERGE | `CAN-L3-G-513` | same | SG-10 |
| `CAN-L3-G-496` | MERGE | `CAN-L3-G-495` | L2-G10 §32 "Inspection Workflow" diagram | SG-11 |
| `CAN-L3-G-497` | MERGE | `CAN-L3-G-495` | same | SG-11 |
| `CAN-L3-G-520` | DUPLICATE | `CAN-L3-G-518` | Same Section-A effective-dating mechanism as `G-518` | SG-12 |

**Not absorbed (REVALIDATION REQUIRED):** `CAN-L3-G-002` — see below.

Every absorbed ID retains its Phase 1 identity in the register (Phase 1 ID column unchanged); no ID was
deleted, renumbered, or reused.

---

# 6. Surviving Parent Changes

11 parents' Title/Trigger/Final Outcome were broadened to explicitly cover the absorbed span, sourced only
from the absorbed rows' own existing wording (no invented content):

- `CAN-L3-G-001` — retitled "Campus Readiness Assessment to Governance Readiness Decision"; now generically covers Daily/Admission/Event/Emergency/Academic Session readiness types. Domains extended to `G, A, B, C, D, H` (union of absorbed rows' Domains, minus `I` since `G-002` was not absorbed).
- `CAN-L3-G-025` — wording unchanged (already covered the inspection-sourced case); evidence note added only.
- `CAN-L3-G-269` — retitled "Security Incident Report to Closed Resolution"; now spans the full Reported→Closed lifecycle. Domains extended to `G, H, J, A`.
- `CAN-L3-G-310` — retitled "Cleaning Schedule Planning to Active Cleaning Programme"; now spans Draft→Active.
- `CAN-L3-G-312` — retitled "Active Cleaning Schedule to Verified Task Completion"; now spans Generated→Completed. Type updated Simple→Cross-domain (absorbed row's E-domain participation).
- `CAN-L3-G-321` — retitled "Housekeeping Complaint Registration to Verified Resolution"; now spans Registered→Closure. Type updated Simple→Cross-domain.
- `CAN-L3-G-330` — retitled "Hygiene Inspection to Compliance or Corrective Outcome"; now spans Scheduled→Closed across all branch outcomes. Domains extended to `G, H`.
- `CAN-L3-G-390` — wording extended to explicitly include the Student-Advised/Status-Updated/Closed spans; Domains unchanged (already covered).
- `CAN-L3-G-495` — retitled "Infrastructure Inspection Requirement to Closed Risk-Reviewed Outcome"; now spans Scheduled→Closed.
- `CAN-L3-G-513` — retitled "Infrastructure Decommissioning Initiation to Historical Closure"; now spans Initiation→Closure.
- `CAN-L3-G-518` — wording unchanged (already generic); evidence note added only.

No new business event, lifecycle state, aggregate, or ownership was introduced. Every broadened phrase
traces to text already present in an absorbed row.

---

# 7. Domains Corrections

| Candidate | Before | After | Action |
| --- | --- | --- | --- |
| `G-022` | `G, D, I, H` | `G, H` | D, I removed (confirmed impact-only) |
| `G-032` | `G, E, D, I` | `G, E` | D, I removed; E retained (valid participation) |
| `G-044` | `G, H, D, I` | `G, H` | D, I removed; H left REVIEW-REQUIRED |
| `G-045` | `G, H, D, I` | `G, H` | D, I removed; H left REVIEW-REQUIRED |
| `G-055` | `G, D, I, H` | `G, H` | D, I removed; H left REVIEW-REQUIRED |
| `G-076` | `G, H, D, I` | `G, H` | D, I removed; H left REVIEW-REQUIRED |
| `G-058` | `G, E, D, I` | `G, E, D, I` (unchanged) | Not auto-changed; Domains participation itself flagged REVIEW-REQUIRED |

H was not removed from any row — per instruction, its removal requires authoritative evidence that it is
merely a notification consumer, which this pass did not obtain.

---

# 8. Review-Required Items

- **`CAN-L3-G-202`** (First Aid Incident to Stabilised Patient Outcome) — disposition set to
  `REVIEW-REQUIRED`. `INSUFFICIENT EVIDENCE — DO NOT CONSOLIDATE`. L2-G06 documents "Emergency & First Aid
  Management" as one §19 heading with a single diagrammed Emergency Workflow; First Aid itself has no
  independent workflow diagram, so neither independence nor absorption can be confirmed. Not guessed
  either way.
- **`CAN-L3-G-002`** — remains `BLOCKED` under `I-XI-03` (unchanged, pre-existing). SG-01's attempt to
  absorb it was withheld; recorded as `REVALIDATION REQUIRED` in the watchlist §11 and the repository-wide
  review. Resolving `I-XI-03` (a Section G/Section I boundary confirmation) is a prerequisite for any future
  Section G disposition on this candidate.
- **`G-058` Domains participation** — `REVIEW-REQUIRED`, per instruction, not resolved by this pass.
- **`G-044`, `G-045`, `G-055`, `G-076` — H participation** — `REVIEW-REQUIRED`, not resolved by this pass (no authoritative evidence found either way).

---

# 9. Post-Remediation Counts

Section G Phase 2 register, verified by direct disposition-column count after all edits:

| Disposition | Count |
| --- | ---: |
| KEEP | 320 |
| MERGE | 20 |
| VARIANT | 6 |
| DUPLICATE | 2 |
| REVIEW-REQUIRED | 1 |
| BLOCKED | 1 |
| **Total** | **350** |

Independently proposed Section G scenarios (KEEP + BLOCKED + REVIEW-REQUIRED, each still a standing
candidate): **320 + 1 + 1 = 322.**

This is **not** the audit's pre-computed 321 — the difference is exactly the one candidate (`G-002`) that
could not be absorbed under SG-01 due to the pre-existing `I-XI-03` block. The arithmetic was verified from
the actual resulting register, not forced to match the pre-computed estimate, per the remediation brief's
explicit instruction.

---

# 10. Repository-Wide Count Impact

| Disposition | 2026-08-16 | 2026-08-17 | Change |
| --- | ---: | ---: | ---: |
| KEEP | 1,078 | 1,049 | -29 |
| MERGE | 0 | 20 | +20 |
| VARIANT | 0 | 6 | +6 |
| DUPLICATE | 0 | 2 | +2 |
| REVIEW-REQUIRED | 0 | 1 | +1 |
| BLOCKED | 45 | 45 | 0 |
| **Total** | **1,123** | **1,123** | 0 |

Repository-wide proposed final scenario count (independently standing, i.e. not absorbed): **1,095**
(1,123 − 28 absorbed). No other section's (A–F, H–J) counts were touched.

---

# 11. Validation Results

All ten structural checks (remediation brief §16) performed directly against the edited files:

1. ✅ Phase 1 inventory still contains exactly 350 `CAN-L3-G-*` rows (`git diff` on the inventory file: zero changes).
2. ✅ Phase 1 IDs unchanged — confirmed via the same zero-diff check.
3. ✅ Phase 2 register contains the correct number of independently proposed G scenarios: 322, computed directly from the post-edit disposition-column count (320 KEEP + 1 BLOCKED + 1 REVIEW-REQUIRED), not asserted.
4. ✅ Every absorbed candidate has exactly one surviving parent — verified by counting "Absorbed Into: CAN-L3-G-NNN" occurrences per parent (sums to 28, matching MERGE+VARIANT+DUPLICATE exactly, with no candidate appearing twice).
5. ✅ No candidate absorbed into multiple parents — same check; each of the 28 absorbed IDs appears in exactly one "Absorbed Into" reference.
6. ✅ No surviving parent accidentally removed — all 11 parents (`G-001, G-025, G-269, G-310, G-312, G-321, G-330, G-390, G-495, G-513, G-518`) individually confirmed still present with disposition `KEEP`.
7. ✅ `G-202` remains unresolved — disposition `REVIEW-REQUIRED`, not merged.
8. ✅ Explicitly protected scenarios (`G-006, G-199, G-201, G-275, G-492, G-493, G-494, G-516`) remain independent and textually untouched.
9. ✅ No new L3 final ID was assigned — only existing `P2W-G-*` working IDs and `CAN-L3-G-*` Phase 1 IDs were touched; no final-ID scheme was introduced.
10. ✅ Other sections (A–F, H–J) were not accidentally modified — section header line positions in the register are unchanged outside Section G, and the file's total line-count delta (+2) is fully attributable to the Section G header addition; no other section's disposition counts changed.

---

# 12. Remaining Section G Decisions

Carried forward, unresolved by this remediation (as instructed — scope was limited to SG-01…SG-12 and the
seven named Domains rows):

1. **`I-XI-03`** — cross-inventory boundary conflict between `CAN-L3-G-002` and Section I examination-readiness candidates (`I-021, I-085, I-093, I-095`). Blocks both `G-002`'s BLOCKED status and the full application of SG-01. Requires Section G/Section I joint resolution, not a Section G-only decision.
2. **`CAN-L3-G-202`** — REVIEW-REQUIRED; needs an L2-G06 owner's judgment on whether First Aid is independent of the Emergency Workflow.
3. **`G-058` Domains participation** — REVIEW-REQUIRED; whether D/I materially create the scheduling conflict.
4. **H participation on `G-044, G-045, G-055, G-076`** — REVIEW-REQUIRED; whether H is a genuine business participant or a notification consumer (continues the watchlist §4A residual review, not yet closed).
5. The broader watchlist §4A residual participation review remains open for any further G01/G02 rows beyond the seven already named — not re-swept by this targeted pass, which was scoped to SG-01…SG-12 and the Domains rows explicitly listed in the remediation brief.
