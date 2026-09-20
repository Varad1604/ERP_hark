# SutraERP L6 Stage 2 — Group 11 Examination I1 Reconciliation

**Status:** COMPLETE-FOR-MASTER-CROSS-GROUP-RECONCILIATION  
**Scope:** Section I — Examination I1 (`L3-I-001 … L3-I-082`)  
**Scenario population:** 82  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Branch:** `l6-stage2-group11-i1-reconciliation`  
**Date:** 2026-09-20

> `G11-WF-###` identifiers are temporary Stage-2 normalization keys only. They are not business-architecture IDs and must not appear in ordinary product UX.

## 1. Reconciliation Result

```text
Assigned canonical scenarios       82
Reconciled scenario rows           82 / 82
Unique scenario IDs                82
Missing scenarios                  0
Duplicate scenario assignments     0
Cross-workspace workflow merges    0
Material open issues               0
Distinct normalized workflows      41
User-addressable workflows         41
Tracking / non-mock workflows       0
Upstream-blocked workflows          0
```

Group 11 is fully reconciled.

## 2. Canonical Population and Workspace Boundaries

The canonical range `L3-I-001 … L3-I-082` contains exactly 82 scenarios and maps to eight Examination workspaces:

- I01 `I-001…003` — Examination Policy & Configuration — `W-I01`
- I02 `I-004…006` — Examination Sessions — `W-I02`
- I03 `I-007…016` — Examination Planning & Scheduling — `W-I03`
- I04 `I-017…036` — Examination Registration — `W-I04`
- I05 `I-037…042` — Hall Tickets — `W-I05`
- I06 `I-043…054` — Practical & Viva Conduct — `W-I06`
- I07 `I-055…070` — Theory Examination Conduct — `W-I07`
- I08 `I-071…082` — Marks Collection & Evaluation — `W-I08`

No normalized workflow crosses these workspace boundaries.

## 3. Semantic Reconciliation Findings

The I1 worker is mechanically complete but materially under-classifies human examination activity as background or non-mock.

Evidence-backed corrections include:

- I001/I002 examination policy creation/revision require Examination policy actors, Controller/Academic governance and formal approval.
- I004/I005 Examination Session creation/revision require Examination Administrator and Controller actions.
- I006 duplicate-session reconciliation requires authorized examination review and resolution.
- I008–I015 schedule revision, venue allocation, invigilation, shortage resolution, examiner allocation and seating all have explicit Examination Administration/Controller/departmental responsibilities.
- I022/I032 registration error and duplicate resolution require Examination Administrator review and applicable approval.
- I037 Hall Ticket issuance requires generation/review/publication authority; I039/I041 invalidation and conflict resolution likewise require governed examination action.
- I045–I054 practical/viva conduct exceptions, identity decisions, examiner/resource changes, interruptions and corrections expose Candidate, Examiner, Chief Examiner and Examination Administrator responsibilities.
- I055–I070 theory conduct and exception scenarios expose centre heads, invigilators, observers, Controller roles, medical participation where applicable, answer-script custody and governed correction.
- I071–I082 marks collection/evaluation scenarios require authorized evaluators, Chief Evaluator supervision, moderation, second evaluation and adjudicating evaluation where policy requires it.

No worker row in Group 11 remains legitimately system-only after current L2/L3/L4 validation.

## 4. Ownership and Boundary Observations

- Examination retains canonical ownership of examination policy, session, scheduling, registration, hall-ticket, conduct and evaluation facts.
- Academic/Student/People/Infrastructure/Health facts are consumed only as authoritative inputs where the Examination workflow requires them.
- G06 medical evidence in conduct-interruption scenarios remains G06-owned; Examination owns the resulting examination-conduct decision.
- Infrastructure/venue facts remain source-owned while Examination owns venue allocation/seating decisions within I03.
- Faculty/personnel facts remain source-owned while Examination owns invigilation/examiner allocation decisions.
- No cross-domain source fact is duplicated or mutated by the Examination workflow.

## 5. Scenario-Level Reconciled Distribution

| Disposition | Rows |
|---|---:|
| INTERACTIVE-WORKFLOW | 23 |
| REVIEW-APPROVAL-WORKFLOW | 29 |
| EXCEPTION-RECOVERY-WORKFLOW | 30 |
| **Total** | **82** |

| Mock interaction | Rows |
|---|---:|
| REQUIRED | 82 |
| NOT-REQUIRED | 0 |
| PROHIBITED | 0 |
| **Total** | **82** |

## 6. Normalized Workflow Population

