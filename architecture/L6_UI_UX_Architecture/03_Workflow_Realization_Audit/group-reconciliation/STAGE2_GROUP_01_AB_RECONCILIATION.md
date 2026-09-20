# SutraERP L6 Stage 2 — Group 1 A+B Reconciliation

**Status:** COMPLETE-FOR-MASTER-CROSS-GROUP-RECONCILIATION  
**Scope:** Section A — Platform Foundation + Section B — Institution & Structure  
**Scenario population:** 95  
**Section A:** 62  
**Section B:** 33  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Branch:** `l6-stage2-full-scenario-realization-audit`  
**Date:** 2026-09-20

> `G1-WF-###` identifiers are temporary Stage-2 normalization keys only. They are not business-architecture IDs and must not appear in ordinary product UX.

## 1. Group-1 Completion Result

```text
Assigned canonical scenarios       95
Reconciled scenario rows           95 / 95
Unique scenario IDs                95
Missing scenarios                  0
Duplicate scenario assignments     0
Cross-workspace workflow merges    0
Material open issues               0
Distinct normalized workflows      50
User-addressable workflows         48
Observable non-interactive         2
```

Group 1 is complete for later cross-group reconciliation. This does not by itself complete Stage 2.

## 2. Semantic Reconciliation Findings

The worker baseline was mechanically complete but materially under-classified human interaction in Sections A and B.

Corrections were limited to L6 realization semantics:

- Section A: 12 rows corrected.
- Section B: 24 rows corrected.
- No L0-L5 architecture was changed by Group-1 reconciliation.
- No actor, approval authority, lifecycle state, event or permission was invented.
- Where Locked L3 defines a human/business action but does not identify a canonical actor, L6 exposes a governed interaction without inventing the missing authority.
- Where Locked L3 is explicitly platform/system driven, the scenario remains observable/non-interactive.

Representative corrections:

- `L3-A-007` authentication request/sign-in: background/prohibited → interactive/required.
- `L3-A-031` module retirement: background/prohibited → review-oriented/required.
- `L3-A-036` governed dataset import: background/prohibited → interactive/required.
- `L3-A-039` localized dataset preparation/review: background/prohibited → review-oriented/required.
- Section-B institutional, organisation, campus, curriculum and calendar scenarios with explicit initiation/decision semantics were changed from background/prohibited to user-facing governed workflows.
- Exception/recovery rows became mock-required only where the Locked parent provides a real human recovery/reconciliation action.

## 3. Scenario-Level Reconciled Distribution

| Disposition | Rows |
|---|---:|
| INTERACTIVE-WORKFLOW | 50 |
| REVIEW-APPROVAL-WORKFLOW | 14 |
| TRACKING-WORKFLOW | 2 |
| EXCEPTION-RECOVERY-WORKFLOW | 19 |
| CROSS-WORKSPACE-HANDOFF | 0 |
| SYSTEM-BACKGROUND-OBSERVABLE | 8 |
| NO-DIRECT-END-USER-INTERACTION | 2 |
| **Total** | **95** |

| Mock interaction | Rows |
|---|---:|
| REQUIRED | 77 |
| NOT-REQUIRED | 8 |
| PROHIBITED | 10 |
| **Total** | **95** |

## 4. Workflow Normalization Method

Scenarios were merged only when they share the same primary workspace, institutional objective and owner, and represent lifecycle variants, exception/recovery paths, approval/review stages or successor states of the same user job.

Labels alone were not used as workflow identity. No workflow family crosses a workspace boundary.

The workflow-level disposition is the most user-governed realization present in the family. Mock interaction is `REQUIRED` when at least one member scenario needs a legitimate mock action.

## 5. Normalized Workflow Population

