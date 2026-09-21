# SutraERP L6 — Stage 3 Batch 4 Deterministic Mock Clock & Operational Context — Execution Record — 2026-09-21

**Version:** 1.0.0  
**Status:** Architecture Baseline  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Programme:** SutraERP Full Workflow Realization & Mock ERP Programme  
**Stage:** Stage 3 — Coherent Mock ERP Foundation  
**Batch:** Batch 4 — Deterministic Mock Clock and Seeded Operational Context  
**Depends On:** `L6_Stage3_Coherent_Mock_ERP_Foundation_Reviewed_Gameplan_2026-09-21.md`; Batch-1, Batch-2 and Batch-3 execution records; locked L1 canonical Person scope; Stage-2 realization register.  

---

## 1. Verified Starting Repository State

The live repositories were rechecked before implementation.

```text
Architecture main:        10a4ad1aaaaea8cebcd0fb826482263fb21a3de7
Architecture Stage-3:     9ecbbf8840294845018e5afbe55b27cc7e5c9dcf
Frontend main:            ccc36ae88e73d0666c3d1b3ce74be35a1db9e3b0
Frontend Stage-3:         6590e6c96aa3678c7aff2c50f981939034488f36
```

Batch-3 merge verification:

```text
Architecture PR #47: MERGED
Frontend PR #12:     MERGED
```

Both Stage-3 branches were exactly one merge commit behind their respective `main` branches and had zero ahead commits / zero divergent file changes.

Drift classification:

```text
Architecture: SAFE
Frontend:     SAFE
```

Both Stage-3 branches were fast-forwarded to their `main` heads with no force update and no history rewrite.

Verified synchronized Batch-4 starting SHAs:

```text
Architecture main / Stage-3:
10a4ad1aaaaea8cebcd0fb826482263fb21a3de7

Frontend main / Stage-3:
ccc36ae88e73d0666c3d1b3ce74be35a1db9e3b0
```

No work occurred directly on either `main`.

---

## 2. Preserved Batch-1 / Batch-2 / Batch-3 Baseline

Batch 4 preserves:

- immutable Batch-1 canonical institutional seed;
- canonical Tenant-scoped Person identity;
- `person-aarav-desai`;
- `student-context-aarav-sit`;
- `person-meera-shah`;
- `workforce-context-meera-sit`;
- `faculty-context-meera-sit`;
- Batch-2 read-only selectors;
- Batch-3 controlled active state and defensive snapshots;
- `setDemoStudentReviewFocus` as presentation/demo-local mutation only;
- reconstruction-based deterministic reset;
- zero browser/server persistence.

The Batch-1 `canonicalSeed` contract was not enlarged or redefined.

Batch 4 uses the safer layered model:

```text
immutable canonical institutional seed
+
immutable deterministic operational seed
        ↓
fresh active demo state
```

This preserves Batch-1 provenance and avoids silently changing the original seed schema.

---

## 3. Deterministic Mock Clock

Implementation:

```text
app/mock/time/index.ts
getMockNow()
```

The clock reads the existing Batch-1 `mockNow` export and introduces no second time source.

Canonical value:

```text
2026-09-16T10:30:00+05:30
```

Properties:

- fixed deterministic ISO timestamp;
- repeated reads return the same value;
- no system clock read;
- no clock advancement;
- no timer;
- no scheduler;
- no timezone engine;
- no duplicated hard-coded `mockNow` value inside the clock module.

---

## 4. Operational Seed Structure

Implementation:

```text
app/mock/operational/types.ts
app/mock/operational/index.ts
```

Immutable source:

```text
canonicalOperationalSeed
```

Fresh source constructor:

```text
createOperationalSeed()
```

All Batch-4 operational records are explicitly classified:

```text
demo/reviewer-local
```

No Batch-4 record claims to be an institutional obligation, authoritative workflow assignment, official communication, canonical business exception, or canonical business event.

Counts:

```text
tasks:          3
notifications:  3
exceptions:     2
activity:       5
```

### Tasks

| ID | Semantics | Timestamp |
|---|---|---|
| `demo-task-review-aarav-student-context` | demo/reviewer-local | created 09:40; due 14:00 |
| `demo-task-review-aarav-cross-domain-context` | demo/reviewer-local | created 09:50; due 15:00 |
| `demo-task-review-meera-assignment-context` | demo/reviewer-local | created 10:05; due 16:00 |

