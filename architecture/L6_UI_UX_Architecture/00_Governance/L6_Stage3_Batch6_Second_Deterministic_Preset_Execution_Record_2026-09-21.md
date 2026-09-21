# SutraERP L6 — Stage 3 Batch 6 Second Deterministic Preset & Controlled Preset Switching — Execution Record — 2026-09-21

**Version:** 1.0.0  
**Status:** Architecture Baseline  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Programme:** SutraERP Full Workflow Realization & Mock ERP Programme  
**Stage:** Stage 3 — Coherent Mock ERP Foundation  
**Batch:** Batch 6 — Second Deterministic Preset and Controlled Preset Switching  
**Depends On:** `L6_Stage3_Coherent_Mock_ERP_Foundation_Reviewed_Gameplan_2026-09-21.md`; Batch-1 through Batch-5 execution records; locked L0–L5 architecture; Stage-2 realization register.

---

## 1. Verified Starting Repository State

The live repositories were rechecked before Batch-6 implementation.

The prompt entered Batch 6 expecting Batch-5 PRs to remain unmerged. The live repositories were authoritative and showed both Batch-5 PRs already merged:

```text
Architecture PR #49: MERGED
Merge commit: d3fb8f8707389d59c5fcd9d0070883b1b7744f07

Frontend PR #14: MERGED
Merge commit: 8c8bbf2c44e90792b245d5c80b9e4fc73dbf1be9
```

Before reconciliation, both Stage-3 branches were exactly one merge commit behind their respective `main` branches, zero commits ahead, and non-divergent.

Drift classification:

```text
Architecture: SAFE
Frontend:     SAFE
```

Both Stage-3 branches were fast-forwarded to their corresponding merged Batch-5 `main` heads without force update or history rewrite.

Verified Batch-6 starting SHAs:

```text
Architecture main / Stage-3:
d3fb8f8707389d59c5fcd9d0070883b1b7744f07

Frontend main / Stage-3:
8c8bbf2c44e90792b245d5c80b9e4fc73dbf1be9
```

The previously validated Batch-5 UX-hardening heads remained present in the merged histories:

```text
Architecture UX-hardening evidence:
dd2a79e299f460eedd6f75ca64a32db168eef1d7

Frontend UX-hardening implementation:
1eb8d9e2414766fa09d80b4a24c4a0fbf147d284
```

No unrelated post-validation drift was identified.

---

## 2. Preserved Batch-1 Through Batch-5 Invariants

Batch 6 preserves:

- the deeply immutable Batch-1 canonical institutional source;
- canonical Person identity for Aarav and Meera;
- Batch-2 deterministic read-only selectors;
- one authoritative active demo state from Batch 3;
- reconstruction-based demo reset;
- the Batch-4 immutable operational source;
- the single deterministic mock time;
- Batch-5 Experience Lab integration and minimal subscription bridge;
- Batch-5 UX hardening, including collapsed operational details;
- explicit distinction between `Reset Demo Data` and `Reset Review Case`;
- reviewer/end-user separation;
- no persistence, backend, API or database;
- presentation-only maturity.

No L0–L5 business ownership, rule, lifecycle, event, authority or dependency semantics were changed.

---

## 3. Preset Model

Batch 6 introduces exactly two deterministic reviewer presets:

| Stable ID | Reviewer Label | Meaning |
|---|---|---|
| `baseline` | Baseline | Exact previously validated Stage-3 institutional + operational baseline |
| `exception-rich` | Exception-rich | Same canonical institutional world plus deterministic reviewer-local operational overlays |

Fresh application/module state defaults to:

```text
baseline
```

The construction model is:

```text
immutable canonical institutional source
+
immutable canonical operational source
+
selected deterministic preset definition
        ↓
fresh isolated DemoState
        ↓
one authoritative activeDemoState
```

The preset definitions are immutable. No mutable preset singleton or second store exists.

---

## 4. Baseline Preset Definition

The Baseline preset reconstructs the prior validated Stage-3 state exactly.

Institutional source:

```text
canonicalSeed
```

Operational source:

```text
canonicalOperationalSeed
```

Baseline operational counts remain:

```text
tasks:             3
notifications:     3
review attention:  2
activity:          5
```

Mock reference time remains:

```text
2026-09-16T10:30:00+05:30
```

Mechanical Batch-6 validation proves Baseline deep-equals the prior institutional and operational sources and uses fresh isolated objects.

---

## 5. Exception-Rich Preset Definition

The Exception-rich preset changes no canonical institutional business fact.

It derives from the same canonical institutional and operational sources and appends only deterministic `demo/reviewer-local` operational records.

Added reviewer-local records:

