# SutraERP L6 Stage 2 — Full-Corpus Reconciliation & Validation Report — 2026-09-20

**Status:** STAGE-2 RECONCILIATION IN PROGRESS — GROUP 11 RECONCILED  
**Architecture Level:** L6 — UI / UX Architecture Governance  
**Branch:** `l6-stage2-full-scenario-realization-audit`  
**Base Architecture Main:** `11bc2088568d8cb682f606d14e50ba0ca6dd7ae5`  
**Frontend Main Verified:** `e2560877eac4446b925e45f04b7dfa4c24d3625c`  
**Canonical L3 Population:** 1,084  
**Purpose:** Record the independent Stage-2 reconciliation state after validating the 15 worker artifacts, correcting evidence-backed L6 mapping defects, resolving the discovered upstream semantic drift through controlled L3/L4 correction, and continuing workflow normalization before canonical register population.

## 1. Live Repository Baseline

Verified before modification:

- Architecture `main`: `11bc2088568d8cb682f606d14e50ba0ca6dd7ae5`
- Stage-2 branch: `l6-stage2-full-scenario-realization-audit` (current working head advanced during controlled reconciliation)
- Stage-2 branch relation after Group 7 reconciliation: ahead of `main` / 0 behind; exact ahead count advances with each controlled Stage-2 commit
- Frontend `main`: `e2560877eac4446b925e45f04b7dfa4c24d3625c`
- Frontend was not modified by this Stage-2 work.

## 2. Mechanical Corpus Validation

| Gate | Result |
|---|---:|
| Canonical L3 scenario files | 1,084 |
| Expected worker result files | 15 / 15 |
| Worker rows | 1,084 |
| Unique scenario IDs | 1,084 |
| Missing IDs | 0 |
| Duplicate IDs | 0 |
| Assigned-range defects | 0 |
| Invalid workspace references | 0 |
| Invalid archetype references | 0 |
| Invalid disposition values | 0 |
| Invalid mock-interaction values | 0 |
| Ordinary UX architecture-ID leakage | 0 |
| Unsupported Stage-2 maturity upgrades | 0 |
| Worker rows with material open issue after reconciliation | 0 |

All 15 worker artifacts use the required assigned ranges and remain at `SCENARIO-REALIZATION-MAPPED`.

## 3. Evidence-Backed L6 Corrections

The generated worker baseline contained false-background classifications where locked L3 explicitly assigns human preparation, review, approval, closing, locking, publication or owner-preserving correction/handoff responsibilities.

Corrections made:

| Scenario | Prior mapping | Reconciled mapping | Basis |
|---|---|---|---|
| L3-F-031 | SYSTEM-BACKGROUND-OBSERVABLE / PROHIBITED | CROSS-WORKSPACE-HANDOFF / REQUIRED | Accounting actors prepare/review close-readiness evidence and hand it to F09. |
| L3-F-052 | SYSTEM-BACKGROUND-OBSERVABLE / PROHIBITED | REVIEW-APPROVAL-WORKFLOW / REQUIRED | Period Closing Authority and Period Lock Authority perform governed close/lock decisions. |
| L3-I-007 | SYSTEM-BACKGROUND-OBSERVABLE / PROHIBITED | REVIEW-APPROVAL-WORKFLOW / REQUIRED | Examination Administrator plans; Controller of Examination reviews/approves. |
| L3-I-042 | SYSTEM-BACKGROUND-OBSERVABLE / PROHIBITED | REVIEW-APPROVAL-WORKFLOW / REQUIRED | Examination Administrator finalizes; Controller governance/approval applies where required. |
| L3-I-092 | EXCEPTION-RECOVERY-WORKFLOW / NOT-REQUIRED | CROSS-WORKSPACE-HANDOFF / REQUIRED | I09 holds result processing and routes source-owned identity/student correction to canonical A/C owners. |
| L3-I-101 | SYSTEM-BACKGROUND-OBSERVABLE / PROHIBITED | REVIEW-APPROVAL-WORKFLOW / REQUIRED | Publication staff verify readiness and publication authorization before official publication. |

## 4. Reconciled Provisional Disposition Distribution

These are scenario-realization counts, not distinct workflow counts.

| Primary disposition | Scenario rows |
|---|---:|
| INTERACTIVE-WORKFLOW | 210 |
| REVIEW-APPROVAL-WORKFLOW | 173 |
| TRACKING-WORKFLOW | 18 |
| EXCEPTION-RECOVERY-WORKFLOW | 186 |
| CROSS-WORKSPACE-HANDOFF | 12 |
| SYSTEM-BACKGROUND-OBSERVABLE | 476 |
| NO-DIRECT-END-USER-INTERACTION | 9 |
| **Total** | **1,084** |

Mock-interaction row population after the evidence-backed corrections:

