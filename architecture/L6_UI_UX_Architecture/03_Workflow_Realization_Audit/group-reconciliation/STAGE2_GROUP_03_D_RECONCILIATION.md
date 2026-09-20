# SutraERP L6 Stage 2 — Group 3 Section D Reconciliation

**Status:** COMPLETE-FOR-MASTER-CROSS-GROUP-RECONCILIATION  
**Scope:** Section D — Academic Delivery  
**Scenario population:** 57  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Branch:** `l6-stage2-full-scenario-realization-audit`  
**Date:** 2026-09-20

> `G3-WF-###` identifiers are temporary Stage-2 normalization keys only. They are not business-architecture IDs and must not appear in ordinary product UX.

## 1. Completion Result

```text
Assigned canonical scenarios       57
Reconciled scenario rows           57 / 57
Unique scenario IDs                57
Missing scenarios                  0
Duplicate scenario assignments     0
Cross-workspace workflow merges    0
Material open issues               0
Distinct normalized workflows      38
User-addressable workflows         36
Observable non-interactive         2
```

Group 3 is complete for later master cross-group reconciliation. This does not by itself complete Stage 2.

## 2. Semantic Reconciliation Findings

The Worker-D baseline was mechanically complete but materially over-classified governed Academic Delivery work as background-only.

Evidence-backed reconciliation used the Locked Section-D L3 parents and their authoritative D01–D07 workflow sources. The following patterns were corrected without changing upstream business architecture:

- Course Offering establishment/activation, suspension, reactivation, cancellation, archive, revision/rollback, merge/split, freeze/unfreeze, quota/capacity governance and change-impact decisions expose real administrator/governance actions and cannot be background-only.
- Course Offering waitlist resolution retains legitimate manual administration/promotion action where policy permits, while normal policy-based resolution remains source-governed.
- Timetable planning, revision, publication, activation and retirement explicitly involve Academic Scheduler, Academic Administrator and Registrar responsibilities.
- Teaching Session execution and attendance capture/finalization/correction explicitly involve faculty and governed academic authorities.
- Teaching-progress / syllabus-coverage publication is a faculty-facing governed interaction, not tracking-only.
- Academic Monitoring initiation, risk assessment, intervention, review, closure, exception resolution and reopening expose Monitoring Authority / Governance Reviewer / Governance Authority work.
- Flexible-learning opportunity establishment, suspension, archival, provider onboarding, policy configuration, credit-mapping configuration and session archival expose governed Academic Affairs / institutional decisions.
- Seat-allocation conflicts and external-record / exception reconciliation require mock interaction only where the Locked parent provides a legitimate human exception or corrective action.
- External-provider registration and ordinary learning-progress/completion remain deliberately background-observable because their primary path is provider/domain fact exchange rather than a canonical end-user command.
- Internal Assessment submission is interactive; moderation/finalization/archive are governed decision workflows; controlled finalized-assessment correction is an exception/recovery workflow.
- Where Locked L3 records a required business decision but the exact canonical institutional role is unresolved upstream (notably portions of D07), L6 exposes the governed interaction without inventing a role, permission, approval chain or business rule.

No L0–L5 ownership, event, lifecycle, state, actor authority, policy or rule was redefined.

## 3. Scenario-Level Reconciled Distribution

| Disposition | Rows |
|---|---:|
| INTERACTIVE-WORKFLOW | 13 |
| REVIEW-APPROVAL-WORKFLOW | 33 |
| TRACKING-WORKFLOW | 0 |
| EXCEPTION-RECOVERY-WORKFLOW | 7 |
| CROSS-WORKSPACE-HANDOFF | 1 |
| SYSTEM-BACKGROUND-OBSERVABLE | 3 |
| NO-DIRECT-END-USER-INTERACTION | 0 |
| **Total** | **57** |

| Mock interaction | Rows |
|---|---:|
| REQUIRED | 54 |
| NOT-REQUIRED | 0 |
| PROHIBITED | 3 |
| **Total** | **57** |

The three deliberately non-mock scenario rows are `L3-D-035`, `L3-D-037`, and `L3-D-038` on their normal primary paths. Human exception/governance work around those outcomes remains represented by the relevant governed exception or review workflows rather than by inventing a primary action.

