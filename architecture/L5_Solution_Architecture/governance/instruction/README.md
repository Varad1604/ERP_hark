# SutraERP L5 — Team Instruction Pack

This folder contains the governance files expected to be used directly during L5 authoring and review.

## Required Daily Working Set

1. `L5_Governance_and_Architecture_Standard.md` — defines what L5 may and may not own.
2. `L5_Solution_Architecture_Specification_Template_v1.0.md` — canonical authoring template.
3. `L5_Validation_Standard.md` — required validation and quality gates.
4. `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` — canonical 105-specification IDs and responsibility boundaries.
5. `L5_Workload_Complexity_and_Five_Person_Allocation_Decision_2026-09-09.md` — approved author assignment.
6. `L5_Dependency_Gated_Parallel_Authoring_Execution_Timeline_2026-09-09.md` — execution order and dependency gates.
7. `L5_Naming_and_Document_Granularity_Specialization.md` — identifier, filename and granularity rules.
8. `L5_Architecture_Family_Register.md` — family ownership and major-area placement.
9. `L5_L4_to_L5_Traceability_Methodology.md` — L4-to-L5 traceability method.
10. `L5_Technology_Selection_and_ADR_Methodology.md` — governs material technology decisions and ADR use.
11. `L5_ADR_Register.md` — canonical ADR identity register.
12. `L5_Broad_Authoring_Authorization_105_2026-09-09.md` — confirms mass authoring is authorized.
13. `L5_Authoring_Population_Freeze_Decision_105_2026-09-09.md` — confirms IDs/responsibility boundaries are frozen.

## Primary Technical-Depth Benchmark for AI and Human Authors

Before mass-authoring any production L5 specification, review:

`examples/AI_TECHNICAL_DEPTH_REFERENCE_L5-A-07-001_Secure_Computational_Engine_Architecture_Baseline.md`

This is the **primary technical-depth benchmark** for L5 authoring. It is modelled on `L5-A-07-001` and demonstrates the level of concrete technical architecture expected from both human authors and AI authoring agents.

It shows the expected treatment of:

- technical ownership and explicit non-ownership;
- peer and dependency boundaries;
- architecture-level execution flows;
- conceptual contracts;
- trust and isolation boundaries;
- deterministic behavior where applicable;
- material failure behavior and containment;
- local `AD-*` architecture decisions;
- objective `CR-*` conformance requirements;
- unresolved decisions and ADR escalation;
- the distinction between concrete L5 architecture and vague technical prose.

### Mandatory AI authoring rule

AI-generated L5 specifications must be **at approximately this technical depth or deeper where the owning subject requires it**. The reference is not a rigid universal template: database, IAM, API, messaging, deployment, testing and client specifications will naturally require different technical content. However, an AI Draft is not acceptable merely because it follows headings or restates L4 behavior.

An AI-authored Draft must define enough concrete architecture that implementation does not have to invent a major mechanism that properly belongs to the L5 responsibility. At the same time, the AI must not add technical detail that steals another L5 owner's responsibility, changes L1-L4 business meaning, selects technology without governance, becomes L6 UI/UX, or turns into source-code/physical-design documentation.

The AI technical-depth reference is a **non-canonical training document**. It is not part of the frozen 105-specification production corpus and must not be copied into `specifications/` as if it were the canonical production `L5-A-07-001` document.

## Secondary Worked Example

A broader traceability-oriented training example also exists at:

`examples/WORKED_EXAMPLE_L5_Tenant_Institution_Isolation_Architecture.md`

Use the Secure Computational Engine reference above as the primary benchmark for technical depth. Use this secondary example only when additional illustration of L4 traceability, ownership boundaries, `AD-*`, `CR-*`, `OD-*` and ADR treatment is useful.

## Working Rule

Authors should not need to browse the full governance history for normal work. Start here, use the production specification location under `../../specifications/`, and consult `../reference/` only when a planning decision, coverage decision, approval record or provenance question must be checked.

The distinction between `instruction/` and `reference/` is operational convenience only. It does not change the authority or status of any document.


## 2026-09-22 Locked Technology Decision Wave

`ADR-L5-0019` through `ADR-L5-0048` contain the current canonical Locked technology/backend-readiness decisions. Do not re-select these decisions locally. The next ADR allocation is `ADR-L5-0049`. The Final L5 Corpus Lock was explicitly authorized on 2026-09-23 and is canonical on `main` as merge commit `fe9d03d6b70f17f071340b60394589cadb31d5b4`.


## 2026-09-22 Post-Decision Corpus Audit

The full 105-specification corpus has completed a post-decision synchronization audit. Confirmed stale contradictions were repaired; D03 lifecycle metadata was restored to the `Review` state required by the locked full-corpus Draft-to-Review promotion decision.

Current lifecycle distribution: **0 Draft / 51 Review / 21 Architecture Approved / 33 Locked**.

Do not infer Final L5 Corpus Lock from backend-readiness completion. Remaining Review specifications require formal validation, and materially impacted approved specifications remain subject to normal impact revalidation.


## 2026-09-23 R3C Non-Human Principal Lifecycle

`ADR-L5-0048` is the Locked canonical non-human technical-principal lifecycle decision. Current ADR allocation is 48; next allocation is `ADR-L5-0049`. This decision does not alter the Batch-9 NO-LOCK verdict.


## Final L5 Corpus Lock — 2026-09-23

The final-lock gate passed with zero current hard failures and zero mandatory pending architecture decisions. Explicit Lead Architect authorization was then issued for the distinct Final L5 Corpus Lock action.

The controlled lock branch records:

```text
Canonical L5 specifications       105
Draft                               0
Review                              0
Architecture Approved               0
Locked                             105
ADR allocations                     48
Next ADR                   ADR-L5-0049
L4→L5 relations                    571
Duplicate relation identities        0
Stale L5 bindings                    0
```

The final lock changes lifecycle governance only. Material future changes to locked L5 architecture require governed impact assessment and revalidation.
