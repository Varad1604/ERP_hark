# SutraERP L6 — Stage 3 Batch 3 Demo State and Deterministic Reset Execution Record — 2026-09-21

**Version:** 1.0.0  
**Status:** Architecture Baseline  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Programme:** SutraERP Full Workflow Realization & Mock ERP Programme  
**Stage:** Stage 3 — Coherent Mock ERP Foundation  
**Batch:** Batch 3 — Demo State and Deterministic Reset  
**Depends On:** `L6_Stage3_Coherent_Mock_ERP_Foundation_Reviewed_Gameplan_2026-09-21.md`; `L6_Stage3_Batch1_Canonical_Seed_Execution_Record_2026-09-21.md`; `L6_Stage3_Batch2_Shared_Read_Projection_Layer_Execution_Record_2026-09-21.md`; locked L1 canonical Person amendment; Stage-2 realization register.

---

## 1. Verified Starting Baseline

The live repositories and prior merged PRs were rechecked before implementation.

```text
Architecture main:        e705b051e931eaa948c5dc75fcb29911bc2db62b
Architecture Stage-3:     254e24be75570c775f16c4ac368b433f060fac40
Architecture PR #46:      MERGED

Frontend main:            13b5599cf397cf4f5bf13f0fcb50cdc7444329c7
Frontend Stage-3:         a88c33f03cd6dfd2a9150c42cc5c657ffb35a320
Frontend PR #11:          MERGED
```

Both Stage-3 branches were behind `main` by exactly one merge commit, ahead by zero commits, with the merge base equal to the prior pre-merge Stage-3 head.

Drift classification:

```text
Architecture: SAFE
Frontend:     SAFE
```

Both Stage-3 branches were fast-forward synchronized to the corresponding merged `main` SHA before Batch-3 changes. No force push or history rewrite was used.

---

## 2. Batch-1 and Batch-2 Baselines Preserved

Batch 3 does not modify:

```text
app/mock/seed/canonical-seed.json
app/mock/seed/index.ts
app/mock/seed/types.ts
app/mock/selectors/index.ts
```

Canonical identity remains:

```text
Aarav Person:              person-aarav-desai
Aarav Student Context:     student-context-aarav-sit

Meera Person:              person-meera-shah
Meera Workforce Context:   workforce-context-meera-sit
Meera Faculty Context:     faculty-context-meera-sit

Canonical reference time:  2026-09-16T10:30:00+05:30
```

The Batch-2 selectors remain read-only and unchanged.

---

## 3. Frontend Batch-3 Revision

Validated frontend Stage-3 head:

```text
6590e6c96aa3678c7aff2c50f981939034488f36
```

Frontend PR:

```text
#12 — Stage 3 Batch 3 — demo state and deterministic reset
State: OPEN / NOT MERGED
```

Batch-3 frontend changes:

```text
app/mock/state/index.ts
scripts/validate-mock-state.mjs
package.json
```

No CI workflow file changed.

---

## 4. State-Layer Responsibilities

`app/mock/state/index.ts` establishes the smallest required Batch-3 state boundary.

Actual exports:

```text
createDemoState
getActiveDemoState
setDemoStudentReviewFocus
resetDemoState
```

Responsibilities:

- `createDemoState()` creates a fresh `structuredClone` of the immutable canonical seed;
- one module-local `activeDemoState` holds the frontend-only active demonstration graph;
- `getActiveDemoState()` returns a defensive clone so callers cannot mutate the live state graph directly;
- `setDemoStudentReviewFocus(...)` is the only Batch-3 write and validates the referenced Student Context before replacing active state;
- `resetDemoState()` discards the active graph and reconstructs it from a fresh canonical-seed copy.

No Redux, Zustand, MobX, React Context, reducer framework, repository/service layer, event bus, workflow engine or persistence framework was introduced.

---

## 5. Mutation Chosen and Authority Boundary

The existing seed contains no safe business-owned mutable field that can be changed in Batch 3 without introducing unsupported business semantics.

Therefore the controlled mutation proof is intentionally presentation/demo-local:

```text
Operation:
setDemoStudentReviewFocus("student-context-aarav-sit")
```

Before mutation:

```text
activeDemoState
deep-equal canonical seed
demoPresentation absent
```

After mutation:

```text
demoPresentation.reviewFocus.kind
= "student-context"

demoPresentation.reviewFocus.studentContextId
= "student-context-aarav-sit"
```

Canonical seed after mutation:

```text
unchanged
demoPresentation absent
```

This marker is not a Student Lifecycle fact, workflow state, approval, authority, eligibility decision or canonical business entity. It is explicitly frontend presentation/demo context that references the existing canonical Student Context by ID.

No business transition authority is therefore invented.

---

## 6. Reset Proof

Reset reconstructs state from the canonical seed rather than manually reversing mutation fields.

CI-executed proof:

```text
deep equality with canonical seed:     PASS
root object isolation:                 PASS
nested Person object isolation:        PASS
canonical seed unchanged:              PASS
repeated reset deterministic:          PASS
repeated mutate/reset deterministic:   PASS
```