| Mock interaction | Scenario rows |
|---|---:|
| REQUIRED | 432 |
| NOT-REQUIRED | 167 |
| PROHIBITED | 485 |
| **Total** | **1,084** |

These row counts do not establish the final number of distinct user-addressable workflows. Workflow normalization must not use unique labels as workflow identity.

## 5. Upstream Architecture Correction and Resolution

The initial semantic review found seven Phase-5 Section-I scenario bodies whose content had drifted onto neighbouring frozen canonical scenario meanings:

- `L3-I-148`
- `L3-I-149`
- `L3-I-150`
- `L3-I-157`
- `L3-I-158`
- `L3-I-159`
- `L3-I-160`

A controlled L3 amendment was created and the seven scenarios were restored to their frozen canonical meanings without changing final IDs, canonical titles, ownership, Type, Priority or total corpus size.

Downstream action:

- 7 / 7 L3 scenarios corrected as Locked v1.1 successors;
- 7 / 7 dependent L4 blocker-only Drafts re-derived as real behavioral Drafts v1.1.0;
- direct L5 artifacts keyed to these seven identities found: 0;
- corresponding Stage-2 rows revalidated and reconciled;
- prior open issues on `L3-I-148` and `L3-I-160` cleared.

Controlled evidence:

- `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Controlled_Semantic_Amendment_Decision_I148_I150_I157_I160_2026-09-20.md`
- `architecture/L4_System_Behavior_Specification/00_Governance/L4_Section_I_Corrected_Parent_Revalidation_I148_I150_I157_I160_2026-09-20.md`

**Material upstream blockers remaining after Group-6 remediation:** **0**.

## 6. Canonical Register Decision

`L6_Workflow_Realization_Register_DRAFT.md` remains unpopulated.

This is intentional and correct.

The Stage-2 process requires completion of cross-worker semantic normalization and distinct-workflow discovery before canonicalization. The previously discovered Locked-L3 contradictions have been corrected; canonical population remains deferred only until the remaining Stage-2 normalization work is completed.

Therefore:

```text
Canonical realization rows          0 / 1,084
Canonical register population       PENDING FINAL NORMALIZATION
Completion claim                     NOT AUTHORIZED
```

Controlled L3 and L4 corrections were made under explicit change governance. No L0, L1, L2 or direct L5 artifact was modified.

## 7. Workflow Count Decision

The generated worker baseline contains near-scenario-cardinality labels. Those labels are not canonical workflow identities and are not counted as workflows.

The upstream semantic blockers are now resolved. Full-corpus workflow normalization is therefore unblocked and remains the active Stage-2 task.

Completed PASS-group reconciliation now covers Groups 1–11:

- Group 1 — A+B: 95 scenarios → 50 distinct workflows;
- Group 2 — C: 75 scenarios → 32 distinct workflows;
- Group 3 — D: 57 scenarios → 38 distinct workflows;
- Group 4 — E: 75 scenarios → 55 distinct workflows;
- Group 5 — F: 85 scenarios → 59 distinct workflows — **PASS after canonical F13 KPI Definition / Analytical Model governance remediation**;
- Group 6 — G1: 77 scenarios → 39 distinct workflows — **PASS after canonical G00 exception-recovery and A01 configuration-governance remediation**;
- Group 7 — G2: 77 scenarios → 38 distinct workflows — **PASS after semantic correction of under-classified Hostel, Transport and Library realization rows**;
- Group 8 — G3: 97 scenarios → 46 distinct workflows — **PASS after semantic correction of under-classified Health Services, Security Operations and Housekeeping realization rows, while consuming the corrected G200/G212/G214 authority baseline**;
- Group 9 — G4: 69 scenarios → 31 distinct workflows — **PASS after semantic correction of under-classified Teacher Guardian Scheme and Infrastructure Management realization rows while preserving Academic, B03 and G01 ownership boundaries**;
- Group 10 — H: 81 scenarios → 54 distinct workflows — **PASS after semantic reconciliation across H01–H10, preserving genuine automatic behavior and consuming current H governance amendments**;
- Group 11 — I1: 82 scenarios → 41 distinct workflows — **PASS after semantic correction of under-classified Examination policy, session, scheduling, registration, Hall Ticket, conduct and evaluation rows**.

Cumulative semantic reconciliation coverage is **870 / 1,084 scenarios** across Groups 1–11, contributing **483 distinct workflows**. There are **0 material Stage-2 group blockers**. The former F076 blocker was resolved through ADR-FIN-ANALYTICS-001, the locked 2026-09-20 F13 analytics-governance amendment, targeted L3 revalidation, and re-derived L4-F-076 behavior. The Group-6 G013/G014 blockers were resolved through ADR-G-EXCEPTION-001, ADR-PLAT-CONFIG-001, locked 2026-09-20 G00/A01 amendments, targeted L3 revalidation, and re-derived L4-A-056/L4-G-013/L4-G-014 behavior. These remain temporary group-normalized workflow identities pending final cross-group reconciliation.

