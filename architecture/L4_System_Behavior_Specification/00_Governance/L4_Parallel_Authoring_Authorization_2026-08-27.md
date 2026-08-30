# SutraERP L4 Parallel Authoring Authorization — 2026-08-27


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
**Status:** Locked  
**Owner:** Lead Architect  
**Architectural Level:** L4 — System Behavior Specification Governance  
**Domain:** Cross-domain  
**Decision Date:** 2026-08-27  
**Approval Source:** Explicit Lead Architect / repository-authority instruction to complete the L4 broad-authoring enablement gates and proceed  
**Depends On:** `L4_Production_Parent_Baseline_48_2026-08-27.md`; `L4_Parallel_Authoring_Execution_Plan_48_Parents_2026-08-27.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`; `../README.md`; `../../ARCHITECTURE_INDEX.md`  
**Referenced By:** First canonical L4 production wave  

---

## 1. Purpose

This decision closes the final broad-authoring authorization gate for the first canonical L4 production wave.

All Locked L4 broad-authoring preconditions were verified before activation:

- certified eligible-parent scope established;
- Architecture Index synchronized to current L3/L4 reality;
- L4 navigation synchronized;
- canonical `01_Production/` workspace established separately from `00_Pilot/`;
- exact author/reviewer/batch/concurrency controls approved;
- Locked governance/template/validation standards remain current;
- no post-approval material drift exists in the 48 certified L3 parents.

---

## 2. Authorization Scope

**Broad/parallel canonical L4 production authoring is authorized for exactly the 48 certified L3 parents** in:

`L4_Production_Parent_Baseline_48_2026-08-27.md`.

No parent outside that baseline is authorized by inference.

The default production expectation is:

```text
1 certified L3 parent
        ↓
1 canonical parent L4 specification
        ↓
N System Behavioral Obligations (BHs)
```

---

## 3. Activation Evidence

### Parent baseline

- Current certified production parents: **48**.
- Section D: 1.
- Section G: 8.
- Section H: 37.
- Section I: 2.
- Remaining canonical L3 scenarios outside this wave: 1,036.

### Parent integrity

The 45 newly approved G/H parents passed a separate L4 parent-readiness gate 45/45. The three pre-existing pilot-derived certified parents remain part of the certified baseline.

A final compare from the independent L3 approval commit `a6c50398e97b97a2daba2cc900cf06b9bb8e53a3` through the pre-activation index synchronization head `8bd47ba425e061b4d1d8cc89b15ceb7d19efc9f2` showed only L4 governance/navigation/production-workspace and master-index changes. No L3 scenario file changed after approval.

### Navigation and workspace

- `architecture/ARCHITECTURE_INDEX.md` now records current L3/L4 architecture instead of the obsolete no-L3/no-L4 placeholder state.
- L4 README identifies the 48-parent production wave, Locked governance, pilot separation and production workspace.
- Canonical production location is `01_Production/` with sections D/G/H/I materialized.
- Pilot artifacts remain under `00_Pilot/` as historical methodology evidence only.

### Execution controls

`L4_Parallel_Authoring_Execution_Plan_48_Parents_2026-08-27.md` establishes:

- 10 controlled batches;
- maximum batch size 5;
- maximum 4 concurrently active authoring batches;
- distinct primary author and reviewer per batch;
- Pallav, Varad, Chetan and Devang as author/reviewer pool;
- Dhruv / Lead Architect reserved for final Architecture Approval;
- continuous hard-fail validation and cross-batch consistency checks;
- no self-lock.

---

## 4. Authorized Actions

Effective immediately under this Locked decision, assigned authors may:

1. create canonical L4 System Behavior Specification files for the 48 certified parents under `01_Production/`;
2. use the Locked L4 template to derive selective, observable, implementation-independent BH obligations from the approved L3 parent;
3. perform author self-review and move a passing authored file `Draft → Review`;
4. submit each file to its assigned independent reviewer;
5. return material defects for correction or upstream resolution without contaminating unrelated parents;
6. perform cross-batch consistency review as defined in the execution plan.

After independent reviewer evidence exists, Dhruv / Lead Architect may independently decide `Review → Architecture Approved` for each passing L4 specification.

---

## 5. Mandatory Authoring Boundaries

This authorization does not permit L4 to:

- invent or redefine business ownership;
- invent or modify business rules;
- invent, rename or skip business lifecycle states/transitions;
- invent or rename canonical business events or publishers;
- invent business authority or approvers;
- silently repair a material L3 omission from L2/L1;
- convert every L3 flow step mechanically into a BH;
- prescribe APIs, payloads, databases, persistence, services/classes, queues/brokers, retries, transactions, rollback, infrastructure, technical IAM/RBAC, screens/forms/navigation, or other L5/L6 implementation detail.

Any such need triggers stop/escalation under the Locked L4 validation standard.

---

## 6. Pilot Boundary

The three existing pilot files for D-007, I-007 and I-118 are not automatically promoted.

Canonical production files for those parents must be authored under `01_Production/` from the current approved parent and Locked L4 template. Pilot content may be consulted as historical methodology evidence only.

---

## 7. Exclusions

This authorization does **not** authorize:

- L4 authoring for the remaining 1,036 L3 scenarios outside the certified production baseline;
- modification of L1/L2/L3 as part of ordinary L4 authoring;
- physical split of an L4 parent without controlled architecture-review exception;
- automatic batch-level Architecture Approval;
- any L4 specification to move to `Locked`;
- final L4 corpus Lock;
- L5/L6 authoring merely because an L4 Draft or Review file exists.

---

## 8. Lock Statement

**L4 BROAD/PARALLEL PRODUCTION AUTHORING:** **AUTHORIZED**  
**AUTHORIZED CERTIFIED PARENTS:** **48**  
**AUTHORIZED PRODUCTION BATCHES:** **10**  
**MAXIMUM CONCURRENT AUTHORING BATCHES:** **4**  
**CANONICAL PRODUCTION LOCATION:** `01_Production/`  
**L4 SPECIFICATION LOCK:** **NOT AUTHORIZED**  
**FINAL CORPUS LOCK:** **NOT AUTHORIZED**  
**STATUS:** **LOCKED — ACTIVE**