### Tasks

```text
demo-task-exception-rich-aarav-review
demo-task-exception-rich-meera-review
```

### Notification

```text
demo-notification-exception-rich-review-context
```

### Review-attention markers

```text
demo-exception-aarav-finance-review-attention
demo-exception-meera-course-review-attention
```

### Demonstration activity

```text
demo-activity-exception-rich-preset-ready
demo-activity-exception-rich-cross-domain-review-ready
```

Resulting counts:

```text
tasks:             5
notifications:     4
review attention:  4
activity:          7
```

All additions are classified:

```text
demo/reviewer-local
```

There are no Batch-6 `upstream-business-supported` mutations.

The alternate state does not claim payment failure, disciplinary action, malpractice, incident state, SLA breach, approval, exception resolution or any other new business outcome.

---

## 6. Preset Switching Design

State authority remains:

```text
selectedDemoPresetId
        ↓
createDemoState(selected preset)
        ↓
one activeDemoState
```

Preset switching executes:

```text
select preset
        ↓
discard current active demo state
        ↓
construct a fresh deterministic state
        ↓
replace activeDemoState
        ↓
emit one presentation revision
```

No field-by-field transform is used.

No second React store, state manager, workflow engine or event bus was introduced.

Mechanical proof confirms:

```text
Baseline → Exception-rich
= exact fresh Exception-rich state

Exception-rich → Baseline
= exact fresh Baseline state

Repeated switching
= deterministic and reversible
```

Four controlled switch operations emitted exactly four observer signals.

---

## 7. Selected-Preset Reset Semantics

Batch 6 explicitly changes the reviewer reset interpretation from “restore the sole baseline” to:

```text
Reset Demo Data
= reconstruct the currently selected deterministic preset
```

This preserves reviewer intent after selecting Exception-rich.

For both presets, validation performs:

```text
select preset
→ apply allowed presentation-local Student review focus
→ Reset Demo Data
→ selected preset remains selected
→ presentation-local mutation removed
→ entire selected preset reconstructed exactly
```

The mock clock is restored exactly in both cases.

`Reset Review Case` remains a separate pre-existing Experience Lab control and was not changed to select presets, reset demo data, or alter operational state.

---

## 8. Experience Lab Integration

The existing shared Stage-3 reviewer panel is extended rather than creating another administration surface.

The reviewer control is a compact native radio-group pattern:

```text
Review preset

Baseline
Exception-rich
```

Implementation properties:

- semantic `fieldset` and `legend`;
- two native radio inputs;
- visible selected state independent of color alone;
- visible `:focus-visible` treatment;
- keyboard-native radio behavior;
- accessible polite status feedback after switching/reset;
- narrow layout retains both choices without horizontal overflow;
- existing detailed operational sections remain collapsed by default.

Switch feedback uses non-production wording such as:

```text
Exception-rich review state loaded.
```

Reset feedback identifies the selected reviewer preset without implying database or workflow rollback.

---

## 9. Aarav / Meera Identity Coherence

Both presets preserve the same canonical references.

Aarav:

```text
person-aarav-desai
→ student-context-aarav-sit
→ Student projection
→ Attendance projection
→ Examination projection
→ Finance projection
```

All four projections continue to resolve:

```text
personId:         person-aarav-desai
studentContextId: student-context-aarav-sit
```

Meera:

```text
person-meera-shah
→ workforce-context-meera-sit
→ faculty-context-meera-sit
→ existing Academic Assignments
```

No alternate Aarav, Meera, Student Context, Workforce Context or Faculty Context is created for the second preset.

---

## 10. Mock-Time Proof

Both presets use the same existing deterministic clock:

```text
2026-09-16T10:30:00+05:30
```

Batch 6 introduces no:

- `Date.now()`;
- empty `new Date()`;
- live timer;
- countdown;
- `setInterval()`;
- scheduler;
- alternate clock;
- time-advance control.

Preset variation is state variation only.

---

## 11. Validation Results

The final Batch-6 validator reports:

```text
presetCountFailures:                 0
presetIdFailures:                    0
baselinePresetEqualityFailures:      0
exceptionRichDifferenceFailures:     0
presetDeterminismFailures:           0
presetObjectIsolationFailures:       0
presetReferenceIntegrityFailures:    0
presetSwitchFailures:                0
selectedPresetResetFailures:         0
presetUiBindingFailures:             0
presetUiRefreshFailures:             0
presetAccessibilityFailures:         0
runtimeTimeViolations:               0
persistenceBoundaryViolations:       0
unsupportedActionFailures:           0
architectureBoundaryViolations:      0
```

Proof summary:

```text
preset count:                  2
preset IDs:                    baseline, exception-rich
default preset:                baseline

Baseline counts:
tasks                          3
notifications                  3
review attention               2
activity                       5

Exception-rich counts:
tasks                          5
notifications                  4
review attention               4
activity                       7

mockNow:
2026-09-16T10:30:00+05:30

immutable sources unchanged:   true
```

Batch-1 through Batch-5 regression validators also remain green under the full `mock:validate` chain.

Review-model validation remains:

```text
99 unique workspace families
12 UX archetypes
81 shell-level implementations
18 interactive deep/reference implementations
presentation-only maturity preserved
```

---

## 12. Frontend Gates and CI

Required gates on the final validated frontend revision:

```text
npm run review:validate   PASS
npm run mock:validate     PASS
npm run typecheck         PASS
npm run build             PASS
rendered screenshot gate  PASS
```

Final validated frontend revision:

```text
7abebe804717e393c7b95e1e0e86c338f500691a
```

CI evidence:

```text
Workflow:                Frontend PR Validation
Run ID:                  35588906374
Run number:              88
Job ID:                  106298578156
Validated frontend SHA:  7abebe804717e393c7b95e1e0e86c338f500691a
Result:                  SUCCESS
Visual artifact:         experience-lab-visual-evidence
Visual artifact ID:      10634170030
```

The first Batch-6 CI run, #87, failed during the mock-validation chain because the pre-existing Batch-4 operational regression validator dynamically transpiled the state module but had not been updated to substitute the new preset-module import. The validator harness was corrected; production preset/state semantics were not weakened. Run #88 then passed every required gate.

---

## 13. Rendered Responsive Evidence

The existing CI process captured:

```text
Desktop viewport: 1440 × 1100
Narrow viewport:   390 × 844
```

Rendered inspection confirms:

- selected preset is immediately visible;
- the two choices remain compact;
- `Reset Demo Data` remains clear;
- `Reset Review Case` remains visibly separate;
- operational detail groups remain collapsed;
- no horizontal overflow is visible;
- no Batch-6 clipping or component overlap is visible;
- the ordinary ERP Student Workspace remains visible/reachable;
- narrow layout retains a usable two-choice preset control;
- no material return of the Batch-5 reviewer vertical-dominance problem was observed.

Screenshots are responsive/rendered evidence only. They are not treated as complete accessibility or assistive-technology certification.

---

## 14. Semantic Boundaries

Mechanical/source validation confirms Batch 6 adds no:

- task completion or assignment;
- notification delivery;
- exception resolution;
- business approval;
- event publication;
- workflow transition engine;
- third preset;
- custom/user-created preset;
- browser persistence;
- runtime clock;
- backend/API/database access.

No Stage-2 `PROHIBITED` workflow became actionable.

The alternate operational records remain inspectable and read-only.

---

## 15. Devil's-Advocate Review

Final Batch-6 diff and evidence were challenged against the stop conditions.

Results:

- exactly two presets: PASS;
- Baseline exact prior state: PASS;
- Exception-rich materially different: PASS;
- alternate state deterministic: PASS;
- no generalized scenario/workflow engine: PASS;
- Aarav identity preserved: PASS;
- Meera identity preserved: PASS;
- one active demo state authority: PASS;
- no duplicate preset stores: PASS;
- reconstructive switching: PASS;
- selected-preset-aware reset: PASS;
- Reset Review Case remains separate: PASS;
- fresh default Baseline: PASS;
- single deterministic clock: PASS;
- reviewer-local operational semantics preserved: PASS;
- no invented business failure/approval/resolution path: PASS;
- preset control reviewer-only: PASS;
- Batch-5 collapsible-detail hardening preserved: PASS;
- 390px rendered state usable: PASS;
- 99 workspace families preserved: PASS;
- 12 archetypes preserved: PASS;
- 81 shell + 18 interactive maturity preserved: PASS;
- no L0–L5 change: PASS;
- no persistence/backend/API/database: PASS;
- no Batch-7 fixture convergence: PASS;
- final CI on exact reported frontend SHA: PASS.

No material Batch-6 defect remains.

---

## 16. Architecture Findings

No L0–L5 remediation required.

Batch 6 remains reviewer/demo infrastructure at L6. It does not create a second business-architecture authority.

---

## 17. Deferred Work

Batch 7 retains ownership of:

```text
broader existing-fixture convergence
```

Batch 8 retains ownership of:

```text
final Stage-3 validation
final evidence reconciliation
Stage-3 PASS/FAIL
```

Neither was started in Batch 6.

---

## 18. Batch Verdict

```text
STAGE 3 — BATCH 6 PASS
```
