# SutraERP L5 — Canonical Production Specifications

**Architecture Level:** L5 — Solution Architecture  
**Purpose:** Canonical repository location for the frozen 105-specification L5 production corpus  
**Broad Authoring:** Authorized  
**Final L5 Corpus Lock:** Executed on final-lock branch; canonicalization pending Final Lock PR merge  

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

As of the authorized 2026-09-23 Final Corpus Lock branch, the 105 canonical specification headers are synchronized to **0 Draft / 0 Review / 0 Architecture Approved / 105 Locked**. The 72 final promotions are governed by the distinct Final L5 Corpus Lock authorization; the locked state becomes canonical only after the Final Lock PR is merged.


## Production Validation Register — 2026-09-22

The canonical full-corpus production-validation source is:

`../governance/reference/L5_Full_105_Row_Production_Validation_Register_2026-09-22.md`

It records exactly one current validation disposition for each of the 105 canonical production specifications.

Current lifecycle distribution remains:

```text
0 Draft
0 Review
0 Architecture Approved
105 Locked
105 Total
```

Validation completion does not itself promote specification lifecycle state. The historical Batch-9 NO-LOCK verdict remains historical; subsequent governed remediation has closed its blockers. Final L5 Corpus Lock is ready for a distinct explicit authorization and has not yet been executed.


## Batch 9 Final-Gate Result

The Full Production Validation & Lock-Readiness Programme is complete.

Final result:

```text
105 / 105 validation dispositions present
0 Review specifications
0 current hard blockers
Final L5 Corpus Lock = READY FOR DISTINCT EXPLICIT AUTHORIZATION; NOT EXECUTED
```

See:

`../governance/reference/L5_Production_Validation_Batch9_Final_Corpus_Lock_Gate_2026-09-22.md`


## Independent Final Synchronization Audit — 2026-09-23

A fresh repository-based synchronization audit confirms the living 105-row corpus at **0 Draft / 0 Review / 72 Architecture Approved / 33 Locked**, with zero current hard blockers. Historical Batch-9 counts and verdicts remain valid only as historical milestones and shall not be read as current lifecycle state. Evidence/profile/future-feature deferrals remain intentionally governed and do not constitute current L5 architecture blockers. Final Corpus Lock remains a separate explicit governance action.


## Final L5 Corpus Lock Execution — 2026-09-23

Following the independent Final L5 Corpus Lock Gate PASS and explicit Lead Architect authorization, the remaining 72 Architecture Approved specifications were transitioned to `Locked` on the controlled final-lock branch.

Branch lifecycle result:

```text
0 Draft
0 Review
0 Architecture Approved
105 Locked
105 Total
```

No specification identity, version, canonical technical ownership, ADR decision, L4→L5 relation identity, or business semantic was changed by the lock transition. Five D03 headers that still displayed historical `Review` text were reconciled under the already-established R7 Architecture Approval authority before being locked.

**Canonicalization status:** pending explicit merge authorization for the Final Lock PR.
