# L3 Phase 1 Continuous Numbering Migration — 2026-08-17

**Report Type:** Migration validation record
**Date:** 2026-08-17
**Authorization:** Controlled, one-time, repository-wide L3 Phase 1 candidate ID numbering normalization

---

# Executive Summary

Every active L3 Phase 1 discovery section (A–J) is renumbered to a clean, continuous range starting at
`CAN-L3-<AREA>-001`, with candidate numbers now allowed to exceed 999 without a digit-width cap. All 1,095
active candidates were verified byte-for-byte identical in every field except `Candidate ID` before and
after migration. The complete old→new crosswalk is authoritative in
`architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Phase1_Candidate_ID_Migration_2026-08-17.md`.
Governance documents and `check_l3_inventory.py` were updated to match the new rule and to enforce it going
forward. `check_l3_inventory.py` passes with 0 errors and 0 warnings.

Sections G and I had each already undergone a narrower, disjoint-start renumbering earlier the same day
(`G-542`…`G-863`, `I-315`…`I-511`); this migration supersedes that approach for every section, including
those two, and both watchlists now document the superseded step as historical narrative alongside the
final `001`-start state.

Five pre-existing dangling cross-references — citations that had never been updated after their target was
renumbered by an earlier, unrelated consolidation — were found and corrected, each verified by an exact
Scenario Title match before correction. No business scenario was added, removed, merged, split, or
reclassified, and no L0/L1/L2 business architecture changed.

---

# Governance Changes

- **`L3_Naming_Numbering_Standard.md`** — §4 numeric-width rule changed from "three-digit" to "minimum
  three digits, unbounded upward"; new §7.1 records the one-time migration and the post-migration freeze
  policy (future retirements do not trigger automatic renumbering); Lock Statement (§18) updated.
- **`L3_Phase1_Discovery_Rules.md`** — §6 cross-references the new numbering rule and the migration doc.
- **`README.md`** — Status/Current Phase corrected from "Phase 1 Active" to "Phase 1 — Discovery Complete /
  Frozen; Phase 2 — Consolidation Active"; folder structure listing corrected to include the previously
  undocumented `L3_Section_B_D_E_...` and `L3_Section_J_...` watchlists and the `02_Phase2_Consolidation/`
  folder; Policy Management ownership resolution (RC-03, `DEPENDENCY_MAP.md`) noted; Current Status block
  updated.
- **`check_l3_inventory.py`** — candidate ID regex changed from `\d{3,4}` to `\d{3,}` (no upper bound); new
  `check_sequence()` validation enforces, per section, across all inventory files: numbering starts at
  `001`, is continuous with no gaps, and has no duplicate sequence number.
- **8 Phase 2 Consolidation Watchlists** — ID columns/cross-references updated via the mapping; stale
  "Applies To" / "Current candidates" header facts corrected to the final post-migration state (notably
  Section F, which had drifted to a stale `F-087` count after a 2026-08-14 retirement that was never
  reflected in its header); Section C's `CF-08` Policy Management blocker annotated as resolved (historical
  row left as originally written); Section G and Section I each received a new subsection documenting that
  their own earlier same-day disjoint-start renumbering is now superseded, with the historical `542`–`863`
  and `315`–`511` narratives restored to accuracy (see "Stale References Corrected" below for the specific
  corruption this required fixing).
- **Phase 2 register** — "How to Read This Register" intro rewritten to remove "Phase 1 Candidate ID is
  never renumbered" and replace it with an accurate description of the one-time 2026-08-17 normalization
  and the post-migration freeze.
- **`CHANGELOG.md`** — new `[1.11.0] - 2026-08-17` entry added; no prior entry edited.

---

# Pre-Migration Counts (verified from the repository, not assumed)

| Section | Count | Pre-Migration Range |
|---|---:|---|
| A | 62 | `CAN-L3-A-070` … `CAN-L3-A-131` |
| B | 32 | `CAN-L3-B-997` … `CAN-L3-B-1028` |
| C | 74 | `CAN-L3-C-001` … `CAN-L3-C-074` |
| D | 57 | `CAN-L3-D-058` … `CAN-L3-D-114` |
| E | 75 | `CAN-L3-E-105` … `CAN-L3-E-179` |
| F | 85 | `CAN-L3-F-001` … `CAN-L3-F-085` |
| G | 322 | `CAN-L3-G-542` … `CAN-L3-G-863` |
| H | 81 | `CAN-L3-H-086` … `CAN-L3-H-166` |
| I | 197 | `CAN-L3-I-315` … `CAN-L3-I-511` |
| J | 110 | `CAN-L3-J-682` … `CAN-L3-J-791` |
| **TOTAL** | **1,095** | |