## 4. Workflow Normalization Method

Scenarios were merged only when they share the same primary workspace, canonical owner, institutional objective and sustained user job, and represent lifecycle variants, governed decision stages or source-supported recovery paths of that job.

Labels alone were not used as workflow identity. No normalized workflow crosses a workspace boundary in Group 3.

A workflow is mock-interactive when at least one member requires a legitimate user/governance action. Automatic source-owned decisions remain observable instead of receiving artificial controls.

## 5. Normalized Workflow Population

| Working workflow key | Workspace | Workflow family | Scenarios | Canonical scenario IDs | Workflow realization | Mock |
|---|---|---|---:|---|---|---|
| G3-WF-001 | W-D01 | Course Offering establishment and activation | 1 | L3-D-001 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-002 | W-D01 | Course Offering suspension and reactivation | 2 | L3-D-002; L3-D-003 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-003 | W-D01 | Course Offering cancellation | 1 | L3-D-004 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-004 | W-D01 | Capacity and quota governance | 2 | L3-D-005; L3-D-015 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-005 | W-D01 | Enrollment restriction and cross-boundary availability governance | 2 | L3-D-006; L3-D-008 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-006 | W-D01 | Waitlist and seat resolution | 1 | L3-D-007 | INTERACTIVE-WORKFLOW | REQUIRED |
| G3-WF-007 | W-D01 | Course Offering version replacement and rollback | 1 | L3-D-009 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-008 | W-D01 | Course Offering archival | 1 | L3-D-010 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-009 | W-D01 | Course Offering merge / split reorganization | 1 | L3-D-011 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-010 | W-D01 | Template-based Course Offering generation | 1 | L3-D-012 | INTERACTIVE-WORKFLOW | REQUIRED |
| G3-WF-011 | W-D01 | Course Offering freeze and unfreeze | 1 | L3-D-013 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-012 | W-D01 | Enrollment-window governance | 1 | L3-D-014 | INTERACTIVE-WORKFLOW | REQUIRED |
| G3-WF-013 | W-D01 | Course Offering change simulation and impact decision | 1 | L3-D-016 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-014 | W-D02 | Faculty Academic Assignment lifecycle | 2 | L3-D-017; L3-D-018 | INTERACTIVE-WORKFLOW | REQUIRED |
| G3-WF-015 | W-D02 | Faculty Academic Assignment governance configuration | 1 | L3-D-019 | INTERACTIVE-WORKFLOW | REQUIRED |
| G3-WF-016 | W-D03 | Timetable lifecycle — plan, revise, publish, activate, retire | 3 | L3-D-020; L3-D-021; L3-D-022 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-017 | W-D04 | Teaching Session execution and outcome | 1 | L3-D-023 | INTERACTIVE-WORKFLOW | REQUIRED |
| G3-WF-018 | W-D04 | Attendance capture, finalization and correction | 2 | L3-D-024; L3-D-025 | INTERACTIVE-WORKFLOW | REQUIRED |
| G3-WF-019 | W-D04 | Teaching progress and syllabus-coverage publication | 1 | L3-D-026 | INTERACTIVE-WORKFLOW | REQUIRED |
| G3-WF-020 | W-D05 | Monitoring initiation, outcome and academic-risk assessment | 2 | L3-D-027; L3-D-028 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-021 | W-D05 | Monitoring intervention, governance review, closure and reopening | 4 | L3-D-030; L3-D-031; L3-D-032; L3-D-033 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-022 | W-D05 | Monitoring governance-exception resolution | 1 | L3-D-029 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G3-WF-023 | W-D06 | Flexible-learning enrollment, seat allocation and waitlist resolution | 4 | L3-D-034; L3-D-035; L3-D-036; L3-D-048 | INTERACTIVE-WORKFLOW | REQUIRED |
| G3-WF-024 | W-D06 | External-learning provider registration | 1 | L3-D-037 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G3-WF-025 | W-D06 | Learning progress and completion | 1 | L3-D-038 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G3-WF-026 | W-D06 | Credit recognition and Academic Records posting handoff | 1 | L3-D-039 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| G3-WF-027 | W-D06 | Learning-opportunity lifecycle | 3 | L3-D-040; L3-D-041; L3-D-042 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-028 | W-D06 | External-learning certificate verification and investigation routing | 1 | L3-D-043 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-029 | W-D06 | Learning-provider onboarding and activation | 1 | L3-D-044 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-030 | W-D06 | Flexible-learning policy and approval configuration | 1 | L3-D-045 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-031 | W-D06 | Credit-mapping rule configuration | 1 | L3-D-046 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-032 | W-D06 | Flexible-learning academic-session archival | 1 | L3-D-047 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-033 | W-D06 | External-learning record reconciliation and exception closure | 2 | L3-D-049; L3-D-050 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G3-WF-034 | W-D07 | Internal Assessment blueprint and window governance | 2 | L3-D-051; L3-D-052 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-035 | W-D07 | Internal Assessment submission and marks capture | 1 | L3-D-053 | INTERACTIVE-WORKFLOW | REQUIRED |
| G3-WF-036 | W-D07 | Internal Assessment moderation | 1 | L3-D-054 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-037 | W-D07 | Internal Assessment finalization and archival | 2 | L3-D-055; L3-D-056 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G3-WF-038 | W-D07 | Finalized Internal Assessment correction | 1 | L3-D-057 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |

## 6. Workflow-Level Counts

| Workflow realization | Distinct workflows |
|---|---:|
| INTERACTIVE-WORKFLOW | 10 |
| REVIEW-APPROVAL-WORKFLOW | 22 |
| TRACKING-WORKFLOW | 0 |
| EXCEPTION-RECOVERY-WORKFLOW | 3 |
| CROSS-WORKSPACE-HANDOFF | 1 |
| SYSTEM-BACKGROUND-OBSERVABLE | 2 |
| NO-DIRECT-END-USER-INTERACTION | 0 |
| **Total** | **38** |

| Workflow mock realization | Distinct workflows |
|---|---:|
| REQUIRED | 36 |
| NOT-REQUIRED | 0 |
| PROHIBITED | 2 |
| **Total** | **38** |

The Group-3 user-addressable population is **36** workflow families. The two deliberately observable/non-interactive primary workflow families are:

- `G3-WF-024` — External-learning provider registration: the normal primary path is a governed hand-off plus provider acceptance/rejection and D06 recording; no canonical end-user command is invented.
- `G3-WF-025` — Learning progress and completion: external/provider evidence and D06 validation produce the authoritative outcome; the UI observes progress/completion rather than fabricating a completion action.

## 7. Canonical Identity Check

The Final Canonical Scenario ID Register confirms Section D contains exactly `L3-D-001 … L3-D-057`.

Group-3 reconciliation preserves the governed Section-D Phase-4 identity treatment:

- the final `L3-D-027` represents the approved canonical monitoring outcome derived from the Phase-4 consolidation of the prior D-027/D-028 lineage;
- final `L3-D-041` and `L3-D-042` are the two approved split outcomes for suspension and archival of a learning opportunity;
- no final scenario is renumbered, removed, duplicated or re-owned by this L6 normalization.

## 8. Group Exit Gate

| Gate | Result |
|---|---|
| Section-D canonical scenarios = 57 | PASS |
| Reconciled realization rows = 57 | PASS |
| Unique IDs = 57 | PASS |
| Missing IDs = 0 | PASS |
| Duplicate scenario assignment = 0 | PASS |
| Cross-workspace normalized merge = 0 | PASS |
| Material open issues = 0 | PASS |
| Distinct workflows discovered | PASS — 38 |
| User-addressable workflows discovered | PASS — 36 |
| Mock-required workflows discovered | PASS — 36 |
| Observable non-interactive workflows discovered | PASS — 2 |

**GROUP 3 VERDICT:** **PASS — COMPLETE FOR MASTER CROSS-GROUP RECONCILIATION**

Stage 2 remains in progress because Groups 4–14 and the final master cross-group reconciliation / canonical-register population remain incomplete.
