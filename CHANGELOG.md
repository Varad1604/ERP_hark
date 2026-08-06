# SutraERP Architecture Changelog

This document tracks changes to the SutraERP Architecture Repository layout, guidelines, baseline, and locks.

## [1.5.2] - 2026-08-06

Remediation against `generated/reviews/review-repository-integrity-2026-08-06.md`.
Reconstructed from commit history and dated in-file notes; the 1.5.0 and 1.5.1 blocks
below were missing entirely, contrary to `CONTRIBUTING.md` section 7 item 2, which
requires a changelog entry on every merge (integrity finding INT-209).

### Changed
- **INT-202 (Critical):** Resolved the Section G dependency conflict and the **D ↔ G
  cycle** by reclassification. Ten Section G documents declared Sections C, D and F under
  `**Upstream Dependencies**` against a foundational set of A, B and E. Each declaration
  was checked against the document's Appendix B Canonical Ownership Matrix and body text;
  all resolved to ownership routing rather than foundational dependency. They were moved
  to a new `**Consumed From**` block in G02–G09 and recorded as 15 new rows in the
  `DEPENDENCY_MAP.md` section 7A register. Section G's foundational set is unchanged.
  Three Section D declarations were deleted rather than reclassified — `L2-G04`, `L2-G05`
  and `L2-G06` each listed Section D upstream with no Academic Delivery consumption
  anywhere in the document. Verified: no Section G document now declares C, D or F as a
  foundational upstream.
- **INT-201:** Indexed the two L5 Solution Architecture documents that every control
  artifact previously denied. `ARCHITECTURE_INDEX.md` gains an `L5 — Solution
  Architecture` section and its statistics move to 2 L5 documents and 115 total.
  `architecture/L5_Solution_Architecture/README.md`, `README.md` and
  `AI_CONTEXT/REPO_UNDERSTANDING.md` corrected to stop describing L5 as an empty
  placeholder. Both documents remain `Draft` and neither traces to an approved L4.
- **INT-203:** `AI_CONTEXT/SECTION_G/Dependency_Contract.md` asserted that no Section G
  document declares dependency tables. Ten of eleven do. Replaced with the actual
  per-file upstream table read from source, including the INT-202 conflict it had
  concealed.
- **INT-204:** `DEPENDENCY_MAP.md` Section I row corrected from "Sections C, D, F" to
  "Sections A, B, C, D, F". It was the only row in the matrix omitting Section A.
- **INT-205:** Removed the legacy `Owner` field from 44 L2 documents — 41 deletions and
  13 conversions to `Review Authority`, 54 occurrences in total. The INT-003 capability
  ownership migration added `Document Owner` without removing its predecessor, leaving
  44 documents with two owners. All 107 documents retain their canonical capability
  owner. `L2-C08`'s contradictory second `Document Owner` reconciled to its header value.
- **INT-206:** Repaired `L2-C08`'s Consumed Domain Events table, in which every row named
  an event or a publishing domain that does not exist. `QualificationCompleted` →
  `QualificationAwarded` from `L2-C05`. `QualificationCompletionVerified` →
  `ProgrammeCompletionVerified` from `L2-I13`, which defines it in §11.3 with a "Published
  When" clause. `ClearanceCompleted` from the non-existent "Clearance Management" →
  `InstitutionalClearanceCompleted` from `L2-C05`, whose Qualification Completion
  Aggregate owns institutional clearance coordination (`L2-C05` line 4085). The
  `AlumniTransitionApprovalCompleted` row was deleted: it was not a consumed event at all,
  but a duplicate of C08's own published `AlumniTransitionApproved` (§10.4), attributed to
  an invented "Approval Management" domain. Three of four consumed events now resolve to a
  document that genuinely publishes them.
- **INT-207:** `Volume_2_Repository_Standards.md` line 147 requires the Architecture Index
  status legend to mirror its canonical table **exactly**; the Index listed 9 of 13 values
  and stated at line 351 that the subsetting was deliberate. Resolved by completing the
  Index legend to all 13 values, reproduced verbatim in Volume 2's order. Because Volume 2
  already describes Proposed, Approved, Deprecated and Archived as "Not yet in active
  use", the legal-versus-in-use distinction is preserved, and **no amendment to the LOCKED
  Volume 2 was required** — it remains v1.1. Also corrected the Index's claim that the
  v1.0 → v1.1 amendment adopted a 9-value vocabulary; it adopted 13. Verified: the two
  tables are now an exact row-for-row match.
