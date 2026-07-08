# SutraERP Architecture Changelog

This document tracks changes to the SutraERP Architecture Repository layout, guidelines, baseline, and locks.

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