| Working workflow key | Workspace | Workflow family | Scenarios | Canonical scenario IDs | Workflow realization | Mock |
|---|---|---|---:|---|---|---|
| G1-WF-001 | W-A01 | Tenant lifecycle | 3 | L3-A-001; L3-A-002; L3-A-003 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-002 | W-A01 | Tenant configuration and controlled change | 2 | L3-A-056; L3-A-057 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-003 | W-A02 | Person identity lifecycle | 3 | L3-A-004; L3-A-053; L3-A-054 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-004 | W-A02 | Identity reconciliation | 2 | L3-A-005; L3-A-006 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G1-WF-005 | W-A03 | Authentication and credential recovery | 2 | L3-A-007; L3-A-008 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-006 | W-A03 | MFA enrollment | 1 | L3-A-009 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-007 | W-A03 | Authentication provider onboarding | 1 | L3-A-055 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-008 | W-A04 | Responsibility assignment | 1 | L3-A-010 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-009 | W-A04 | Responsibility delegation | 1 | L3-A-011 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-010 | W-A04 | Temporary access | 1 | L3-A-012 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-011 | W-A04 | Emergency access | 1 | L3-A-013 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-012 | W-A04 | Workspace definition governance | 1 | L3-A-050 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| G1-WF-013 | W-A04 | Separation-of-duties policy governance | 1 | L3-A-051 | EXCEPTION-RECOVERY-WORKFLOW | NOT-REQUIRED |
| G1-WF-014 | W-A04 | Authorization policy lifecycle | 1 | L3-A-052 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G1-WF-015 | W-A05 | Workflow definition lifecycle | 2 | L3-A-014; L3-A-017 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-016 | W-A05 | Workflow execution and recovery | 2 | L3-A-015; L3-A-016 | EXCEPTION-RECOVERY-WORKFLOW | NOT-REQUIRED |
| G1-WF-017 | W-A05 | Human decision task resolution | 1 | L3-A-058 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G1-WF-018 | W-A06 | Event contract lifecycle | 2 | L3-A-018; L3-A-060 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-019 | W-A06 | Event delivery and recovery | 2 | L3-A-019; L3-A-020 | EXCEPTION-RECOVERY-WORKFLOW | NOT-REQUIRED |
| G1-WF-020 | W-A06 | Historical event replay | 1 | L3-A-021 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-021 | W-A06 | Event subscription lifecycle | 1 | L3-A-059 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-022 | W-A07 | Audit evidence availability | 1 | L3-A-022 | TRACKING-WORKFLOW | NOT-REQUIRED |
| G1-WF-023 | W-A07 | Audit investigation and restored access | 2 | L3-A-023; L3-A-027 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G1-WF-024 | W-A07 | Audit integrity verification | 1 | L3-A-024 | NO-DIRECT-END-USER-INTERACTION | PROHIBITED |
| G1-WF-025 | W-A07 | Audit retention and archival | 2 | L3-A-025; L3-A-026 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G1-WF-026 | W-A08 | Capability module lifecycle | 4 | L3-A-028; L3-A-029; L3-A-030; L3-A-031 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G1-WF-027 | W-A08 | Module dependency governance | 1 | L3-A-032 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-028 | W-A09 | Dataset lifecycle | 5 | L3-A-033; L3-A-034; L3-A-035; L3-A-040; L3-A-041 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-029 | W-A09 | Governed dataset import | 1 | L3-A-036 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-030 | W-A09 | Governed dataset export | 1 | L3-A-037 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-031 | W-A09 | Dataset synchronization | 1 | L3-A-038 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-032 | W-A09 | Dataset localization | 1 | L3-A-039 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G1-WF-033 | W-A10 | Readiness assessment and remediation | 5 | L3-A-042; L3-A-043; L3-A-044; L3-A-045; L3-A-049 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G1-WF-034 | W-A10 | Readiness certificate lifecycle | 4 | L3-A-046; L3-A-047; L3-A-061; L3-A-062 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G1-WF-035 | W-A10 | Readiness significant-change impact review | 1 | L3-A-048 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G1-WF-036 | W-B01 | Institution lifecycle | 3 | L3-B-001; L3-B-002; L3-B-003 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G1-WF-037 | W-B01 | Institution profile revision | 1 | L3-B-004 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-038 | W-B01 | Institutional policy revision | 1 | L3-B-005 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-039 | W-B02 | Organisation unit lifecycle | 5 | L3-B-006; L3-B-007; L3-B-008; L3-B-012; L3-B-016 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-040 | W-B02 | Organisation structure transformation | 2 | L3-B-009; L3-B-010 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G1-WF-041 | W-B02 | Organisation governance configuration | 1 | L3-B-011 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-042 | W-B02 | Position definition lifecycle | 3 | L3-B-013; L3-B-014; L3-B-015 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-043 | W-B03 | Institutional campus lifecycle | 6 | L3-B-017; L3-B-018; L3-B-019; L3-B-020; L3-B-021; L3-B-022 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| G1-WF-044 | W-B04 | Curriculum lifecycle | 3 | L3-B-023; L3-B-024; L3-B-025 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-045 | W-B04 | School academic model lifecycle | 1 | L3-B-026 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-046 | W-B05 | Academic calendar lifecycle | 3 | L3-B-027; L3-B-029; L3-B-033 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-047 | W-B05 | Calendar assignment | 1 | L3-B-028 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-048 | W-B05 | Calendar exception | 1 | L3-B-030 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| G1-WF-049 | W-B05 | Academic year closure | 1 | L3-B-031 | INTERACTIVE-WORKFLOW | REQUIRED |
| G1-WF-050 | W-B05 | Calendar change simulation | 1 | L3-B-032 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |

## 6. Workflow-Level Counts

| Workflow realization | Distinct workflows |
|---|---:|
| INTERACTIVE-WORKFLOW | 29 |
| REVIEW-APPROVAL-WORKFLOW | 12 |
| TRACKING-WORKFLOW | 1 |
| EXCEPTION-RECOVERY-WORKFLOW | 6 |
| CROSS-WORKSPACE-HANDOFF | 0 |
| SYSTEM-BACKGROUND-OBSERVABLE | 1 |
| NO-DIRECT-END-USER-INTERACTION | 1 |
| **Total** | **50** |

| Workflow mock realization | Distinct workflows |
|---|---:|
| REQUIRED | 44 |
| NOT-REQUIRED | 4 |
| PROHIBITED | 2 |
| **Total** | **50** |

The Group-1 user-addressable population is **48** workflow families. The remaining **2** are deliberately observable/non-interactive:

- `G1-WF-012` — Workspace definition governance: upstream does not define the actors/commands/lifecycle required to expose a canonical user action safely.
- `G1-WF-024` — Audit integrity verification: Locked L3 explicitly requires no scenario-specific human actor.

## 7. Canonical Identity Check

The Final Canonical Scenario ID Register confirms:

- Section A maps `CAN-L3-A-001 … CAN-L3-A-062` directly to `L3-A-001 … L3-A-062`.
- Section B preserves the governed campus split: `L3-B-021` and `L3-B-022` are the two final campus-decommission/archive children, and subsequent IDs follow the approved shifted mapping.

Group-1 normalization does not renumber, delete or redefine any canonical L3 scenario.

## 8. Group Exit Gate

| Gate | Result |
|---|---|
| A+B canonical scenarios = 95 | PASS |
| Reconciled realization rows = 95 | PASS |
| Unique IDs = 95 | PASS |
| Missing IDs = 0 | PASS |
| Duplicate scenario assignment = 0 | PASS |
| Cross-workspace normalized merge = 0 | PASS |
| Material open issues = 0 | PASS |
| Distinct workflows discovered | PASS — 50 |
| User-addressable workflows discovered | PASS — 48 |
| Mock-required workflows discovered | PASS — 44 |
| Observable non-interactive workflows discovered | PASS — 2 |

**GROUP 1 VERDICT:** **PASS — COMPLETE FOR MASTER CROSS-GROUP RECONCILIATION**

Stage 2 remains in progress because Groups 2–14 and final cross-group reconciliation/canonical-register population remain incomplete.
