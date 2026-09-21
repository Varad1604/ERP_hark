# SutraERP L6 — Stage 3 Batch 2 Shared Read / Projection Layer Execution Record — 2026-09-21

**Version:** 1.0.0  
**Status:** Architecture Baseline  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Programme:** SutraERP Full Workflow Realization & Mock ERP Programme  
**Stage:** Stage 3 — Coherent Mock ERP Foundation  
**Batch:** Batch 2 — Shared Read / Projection Layer  
**Depends On:** `L6_Stage3_Coherent_Mock_ERP_Foundation_Reviewed_Gameplan_2026-09-21.md`; `L6_Stage3_Batch1_Canonical_Seed_Execution_Record_2026-09-21.md`; locked L1 canonical Person amendment; Stage-2 realization register.

---

## 1. Verified Starting Baseline

The live repositories were rechecked before Batch-2 implementation.

```text
Architecture main:        af2fdf5c79a0f60efb77c9bb1a11c48fc20c52b5
Architecture Stage-3:     1af564896c8bb944907ce14255483666c93957aa
Frontend main:            e2560877eac4446b925e45f04b7dfa4c24d3625c
Frontend Stage-3 start:   bfc1977514cca5736376619a2222d841c15fc589
Frontend PR #11:          OPEN / NOT MERGED
Drift classification:     SAFE
```

Both Stage-3 branches were ahead of `main` and behind by zero commits. No reconciliation was required.

---

## 2. Batch-1 Baseline Preserved

Batch 2 did not modify the canonical Batch-1 seed.

The following remain unchanged:

```text
mockNow                         2026-09-16T10:30:00+05:30
Aarav Person                    person-aarav-desai
Aarav Student Context           student-context-aarav-sit
Meera Person                    person-meera-shah
Meera Workforce Context         workforce-context-meera-sit
Meera Faculty Context           faculty-context-meera-sit
```

Batch 2 consumes these records through deterministic references only.

---

## 3. Frontend Implementation

Validated frontend Stage-3 revision:

```text
a88c33f03cd6dfd2a9150c42cc5c657ffb35a320
```

Batch-2 frontend changes:

```text
app/mock/selectors/index.ts
scripts/validate-mock-projections.mjs
package.json
```

No seed file changed.

### Selector responsibilities

`app/mock/selectors/index.ts` provides:

- `getPerson` — required canonical Person lookup;
- `getStudentContext` — required Student Context lookup;
- `getWorkforceContext` — required Workforce Context lookup;
- `getFacultyContext` — required Faculty Context lookup;
- `getStudentProjection` — Student Context → Person → Programme Version → Programme → Academic Period → Institution;
- `getAttendanceProjection` — Attendance → Student Context → Person → Course Offering → Course → Academic Period → Timetable;
- `getExaminationProjection` — Examination → Student Context → Person → Course → Academic Period;
- `getFinanceProjection` — Finance → Student Context → Person → Institution → Academic Period;
- `getFacultyAssignmentProjection` — Faculty → Workforce → Person plus Academic Assignment → Course Offering → Course.

All selectors are read-only. Required references fail explicitly. No fallback identities, fuzzy matching, runtime randomness, state mutation, reset behavior, reducer, persistence, service abstraction or workflow engine was added.

Projection objects copy presentation values rather than returning live mutable nested seed objects.

---

## 4. Aarav Cross-Domain Proof

CI-executed Batch-2 validation resolved:

```text
Person:
person-aarav-desai

Student Context:
student-context-aarav-sit

Student projection:
student-context-aarav-sit

Attendance projection:
student-context-aarav-sit

Examination projection:
student-context-aarav-sit

Finance projection:
student-context-aarav-sit
```

Resolved academic context:

```text
Institution:
institution-sit
Sutra Institute of Technology

Programme:
programme-btech-cse
B.Tech · Computer Science

Academic Period:
academic-period-2026-27-sem1
2026–27 · Semester 1
```

Attendance resolves:

```text
Course Offering:
offering-cs201-a-2026s1

Course:
course-cs201
CS201 — Data Structures

Timetable:
timetable-cs201-a-wed-1030
```

Examination resolves:

```text
Course:
course-cs201

Academic Period:
academic-period-2026-27-sem1
```

Finance resolves:

```text
Institution:
institution-sit

Academic Period:
academic-period-2026-27-sem1
```

