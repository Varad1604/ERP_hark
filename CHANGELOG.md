# SutraERP Architecture Changelog

This document tracks changes to the SutraERP Architecture Repository layout, guidelines, baseline, and locks.

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
