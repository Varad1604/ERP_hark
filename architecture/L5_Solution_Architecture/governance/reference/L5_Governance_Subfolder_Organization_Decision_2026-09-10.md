# SutraERP L5 — Governance Subfolder Organization Decision

**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Lead Architect  
**Architectural Level:** L5 — Solution Architecture Governance  
**Domain:** Cross-domain  
**Decision Date:** 2026-09-10  

**Depends On:** `L5_Repository_Structure_and_Content_Placement_Decision_2026-09-09.md`; `L5_Governance_and_Architecture_Standard.md`; `L5_Broad_Authoring_Authorization_105_2026-09-09.md`.

---

## 1. Decision

The L5 governance directory shall be subdivided into an operational instruction set and a supporting reference library.

```text
architecture/L5_Solution_Architecture/governance/
├── README.md
├── instruction/
└── reference/
```

This decision supersedes only the earlier statement in `L5_Repository_Structure_and_Content_Placement_Decision_2026-09-09.md` that the governance directory should remain flat unless a later governance decision justified subdivision. This record is that later decision.

No other part of the prior repository-structure decision is superseded.

---

## 2. `instruction/` — Required Team Working Set

The following files are placed in `instruction/` because they are directly relevant to routine L5 authoring, review, assignment, naming, traceability, execution or architecture-decision handling:

1. `L5_Governance_and_Architecture_Standard.md`
2. `L5_Solution_Architecture_Specification_Template_v1.0.md`
3. `L5_Validation_Standard.md`
4. `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md`
5. `L5_Workload_Complexity_and_Five_Person_Allocation_Decision_2026-09-09.md`
6. `L5_Dependency_Gated_Parallel_Authoring_Execution_Timeline_2026-09-09.md`
7. `L5_Naming_and_Document_Granularity_Specialization.md`
8. `L5_Architecture_Family_Register.md`
9. `L5_L4_to_L5_Traceability_Methodology.md`
10. `L5_Technology_Selection_and_ADR_Methodology.md`
11. `L5_ADR_Register.md`
12. `L5_Broad_Authoring_Authorization_105_2026-09-09.md`
13. `L5_Authoring_Population_Freeze_Decision_105_2026-09-09.md`

These files form the canonical L5 team instruction pack.

---

## 3. `reference/` — Supporting Governance and Provenance

The following existing records are placed in `reference/` because they support planning provenance, approval history, coverage evidence or repository governance but are not normally required in every authoring session:

1. `L5_Candidate_Document_Corpus_Register_99_2026-09-09.md`
2. `L5_L4_to_L5_Coverage_Audit_Decision_105_2026-09-09.md`
3. `L5_Major_Architecture_Areas_Baseline.md`
4. `L5_Naming_Specialization_Approval_Decision_2026-09-09.md`
5. `L5_Pilot_ADR_Candidate_Disposition_2026-09-09.md`
6. `L5_Repository_Structure_and_Content_Placement_Decision_2026-09-09.md`
7. this governance-subfolder organization decision.

---

## 4. Authority Preservation

Folder placement does not change document authority.

In particular:

- a Locked record remains Locked;
- an Architecture Approved record remains Architecture Approved;
- a superseded planning record remains preserved as historical decision evidence;
- moving a file to `reference/` does not make it optional when its specific decision becomes relevant;
- moving a file to `instruction/` does not elevate it above the repository authority hierarchy.

Canonical filenames and document contents are preserved during relocation.

---

## 5. Governance Root Rule

The governance root shall contain only its navigation README and the two canonical category folders:

```text
governance/
├── README.md
├── instruction/
└── reference/
```

New governance documents must be classified at creation time:

- place routine authoring/review instructions, standards, templates, active registers and execution controls in `instruction/`;
- place audit decisions, approval provenance, superseded planning evidence and supporting governance records in `reference/`.

A later architecture decision may reclassify a document if its operational role materially changes.

---

## 6. Unchanged Architecture

This reorganization does not change:

- the frozen 105-specification population;
- L5 family or specification identifiers;
- the 30 major architecture areas;
- author workload allocation;
- dependency-gated execution;
- broad-authoring authorization;
- technical or business ownership;
- L4-to-L5 traceability semantics;
- ADR allocation or technology decisions;
- specification approval state;
- Final L5 Corpus Lock state.

Production specifications remain under `architecture/L5_Solution_Architecture/specifications/` and historical extracted technical material remains under `architecture/L5_Solution_Architecture/historical/`.

---

**Status:** Locked  
**Team Working Set:** `governance/instruction/`  
**Supporting Governance:** `governance/reference/`  
**Governance Root:** Navigation only
