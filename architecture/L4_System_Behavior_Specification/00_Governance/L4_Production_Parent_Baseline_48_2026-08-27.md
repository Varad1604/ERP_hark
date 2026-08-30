# SutraERP L4 Production Parent Baseline — 48 Certified Parents — 2026-08-27


> **HISTORICAL / SUPERSEDED FOR ACTIVE EXECUTION — 2026-08-29**
>
> This record preserves the earlier controlled 48-parent production-wave decision. It **must not be used as current authoring scope**.
>
> Current active authority: `L4_Full_Corpus_Parallel_Authoring_Authorization_2026-08-29.md` and `L4_Full_Corpus_Parallel_Authoring_Execution_Plan_1084_Scenarios_2026-08-29.md`.
>
> Current parent baseline: **1,084 / 1,084 canonical L3 scenarios are Locked and ready for L4 mass authoring** under `L4_Full_Corpus_Parent_Readiness_Baseline_1084_2026-08-29.md`.
>
> The original content below remains unchanged as architecture-decision provenance.

**Version:** 1.0.0  
**Status:** Architecture Baseline  
**Owner:** Lead Architect  
**Architectural Level:** L4 — System Behavior Specification Governance  
**Domain:** Cross-domain  
**Last Updated:** 2026-08-27  
**Depends On:** `L4_Initial_Certified_Parent_Eligibility_Baseline_2026-08-27.md`; `L4_Parent_Readiness_Validation_45_Newly_Approved_Scenarios_2026-08-27.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`  
**Referenced By:** L4 production authoring structure, execution plan, parallel-authoring authorization  

---

## 1. Purpose

This document establishes the explicit L3 parent population authorized for the first canonical L4 production wave.

It provides the repository-wide disposition required by the Locked L4 Broad Authoring Gate without inferring eligibility for unreviewed L3 scenarios.

The canonical L3 population remains 1,084 scenarios. This production baseline classifies that population for the current L4 wave as:

- **48 certified / eligible for canonical L4 production authoring**; and
- **1,036 deferred from this production wave pending separate L4 parent-readiness certification or explicit governed disposition**.

`Deferred` in this document is an execution disposition, not a replacement document status and not a judgment that the underlying L3 scenario is architecturally invalid.

---

## 2. Current Production-Eligible Parent Set

### Section D — 1

- `L3-D-007`

### Section G — 8

- `L3-G-279`
- `L3-G-282`
- `L3-G-283`
- `L3-G-287`
- `L3-G-288`
- `L3-G-289`
- `L3-G-290`
- `L3-G-293`

### Section H — 37

- `L3-H-005`
- `L3-H-008`
- `L3-H-020`
- `L3-H-021`
- `L3-H-022`
- `L3-H-025`
- `L3-H-026`
- `L3-H-027`
- `L3-H-030`
- `L3-H-032`
- `L3-H-033`
- `L3-H-034`
- `L3-H-035`
- `L3-H-036`
- `L3-H-037`
- `L3-H-038`
- `L3-H-039`
- `L3-H-040`
- `L3-H-042`
- `L3-H-043`
- `L3-H-046`
- `L3-H-048`
- `L3-H-053`
- `L3-H-054`
- `L3-H-055`
- `L3-H-056`
- `L3-H-058`
- `L3-H-062`
- `L3-H-063`
- `L3-H-066`
- `L3-H-067`
- `L3-H-068`
- `L3-H-073`
- `L3-H-074`
- `L3-H-076`
- `L3-H-079`
- `L3-H-081`

### Section I — 2

- `L3-I-007`
- `L3-I-118`

**Current production-eligible parent population: 48.**

---

## 3. Remaining L3 Corpus Disposition

The remaining **1,036** canonical L3 scenarios are excluded from this production wave unless a later governed parent-readiness decision adds them to the certified baseline.

Rules:

1. No L4 file may be created for a parent outside the certified production baseline merely because the L3 file exists.
2. Historical section completion, authoring completion, or approval summaries do not create L4 eligibility.
3. A later certified parent may be added through a controlled baseline amendment without reopening already certified parents unless an upstream change affects them.
4. Known L3 scenarios currently in `Review`, including `L3-D-025`, `L3-D-029`, and `L3-D-043`, remain outside the production baseline until their status/readiness changes through governance.
5. This disposition does not modify any L3 status, ownership, business rule, lifecycle, event, authority, or scenario identity.

---

## 4. Production Mapping Rule

For every certified parent in this baseline, the canonical default is:

```text
1 eligible L3 scenario
        ↓
1 canonical parent L4 specification
        ↓
N System Behavioral Obligations (BHs)
```

The L4 identifier and filename inherit the approved L3 traceability identity:

```text
L3-X-NNN
  ↓
L4-X-NNN
  ↓
L4-X-NNN_<Canonical_L3_Scenario_Slug>.md
```

A physical split requires a separate architecture-review exception under the Locked L4 governance standard.

---

## 5. Pilot Disposition

The three existing files under `00_Pilot/` are methodology evidence only.

For `L3-D-007`, `L3-I-007`, and `L3-I-118`:

- the pilot artifact shall not be treated as the canonical production file in place;
- canonical production authoring shall create a file under `01_Production/` using the Locked template and current approved parent;
- pilot content may be consulted as historical validation evidence but must be revalidated against the current parent and Locked L4 standards;
- no pilot status is automatically promoted into the production corpus.

---

## 6. Baseline Integrity Rule

A parent leaves the active production baseline if a material upstream change makes its approved L3 version stale or creates a material ownership/rule/lifecycle/event/authority/boundary conflict.

Affected L4 work shall stop until parent readiness is re-established.

---

## 7. Decision

**CANONICAL L3 POPULATION:** **1,084**  
**CURRENT L4 PRODUCTION-ELIGIBLE PARENTS:** **48**  
**DEFERRED FROM CURRENT L4 PRODUCTION WAVE:** **1,036**  
**PRODUCTION SCOPE SECTIONS:** D, G, H, I  
**PILOT ARTIFACTS:** HISTORICAL / NON-CANONICAL PRODUCTION EVIDENCE  
**ELIGIBILITY OUTSIDE THIS BASELINE:** NOT AUTHORIZED BY INFERENCE