After reset, the demo-local presentation marker is absent because the entire active graph is replaced by a fresh canonical-seed copy.

---

## 7. Aarav / Meera Coherence After Mutation and Reset

The actual unchanged Batch-2 selector implementation was executed against both the mutated active state and the reset state.

Aarav remained:

```text
person-aarav-desai
    ↓
student-context-aarav-sit
    ↓
Student projection
Attendance projection
Examination projection
Finance projection
```

All four projections continued to resolve the same canonical Person and Student Context.

Meera remained:

```text
person-meera-shah
    ↓
workforce-context-meera-sit
    ↓
faculty-context-meera-sit
    ↓
academic-assignment-meera-cs201
academic-assignment-meera-cs203
```

No duplicate identity representation or dangling academic reference was introduced.

---

## 8. Validation Results

The existing `mock:validate` gate now runs:

```text
node scripts/validate-mock-seed.mjs
node scripts/validate-mock-projections.mjs
node scripts/validate-mock-state.mjs
```

Batch-3 state validator results from CI:

```text
Batch-1 seed failures                  0
Batch-2 projection failures            0
initial-state equality failures        0
seed mutation failures                 0
active-state mutation failures         0
unintended mutation failures           0
dangling-reference failures            0
canonical Person coherence failures    0
Student Context coherence failures     0
reset equality failures                0
reset object-isolation failures        0
determinism failures                   0
selector-after-mutation failures       0
selector-after-reset failures          0
persistence-boundary violations        0
```

The validator executes the actual Batch-3 state module and actual Batch-2 selector module after TypeScript transpilation. It does not duplicate the mutation/reset implementation.

Static Batch-3 boundary checks reject:

```text
localStorage
sessionStorage
IndexedDB
Date.now
new Date()
Math.random
crypto.randomUUID
fetch
backend URLs
reducer/workflow-engine/event-bus/dispatch patterns
Redux/Zustand/MobX/CQRS/generalized service architecture
```

---

## 9. Frontend Gates and CI Evidence

GitHub Actions:

```text
Workflow:       Frontend PR Validation
Run ID:         35569657599
Run #:          74
Job ID:         106238413397
Result:         SUCCESS
Validated SHA:  6590e6c96aa3678c7aff2c50f981939034488f36
```

Executed gates:

```text
npm run review:validate   PASS
npm run mock:validate     PASS
npm run typecheck         PASS
npm run build             PASS
```

Preserved review baseline:

```text
Workspace families                   99
UX archetypes                        12
Shell-level implementations          81
Interactive implementations          18
Integration maturity                 presentation-only
Integrated maturity                  0
Production-validated maturity        0
```

A mutable frontend demo-state layer does not upgrade integration maturity.

---

## 10. Architecture Findings

No genuine upstream L0–L5 defect was discovered.

The Batch-3 implementation deliberately avoided changing any business-owned seeded field because no safe business mutation was required to prove the state/reset invariant.

Canonical ownership remains unchanged:

- Person remains Platform Foundation-owned canonical identity;
- Student Context remains Student Lifecycle-owned;
- Academic Delivery, Examination and Finance facts remain source-owned;
- the new `demoPresentation` marker is explicitly frontend/demo-local and carries no business authority.

```text
No L0–L5 remediation required.
```

---

## 11. Devil's-Advocate Review

The final Batch-3 frontend diff was reviewed against the Batch-3 hard boundaries.

Confirmed:

- canonical seed remains deeply immutable;
- active state is constructed from a fresh copy;
- active-state callers receive defensive snapshots;
- no active graph shares mutable seed references;
- the chosen mutation is presentation/demo-local and does not invent business semantics;
- invalid Student Context IDs fail deterministically;
- selectors remain read-only;
- the same selector API reads seed-shaped active state;
- reset reconstructs from seed instead of manually undoing fields;
- repeated reset is deterministic;
- Aarav remains one Person and one Student Context;
- Meera remains one Person with the same Workforce/Faculty/Assignment chain;
- no PROHIBITED workflow gains an action;
- no persistence is introduced;
- no backend, database or fake API/service architecture is introduced;
- no generic workflow engine or event bus is introduced;
- no Experience Lab control is added;
- no generalized mock clock is added;
- no fixture convergence is performed;
- workspace/archetype/maturity baselines remain unchanged.

No material Batch-3 defect remains.

---

## 12. Deferred Work

### Batch 4

Deferred:

- mock-clock generalization;
- seeded tasks;
- notifications;
- exceptions;
- history/activity.

### Batch 5

Deferred:

- Experience Lab integration;
- reviewer-facing reset controls;
- demo context visibility.

### Batch 6

Deferred:

- second deterministic preset.

### Batch 7

Deferred:

- broader fixture convergence.

None of those batches were started.

---

## 13. Batch Verdict

```text
STAGE 3 — BATCH 3 PASS
```

Batch 3 is complete on controlled Stage-3 branches and is not merged to `main`.

Stop gate applies. Batch 4 has not started.