The following values remain **PENDING DISCOVERY THROUGH SEMANTIC NORMALIZATION** rather than blocked by architecture:

- Distinct user-addressable workflows
- Distinct interactive/review workflows
- Distinct tracking workflows
- Distinct exception/recovery workflows
- Distinct cross-workspace handoff workflows
- Final mock-interaction-required workflow population
- Final observable non-interactive workflow population

Scenario-row disposition counts in Section 4 remain valid realization-accounting evidence only.

## 8. Validator Hardening

The Stage-2 validator was strengthened to additionally verify:

- exactly 15 expected worker result files;
- worker IDs and assigned ranges;
- assigned/processed/missing/duplicate header consistency;
- required `COMPLETE-FOR-MASTER-REVIEW` result status;
- Stage-2 maturity remains exactly `SCENARIO-REALIZATION-MAPPED`;
- non-empty `openIssue` values use the controlled finding classes.

This does not replace architecture semantic review.

## 9. Stage-2 Exit Gate

| Exit condition | State |
|---|---|
| Canonical L3 scenarios = 1,084 | PASS |
| Worker rows = 1,084 / 1,084 | PASS |
| Unique worker IDs = 1,084 | PASS |
| Missing scenarios = 0 | PASS |
| Duplicate worker rows = 0 | PASS |
| Invalid workspace references = 0 | PASS |
| Invalid archetype references = 0 | PASS |
| Invalid controlled values = 0 | PASS |
| Canonical realization rows = 1,084 / 1,084 | **PENDING — 0 / 1,084 until final normalization** |
| Unexplained / conflicting primary realizations = 0 | **PASS for discovered upstream conflicts — 0 remaining** |
| Material unresolved blockers = 0 | **PASS — 0** |
| Distinct user-addressable workflow count discovered | **PENDING NORMALIZATION** |
| Mock-interaction-required workflow population discovered | **PENDING NORMALIZATION** |
| Observable non-interactive workflow population discovered | **PENDING NORMALIZATION** |

## 10. Verdict

**STAGE 2: IN PROGRESS — GROUP 11 RECONCILED**

The worker corpus is mechanically complete, the discovered upstream semantic drift has been corrected under controlled L3/L4 governance, and Groups 7–11 have completed semantic reconciliation with no new upstream blocker.

Stage 2 is not yet complete because distinct workflow normalization, final workflow-count discovery, canonical 1,084-row register population, and final independent validation still remain.

Stage 3 remains blocked until those Stage-2 exit conditions pass.


## 11. Group 7 — Campus Operations G2 Reconciliation

Group 7 covers `L3-G-078 … L3-G-154` (77 scenarios) across exactly three owner workspaces:

- `W-G03` — Hostel Management;
- `W-G04` — Transport Management;
- `W-G05` — Library Management.

The worker artifact was mechanically complete but materially under-classified explicit human work as background/system-owned behavior. Locked L3/L2 review confirmed human initiation, review, approval, verification, checklist execution, complaint resolution, catalogue governance, stock verification, administrative resolution and owner-preserving cross-workspace hand-offs.

Group 7 normalization result:

```text
Scenarios reconciled                 77 / 77
Distinct workflows                   38
User-addressable workflows           38
Tracking / non-mock workflows         0
Material blockers                     0
Verdict                              PASS
```

The shared A01 configuration-governance amendment is consumed by G092/G123/G153 without transferring G03/G04/G05 configuration semantics to Platform Foundation.

Canonical evidence:

`group-reconciliation/STAGE2_GROUP_07_G2_RECONCILIATION.md`

Updated cumulative position after Group 7:

```text
Groups PASS                          7 / 14
Scenarios semantically reconciled   541 / 1,084
Distinct group-normalized workflows 311
Material Stage-2 blockers             0
Remaining unreconciled scenarios    543
```


## 12. Group 8 — Campus Operations G3 Reconciliation

Group 8 covers `L3-G-155 … L3-G-251` (97 scenarios) across:

- `W-G06` — Institutional Health Services / Medical;
- `W-G07` — Security Operations;
- `W-G08` — Housekeeping Operations & Service Governance.

The G3 worker artifact was mechanically complete but materially under-classified explicit human healthcare, security and housekeeping work as background/system-owned behavior. Exact Locked L3 review, authoritative L2 verification and representative/exact L4 behavior confirmed 77 row corrections.

The current repository also contains the controlled 2026-09-01 authority correction for `G200`, `G212` and `G214`; Group 8 consumes those corrected Locked successor scenarios rather than the superseded authority interpretations.

Group 8 normalization result:

```text
Scenarios reconciled                 97 / 97
Distinct workflows                   46
User-addressable workflows           46
Tracking / non-mock workflows         0
Material blockers                     0
Verdict                              PASS
```

Shared A01 configuration governance is consumed only where appropriate. Domain-specific G07/G08 lifecycle and event semantics remain authoritative and are not overwritten.

Canonical evidence:

`group-reconciliation/STAGE2_GROUP_08_G3_RECONCILIATION.md`

Updated cumulative position after Group 8:

```text
Groups PASS                          8 / 14
Scenarios semantically reconciled   638 / 1,084
Distinct group-normalized workflows 357
Material Stage-2 blockers             0
Remaining unreconciled scenarios    446
```


## 13. Group 9 — Campus Operations G4 Reconciliation

Group 9 covers `L3-G-252 … L3-G-320` (69 scenarios) across:

- `W-G09` — Teacher Guardian Scheme;
- `W-G10` — Infrastructure Management.

The G4 worker artifact was mechanically complete but materially under-classified explicit Teacher Guardian and Infrastructure work as background/system-owned behavior. Exact Locked L3 review and authoritative L2 verification confirmed that mentoring, closure, disclosure, reporting, infrastructure administration, compliance, maintenance governance, recovery, decommissioning, archival and reporting all expose explicit business actors, decisions or owner-preserving hand-offs.

Group 9 normalization result:

```text
Scenarios reconciled                 69 / 69
Distinct workflows                   31
User-addressable workflows           31
Tracking / non-mock workflows         0
Material blockers                     0
Verdict                              PASS
```

Key ownership boundaries preserved:

- Academic owns Teacher Guardian assignment and scheduling; G09 owns the Mentorship Case and mentoring history.
- B03 owns institutional campus governance and Institution-to-Campus association; G10 owns Physical Campus and physical hierarchy.
- G01 owns maintenance planning/execution; G10 owns infrastructure facts, condition, availability and maintenance-requirement evidence.
- Generic A01 configuration governance remains owner-scoped and does not transfer G09/G10 semantics.

Canonical evidence:

`group-reconciliation/STAGE2_GROUP_09_G4_RECONCILIATION.md`

Updated cumulative position after Group 9:

```text
Groups PASS                          9 / 14
Scenarios semantically reconciled   707 / 1,084
Distinct group-normalized workflows 388
Material Stage-2 blockers             0
Remaining unreconciled scenarios    377
```


## 14. Group 10 — Institutional Services Reconciliation

Group 10 covers `L3-H-001 … L3-H-081` (81 scenarios) across ten workspaces `W-H01 … W-H10`.

The H worker artifact is mechanically complete. Semantic review found a mixed pattern: many rows marked background actually contain explicit business authorities, while a smaller set is genuinely automatic/policy-driven and must not receive fake UI actions.

Group 10 normalization result:

```text
Scenarios reconciled                 81 / 81
Distinct workflows                   54
User-addressable workflows           52
Tracking / non-mock workflows         2
Material blockers                     0
Verdict                              PASS
```

Scenario-level mock realization:

```text
REQUIRED                             71
NOT-REQUIRED                          3
PROHIBITED                            7
```

Current H amendments for H01/H02/H03/H04/H08/H09/H10 were consumed. No superseded pre-amendment authority interpretation was restored.

Canonical evidence:

`group-reconciliation/STAGE2_GROUP_10_H_RECONCILIATION.md`

Updated cumulative position after Group 10:

```text
Groups PASS                          10 / 14
Scenarios semantically reconciled   788 / 1,084
Distinct group-normalized workflows 442
Material Stage-2 blockers             0
Remaining unreconciled scenarios    296
```


## 15. Group 11 — Examination I1 Reconciliation

Group 11 covers `L3-I-001 … L3-I-082` (82 scenarios) across `W-I01 … W-I08`.

The I1 worker artifact is mechanically complete but materially under-classified explicit Examination governance and operations as background/system-owned behavior. Locked L3 review confirmed policy approval, session governance, schedule/venue/invigilation/examiner decisions, registration/hall-ticket correction, practical/theory conduct, exception handling, marks collection and evaluation all expose source-defined business actors or governed decisions.

Group 11 normalization result:

```text
Scenarios reconciled                 82 / 82
Distinct workflows                   41
User-addressable workflows           41
Tracking / non-mock workflows         0
Material blockers                     0
Verdict                              PASS
```

All 82 scenario rows require meaningful mock interaction; no Group-11 row remains legitimately system-only after current source validation.

Canonical evidence:

`group-reconciliation/STAGE2_GROUP_11_I1_RECONCILIATION.md`

Updated cumulative position after Group 11:

```text
Groups PASS                          11 / 14
Scenarios semantically reconciled   870 / 1,084
Distinct group-normalized workflows 483
Material Stage-2 blockers             0
Remaining unreconciled scenarios    214
```