- **INT-208:** Removed the `tools/` entry from `README.md`; the directory no longer
  exists. Corrected the `generated/` description from HTML presentation artifacts to
  review artifacts, and dropped "helper scripts" from the repository philosophy.
- **INT-210:** Repaired the corrupted YAML dependency entry at `L2-C04` line 20 and its
  stale `L2-B05` title reference.
- **INT-211:** `AI_CONTEXT/INDEX.md` no longer lists the deleted
  `AI_CONTEXT/SECTION_H/README.md`.
- **INT-212:** `L2-C08` section 26.8 retitled "Proposed Repository Structure — Not Yet
  Created"; it presented a `diagrams/`/`adr/`/`assets/` tree that does not exist.
- **INT-213:** Removed the duplicate `document-owner` frontmatter key from `L2-B05`.
- Corrected one factual error in the 2026-08-05 review addendum: it dismissed INT-018 on
  the grounds that `architecture_audit_report.md` does not exist. It does; the finding is
  resolved by that file's own Superseded label.

- **Minor changes closed 2026-08-06.** Both items the review left open were resolved:
  - **Section G runtime routings.** Added the two Section J rows (`L2-G07` Security Vendor
    Contracts, `L2-G08` Vendor Contracts) to the `DEPENDENCY_MAP.md` section 7A register.
    Registered the nine Section H notification routings as an **exemption** rather than
    entries, because section 5 of the map already states that cross-cutting Section H
    services do not create a foundational upstream dependency for consuming sections.
  - **Missing bounded contexts.** Investigating the Alumni Transition policy gap showed it
    was not confined to one row. Added `DEPENDENCY_MAP.md` **section 7B — Known Missing
    Bounded Contexts**, recording **Policy Management** as depended upon by `L2-C08` (10
    references) and `L2-C04` (3 references, including an unpublished `AcademicPolicyUpdated`
    event and a `Get Academic Policy` query). No owner was assigned: `L2-C08` line 5037
    states policy ownership is external to it, `L2-A04` is access control and explicitly
    independent of business domains, and `L2-A08` section 18 governs "architectural
    metadata only". Assigning an owner is an architecture decision requiring Lead Architect
    sign-off, not a documentation correction. Consuming documents are marked at the point
    of use with a do-not-implement instruction.
  - **Two further invented bounded contexts** were found in `L2-C08`'s primary upstream
    table while doing this. **Clearance Management** was replaced with `L2-C05`, which
    aggregates clearances from Sections F, G03 and G05 and publishes
    `InstitutionalClearanceCompleted`. **Approval Management** was removed as an upstream
    entirely — C08 publishes `AlumniTransitionApproved` itself. A governing note at C08
    line 188 maps all three names for the fifteen scattered occurrences left in place.
  - **`L2-D03` false attribution.** Section 7.6 stated "Enterprise governance
    responsibilities are provided by Section I." Section I is Examination.
    `DEPENDENCY_MAP.md` lines 277-286 scope it to assessment and results. Replaced with a
    table routing Compliance Validation to `L2-H09`, Audit Governance to `L2-A07`,
    Operational Readiness to `L2-A10`, Architectural Governance and Enterprise Standards to
    the Repository Constitution, and marking Policy Management, Risk Management and
    Decision Governance as unowned.

### Open
- **Policy Management has no owner** — see section 7B of `DEPENDENCY_MAP.md`. Needs a Lead
  Architect decision, not research; the three candidate homes are excluded on the evidence.
- **Risk Management and Decision Governance have no owner.** Named as governance
  capabilities by `L2-D03` section 7.6; no document in the repository claims either.
- **INT-207 (remainder):** `Volume_2_Repository_Standards.md` line 147 requires the
  Architecture Index legend to mirror its 13-value table exactly, which the Index
  knowingly does not. Volume 2 is LOCKED; amending it needs Lead Architect approval.

