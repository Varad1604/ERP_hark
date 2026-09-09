# SutraERP L5 — Canonical Production Specifications

**Architecture Level:** L5 — Solution Architecture  
**Purpose:** Canonical repository location for the frozen 105-specification L5 production corpus  
**Broad Authoring:** Authorized  
**Final L5 Corpus Lock:** Not Authorized  

---

## Canonical Authoring Location

All newly authored canonical L5 Solution Architecture specifications belong under this directory.

The frozen authoring population contains exactly **105 specifications** across seven technical architecture families:

| Family | Specifications |
|---|---:|
| `L5-A` — Core Platform & Application Architecture | 20 |
| `L5-B` — Data & Information Architecture | 21 |
| `L5-C` — Contracts, Events & Integration Architecture | 18 |
| `L5-D` — Identity, Security & Trust Architecture | 19 |
| `L5-E` — Reliability, Deployment & Operations Architecture | 16 |
| `L5-F` — Engineering Delivery & Quality Architecture | 7 |
| `L5-G` — Client Platform Architecture | 4 |
| **Total** | **105** |

The canonical specification identifiers and responsibility boundaries are frozen by `../governance/L5_Authoring_Population_Freeze_Decision_105_2026-09-09.md`.

---

## Family Directories

When the first real production specification for a family is committed, place it under the matching family directory:

```text
specifications/
├── L5-A/
├── L5-B/
├── L5-C/
├── L5-D/
├── L5-E/
├── L5-F/
└── L5-G/
```

Do not create empty family folders merely as placeholders. Git itself does not preserve empty directories, and repository structure shall represent real content rather than implied completion/readiness.

---

## Filename Rule

Every canonical specification shall use:

```text
L5-<Family>-<Area>-<Document>_<Canonical_Solution_Slug>.md
```

Example:

```text
L5-A-07-003_Examination_Result_Computation_Engine_Architecture.md
```

Identifiers shall match the frozen 105-specification register exactly.

---

## Authoring Controls

Every production specification shall conform to the current L5 governance corpus, including:

- `../governance/L5_Governance_and_Architecture_Standard.md`;
- `../governance/L5_Solution_Architecture_Specification_Template_v1.0.md`;
- `../governance/L5_Validation_Standard.md`;
- `../governance/L5_L4_to_L5_Traceability_Methodology.md`;
- `../governance/L5_Technology_Selection_and_ADR_Methodology.md`;
- `../governance/L5_Dependency_Gated_Parallel_Authoring_Execution_Timeline_2026-09-09.md`;
- `../governance/L5_Broad_Authoring_Authorization_105_2026-09-09.md`.

The execution model is dependency-gated parallel authoring. A specification may begin when its required upstream L5 dependencies are stable enough that the author does not need to invent those responsibilities.

---

## Content That Does Not Belong Here

Do not place the following under `specifications/`:

- governance standards, templates, registers or decisions — use `../governance/`;
- historical extracted technical material — use `../historical/`;
- canonical ADR decision documents — use `../adr/` once the first ADR is formally allocated;
- L6 screen, navigation, journey or visual-design specifications;
- implementation source code or deployment configuration.

---

## Repository Control Boundary

This directory defines canonical document placement only. Branch selection, push permissions, merge timing and manual assignment remain under Lead Architect control.

Broad L5 authoring is authorized, but no authored specification becomes Architecture Approved or Locked merely by being committed here.
