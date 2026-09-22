# SutraERP L5 — Canonical Production Specifications

**Architecture Level:** L5 — Solution Architecture  
**Purpose:** Canonical repository location for the frozen 105-specification L5 production corpus  
**Broad Authoring:** Authorized  
**Final L5 Corpus Lock:** Not Authorized  

---

## Canonical Authoring Location

All newly authored canonical L5 Solution Architecture specifications belong under this directory.

The frozen authoring population contains exactly **105 specifications** across seven technical architecture families and thirty canonical major areas:

| Family | Specifications | Major Areas |
|---|---:|---:|
| `L5-A` — Core Platform & Application Architecture | 20 | 8 |
| `L5-B` — Data & Information Architecture | 21 | 6 |
| `L5-C` — Contracts, Events & Integration Architecture | 18 | 4 |
| `L5-D` — Identity, Security & Trust Architecture | 19 | 4 |
| `L5-E` — Reliability, Deployment & Operations Architecture | 16 | 5 |
| `L5-F` — Engineering Delivery & Quality Architecture | 7 | 2 |
| `L5-G` — Client Platform Architecture | 4 | 1 |
| **Total** | **105** | **30** |

The canonical specification identifiers and responsibility boundaries are frozen by `../governance/instruction/L5_Authoring_Population_Freeze_Decision_105_2026-09-09.md` and defined by the active 105-specification register.

---

## Canonical Family-Level Hierarchy

Production specifications shall be placed directly under the exact family folder matching their canonical ID.

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

Do not introduce major-area subdirectories such as `L5-A/L5-A-01/` or `L5-B/L5-B-02/`. Major-area identity is already encoded in each canonical specification ID and filename.

Canonical placement:

```text
specifications/L5-<Family>/L5-<Family>-<Area>-<Document>_<Canonical_Solution_Slug>.md
```

Example:

```text
L5-A-07-003
→ specifications/L5-A/
→ L5-A-07-003_Examination_Result_Computation_Engine_Architecture.md
```

Do not create new family folders without an explicit architecture-governance decision changing the frozen corpus structure.

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

## Required Governance Instruction Pack

For routine production authoring and review, start at:

```text
../governance/instruction/README.md
```

Every production specification shall conform to the active instruction set, including:

- `../governance/instruction/L5_Governance_and_Architecture_Standard.md`;
- `../governance/instruction/L5_Solution_Architecture_Specification_Template_v1.0.md`;
- `../governance/instruction/L5_Validation_Standard.md`;
- `../governance/instruction/L5_Candidate_Document_Corpus_Register_105_2026-09-09.md`;
- `../governance/instruction/L5_Workload_Complexity_and_Five_Person_Allocation_Decision_2026-09-09.md`;
- `../governance/instruction/L5_Dependency_Gated_Parallel_Authoring_Execution_Timeline_2026-09-09.md`;
- `../governance/instruction/L5_Naming_and_Document_Granularity_Specialization.md`;
- `../governance/instruction/L5_Architecture_Family_Register.md`;
- `../governance/instruction/L5_L4_to_L5_Traceability_Methodology.md`;
- `../governance/instruction/L5_Technology_Selection_and_ADR_Methodology.md`;
- `../governance/instruction/L5_ADR_Register.md`;
- `../governance/instruction/L5_Broad_Authoring_Authorization_105_2026-09-09.md`;
- `../governance/instruction/L5_Authoring_Population_Freeze_Decision_105_2026-09-09.md`.

The execution model is dependency-gated parallel authoring. A specification may begin when its required upstream L5 dependencies are stable enough that the author does not need to invent those responsibilities.

Supporting planning decisions and provenance records are available under `../governance/reference/` when needed; they are not part of the normal daily authoring pack.

---

## Content That Does Not Belong Here

Do not place the following under `specifications/`:

- governance material — use `../governance/instruction/` or `../governance/reference/` according to operational role;
- historical extracted technical material — use `../historical/`;
- canonical ADR decision documents — use `../adr/` once the first ADR is formally allocated;
- L6 screen, navigation, journey or visual-design specifications;
- implementation source code or deployment configuration.

---

## Repository Control Boundary

This directory defines canonical document placement only. Branch selection, push permissions, merge timing and manual assignment remain under Lead Architect control.

Broad L5 authoring is authorized, but no authored specification becomes Architecture Approved or Locked merely by being committed here.


## Locked Technology Decision Wave — 2026-09-22

Production specifications shall inherit all applicable Locked decisions through `ADR-L5-0047`. Backend-readiness ADRs `ADR-L5-0034`–`ADR-L5-0047` add contract, persistence/cache, IAM/security, asynchronous execution, deployment/operations and capacity/recovery/optimization governance without altering the frozen 105-specification responsibility population. Final L5 Corpus Lock is not authorized.


## Post-Decision Audit State

As of 2026-09-22, the 105 canonical specification headers are synchronized to **0 Draft / 51 Review / 21 Architecture Approved / 33 Locked**. Lifecycle status shall not be strengthened merely because a technology ADR is Locked.