---

## [1.5.1] - 2026-08-06

### Changed
- INT-003 follow-up: aligned the `ARCHITECTURE_INDEX.md` owner column with the capability
  owners established in `TEAM_ASSIGNMENTS.md` (`0613700`).

---

## [1.5.0] - 2026-08-05

Remediation round 2 against `generated/reviews/integrity-2026-08-03-repository-wide-review.md`
and `generated/reviews/review-repository-integrity-2026-08-05.md`.

### Added
- `TEAM_ASSIGNMENTS.md` Canonical Capability Owners table: every L2 document now has a
  capability-area owner with a named Accountable Architect, so ownership survives
  reassignment of people (INT-003, `780eb65`).
- `DEPENDENCY_MAP.md` section 7A Dependency Classes and the Runtime Contract and Event
  Consumption Register, separating foundational dependencies from runtime ones
  (INT-035, `f578dca`).
- `architecture/L5_Solution_Architecture/L5-01_Infrastructure_Persistence_Baseline.md`,
  extracted from `L2-B03` Part 14 (INT-008, `356bf5b`).
- Integrity reviews brought under version control so findings stop being re-raised
  (`631690b`).

### Changed
- INT-008: canonicalized the result publication event as `ExaminationResultPublished`
  (`9bdde91`), then removed `FinalResultPublished` as a sixth variant rather than a
  distinct event (`6914ec8`).
- INT-008: migrated the physical infrastructure architecture from `L2-B03` to `L2-G10`
  across six steps (`5d5754c`, `1c0adda`, `7ea1e58`, `356bf5b`).
- INT-004/005/017: separated the infrastructure, scholarship eligibility and payroll
  boundaries (`d10593f`).
- INT-009/010/011: removed three reversed dependency declarations in A10, D07 and F14
  (`d84f0cf`).
- INT-001/004/035: ownership reconciliation in C08, I13 and J06 (`f578dca`).
- Prompts 0/1/2 re-run across all ten sections, replacing existing AI context content
  (`f6ce1ff`).

---

## [1.4.0] - 2026-08-04

Remediation round 1 against `generated/reviews/integrity-2026-08-03-repository-wide-review.md`.

### Changed
- INT-005: L1 closing metadata version corrected to 2.1 to match its header.
- INT-019: `ARCHITECTURE_INDEX.md` core-document count corrected to 6 and total to 113.
- INT-021: Normalised L2 H1 separators and corrected section-name drift in D01, E06,
  E07, F00, and F01 to match `ARCHITECTURE_INDEX.md` verbatim.
- INT-022: Corrected D01 self-references to its real filename.
- INT-024: Prompts 0/1/2 now use the prefixed AI-context basename; Section J file
  layout inverted to match Sections A-I.
- INT-025: Section J README trust-source labels replaced with exact A03-A07 names.
- INT-003: **L1 section 25 replaced.** The obsolete two-team ownership model was removed
  from the locked L1 document and from L0, and replaced with Section-based ownership
  sourced from `TEAM_ASSIGNMENTS.md`. L1 is a LOCKED document; this amendment is
  recorded here and in the L1 header.

- INT-007: **L1 StudentContext corrected.** Section 3.5 gated a new Context on a
  "Completed" state that exists in neither L1's own state machine nor C02's canonical
  list. The gate is now the five terminal states, and Suspended (non-terminal,
  reversible) was added to L1's machine to match C02 section 11.10.1. A terminal-state
  table was added so the enumeration is unambiguous.
- INT-006: **L1 Tenant lifecycle expanded.** L1 defined only Requested/Provisioning/
  Active/Failed. L2-A01 adds Maintenance, Suspended, Upgrade and Archived. L1 now
  carries the full set with a terminal-state table. A01's own contradiction was fixed
  first: its diagram showed Upgrade while its state table (5.5) and transition table
  (5.8) omitted it, and the Provisioning Failed retry transition was undeclared.

