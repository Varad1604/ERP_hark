# SutraERP L4 Governance — Active Execution Guide

**Status:** Architecture Baseline  
**Owner:** Lead Architect  
**Last Updated:** 2026-08-29

---

## Active Authoring Controls

The current L4 execution model is **full-corpus authoring across the 1,084 canonical L3 scenario identities**, using the inherited locked L3 Phase 5 human allocation.

Use these documents for current work:

1. `L4_Full_Corpus_Parallel_Authoring_Authorization_2026-08-29.md` — **Locked active authorization**.
2. `L4_Full_Corpus_Parallel_Authoring_Execution_Plan_1084_Scenarios_2026-08-29.md` — active person-wise workload and execution rules.
3. `L4_Governance_and_Architecture_Standard.md` — Locked L4 architecture boundary and BH rules.
4. `L4_System_Behavior_Specification_Template_v1.0.md` — Locked canonical authoring template.
5. `L4_Validation_Standard.md` — Locked parent-readiness and hard-fail validation standard.
6. `L4_Full_Corpus_Parent_Readiness_Certification_Workplan_2026-08-28.md` — per-scenario readiness procedure.
7. `../01_Production/README.md` — canonical production placement and workflow.

## Current Scope Rule

```text
1,084 canonical L3 scenario identities
        ↓
1,084 assigned L4 workload identities
        ↓
Per-scenario L3 parent-readiness gate
        ↓
Eligible parent → create canonical L4 Draft
Blocked/ineligible parent → record/escalate; do not invent upstream architecture
```

**The static 48-parent execution limit is superseded and must not be used to restrict current assigned L4 work.**

---

## Historical 48-Parent Records

The following documents remain in the repository only as architecture-decision provenance for the earlier controlled-wave model:

- `L4_Production_Parent_Baseline_48_2026-08-27.md`
- `L4_Parallel_Authoring_Execution_Plan_48_Parents_2026-08-27.md`
- `L4_Parallel_Authoring_Authorization_2026-08-27.md`
- `L4_Initial_Certified_Parent_Eligibility_Baseline_2026-08-27.md`
- `L4_Parent_Readiness_Validation_45_Newly_Approved_Scenarios_2026-08-27.md`
- `L4_Broad_Authoring_Gate_Closure_2026-08-27.md`

These files are not deleted because they preserve decision history. Their earlier scope statements are superseded for active execution by the Locked 2026-08-29 full-corpus authorization.

---

## Important L3 Readiness Distinction

Full-corpus workload authorization does not automatically convert every L3 parent to `Architecture Approved` or `Locked`.

Current L3 final-lock state is governed separately. See:

`../../L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/L3_Full_Corpus_Final_Lock_and_L4_Readiness_Audit_2026-08-29.md`

Authors must not use the historical 48-parent list as a scope restriction, and must not bypass the current per-scenario L3 readiness gate.