No projection establishes an independent Person or Student Context identity.

---

## 5. Meera Faculty / Workforce Proof

CI-executed validation resolved:

```text
Person:
person-meera-shah
Dr. Meera Shah

→ Workforce Context:
workforce-context-meera-sit

→ Faculty Context:
faculty-context-meera-sit

→ Academic Assignment:
academic-assignment-meera-cs201
→ Course Offering:
offering-cs201-a-2026s1
→ Course:
course-cs201 — Data Structures

and

→ Academic Assignment:
academic-assignment-meera-cs203
→ Course Offering:
offering-cs203-b-2026s1
→ Course:
course-cs203 — Database Systems
```

---

## 6. Validation

The existing `mock:validate` command remains the single cohesive mock-integrity gate.

It now runs:

```text
node scripts/validate-mock-seed.mjs
node scripts/validate-mock-projections.mjs
```

The projection validator transpiles and executes the actual TypeScript selector module; it does not maintain a duplicate selector implementation.

Validated counters:

```text
Batch-1 seed failures                         0
selector lookup failures                      0
dangling projection references                0
canonical Person projection violations        0
Student Context projection mismatches          0
academic projection mismatches                 0
Faculty/workforce projection failures          0
determinism failures                           0
read-only mutation failures                    0
independent identity copy violations           0
```

The validator also verifies that mutating a returned projection does not mutate the canonical seed.

---

## 7. Frontend Gates and CI Evidence

GitHub Actions:

```text
Workflow: Frontend PR Validation
Run ID:   35568039790
Run #:    73
Job ID:   106233677094
Result:   SUCCESS
Frontend head validated:
a88c33f03cd6dfd2a9150c42cc5c657ffb35a320
```

Required gates:

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

Batch 2 does not upgrade workflow, integration or production maturity.

---

## 8. CI Strategy

No Batch-2 workflow file change was required.

The existing `.github/workflows/frontend-pr-validation.yml` already executes `npm run mock:validate`, so extending that command in `package.json` automatically incorporated the projection validator without creating a redundant CI path.

PR #11 remains the cumulative Stage-3 evidence vehicle and was retitled to:

```text
Stage 3 Batches 1–2 — canonical seed and shared read projections
```

It remains open and unmerged.

---

## 9. Architecture Findings

No upstream L0–L5 defect was discovered.

The read/projection design preserves existing canonical ownership:

- Person remains canonical and is only resolved, never recreated;
- Student Context remains Student Lifecycle-owned source context;
- Academic facts remain source-owned;
- Attendance remains Academic Delivery context;
- Examination remains Examination context;
- Finance remains Finance context;
- projections are presentation/read models only.

**No L0–L5 remediation required.**

---

## 10. Migration Debt / Deferred Work

Existing frontend fixture islands intentionally remain in place.

Batch 2 did not bulk-migrate:

- Student fixtures;
- Attendance fixtures;
- Examination fixtures;
- Finance fixtures;
- `app/data/demo.ts`;
- Experience Lab fixtures;
- the 18 existing interactive implementations.

Broader fixture convergence remains Batch 7.

### Batch 3

Deferred:

- active mutable demo state;
- controlled frontend-local mutation;
- deterministic reset.

### Batch 4+

Deferred:

- generalized mock clock;
- tasks;
- notifications;
- exceptions;
- history/activity;
- Experience Lab controls;
- alternate deterministic presets.

### Batch 7

Deferred:

- broader fixture convergence onto the shared canonical seed and projections.

---

## 11. Devil's-Advocate Review

The final Batch-2 diff was reviewed for architectural and engineering boundary violations.

Confirmed absent:

- duplicate canonical Person records;
- duplicate Student Context identities;
- giant cross-domain ownership object;
- business-rule derivation;
- lifecycle invention;
- authority or permission invention;
- generic repository/service framework;
- fake API layer;
- global state;
- reducers;
- reset behavior;
- persistence;
- workflow mutation;
- bulk fixture migration;
- runtime clock dependence.

One validation weakness was found during self-review: the first projection validator duplicated the projection logic instead of executing the actual selector exports. That weakness was corrected before the final validated revision. The final validator transpiles and invokes the real selector module.

---

## 12. Batch Verdict

```text
STAGE 3 — BATCH 2 PASS
```

Batch 3 has not started.

No merge to `main` was performed.