- INT-002: **L0 content boundary restored.** Volume 1 requires L0 to be
  business-facing and technology-independent. L0 section 3 prescribed a concrete
  stack (Rust, Axum, Tokio, PostgreSQL, SQLx, PgBouncer, Redis, React, Vite,
  TypeScript, Zustand, Axios, MinIO, JWT/Argon2id). That content moved verbatim to
  the new `architecture/L5_Solution_Architecture/L5-00_Technology_Stack_Baseline.md`
  (Draft, explicitly not yet re-reviewed as L5). L0 section 3 now states the modular
  platform decision and its standing risk without naming technology. The event-bus
  passage in section 13 was restated technology-independently, and section 25's
  stray H1 asserting L1 content inside L0 was demoted and marked historical.

- INT-001: **Architecture hierarchy made single-valued.** L0 Section 23 gave the levels
  different meanings from Section 0, calling L4 "Individual Module Workflows" and L5
  "Role / Position-Based Workflows". Section 0 confirmed canonical by the repository
  owner; Section 23 now reproduces it verbatim. L2-C02 carried a separate non-canonical
  five-level scheme ending at "L4 Technical Design & Implementation" and was corrected
  to the canonical seven levels.
- INT-004: **L1 dependency order replaced.** L1 described a single serial chain ending
  with Communication and then the Workflow Engine. The Workflow Engine is L2-A05, inside
  foundational Section A, so it cannot be terminal; the chain also placed Examination
  before Finance, reversing the canonical matrix and creating a false cycle. Per the
  owner's decision, DEPENDENCY_MAP.md governs. L1 now carries its directed matrix and a
  locked rule that Section A is foundational.
- INT-035: **Dependency classes added to DEPENDENCY_MAP.md** (new Section 7A). The register
  recorded only foundational dependencies, so runtime integrations were invisible.
  Foundational, runtime-contract, event-consumption and read-model classes are now defined,
  with a register recording F14 to Section J, Section C to Section I, and the J07 stock
  movement events. Only foundational dependencies constrain build order.
- INT-009, INT-010, INT-011, INT-014: ownership boundaries reconciled in I13, C08 and J06.
  Fixing J06's stock ownership removed the InventoryStockIssued/InventoryStockReturned
  names entirely rather than renaming them.

### Removed
- INT-003: Deleted `generated/html/` (untracked, gitignored). To be regenerated with
  `tools/convert_all.js` after remediation completes.

---

## [1.3.0] - 2026-07-29

### Added
- Added placeholder infrastructure for L3 through L6 architecture folders.
- Added Section J AI context placeholders for Enterprise Resource Management.
- Added `governance/policies/Architecture_Level_Validation_Standards.md`.

### Changed
- Standardized future-level architecture folder names to the approved L0-L6 hierarchy.
- Updated templates, prompts, repository metadata, and validation references for Phase 2 governance migration.
- Updated Section J repository references to Enterprise Resource Management.

---

## [1.2.0] - 2026-07-08

### Added
- Created `DEPENDENCY_MAP.md` mapping downstream/upstream constraints, level structures, L2 section flows, and LLM context segmentation strategy.

### Changed
- Updated `README.md` to include a Repository Governance registry summary.
- Updated `CONTRIBUTING.md` to add Repository Maintenance post-merge requirements.
- Updated `ARCHITECTURE_INDEX.md` to index the new `DEPENDENCY_MAP.md` file.

---

## [1.1.0] - 2026-07-08

### Added
- Created `templates/` folder with templates for Levels 2, 3, 4, 5, and 6.
- Added `CONTRIBUTING.md` defining Git, PR, and review guidelines.
- Added `ARCHITECTURE_INDEX.md` mapping domain ownership and lifecycle status.
- Added main `README.md` navigation hub.
- Added `.gitignore`.

### Changed
- Reorganized the repository structure to align with the target architecture blueprint.
- Relocated baseline documents, Level 1 architecture, Level 2 matrix, and presentation files into their corresponding hierarchical folders:
  - `docs/baseline/`
  - `docs/collaboration/`
  - `docs/level-1/`
  - `docs/level-2/`
  - `presentation/`

---

## [1.0.0] - 2026-07-08

### Added
- Initial repository setup containing baseline workflows, collaboration rules, Level 1 locks, Level 2 domain planning matrix, and presentation pages.