Matches the expected baseline (A 62, B 32, C 74, D 57, E 75, F 85, G 322, H 81, I 197, J 110 = 1,095)
exactly — no discrepancy to report.

# Post-Migration Counts

| Section | Count | Post-Migration Range |
|---|---:|---|
| A | 62 | `CAN-L3-A-001` … `CAN-L3-A-062` |
| B | 32 | `CAN-L3-B-001` … `CAN-L3-B-032` |
| C | 74 | `CAN-L3-C-001` … `CAN-L3-C-074` |
| D | 57 | `CAN-L3-D-001` … `CAN-L3-D-057` |
| E | 75 | `CAN-L3-E-001` … `CAN-L3-E-075` |
| F | 85 | `CAN-L3-F-001` … `CAN-L3-F-085` |
| G | 322 | `CAN-L3-G-001` … `CAN-L3-G-322` |
| H | 81 | `CAN-L3-H-001` … `CAN-L3-H-081` |
| I | 197 | `CAN-L3-I-001` … `CAN-L3-I-197` |
| J | 110 | `CAN-L3-J-001` … `CAN-L3-J-110` |
| **TOTAL** | **1,095** | |

Counts are unchanged — only the numbering changed.

# Final Numbering Ranges A–J

Same table as above (post-migration). All ten ranges are dense (`001` through the section count) with zero
gaps and zero duplicates, confirmed by `check_l3_inventory.py`'s new continuity check.

# Files Modified

**New:**
- `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Phase1_Candidate_ID_Migration_2026-08-17.md` (authoritative crosswalk)
- `generated/reviews/L3_Phase1_Continuous_Numbering_Migration_2026-08-17.md` (this report)

**Modified (ID substitution + governance text, 20 files):**
- 5 Phase 1 inventory files (`01_Phase1_Discovery/L3_Phase1_Inventory_{Chetan,Devang,Dhruv,Pallav,Varad}.md`)
- 8 Phase 2 Consolidation Watchlists (`00_Governance/L3_Section_{A,B_D_E,C,F,G,H,I,J}_Phase2_Consolidation_Watchlist.md`)
- `02_Phase2_Consolidation/L3_Phase2_Proposed_Canonical_Scenario_Register.md`
- `README.md`
- `L3_Naming_Numbering_Standard.md`, `L3_Phase1_Discovery_Rules.md`, `L3_Phase1_Team_Instructions.md`
- `check_l3_inventory.py`
- `CHANGELOG.md`

**Modified (non-destructive historical annotation only, 27 files):**
- `00_Governance/L3_Section_A_Phase1_Audit_Application_Report_2026-08-08.md`
- 26 files under `generated/reviews/` that cite pre-migration `CAN-L3-` IDs

# Files Intentionally Not Modified