| Key | Workspace | Workflow family | Scenario IDs | Realization | Mock |
|---|---|---|---|---|---|
| G11-WF-001 | W-I01 | Examination Policy definition, revision, activation and retirement | I001,I002,I003 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G11-WF-002 | W-I02 | Examination Session creation and governed revision | I004,I005 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G11-WF-003 | W-I02 | Duplicate Examination Session resolution | I006 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-004 | W-I03 | Examination Schedule planning, revision, publication and withdrawal | I007,I008,I013,I016 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G11-WF-005 | W-I03 | Examination venue allocation and seating arrangement | I009,I015 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G11-WF-006 | W-I03 | Invigilation assignment and shortage resolution | I010,I011 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G11-WF-007 | W-I03 | Practical/Viva examiner allocation | I012 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G11-WF-008 | W-I03 | Examination scheduling conflict resolution | I014 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-009 | W-I04 | Candidate, assisted, late and administrative registration | I017,I018,I019,I034 | INTERACTIVE-WORKFLOW | REQUIRED |
| G11-WF-010 | W-I04 | Registration deadline override and reopening | I020,I025 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G11-WF-011 | W-I04 | Registration amendment, error, duplicate, scope conflict and transfer resolution | I021,I022,I027,I032,I033,I035 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-012 | W-I04 | Registration withdrawal, reinstatement and opportunity-change withdrawal | I023,I024,I031 | INTERACTIVE-WORKFLOW | REQUIRED |
| G11-WF-013 | W-I04 | Registration rejection/ineligibility decision | I026 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-014 | W-I04 | Special-permission and special-arrangement registration | I028,I029 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G11-WF-015 | W-I04 | Supplementary/backlog/improvement registration | I030 | INTERACTIVE-WORKFLOW | REQUIRED |
| G11-WF-016 | W-I04 | Registration population finalization | I036 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G11-WF-017 | W-I05 | Hall Ticket issuance and population finalization | I037,I042 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G11-WF-018 | W-I05 | Hall Ticket withheld-entry decision | I038 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-019 | W-I05 | Hall Ticket invalidation following registration withdrawal | I039 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G11-WF-020 | W-I05 | Special-arrangement Hall Ticket governance | I040 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G11-WF-021 | W-I05 | Duplicate/conflicting Hall Ticket resolution | I041 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-022 | W-I06 | Practical/Viva conduct, special arrangement, re-conduct and panel viva | I043,I044,I047,I050 | INTERACTIVE-WORKFLOW | REQUIRED |
| G11-WF-023 | W-I06 | Non-theory entry/non-participation and identity decision | I045,I046 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-024 | W-I06 | Practical resource and batch-allocation conflict resolution | I048,I049 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-025 | W-I06 | Mid-conduct examiner replacement | I051 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-026 | W-I06 | Medical interruption and candidate withdrawal during non-theory conduct | I052,I053 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-027 | W-I06 | Non-theory conduct record correction | I054 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-028 | W-I07 | Theory conduct, multi-centre conduct, re-conduct and special arrangement | I055,I056,I061,I062 | INTERACTIVE-WORKFLOW | REQUIRED |
| G11-WF-029 | W-I07 | Theory entry/non-participation and identity decision | I057,I058 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-030 | W-I07 | Theory seating-conflict resolution | I059 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-031 | W-I07 | Mid-conduct invigilator replacement | I060 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-032 | W-I07 | Question-paper unavailability, compromise and conduct-issue disposition | I063,I064,I065 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-033 | W-I07 | Answer-script collection/custody discrepancy resolution | I066,I067 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-034 | W-I07 | Medical interruption and candidate withdrawal during theory conduct | I068,I069 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-035 | W-I07 | Theory conduct record correction | I070 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-036 | W-I08 | Marks collection, evaluation and final approval | I071,I072,I073 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G11-WF-037 | W-I08 | Marks submission exception resolution | I074 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-038 | W-I08 | Marks moderation under approved policy | I075 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G11-WF-039 | W-I08 | Incomplete evaluation and withheld approval | I076 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G11-WF-040 | W-I08 | Supplementary/improvement, re-conduct and special-arrangement evaluation | I077,I078,I079 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G11-WF-041 | W-I08 | Second evaluation, evaluation-difference and adjudicating evaluation | I080,I081,I082 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |

## 7. Workflow-Level Counts

| Realization | Workflows |
|---|---:|
| INTERACTIVE-WORKFLOW | 5 |
| REVIEW-APPROVAL-WORKFLOW | 16 |
| EXCEPTION-RECOVERY-WORKFLOW | 20 |
| **Total** | **41** |

| Workflow mock realization | Workflows |
|---|---:|
| REQUIRED | 41 |
| NOT-REQUIRED | 0 |
| PROHIBITED | 0 |
| **Total** | **41** |

## 8. Canonical Identity Check

```text
Expected first ID        L3-I-001
Expected last ID         L3-I-082
Expected count           82
Observed scenario rows   82
Unique primary IDs       82
Missing IDs              0
Duplicate primary IDs    0
Primary workflow count   41
Scenario assignments     82 / 82 exactly once
Workspace set            W-I01 … W-I08
```

## 9. Group Exit Gate

| Gate | Result |
|---|---|
| I1 canonical scenarios = 82 | PASS |
| Reconciled realization rows = 82 | PASS |
| Unique IDs = 82 | PASS |
| Missing IDs = 0 | PASS |
| Duplicate primary assignment = 0 | PASS |
| Every scenario assigned to one normalized workflow | PASS |
| Cross-workspace normalized merge = 0 | PASS |
| Distinct workflows discovered | PASS — 41 |
| User-addressable workflows | PASS — 41 |
| Mock-required workflows | PASS — 41 |
| Material upstream blockers = 0 | PASS |
| Examination ownership preserved | PASS |
| Cross-domain source facts remain source-owned | PASS |
| No invented actors/approvals/events | PASS |
| No automatic/system behavior fabricated | PASS |
| No implementation leakage | PASS |

**GROUP 11 VERDICT:** **PASS — COMPLETE FOR MASTER CROSS-GROUP RECONCILIATION**

Stage 2 may continue to Group 12. The canonical Workflow Realization Register remains intentionally unpopulated until Groups 1–14 and master cross-group reconciliation complete.