The due times are demo-review deadlines only. They are not institutional SLAs or business deadlines.

### Notifications

| ID | Semantics | Timestamp |
|---|---|---|
| `demo-notification-aarav-student-context-available` | demo/reviewer-local | 09:55 |
| `demo-notification-aarav-cross-domain-context-available` | demo/reviewer-local | 10:00 |
| `demo-notification-meera-assignment-context-available` | demo/reviewer-local | 10:10 |

These are deterministic informational records only. They do not imply email, SMS, push, delivery, read receipt, or official communication.

### Exceptions

| ID | Semantics | Timestamp |
|---|---|---|
| `demo-exception-aarav-cross-domain-review-attention` | demo/reviewer-local | 10:15 |
| `demo-exception-meera-assignment-review-attention` | demo/reviewer-local | 10:20 |

These are review-attention markers, not canonical failure states or exception lifecycles.

### Activity / History

| ID | Semantics | Timestamp |
|---|---|---|
| `demo-activity-institutional-fixture-available` | demo/reviewer-local | 09:30 |
| `demo-activity-aarav-student-context-observed` | demo/reviewer-local | 09:35 |
| `demo-activity-meera-workforce-chain-observed` | demo/reviewer-local | 09:45 |
| `demo-activity-aarav-cross-domain-context-observed` | demo/reviewer-local | 10:00 |
| `demo-activity-operational-baseline-ready` | demo/reviewer-local | 10:25 |

These are neutral read-model activity descriptions. No canonical event name or publisher was invented.

---

## 5. Temporal-Coherence Proof

Mock now:

```text
2026-09-16T10:30:00+05:30
```

Activity chronology:

```text
09:30
09:35
09:45
10:00
10:25
≤
10:30 mock now
```

Notification chronology:

```text
09:55
10:00
10:10
≤
10:30 mock now
```

Exception review markers:

```text
10:15
10:20
≤
10:30 mock now
```

Task creation times:

```text
09:40
09:50
10:05
≤
10:30 mock now
```

Task due times:

```text
14:00
15:00
16:00
```

All due times follow their creation times and are presentation/demo-local. No business SLA meaning is asserted.

---

## 6. Active State and Reset Integration

`DemoState` now contains:

```text
Batch-1 institutional graph
+
Batch-4 operational graph
+
optional Batch-3 presentation-local review focus
```

`createDemoState()` creates fresh mutable copies from both immutable sources.

`getActiveDemoState()` remains defensive.

`resetDemoState()` reconstructs the complete deterministic baseline rather than reversing records individually.

Validated properties:

```text
institutional state restored:        PASS
operational state restored:          PASS
mock clock restored:                 PASS
deep equality:                       PASS
object isolation:                    PASS
canonical institutional seed intact: PASS
canonical operational seed intact:   PASS
repeated reset deterministic:        PASS
```

A mutation applied to a returned defensive snapshot did not leak into active state or immutable operational sources.

---

## 7. Aarav / Meera Identity and Reference Proof

### Aarav

```text
person-aarav-desai
    ↓
student-context-aarav-sit
    ↓
Student projection
Attendance:  attendance-aarav-cs201-20260916
Examination: examination-aarav-cs201-2026s1
Finance:     finance-aarav-2026s1
    ↓
Batch-4 review tasks / informational notifications /
review-attention marker / neutral activity
```

Every Aarav Batch-4 relation is ID-based.

No name is used as a relational key.

### Meera

```text
person-meera-shah
    ↓
workforce-context-meera-sit
    ↓
faculty-context-meera-sit
    ↓
academic-assignment-meera-cs201
    ↓
offering-cs201-a-2026s1
    ↓
Batch-4 review task / informational notification /
review-attention marker / neutral activity
```

The same canonical Person, Workforce and Faculty chain is preserved.

---

## 8. Validation Implementation

Existing command retained and extended:

```text
npm run mock:validate
```

No parallel validation ecosystem was introduced.

Batch-4 validator:

```text
scripts/validate-mock-operational.mjs
```

It executes the actual mock clock, operational seed, active-state/reset implementation and Batch-2 selectors.

Required Batch-4 counters:

```text
batch1SeedFailures                        0
batch2ProjectionFailures                  0
batch3StateResetFailures                  0
mockClockSourceFailures                   0
mockClockDeterminismFailures              0
operationalMissingIdFailures              0
operationalDuplicateIdFailures            0
operationalDanglingReferenceFailures      0
taskCoherenceFailures                     0
notificationCoherenceFailures             0
exceptionCoherenceFailures                0
historyActivityCoherenceFailures          0
temporalOrderingFailures                  0
canonicalPersonCoherenceFailures          0
studentContextCoherenceFailures           0
workforceFacultyCoherenceFailures         0
resetOperationalEqualityFailures          0
resetOperationalIsolationFailures         0
selectorAfterOperationalSeedFailures      0
selectorAfterResetFailures                0
runtimeTimeViolations                     0
persistenceBoundaryViolations             0
architectureBoundaryViolations            0
```

Additional Batch-1, Batch-2 and Batch-3 validators also passed with zero material failures.

---

## 9. Frontend Validation and CI Evidence

Frontend Batch-4 branch head validated by CI:

```text
133bbfd73d0b6b6d92cc667d86a9a01f03a78dd4
```

Frontend PR:

```text
#13 — Stage 3 Batch 4 — deterministic mock clock and operational context
State: OPEN
Merged: NO
```

Existing workflow:

```text
Frontend PR Validation
```

CI evidence:

```text
Workflow run ID: 35571638963
Run number:      75
Job:             validate
Job ID:          106244322142
Result:          SUCCESS
```

Required gates:

```text
npm run review:validate   PASS
npm run mock:validate     PASS
npm run typecheck         PASS
npm run build             PASS
```

Review baseline preserved:

```text
99 workspace families
12 UX archetypes
81 shell-level implementations
18 interactive implementations
99 presentation-only integration maturity
0 integrated maturity
0 production-validated maturity
```

No maturity upgrade occurred.

---

## 10. Architecture Boundary Review

No L0–L5 remediation was required.

Batch 4 does not create or alter:

- Person ownership;
- Student Context ownership;
- Workforce/Faculty ownership;
- workflow state;
- approval authority;
- task completion authority;
- business deadline/SLA;
- notification obligation;
- canonical exception lifecycle;
- canonical business event;
- canonical event publisher;
- Stage-2 realization classification.

Stage-2 `PROHIBITED` workflows remain non-actionable.

---

## 11. Devil's-Advocate Review

Final diff was independently challenged.

### Clock

- exactly one fixed mock-time source: PASS;
- source derives from Batch-1 `mockNow`: PASS;
- real runtime clock in Batch-4 layer: NONE;
- timers/scheduling: NONE.

### Operational records

- tasks are explicitly demo/reviewer-local: PASS;
- notifications are records only: PASS;
- exceptions are review-attention markers: PASS;
- history is neutral read-model activity: PASS;
- business state invention: NONE.

### Identity

- Aarav canonical Person preserved: PASS;
- Aarav single Student Context preserved: PASS;
- Meera canonical Person → Workforce → Faculty chain preserved: PASS;
- name-based relational keys: NONE.

### Architecture

- task authority invented: NO;
- approvals invented: NO;
- business deadlines invented: NO;
- notification obligations invented: NO;
- canonical event names/publishers invented: NO;
- exception lifecycle invented: NO;
- PROHIBITED workflow actionability introduced: NO.

### State / reset

- full operational baseline restored: PASS;
- immutable sources unchanged: PASS;
- active-state unsafe sharing: NONE;
- repeated resets deterministic: PASS.

### Engineering

- scheduler: NONE;
- timers: NONE;
- persistence: NONE;
- fake service layer: NONE;
- workflow engine: NONE;
- fake backend/API/database: NONE;
- validator executes real implementation: PASS.

### Repository integrity

- review validation: PASS;
- 99 workspace families unchanged;
- 12 archetypes unchanged;
- 81 / 18 maturity unchanged;
- integration maturity unchanged;
- no architecture IDs added to ordinary UI;
- no UI implementation added in Batch 4.

No material Batch-4 defect remains.

---

## 12. Deferred Work

### Batch 5

Deferred:

- Experience Lab integration;
- reviewer-facing reset controls;
- demo context visibility;
- operational-context inspection UI.

### Batch 6

Deferred:

- second deterministic preset.

### Batch 7

Deferred:

- broader fixture convergence.

None of these batches was started.

---

## 13. Batch Verdict

```text
STAGE 3 — BATCH 4 PASS
```

Batch 4 is complete on controlled Stage-3 branches.

Stop gate applies.

Batch 5 has not started.

Neither Batch-4 PR is merged to `main`.