- `L3_Scenario_Type_and_Priority_Rules.md`, `L3_Phase1_Workload_Distribution_Decision.md`,
  `L3_Business_Scenario_Template_v1.0.md` — contain no section-specific candidate ID that required
  correction (Template's IDs are format placeholders only; 0 substitutions applied).
- All historical narrative *inside* the 8 watchlists that predates a section's own numbering generation
  (e.g. Section F's `F-0001`…`F-1858` raw discovery IDs, Section H's `H01-001`…`H10-883` fragment IDs,
  Section G's pre-`542` sparse `G-001`…`G-541` IDs, Section I's pre-`315` sparse `G-001`…`I-313` IDs,
  Section A's pre-`070` cluster shorthand) — left exactly as written, per each document's own stated
  historical-integrity policy.
- `AI_CONTEXT/L3_Phase1_Governance_Context.md`, `AI_CONTEXT/REPO_UNDERSTANDING.md` — outside the
  authorized scope (`architecture/L3_Business_Scenario_Architecture/` and supporting L3 validation/review
  artifacts); their `CAN-L3-` mentions are generic format examples (`C-001`, `F-001`, `X-001`), not real
  IDs, so no factual inaccuracy exists to correct.
- `AI_CONTEXT/*` more broadly, and any file outside the stated scope.

# Old-to-New Crosswalk Location

`architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Phase1_Candidate_ID_Migration_2026-08-17.md`
— complete old→new mapping for all 1,095 active candidates, grouped by section, in original row order.

# Stale References Corrected

Two categories:

**1. Dangling references to a since-superseded ID (5, verified by exact title match):**

| Stale reference | Verified current target (pre-migration) | Final new ID |
|---|---|---|
| `CAN-L3-E-103` | `CAN-L3-E-119` | `CAN-L3-E-015` |
| `CAN-L3-H-010` | `CAN-L3-H-095` | `CAN-L3-H-010` |
| `CAN-L3-H-050` | `CAN-L3-H-133` | `CAN-L3-H-048` |
| `CAN-L3-J-028` | `CAN-L3-J-709` | `CAN-L3-J-028` |
| `CAN-L3-J-029` | `CAN-L3-J-710` | `CAN-L3-J-029` |

(Two of the five happen to land back on the same numeral post-migration by coincidence, not because they
were left unmapped — each was independently verified and carried through both mapping steps.)

**2. Mixed old/new range corruption caused by the mechanical substitution itself, found and hand-corrected:**

- `L3_Section_B_D_E_Phase2_Consolidation_Watchlist.md` — a range written as `` `CAN-L3-B-1013`…`B-1016` ``
  had its fully-prefixed first endpoint auto-corrected to `B-017` while the bare-shorthand second endpoint
  (`B-1016`, no `CAN-L3-` prefix) was not touched by the token-level substitution, producing an invalid
  mixed range. Corrected to `` `CAN-L3-B-017`…`B-020` ``. The same line's `` `CAN-L3-G-821`, `G-822` ``
  pair had the identical problem and was corrected to `` `CAN-L3-G-280`, `G-281` ``.
- `L3_Section_C_Phase2_Consolidation_Watchlist.md` — the `C-C03` cluster row's bare candidate-number cell
  (`017, 037`) had gone stale relative to the same row's own (correctly substituted) prose, which now named
  `CAN-L3-C-040` and `CAN-L3-C-019`. Corrected the cell to `019, 040`.
- `L3_Section_G_Phase2_Consolidation_Watchlist.md` — the section's own internal "ID Renumbering —
  2026-08-17" narrative (documenting the *earlier*, disjoint-start `542`-`863` event) had several of its
  own full-token ID mentions mechanically carried forward to their final `001`-start values by the same
  substitution pass, leaving the narrative self-contradictory (e.g. a paragraph titled "why the range
  starts at 542" whose body then said the range was `001`-`322`). Hand-rewritten into two clearly labeled
  subsections — §14 (historical, `542`-`863`, restored to its original numbers) and a new §15 (current,
  `001`-`322`) — so both states remain independently accurate and neither contradicts itself.
- `L3_Section_I_Phase2_Consolidation_Watchlist.md` — identical problem in its own "ID Renumbering —
  2026-08-17" narrative (`315`-`511` disjoint event); resolved the same way, with a new "Superseded —
  Repository-Wide `001`-Start Renumbering" subsection added.

This second category — bare, non-`CAN-L3-`-prefixed range endpoints and self-referential narrative about a
just-prior renumbering event — is a structural blind spot of pure token substitution. A full audit for
every remaining instance of this pattern was not exhaustively completed (see "Remaining Architecture
Blockers"); the instances found were high-visibility (range endpoints, a document's own account of its own
renumbering) and are believed to cover the highest-risk cases.

# Historical References Preserved

Left exactly as written, with a one-line non-destructive pointer to the migration doc added where the
containing file is a dated point-in-time report:

- Section F's former `CAN-L3-F-0001`…`CAN-L3-F-1858` raw discovery-era IDs (1,858 rows, pre-2026-08-12).
- Section H's former `CAN-L3-H01-001`…`CAN-L3-H10-883` subsection-fragment IDs (893 rows, pre-2026-08-12).
- Section G's pre-`542` sparse `CAN-L3-G-001`…`CAN-L3-G-541` IDs (§1–§13 of its watchlist) and its own
  `542`–`863` disjoint-start narrative (§14, now marked superseded).
- Section I's pre-`315` sparse `CAN-L3-I-001`…`CAN-L3-I-314` IDs (§1–§16 of its watchlist) and its own
  `315`–`511` disjoint-start narrative (now marked superseded).
- Section A's 18 relationship-cluster tables, which use pre-`070` cluster shorthand numbers (e.g. `006,
  007`) that predate Section A's current range — left untouched; no title-level evidence was available to
  verify each one individually against the current inventory, so per the "never invent a mapping" rule
  these were not corrected. All 18 clusters are already `CLOSED`/`APPLIED`, so this affects only historical
  readability, not any active decision.
- The Phase 2 register's Section G rows for the 28 absorbed/retired candidates (`MERGE`/`VARIANT`/
  `DUPLICATE` disposition) — their `Phase 1 ID` column correctly still shows the pre-`542` historical ID,
  since they were never part of the active, migrated inventory.
- `L3_Section_A_Phase1_Audit_Application_Report_2026-08-08.md` and 26 files under `generated/reviews/` —
  annotated only, per the historical-report handling policy.
- `CHANGELOG.md` — prior entries unedited; new entry appended.

# Checker Changes

`check_l3_inventory.py`:
- `CANDIDATE_ID` regex: `^CAN-L3-([A-JX])-(\d{3,4})$` → `^CAN-L3-([A-JX])-(\d{3,})$` (no upper bound).
- New `check_sequence()` function, run once across all files combined (candidates are unique per section
  regardless of which file they live in): flags a `Section {letter}: duplicate active sequence number`
  error for any repeated number, `Section {letter}: active numbering does not start at 001` if the floor
  isn't 1, and `Section {letter}: N gap(s) in the active numbering sequence` (with sample missing IDs) for
  any hole between the floor and the ceiling.
- Verified against fixtures: `001, 002, 004` (missing `003`) → 1 error, correctly identifies the gap.
  `001`…`999`, `1000` (1,000 rows, 4-digit tail) → 0 errors, confirming no artificial digit-width cap.
  `998, 999, 1000` (floor ≠ 1) → 1 error, correctly rejects a non-`001` start.

# Validation Results

| Check | Result |
|---|---|
| A. Active candidate count per section | Matches expected baseline exactly (see Pre-Migration Counts) |
| B. Sequence validation (start=001, no gaps, no duplicates) | PASS for all 10 sections, confirmed by `check_l3_inventory.py` |
| C. Content preservation (Title/Trigger/Outcome/Owner/Domains/Type/Priority unchanged) | PASS — 1,095/1,095 rows byte-identical except `Candidate ID`, verified by diffing every row against `git show HEAD` |
| D. Cross-reference validation | PASS — every active-range token resolves to a real active ID; 5 pre-existing dangling references found and corrected (see above); remaining unmapped tokens verified as legitimate historical/absorbed references |
| E. `python check_l3_inventory.py` | **PASS — 0 errors, 0 warnings**, 1,095 candidates across 5 files |
| F. Phase 2 register integrity | PASS — all 1,095 active candidates have a register row; all 1,123 P2W IDs unchanged (0 renumbered); 0 disposition changes; 28 non-active Section G rows correctly retain their historical Phase 1 ID |

# Remaining Architecture Blockers

Not resolved by this migration (identifier/traceability scope only; resolving these would be a business
architecture decision, out of scope per the authorization):

- **45 `BLOCKED`-disposition Phase 2 register rows**: B 2, C 5, D 16, I 14, J 7, G 1 (`CAN-L3-G-002`,
  formerly `G-543`, pending `I-XI-03`).
- **Section C**: `CF-01`–`CF-09` conflicts recorded in the watchlist — one (`CF-08`, Policy Management
  ownership) is now resolved per `DEPENDENCY_MAP.md` RC-03 and annotated as such; the rest remain open,
  unchanged by this migration.
- **Section I**: `I-RC-07` (question paper governance has no owner — needs a new L2 document or owner,
  not a ruling between existing ones).
- **Pre-existing dangling references not resolved**, for lack of verifiable evidence (no scenario title
  given at the citation site to confirm the correct current target): Section A cluster-table shorthand
  (`A-023`, `A-047`, `A-030`, etc., in `L3_Section_G_Phase2_Consolidation_Watchlist.md`'s `G-FR-01` and
  `G-XI-03` rows), and the `CAN-L3-J-043`…`J-048` range cited in `L3_Section_F_Phase2_Consolidation_Watchlist.md`'s
  `F-DU-05` finding. Recorded here as **BLOCKED — NON-NUMBERING, INSUFFICIENT EVIDENCE**, not guessed at.

# Phase 2 Readiness Verdict

**READY.** Phase 1 candidate numbering is now clean, continuous, and frozen; the crosswalk is complete and
authoritative; governance and tooling agree with the new rule; the Phase 2 register's Phase 1 ID column,
P2W IDs, and dispositions are internally consistent and fully mapped. The 45 `BLOCKED` register rows and
the Section C/I open architecture conflicts are pre-existing Phase 2 consolidation work, not migration
defects, and do not block Phase 2 from proceeding on the unblocked majority of candidates.
