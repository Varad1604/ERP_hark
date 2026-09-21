# SutraERP L6 Stage 2 — Group 15 Master Cross-Group Reconciliation

**Status:** COMPLETE — MASTER CROSS-GROUP RECONCILIATION
**Scope:** Groups 1–14; all 1,084 canonical L3 scenarios
**Architecture Level:** L6 — UI / UX Architecture Support
**Branch:** `l6-stage2-group15-master-reconciliation`
**Date:** 2026-09-21

> `L6-RWF-####` is a final L6 realization-workflow identity for reviewer traceability. It is not a business-architecture ID and must not appear in ordinary end-user UX.

## 1. Master Result

```text
Canonical scenarios reconciled          1084 / 1,084
Unique scenario assignments             1084
Duplicate primary assignments           0
Temporary group workflows               595
Final enterprise realization workflows  595
Cross-group workflow merges              0
Cross-group workflow splits              0
Material upstream blockers               0
```

## 2. Cross-Group Identity Decision

All 595 group-normalized workflow families were compared under the Stage-2 identity rule: same primary workspace, same canonical owner, same sustained institutional job, and same aggregate/lifecycle context.

No L6 workspace occurs in more than one reconciliation group. A cross-group merge would therefore cross a canonical workspace/owner boundary and violate the Stage-2 normalization rule. Shared framework semantics (for example A01 configuration governance) do not collapse owner-domain workflows into one workflow.

**Final distinct workflow population: 595.**

## 3. Final Workflow-Level Distribution

| Realization | Workflows |
|---|---:|
| CROSS-WORKSPACE-HANDOFF | 28 |
| EXCEPTION-RECOVERY-WORKFLOW | 103 |
| INTERACTIVE-WORKFLOW | 154 |
| NO-DIRECT-END-USER-INTERACTION | 1 |
| REVIEW-APPROVAL-WORKFLOW | 271 |
| SYSTEM-BACKGROUND-OBSERVABLE | 32 |
| TRACKING-WORKFLOW | 6 |
| **Total** | **595** |

| Mock realization | Workflows |
|---|---:|
| NOT-REQUIRED | 11 |
| PROHIBITED | 33 |
| REQUIRED | 551 |
| **Total** | **595** |

User-addressable / reviewer-addressable workflows are those requiring meaningful mock interaction; system/tracking workflows remain visible without fabricated mutation controls.

## 4. Shared Governance Reconciliation

- A01 generic configuration governance remains framework-owned and owner-scoped; domain configuration semantics stay with their canonical owners.
- F13 KPI Definition / Analytical Model governance remains Finance-owned and uses the locked analytics-definition lifecycle and approval authority.
- G00 exception recovery remains governed by the locked G00 amendment and `GovernanceExceptionRecoveryRecorded` for authorized non-escalation recovery.
- Corrected Section-I successors, including I148–I160 and the I161 semantic correction, remain authoritative.
- No later group reintroduced any superseded blocker-only interpretation.

## 5. Cross-Domain Handoff Rule

Every handoff workflow preserves the provider's source facts and the receiving domain's canonical operational ownership. A handoff may expose initiation, evidence, acknowledgement and outcome visibility, but never fake execution controls for another owner.

## 6. Terminology Reconciliation

Workflow-family labels preserve canonical upstream terminology. Stylistic differences were not used to rename business concepts. No event, aggregate, lifecycle state, actor or owner was renamed in L6.

## 7. Final Workflow Catalog

| Final workflow | Source key | Group | Workspace | Workflow family | Canonical scenarios | Realization | Mock |
|---|---|---:|---|---|---|---|---|
| L6-RWF-0001 | G1-WF-001 | 1 | W-A01 | Tenant lifecycle | L3-A-001; L3-A-002; L3-A-003 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0002 | G1-WF-002 | 1 | W-A01 | Tenant configuration and controlled change | L3-A-056; L3-A-057 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0003 | G1-WF-003 | 1 | W-A02 | Person identity lifecycle | L3-A-004; L3-A-053; L3-A-054 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0004 | G1-WF-004 | 1 | W-A02 | Identity reconciliation | L3-A-005; L3-A-006 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0005 | G1-WF-005 | 1 | W-A03 | Authentication and credential recovery | L3-A-007; L3-A-008 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0006 | G1-WF-006 | 1 | W-A03 | MFA enrollment | L3-A-009 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0007 | G1-WF-007 | 1 | W-A03 | Authentication provider onboarding | L3-A-055 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0008 | G1-WF-008 | 1 | W-A04 | Responsibility assignment | L3-A-010 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0009 | G1-WF-009 | 1 | W-A04 | Responsibility delegation | L3-A-011 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0010 | G1-WF-010 | 1 | W-A04 | Temporary access | L3-A-012 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0011 | G1-WF-011 | 1 | W-A04 | Emergency access | L3-A-013 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0012 | G1-WF-012 | 1 | W-A04 | Workspace definition governance | L3-A-050 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0013 | G1-WF-013 | 1 | W-A04 | Separation-of-duties policy governance | L3-A-051 | EXCEPTION-RECOVERY-WORKFLOW | NOT-REQUIRED |
| L6-RWF-0014 | G1-WF-014 | 1 | W-A04 | Authorization policy lifecycle | L3-A-052 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0015 | G1-WF-015 | 1 | W-A05 | Workflow definition lifecycle | L3-A-014; L3-A-017 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0016 | G1-WF-016 | 1 | W-A05 | Workflow execution and recovery | L3-A-015; L3-A-016 | EXCEPTION-RECOVERY-WORKFLOW | NOT-REQUIRED |
| L6-RWF-0017 | G1-WF-017 | 1 | W-A05 | Human decision task resolution | L3-A-058 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0018 | G1-WF-018 | 1 | W-A06 | Event contract lifecycle | L3-A-018; L3-A-060 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0019 | G1-WF-019 | 1 | W-A06 | Event delivery and recovery | L3-A-019; L3-A-020 | EXCEPTION-RECOVERY-WORKFLOW | NOT-REQUIRED |
| L6-RWF-0020 | G1-WF-020 | 1 | W-A06 | Historical event replay | L3-A-021 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0021 | G1-WF-021 | 1 | W-A06 | Event subscription lifecycle | L3-A-059 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0022 | G1-WF-022 | 1 | W-A07 | Audit evidence availability | L3-A-022 | TRACKING-WORKFLOW | NOT-REQUIRED |
| L6-RWF-0023 | G1-WF-023 | 1 | W-A07 | Audit investigation and restored access | L3-A-023; L3-A-027 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0024 | G1-WF-024 | 1 | W-A07 | Audit integrity verification | L3-A-024 | NO-DIRECT-END-USER-INTERACTION | PROHIBITED |
| L6-RWF-0025 | G1-WF-025 | 1 | W-A07 | Audit retention and archival | L3-A-025; L3-A-026 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0026 | G1-WF-026 | 1 | W-A08 | Capability module lifecycle | L3-A-028; L3-A-029; L3-A-030; L3-A-031 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0027 | G1-WF-027 | 1 | W-A08 | Module dependency governance | L3-A-032 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0028 | G1-WF-028 | 1 | W-A09 | Dataset lifecycle | L3-A-033; L3-A-034; L3-A-035; L3-A-040; L3-A-041 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0029 | G1-WF-029 | 1 | W-A09 | Governed dataset import | L3-A-036 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0030 | G1-WF-030 | 1 | W-A09 | Governed dataset export | L3-A-037 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0031 | G1-WF-031 | 1 | W-A09 | Dataset synchronization | L3-A-038 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0032 | G1-WF-032 | 1 | W-A09 | Dataset localization | L3-A-039 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0033 | G1-WF-033 | 1 | W-A10 | Readiness assessment and remediation | L3-A-042; L3-A-043; L3-A-044; L3-A-045; L3-A-049 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0034 | G1-WF-034 | 1 | W-A10 | Readiness certificate lifecycle | L3-A-046; L3-A-047; L3-A-061; L3-A-062 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0035 | G1-WF-035 | 1 | W-A10 | Readiness significant-change impact review | L3-A-048 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0036 | G1-WF-036 | 1 | W-B01 | Institution lifecycle | L3-B-001; L3-B-002; L3-B-003 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0037 | G1-WF-037 | 1 | W-B01 | Institution profile revision | L3-B-004 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0038 | G1-WF-038 | 1 | W-B01 | Institutional policy revision | L3-B-005 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0039 | G1-WF-039 | 1 | W-B02 | Organisation unit lifecycle | L3-B-006; L3-B-007; L3-B-008; L3-B-012; L3-B-016 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0040 | G1-WF-040 | 1 | W-B02 | Organisation structure transformation | L3-B-009; L3-B-010 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0041 | G1-WF-041 | 1 | W-B02 | Organisation governance configuration | L3-B-011 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0042 | G1-WF-042 | 1 | W-B02 | Position definition lifecycle | L3-B-013; L3-B-014; L3-B-015 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0043 | G1-WF-043 | 1 | W-B03 | Institutional campus lifecycle | L3-B-017; L3-B-018; L3-B-019; L3-B-020; L3-B-021; L3-B-022 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0044 | G1-WF-044 | 1 | W-B04 | Curriculum lifecycle | L3-B-023; L3-B-024; L3-B-025 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0045 | G1-WF-045 | 1 | W-B04 | School academic model lifecycle | L3-B-026 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0046 | G1-WF-046 | 1 | W-B05 | Academic calendar lifecycle | L3-B-027; L3-B-029; L3-B-033 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0047 | G1-WF-047 | 1 | W-B05 | Calendar assignment | L3-B-028 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0048 | G1-WF-048 | 1 | W-B05 | Calendar exception | L3-B-030 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0049 | G1-WF-049 | 1 | W-B05 | Academic year closure | L3-B-031 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0050 | G1-WF-050 | 1 | W-B05 | Calendar change simulation | L3-B-032 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0051 | G2-WF-001 | 2 | W-C01 | Admission status lifecycle | L3-C-002; L3-C-003; L3-C-004; L3-C-005; L3-C-006 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0052 | G2-WF-002 | 2 | W-C01 | Admission document verification | L3-C-007 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0053 | G2-WF-003 | 2 | W-C02 | Student activation | L3-C-001 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0054 | G2-WF-004 | 2 | W-C02 | Student enrollment and assignment | L3-C-008; L3-C-009; L3-C-012; L3-C-013 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0055 | G2-WF-005 | 2 | W-C02 | Student suspension and restoration | L3-C-010; L3-C-011 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0056 | G2-WF-006 | 2 | W-C02 | Student profile and communication preferences | L3-C-014; L3-C-015 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0057 | G2-WF-007 | 2 | W-C02 | Student identity and duplicate reconciliation | L3-C-016; L3-C-017 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0058 | G2-WF-008 | 2 | W-C02 | Student document and certificate request | L3-C-018 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0059 | G2-WF-009 | 2 | W-C02 | Student context archival | L3-C-019 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0060 | G2-WF-010 | 2 | W-C03 | Academic registration window | L3-C-020 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0061 | G2-WF-011 | 2 | W-C03 | Academic registration lifecycle | L3-C-021; L3-C-022; L3-C-023; L3-C-024; L3-C-025 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0062 | G2-WF-012 | 2 | W-C03 | Registration hold lifecycle | L3-C-026 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0063 | G2-WF-013 | 2 | W-C04 | Academic progression decision lifecycle | L3-C-027; L3-C-029; L3-C-030; L3-C-031; L3-C-038 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0064 | G2-WF-014 | 2 | W-C04 | Academic credit ledger | L3-C-028; L3-C-037 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0065 | G2-WF-015 | 2 | W-C04 | Credit transfer and recognition | L3-C-032 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0066 | G2-WF-016 | 2 | W-C04 | Programme migration | L3-C-033 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0067 | G2-WF-017 | 2 | W-C04 | Academic break and rejoining | L3-C-034; L3-C-035 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0068 | G2-WF-018 | 2 | W-C04 | Qualification eligibility determination | L3-C-036 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0069 | G2-WF-019 | 2 | W-C04 | Academic progression policy governance | L3-C-075 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0070 | G2-WF-020 | 2 | W-C05 | Qualification completion and conferment | L3-C-039; L3-C-041; L3-C-049; L3-C-050 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0071 | G2-WF-021 | 2 | W-C05 | Student exit lifecycle | L3-C-040; L3-C-042; L3-C-043; L3-C-044; L3-C-045; L3-C-047; L3-C-048 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0072 | G2-WF-022 | 2 | W-C05 | Institutional exit clearance | L3-C-046 | TRACKING-WORKFLOW | NOT-REQUIRED |
| L6-RWF-0073 | G2-WF-023 | 2 | W-C06 | Guardian relationship lifecycle | L3-C-051; L3-C-052; L3-C-059; L3-C-060; L3-C-061 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0074 | G2-WF-024 | 2 | W-C06 | Guardian custody and consent governance | L3-C-053; L3-C-054 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0075 | G2-WF-025 | 2 | W-C06 | Guardian communication and responsibility governance | L3-C-055; L3-C-057 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0076 | G2-WF-026 | 2 | W-C06 | Emergency and pickup authorization | L3-C-056; L3-C-058 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0077 | G2-WF-027 | 2 | W-C07 | Scholarship definition and policy lifecycle | L3-C-062; L3-C-063 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0078 | G2-WF-028 | 2 | W-C07 | Scholarship eligibility assessment | L3-C-064; L3-C-065; L3-C-066; L3-C-067; L3-C-070 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0079 | G2-WF-029 | 2 | W-C07 | Scholarship eligibility expiry and archive | L3-C-068 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0080 | G2-WF-030 | 2 | W-C07 | External scholarship synchronization | L3-C-069 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0081 | G2-WF-031 | 2 | W-C07 | Scholarship eligibility to financial aid handoff | L3-C-071 | CROSS-WORKSPACE-HANDOFF | NOT-REQUIRED |
| L6-RWF-0082 | G2-WF-032 | 2 | W-C08 | Alumni transition lifecycle | L3-C-072; L3-C-073; L3-C-074 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0083 | G3-WF-001 | 3 | W-D01 | Course Offering establishment and activation | L3-D-001 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0084 | G3-WF-002 | 3 | W-D01 | Course Offering suspension and reactivation | L3-D-002; L3-D-003 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0085 | G3-WF-003 | 3 | W-D01 | Course Offering cancellation | L3-D-004 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0086 | G3-WF-004 | 3 | W-D01 | Capacity and quota governance | L3-D-005; L3-D-015 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0087 | G3-WF-005 | 3 | W-D01 | Enrollment restriction and cross-boundary availability governance | L3-D-006; L3-D-008 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0088 | G3-WF-006 | 3 | W-D01 | Waitlist and seat resolution | L3-D-007 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0089 | G3-WF-007 | 3 | W-D01 | Course Offering version replacement and rollback | L3-D-009 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0090 | G3-WF-008 | 3 | W-D01 | Course Offering archival | L3-D-010 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0091 | G3-WF-009 | 3 | W-D01 | Course Offering merge / split reorganization | L3-D-011 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0092 | G3-WF-010 | 3 | W-D01 | Template-based Course Offering generation | L3-D-012 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0093 | G3-WF-011 | 3 | W-D01 | Course Offering freeze and unfreeze | L3-D-013 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0094 | G3-WF-012 | 3 | W-D01 | Enrollment-window governance | L3-D-014 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0095 | G3-WF-013 | 3 | W-D01 | Course Offering change simulation and impact decision | L3-D-016 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0096 | G3-WF-014 | 3 | W-D02 | Faculty Academic Assignment lifecycle | L3-D-017; L3-D-018 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0097 | G3-WF-015 | 3 | W-D02 | Faculty Academic Assignment governance configuration | L3-D-019 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0098 | G3-WF-016 | 3 | W-D03 | Timetable lifecycle — plan, revise, publish, activate, retire | L3-D-020; L3-D-021; L3-D-022 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0099 | G3-WF-017 | 3 | W-D04 | Teaching Session execution and outcome | L3-D-023 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0100 | G3-WF-018 | 3 | W-D04 | Attendance capture, finalization and correction | L3-D-024; L3-D-025 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0101 | G3-WF-019 | 3 | W-D04 | Teaching progress and syllabus-coverage publication | L3-D-026 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0102 | G3-WF-020 | 3 | W-D05 | Monitoring initiation, outcome and academic-risk assessment | L3-D-027; L3-D-028 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0103 | G3-WF-021 | 3 | W-D05 | Monitoring intervention, governance review, closure and reopening | L3-D-030; L3-D-031; L3-D-032; L3-D-033 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0104 | G3-WF-022 | 3 | W-D05 | Monitoring governance-exception resolution | L3-D-029 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0105 | G3-WF-023 | 3 | W-D06 | Flexible-learning enrollment, seat allocation and waitlist resolution | L3-D-034; L3-D-035; L3-D-036; L3-D-048 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0106 | G3-WF-024 | 3 | W-D06 | External-learning provider registration | L3-D-037 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0107 | G3-WF-025 | 3 | W-D06 | Learning progress and completion | L3-D-038 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0108 | G3-WF-026 | 3 | W-D06 | Credit recognition and Academic Records posting handoff | L3-D-039 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0109 | G3-WF-027 | 3 | W-D06 | Learning-opportunity lifecycle | L3-D-040; L3-D-041; L3-D-042 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0110 | G3-WF-028 | 3 | W-D06 | External-learning certificate verification and investigation routing | L3-D-043 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0111 | G3-WF-029 | 3 | W-D06 | Learning-provider onboarding and activation | L3-D-044 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0112 | G3-WF-030 | 3 | W-D06 | Flexible-learning policy and approval configuration | L3-D-045 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0113 | G3-WF-031 | 3 | W-D06 | Credit-mapping rule configuration | L3-D-046 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0114 | G3-WF-032 | 3 | W-D06 | Flexible-learning academic-session archival | L3-D-047 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0115 | G3-WF-033 | 3 | W-D06 | External-learning record reconciliation and exception closure | L3-D-049; L3-D-050 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0116 | G3-WF-034 | 3 | W-D07 | Internal Assessment blueprint and window governance | L3-D-051; L3-D-052 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0117 | G3-WF-035 | 3 | W-D07 | Internal Assessment submission and marks capture | L3-D-053 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0118 | G3-WF-036 | 3 | W-D07 | Internal Assessment moderation | L3-D-054 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0119 | G3-WF-037 | 3 | W-D07 | Internal Assessment finalization and archival | L3-D-055; L3-D-056 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0120 | G3-WF-038 | 3 | W-D07 | Finalized Internal Assessment correction | L3-D-057 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0121 | G4-WF-001 | 4 | W-E01 | Workforce Engagement establishment and activation | L3-E-001 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0122 | G4-WF-002 | 4 | W-E01 | Active Workforce Engagement modification | L3-E-002 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0123 | G4-WF-003 | 4 | W-E01 | Workforce organisational transfer | L3-E-003; L3-E-010 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0124 | G4-WF-004 | 4 | W-E01 | Workforce suspension and operational restoration | L3-E-004 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0125 | G4-WF-005 | 4 | W-E01 | Workforce deputation lifecycle | L3-E-005 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0126 | G4-WF-006 | 4 | W-E01 | Long-leave operational availability | L3-E-006 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0127 | G4-WF-007 | 4 | W-E01 | Workforce Engagement closure and exit consequence | L3-E-007 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0128 | G4-WF-008 | 4 | W-E01 | Workforce rejoining consequence | L3-E-008 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0129 | G4-WF-009 | 4 | W-E01 | Duplicate Workforce Profile resolution | L3-E-009 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0130 | G4-WF-010 | 4 | W-E02 | Position Assignment lifecycle | L3-E-011; L3-E-012; L3-E-013; L3-E-025 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0131 | G4-WF-011 | 4 | W-E02 | Supplementary Position occupancy | L3-E-014 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0132 | G4-WF-012 | 4 | W-E02 | Delegated authority and additional charge | L3-E-015 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0133 | G4-WF-013 | 4 | W-E02 | Committee membership appointment | L3-E-016 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0134 | G4-WF-014 | 4 | W-E02 | Responsibility Assignment lifecycle | L3-E-017 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0135 | G4-WF-015 | 4 | W-E02 | Reporting Relationship lifecycle | L3-E-018; L3-E-019 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0136 | G4-WF-016 | 4 | W-E02 | Service Jurisdiction lifecycle | L3-E-020; L3-E-021 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0137 | G4-WF-017 | 4 | W-E02 | Organisational authority resolution | L3-E-022 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0138 | G4-WF-018 | 4 | W-E02 | Position occupancy conflict resolution | L3-E-023 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0139 | G4-WF-019 | 4 | W-E02 | Assignment suspension / freeze and restoration | L3-E-024 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0140 | G4-WF-020 | 4 | W-E02 | Position Assignment correction and rollback | L3-E-071; L3-E-072 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0141 | G4-WF-021 | 4 | W-E02 | Bulk Position Assignment operation | L3-E-073 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0142 | G4-WF-022 | 4 | W-E03 | Faculty Workforce lifecycle | L3-E-026; L3-E-031; L3-E-032 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0143 | G4-WF-023 | 4 | W-E03 | Faculty capability lifecycle | L3-E-027 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0144 | G4-WF-024 | 4 | W-E03 | Faculty qualification verification | L3-E-028 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0145 | G4-WF-025 | 4 | W-E03 | Faculty teaching eligibility | L3-E-029 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0146 | G4-WF-026 | 4 | W-E03 | Faculty availability, workload and capacity planning | L3-E-030 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0147 | G4-WF-027 | 4 | W-E04 | Employee onboarding and accepted owner handoffs | L3-E-033 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0148 | G4-WF-028 | 4 | W-E04 | HR leave-case coordination | L3-E-034 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0149 | G4-WF-029 | 4 | W-E04 | HR service and grievance case resolution | L3-E-035; L3-E-036 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0150 | G4-WF-030 | 4 | W-E04 | Employment-change case handoffs | L3-E-037; L3-E-038; L3-E-040 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0151 | G4-WF-031 | 4 | W-E04 | Employee certificate request | L3-E-039 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0152 | G4-WF-032 | 4 | W-E04 | Employee separation case and exit handoff | L3-E-041 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0153 | G4-WF-033 | 4 | W-E04 | HR compliance case | L3-E-042 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0154 | G4-WF-034 | 4 | W-E04 | Probation review and confirmation | L3-E-043 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0155 | G4-WF-035 | 4 | W-E05 | Leave Policy lifecycle | L3-E-044 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0156 | G4-WF-036 | 4 | W-E05 | Leave request decision and cancellation | L3-E-045; L3-E-048 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0157 | G4-WF-037 | 4 | W-E05 | Leave entitlement / balance automatic lifecycle | L3-E-046; L3-E-047; L3-E-050 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0158 | G4-WF-038 | 4 | W-E05 | Governed leave balance adjustment | L3-E-049 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0159 | G4-WF-039 | 4 | W-E05 | Leave encashment eligibility | L3-E-051 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0160 | G4-WF-040 | 4 | W-E05 | Leave exception resolution | L3-E-052 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0161 | G4-WF-041 | 4 | W-E06 | Regular Payroll Run lifecycle | L3-E-053 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0162 | G4-WF-042 | 4 | W-E06 | Payroll adjustment and retroactive pay | L3-E-054; L3-E-055 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0163 | G4-WF-043 | 4 | W-E06 | Final-settlement payroll | L3-E-056 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0164 | G4-WF-044 | 4 | W-E06 | Off-cycle payroll | L3-E-057 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0165 | G4-WF-045 | 4 | W-E06 | Payroll Configuration lifecycle | L3-E-074 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0166 | G4-WF-046 | 4 | W-E07 | Employee Profile establishment and activation | L3-E-058 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0167 | G4-WF-047 | 4 | W-E07 | Employee core-profile maintenance | L3-E-059 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0168 | G4-WF-048 | 4 | W-E07 | Employee qualification and certification verification | L3-E-060; L3-E-061 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0169 | G4-WF-049 | 4 | W-E07 | Employee skills and language profile maintenance | L3-E-062; L3-E-063 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0170 | G4-WF-050 | 4 | W-E07 | Employee Profile document registration and verification | L3-E-064 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0171 | G4-WF-051 | 4 | W-E07 | Employee Profile correction | L3-E-065 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0172 | G4-WF-052 | 4 | W-E07 | Source-owned outcome to Employee Profile synchronization | L3-E-066; L3-E-067 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0173 | G4-WF-053 | 4 | W-E07 | Separation-triggered Employee Profile archival | L3-E-068 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0174 | G4-WF-054 | 4 | W-E07 | Employee Profile suspension and restoration | L3-E-069; L3-E-075 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0175 | G4-WF-055 | 4 | W-E07 | Duplicate Employee Profile merge | L3-E-070 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0176 | G5-WF-001 | 5 | W-F01 | Student Charge establishment, schedule and financial closure | L3-F-001; L3-F-005; L3-F-006 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0177 | G5-WF-002 | 5 | W-F01 | Published Student Charge correction handoff | L3-F-002 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0178 | G5-WF-003 | 5 | W-F01 | Source-owned Student Charge lifecycle consequence | L3-F-003 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0179 | G5-WF-004 | 5 | W-F01 | Student Charge dispute resolution | L3-F-004 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0180 | G5-WF-005 | 5 | W-F02 | Payment receipt and allocation | L3-F-007; L3-F-008 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0181 | G5-WF-006 | 5 | W-F02 | Unidentified / unallocated receipt reconciliation | L3-F-009 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0182 | G5-WF-007 | 5 | W-F02 | Payment failure, timeout, reversal and chargeback recovery | L3-F-010; L3-F-011 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0183 | G5-WF-008 | 5 | W-F02 | Payment cancellation | L3-F-012 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0184 | G5-WF-009 | 5 | W-F02 | Bulk collection outcome | L3-F-013 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0185 | G5-WF-010 | 5 | W-F03 | Refund request, approval, execution and cancellation | L3-F-014; L3-F-015; L3-F-016 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0186 | G5-WF-011 | 5 | W-F03 | Refund failure, reversal, recovery and history | L3-F-017; L3-F-018; L3-F-019 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0187 | G5-WF-012 | 5 | W-F04 | Credit, debit and write-off Financial Adjustment | L3-F-020; L3-F-021; L3-F-022 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0188 | G5-WF-013 | 5 | W-F04 | Financial Adjustment reversal and corrected successor | L3-F-023; L3-F-024 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0189 | G5-WF-014 | 5 | W-F05 | Automatic financial transaction ledger posting | L3-F-025 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0190 | G5-WF-015 | 5 | W-F05 | Manual Journal governance | L3-F-026 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0191 | G5-WF-016 | 5 | W-F05 | Accounting Posting Batch | L3-F-027 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0192 | G5-WF-017 | 5 | W-F05 | Posted Journal reversal | L3-F-028 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0193 | G5-WF-018 | 5 | W-F05 | Ledger reconciliation | L3-F-029 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0194 | G5-WF-019 | 5 | W-F05 | Trial Balance generation | L3-F-030 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0195 | G5-WF-020 | 5 | W-F05 | Accounting close-readiness handoff | L3-F-031 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0196 | G5-WF-021 | 5 | W-F05 | Closed-ledger audit and historical evidence | L3-F-032 | TRACKING-WORKFLOW | NOT-REQUIRED |
| L6-RWF-0197 | G5-WF-022 | 5 | W-F06 | Budget proposal, revision/virement and closure | L3-F-033; L3-F-034; L3-F-038 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0198 | G5-WF-023 | 5 | W-F06 | Budget allocation | L3-F-035 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0199 | G5-WF-024 | 5 | W-F06 | Budget control decision | L3-F-036 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0200 | G5-WF-025 | 5 | W-F06 | Cost Center lifecycle | L3-F-037 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0201 | G5-WF-026 | 5 | W-F07 | Receivable recognition and settlement | L3-F-039; L3-F-041 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0202 | G5-WF-027 | 5 | W-F07 | Receivable dunning / collection governance | L3-F-040 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0203 | G5-WF-028 | 5 | W-F07 | Receivable write-off decision | L3-F-042 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0204 | G5-WF-029 | 5 | W-F07 | Receivable reopening | L3-F-043 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0205 | G5-WF-030 | 5 | W-F08 | Vendor invoice approval and payable settlement | L3-F-044; L3-F-045 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0206 | G5-WF-031 | 5 | W-F08 | Payable dispute / hold resolution | L3-F-046 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0207 | G5-WF-032 | 5 | W-F08 | Payable change incorporation | L3-F-047 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0208 | G5-WF-033 | 5 | W-F08 | Payable suspension, reopening and write-off referral | L3-F-048 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0209 | G5-WF-034 | 5 | W-F08 | Payable reconciliation and historical closure | L3-F-049; L3-F-050 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0210 | G5-WF-035 | 5 | W-F09 | Financial Year / Period establishment and opening | L3-F-051 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0211 | G5-WF-036 | 5 | W-F09 | Financial Period close, reopening/reclose and archival | L3-F-052; L3-F-054; L3-F-055 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0212 | G5-WF-037 | 5 | W-F09 | Financial Year end closure | L3-F-053 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0213 | G5-WF-038 | 5 | W-F10 | Treasury bank-account lifecycle | L3-F-056 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0214 | G5-WF-039 | 5 | W-F10 | Approved payment population bank execution | L3-F-057 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0215 | G5-WF-040 | 5 | W-F10 | Bank / cash reconciliation | L3-F-058 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0216 | G5-WF-041 | 5 | W-F10 | Treasury liquidity decision | L3-F-059 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0217 | G5-WF-042 | 5 | W-F10 | Inter-account fund transfer | L3-F-060 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0218 | G5-WF-043 | 5 | W-F10 | Failed / returned / uncertain bank transaction resolution | L3-F-061 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0219 | G5-WF-044 | 5 | W-F11 | Financial Aid allocation, renewal and revocation | L3-F-062; L3-F-064; L3-F-065 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0220 | G5-WF-045 | 5 | W-F11 | Financial Aid disbursement / charge consequence | L3-F-063 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0221 | G5-WF-046 | 5 | W-F11 | Aid funding and utilization reconciliation | L3-F-066 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0222 | G5-WF-047 | 5 | W-F12 | Financial Asset capitalization | L3-F-067 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0223 | G5-WF-048 | 5 | W-F12 | Asset depreciation, revaluation and impairment | L3-F-068; L3-F-069; L3-F-070 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0224 | G5-WF-049 | 5 | W-F12 | Asset disposal and financial write-off accounting | L3-F-071; L3-F-072 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0225 | G5-WF-050 | 5 | W-F13 | Official Financial Report lifecycle and disposition | L3-F-073; L3-F-078 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0226 | G5-WF-051 | 5 | W-F13 | Statutory financial-reporting handoff | L3-F-074 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0227 | G5-WF-052 | 5 | W-F13 | Management financial reporting | L3-F-075 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0228 | G5-WF-053 | 5 | W-F13 | KPI Definition / Analytical Model version governance | L3-F-076 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0229 | G5-WF-054 | 5 | W-F13 | Financial KPI and dashboard projection | L3-F-077 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0230 | G5-WF-055 | 5 | W-F13 | Financial report audit and historical reconstruction | L3-F-079 | TRACKING-WORKFLOW | NOT-REQUIRED |
| L6-RWF-0231 | G5-WF-056 | 5 | W-F14 | Procurement budget commitment | L3-F-080 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0232 | G5-WF-057 | 5 | W-F14 | Supplier invoice match, vendor liability and Finance closure | L3-F-081; L3-F-082; L3-F-083 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0233 | G5-WF-058 | 5 | W-F00 | Finance Configuration lifecycle | L3-F-084 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0234 | G5-WF-059 | 5 | W-F00 | Financial Rule governance lifecycle | L3-F-085 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0235 | G6-WF-001 | 6 | W-G00 | Campus readiness assessment and governance decision | L3-G-001 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0236 | G6-WF-002 | 6 | W-G00 | Accreditation evidence preparation and H09 handoff | L3-G-002; L3-G-015 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0237 | G6-WF-003 | 6 | W-G00 | Campus Operational Review lifecycle | L3-G-003 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0238 | G6-WF-004 | 6 | W-G00 | Executive governance review and recommendation | L3-G-004 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0239 | G6-WF-005 | 6 | W-G00 | Operational Risk lifecycle | L3-G-005 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0240 | G6-WF-006 | 6 | W-G00 | Multi-Domain Action Plan lifecycle | L3-G-006 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0241 | G6-WF-007 | 6 | W-G00 | Multi-domain SLA deviation governance | L3-G-007 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0242 | G6-WF-008 | 6 | W-G00 | Campus Operational Health assessment | L3-G-008 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0243 | G6-WF-009 | 6 | W-G00 | Governance KPI review | L3-G-009 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0244 | G6-WF-010 | 6 | W-G00 | Operational Maturity Assessment | L3-G-010 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0245 | G6-WF-011 | 6 | W-G00 | Governance Finding and corrective-action decision | L3-G-011 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0246 | G6-WF-012 | 6 | W-G00 | Executive Governance Resolution | L3-G-012 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0247 | G6-WF-013 | 6 | W-G00 | Governance evidence/dependency exception recovery | L3-G-013 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0248 | G6-WF-014 | 6 | W-G00 | Governance configuration lifecycle | L3-G-014 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0249 | G6-WF-015 | 6 | W-G01 | Facility Service Request and administrative closure | L3-G-016; L3-G-046 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0250 | G6-WF-016 | 6 | W-G01 | Facility Complaint lifecycle, duplicate consolidation and split work | L3-G-017; L3-G-018; L3-G-019 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0251 | G6-WF-017 | 6 | W-G01 | Work Order lifecycle, reopening, reassignment and replanning | L3-G-020; L3-G-021; L3-G-022; L3-G-047; L3-G-048 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0252 | G6-WF-018 | 6 | W-G01 | Preventive Maintenance cycle, deferral and missed-cycle recovery | L3-G-023; L3-G-024; L3-G-025 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0253 | G6-WF-019 | 6 | W-G01 | Corrective maintenance, root-cause and repeat-failure improvement | L3-G-026; L3-G-027; L3-G-028 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0254 | G6-WF-020 | 6 | W-G01 | Emergency maintenance, incident intervention and post-incident review | L3-G-029; L3-G-030; L3-G-045 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0255 | G6-WF-021 | 6 | W-G01 | Vendor assignment, service failure and performance evaluation | L3-G-031; L3-G-032; L3-G-033 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0256 | G6-WF-022 | 6 | W-G01 | AMC service execution, failure and renewal action | L3-G-034; L3-G-035; L3-G-036 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0257 | G6-WF-023 | 6 | W-G01 | Utility monitoring, outage and anomaly recovery | L3-G-037; L3-G-038; L3-G-039 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0258 | G6-WF-024 | 6 | W-G01 | Facility Inspection lifecycle | L3-G-040 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0259 | G6-WF-025 | 6 | W-G01 | Facility SLA governance and operational escalation | L3-G-041; L3-G-042 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0260 | G6-WF-026 | 6 | W-G01 | Maintenance consumable and operational-cost history | L3-G-043; L3-G-044 | TRACKING-WORKFLOW | NOT-REQUIRED |
| L6-RWF-0261 | G6-WF-027 | 6 | W-G01 | Facilities operational-configuration governance | L3-G-049 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0262 | G6-WF-028 | 6 | W-G02 | Laboratory readiness, warning and availability recovery | L3-G-050; L3-G-051; L3-G-052 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0263 | G6-WF-029 | 6 | W-G02 | Practical Session operational lifecycle | L3-G-053; L3-G-054; L3-G-055 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0264 | G6-WF-030 | 6 | W-G02 | Laboratory scheduling, batch allocation and reallocation | L3-G-056; L3-G-057; L3-G-058; L3-G-059; L3-G-074 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0265 | G6-WF-031 | 6 | W-G02 | Equipment operational status, restriction and usage | L3-G-060; L3-G-061; L3-G-062 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0266 | G6-WF-032 | 6 | W-G02 | Laboratory safety verification, warning and emergency stabilization | L3-G-063; L3-G-064; L3-G-075 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0267 | G6-WF-033 | 6 | W-G02 | Laboratory Incident resolution | L3-G-065 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0268 | G6-WF-034 | 6 | W-G02 | Laboratory incident maintenance-recommendation handoff to G01 | L3-G-066 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0269 | G6-WF-035 | 6 | W-G02 | Laboratory Inspection, findings and controlled follow-up | L3-G-067; L3-G-068; L3-G-076 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0270 | G6-WF-036 | 6 | W-G02 | Laboratory Compliance and non-compliance resolution | L3-G-069; L3-G-070 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0271 | G6-WF-037 | 6 | W-G02 | Laboratory consumable-usage history | L3-G-071 | TRACKING-WORKFLOW | NOT-REQUIRED |
| L6-RWF-0272 | G6-WF-038 | 6 | W-G02 | Laboratory operational-restriction / utility-failure recovery | L3-G-072; L3-G-073 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0273 | G6-WF-039 | 6 | W-G02 | Laboratory operational-configuration governance | L3-G-077 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0274 | G7-WF-001 | 7 | W-G03 | Hostel admission, waitlist, residency activation and closure lifecycle | L3-G-078; L3-G-079; L3-G-080; L3-G-081 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0275 | G7-WF-002 | 7 | W-G03 | Resident leave and verified-return lifecycle | L3-G-082 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0276 | G7-WF-003 | 7 | W-G03 | Curfew exception and authorised-late-entry decision | L3-G-083 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0277 | G7-WF-004 | 7 | W-G03 | Hostel visitor request, visit and closure | L3-G-084 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0278 | G7-WF-005 | 7 | W-G03 | Hostel discipline and curfew-violation resolution | L3-G-085; L3-G-086 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0279 | G7-WF-006 | 7 | W-G03 | Room transfer and reallocation | L3-G-087 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0280 | G7-WF-007 | 7 | W-G03 | Hostel complaint review, resolution and closure | L3-G-088 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0281 | G7-WF-008 | 7 | W-G03 | Hostel room inspection and damage resolution | L3-G-089 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0282 | G7-WF-009 | 7 | W-G03 | Mess allocation and reallocation lifecycle | L3-G-090; L3-G-091 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0283 | G7-WF-010 | 7 | W-G03 | Hostel operational-configuration governance | L3-G-092 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0284 | G7-WF-011 | 7 | W-G04 | Transport Route creation, modification, sequencing, suspension and permanent revision | L3-G-093; L3-G-094; L3-G-095; L3-G-098; L3-G-119 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0285 | G7-WF-012 | 7 | W-G04 | Transport Stop creation and suspension lifecycle | L3-G-096; L3-G-097 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0286 | G7-WF-013 | 7 | W-G04 | Transport allocation assignment, change and withdrawal | L3-G-099; L3-G-100; L3-G-101 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0287 | G7-WF-014 | 7 | W-G04 | Driver assignment and replacement | L3-G-102; L3-G-103 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0288 | G7-WF-015 | 7 | W-G04 | Trip planning, readiness and pre-execution cancellation | L3-G-104; L3-G-105 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0289 | G7-WF-016 | 7 | W-G04 | Trip execution, route deviation and completion | L3-G-106; L3-G-107 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0290 | G7-WF-017 | 7 | W-G04 | Vehicle operational assignment and breakdown replacement | L3-G-108; L3-G-109 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0291 | G7-WF-018 | 7 | W-G04 | Vehicle operational issue to G01 maintenance recommendation | L3-G-110 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0292 | G7-WF-019 | 7 | W-G04 | Pre-trip and post-trip operational checks | L3-G-111; L3-G-112 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0293 | G7-WF-020 | 7 | W-G04 | Transport Pass issue, renewal, replacement and suspension lifecycle | L3-G-113; L3-G-114; L3-G-115; L3-G-116 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0294 | G7-WF-021 | 7 | W-G04 | Emergency route diversion and temporary-stop adjustment | L3-G-117; L3-G-118 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0295 | G7-WF-022 | 7 | W-G04 | Transport Complaint review and corrective-resolution lifecycle | L3-G-120; L3-G-121; L3-G-122 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0296 | G7-WF-023 | 7 | W-G04 | Transport operational-configuration governance | L3-G-123 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0297 | G7-WF-024 | 7 | W-G05 | Library Membership activation, suspension and closure lifecycle | L3-G-124; L3-G-125; L3-G-126 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0298 | G7-WF-025 | 7 | W-G05 | Catalogue creation, review, publication and update lifecycle | L3-G-127; L3-G-128 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0299 | G7-WF-026 | 7 | W-G05 | Library Copy registration and condition restriction lifecycle | L3-G-129; L3-G-130 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0300 | G7-WF-027 | 7 | W-G05 | Circulation issue, return and renewal lifecycle | L3-G-131; L3-G-132; L3-G-133 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0301 | G7-WF-028 | 7 | W-G05 | Reservation, availability/collection and expiry lifecycle | L3-G-134; L3-G-135; L3-G-136 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0302 | G7-WF-029 | 7 | W-G05 | Overdue fine and waiver governance | L3-G-137; L3-G-138 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0303 | G7-WF-030 | 7 | W-G05 | Lost or damaged library-resource recovery | L3-G-139; L3-G-140 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0304 | G7-WF-031 | 7 | W-G05 | Library acquisition approval, receipt and catalogued-copy establishment | L3-G-141; L3-G-142 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0305 | G7-WF-032 | 7 | W-G05 | Reading Room opening, usage and operational closure | L3-G-143; L3-G-144; L3-G-145 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0306 | G7-WF-033 | 7 | W-G05 | Stock verification, discrepancy investigation and administrative review | L3-G-146; L3-G-147; L3-G-148 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0307 | G7-WF-034 | 7 | W-G05 | Library clearance approval and institutional-exit hand-off | L3-G-149; L3-G-150 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0308 | G7-WF-035 | 7 | W-G05 | Resource-availability conflict and alternative resolution | L3-G-151 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0309 | G7-WF-036 | 7 | W-G05 | Borrowing restriction administrative resolution | L3-G-152 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0310 | G7-WF-037 | 7 | W-G05 | Library operational-configuration governance | L3-G-153 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0311 | G7-WF-038 | 7 | W-G05 | Library operational reporting and governance review | L3-G-154 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0312 | G8-WF-001 | 8 | W-G06 | Health Centre configuration, activation and temporary-closure lifecycle | L3-G-155; L3-G-156 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0313 | G8-WF-002 | 8 | W-G06 | Patient/medical-encounter registration, identity exception, readiness, consultation and follow-up | L3-G-157; L3-G-158; L3-G-159; L3-G-160; L3-G-161 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0314 | G8-WF-003 | 8 | W-G06 | Emergency/first-aid response, external referral and ambulance coordination | L3-G-162; L3-G-163; L3-G-164 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0315 | G8-WF-004 | 8 | W-G06 | Prescription creation | L3-G-165 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0316 | G8-WF-005 | 8 | W-G06 | Medicine dispensing and availability-exception resolution | L3-G-166; L3-G-167 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0317 | G8-WF-006 | 8 | W-G06 | Medicine inventory condition assessment and medical-operations alert | L3-G-168 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0318 | G8-WF-007 | 8 | W-G06 | Observation-stay admission/completion and capacity-exception resolution | L3-G-169; L3-G-170 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0319 | G8-WF-008 | 8 | W-G06 | Medical referral creation/completion and institutional follow-up | L3-G-171; L3-G-172 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0320 | G8-WF-009 | 8 | W-G06 | Medical certificate issuance and revocation | L3-G-173; L3-G-174 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0321 | G8-WF-010 | 8 | W-G06 | Health programme planning/completion and screening/camp follow-up | L3-G-175; L3-G-178 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0322 | G8-WF-011 | 8 | W-G06 | Vaccination campaign and recorded vaccination outcome | L3-G-176 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0323 | G8-WF-012 | 8 | W-G06 | Annual health-checkup assessment | L3-G-177 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0324 | G8-WF-013 | 8 | W-G06 | Occupational-health assessment and medical outcome | L3-G-179 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0325 | G8-WF-014 | 8 | W-G06 | Medical-event disclosure decision and Student/Employee Portal hand-off | L3-G-180; L3-G-181 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0326 | G8-WF-015 | 8 | W-G06 | Medical-professional unavailability and alternative operational assignment | L3-G-182 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0327 | G8-WF-016 | 8 | W-G06 | Medical operational-configuration governance | L3-G-183 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0328 | G8-WF-017 | 8 | W-G06 | Healthcare operational reporting and governance review | L3-G-184 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0329 | G8-WF-018 | 8 | W-G06 | Medical clearance decision | L3-G-185 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0330 | G8-WF-019 | 8 | W-G07 | Security Office activation and temporary-closure lifecycle | L3-G-186; L3-G-187 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0331 | G8-WF-020 | 8 | W-G07 | Gate configuration/open/close and person-access decision/override | L3-G-188; L3-G-189; L3-G-190; L3-G-191 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0332 | G8-WF-021 | 8 | W-G07 | Visitor/courier/contractor access, check-in, closure and exit | L3-G-192; L3-G-193; L3-G-196; L3-G-197 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0333 | G8-WF-022 | 8 | W-G07 | Vehicle campus-movement lifecycle | L3-G-194 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0334 | G8-WF-023 | 8 | W-G07 | Material gate-pass request, decision and movement | L3-G-195 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0335 | G8-WF-024 | 8 | W-G07 | Patrol execution and exception escalation | L3-G-198 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0336 | G8-WF-025 | 8 | W-G07 | Security shift handover and continuity | L3-G-199 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0337 | G8-WF-026 | 8 | W-G07 | Security incident, emergency, missing-person and fire/safety response | L3-G-200; L3-G-201; L3-G-203; L3-G-204 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0338 | G8-WF-027 | 8 | W-G07 | Medical-security incident hand-off to G06 | L3-G-202 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0339 | G8-WF-028 | 8 | W-G07 | Lost & Found report, registration, claim, handover and unclaimed disposition | L3-G-205; L3-G-206; L3-G-207; L3-G-208; L3-G-209 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0340 | G8-WF-029 | 8 | W-G07 | Escort request, assignment, completion and exception response | L3-G-210; L3-G-211; L3-G-212 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0341 | G8-WF-030 | 8 | W-G07 | Security policy configuration governance | L3-G-213 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0342 | G8-WF-031 | 8 | W-G07 | Security delegation to temporary operational authority | L3-G-214 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0343 | G8-WF-032 | 8 | W-G07 | Security operational reporting and dashboard governance | L3-G-215 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0344 | G8-WF-033 | 8 | W-G08 | Housekeeping Zone configuration, change and deactivation lifecycle | L3-G-216; L3-G-217; L3-G-218 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0345 | G8-WF-034 | 8 | W-G08 | Cleaning schedule, assignment and recurring/deep/event/emergency/special task execution | L3-G-219; L3-G-220; L3-G-221; L3-G-223; L3-G-224; L3-G-225; L3-G-226 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0346 | G8-WF-035 | 8 | W-G08 | Cleaning complaint, repeat-corrective and request-SLA escalation lifecycle | L3-G-227; L3-G-228; L3-G-229; L3-G-230 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0347 | G8-WF-036 | 8 | W-G08 | Hygiene inspection and compliance/corrective outcome | L3-G-231 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0348 | G8-WF-037 | 8 | W-G08 | Consumable shortage/availability monitoring, alert and continuity recovery | L3-G-232; L3-G-241; L3-G-242 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0349 | G8-WF-038 | 8 | W-G08 | Quality inspection, re-clean and cleanliness-improvement governance | L3-G-222; L3-G-233; L3-G-247; L3-G-248 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0350 | G8-WF-039 | 8 | W-G08 | Housekeeping incident resolution and escalation | L3-G-234; L3-G-236 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0351 | G8-WF-040 | 8 | W-G08 | Facility issue to G01 maintenance hand-off | L3-G-235 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0352 | G8-WF-041 | 8 | W-G08 | Waste collection, exception, segregation and specialised external-handling coordination | L3-G-237; L3-G-238; L3-G-239; L3-G-240 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0353 | G8-WF-042 | 8 | W-G08 | Vendor operational assignment and service-performance outcome | L3-G-243; L3-G-244 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0354 | G8-WF-043 | 8 | W-G08 | Housekeeping SLA monitoring and vendor-breach corrective governance | L3-G-245; L3-G-246 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0355 | G8-WF-044 | 8 | W-G08 | Housekeeping policy configuration governance | L3-G-249 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0356 | G8-WF-045 | 8 | W-G08 | Housekeeping delegation to temporary operational authority | L3-G-250 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0357 | G8-WF-046 | 8 | W-G08 | Housekeeping operational reporting and dashboard governance | L3-G-251 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0358 | G9-WF-001 | 9 | W-G09 | Mentorship relationship activation, assignment change/review and workspace readiness | L3-G-252; L3-G-253; L3-G-254; L3-G-255; L3-G-256 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0359 | G9-WF-002 | 9 | W-G09 | Scheduled/unscheduled mentoring session, context readiness/continuation and completion | L3-G-257; L3-G-258; L3-G-259; L3-G-260; L3-G-261 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0360 | G9-WF-003 | 9 | W-G09 | Parent-interaction requirement, discussion and follow-up | L3-G-262; L3-G-263; L3-G-264 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0361 | G9-WF-004 | 9 | W-G09 | Mentoring issue to governed institutional referral | L3-G-265 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0362 | G9-WF-005 | 9 | W-G09 | Mentoring follow-up cycle and progress outcome | L3-G-266 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0363 | G9-WF-006 | 9 | W-G09 | Mentorship case closure, reassignment/exit closure and pending-follow-up resolution | L3-G-267; L3-G-268; L3-G-269; L3-G-270; L3-G-271 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0364 | G9-WF-007 | 9 | W-G09 | Mentorship record correction and audited successor | L3-G-272 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0365 | G9-WF-008 | 9 | W-G09 | Mentoring information disclosure decision | L3-G-273 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0366 | G9-WF-009 | 9 | W-G09 | Mentorship timeline and student/management reporting | L3-G-274; L3-G-275; L3-G-276 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0367 | G9-WF-010 | 9 | W-G09 | Mentorship extension configuration governance | L3-G-277 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0368 | G9-WF-011 | 9 | W-G10 | Physical Campus registration, institutional reference association and governed attribute update | L3-G-278; L3-G-279; L3-G-280 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0369 | G9-WF-012 | 9 | W-G10 | Physical hierarchy creation — Building, Floor, Space and Zone | L3-G-281; L3-G-282; L3-G-283; L3-G-284; L3-G-285 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0370 | G9-WF-013 | 9 | W-G10 | Space classification and operational-capability governance | L3-G-286; L3-G-287 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0371 | G9-WF-014 | 9 | W-G10 | Capacity and availability profile definition/change | L3-G-288; L3-G-289; L3-G-290 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0372 | G9-WF-015 | 9 | W-G10 | Infrastructure reservation request, reschedule and cancellation | L3-G-291; L3-G-292; L3-G-293 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0373 | G9-WF-016 | 9 | W-G10 | Infrastructure allocation establishment and controlled reallocation | L3-G-294; L3-G-295 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0374 | G9-WF-017 | 9 | W-G10 | Operational responsibility assignment and transfer | L3-G-296; L3-G-297 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0375 | G9-WF-018 | 9 | W-G10 | Infrastructure validation and governed administrative update | L3-G-298; L3-G-299 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0376 | G9-WF-019 | 9 | W-G10 | Compliance registration, status update and restriction | L3-G-300; L3-G-301; L3-G-302 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0377 | G9-WF-020 | 9 | W-G10 | Infrastructure inspection and risk-reviewed closure | L3-G-303 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0378 | G9-WF-021 | 9 | W-G10 | Preventive/corrective/emergency maintenance requirement and G01 completion reconciliation | L3-G-304; L3-G-305; L3-G-306; L3-G-307 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0379 | G9-WF-022 | 9 | W-G10 | Capital renovation proposal, governance and physical reconciliation | L3-G-308; L3-G-309 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0380 | G9-WF-023 | 9 | W-G10 | Infrastructure sustainability governance | L3-G-310 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0381 | G9-WF-024 | 9 | W-G10 | Physical infrastructure security classification | L3-G-311 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0382 | G9-WF-025 | 9 | W-G10 | Disaster-recovery requirement and controlled recovery-state governance | L3-G-312; L3-G-313 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0383 | G9-WF-026 | 9 | W-G10 | Infrastructure lifecycle initiation and activation | L3-G-314 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0384 | G9-WF-027 | 9 | W-G10 | Infrastructure decommissioning and controlled archival | L3-G-315; L3-G-316 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0385 | G9-WF-028 | 9 | W-G10 | Infrastructure configuration-change governance | L3-G-317 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0386 | G9-WF-029 | 9 | W-G10 | Physical Campus boundary change and institutional-context collaboration | L3-G-318 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0387 | G9-WF-030 | 9 | W-G10 | Infrastructure correction and audited historical amendment | L3-G-319 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0388 | G9-WF-031 | 9 | W-G10 | Infrastructure portfolio reporting and dashboard governance | L3-G-320 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0389 | G10-WF-001 | 10 | W-H01 | Learner-account linking | L3-H-001 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0390 | G10-WF-002 | 10 | W-H01 | Credential publication, revocation, republication, bulk execution and recovery | L3-H-002; L3-H-004; L3-H-005; L3-H-006; L3-H-008 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0391 | G10-WF-003 | 10 | W-H01 | Government publication consent lifecycle | L3-H-003 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0392 | G10-WF-004 | 10 | W-H01 | Institutional/government credential reconciliation | L3-H-007 | EXCEPTION-RECOVERY-WORKFLOW | NOT-REQUIRED |
| L6-RWF-0393 | G10-WF-005 | 10 | W-H01 | Government connector lifecycle | L3-H-009 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0394 | G10-WF-006 | 10 | W-H02 | Notification scheduling, delivery, emergency routing, recovery and provider evidence | L3-H-010; L3-H-011; L3-H-014; L3-H-015; L3-H-016 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0395 | G10-WF-007 | 10 | W-H02 | Notification template governance | L3-H-012 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0396 | G10-WF-008 | 10 | W-H02 | Recipient preference/subscription governance | L3-H-013 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0397 | G10-WF-009 | 10 | W-H03 | Related-enterprise discovery | L3-H-017 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0398 | G10-WF-010 | 10 | W-H03 | Search projection synchronization, recovery and governed onboarding | L3-H-018; L3-H-019; L3-H-020 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0399 | G10-WF-011 | 10 | W-H03 | Enterprise Search configuration governance | L3-H-021 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0400 | G10-WF-012 | 10 | W-H04 | Employer registration and participation | L3-H-022 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0401 | G10-WF-013 | 10 | W-H04 | Placement Drive publication, disruption and closure | L3-H-023; L3-H-027 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0402 | G10-WF-014 | 10 | W-H04 | Student application, selection, offer and joining | L3-H-024; L3-H-025 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0403 | G10-WF-015 | 10 | W-H04 | Internship lifecycle and verified completion | L3-H-026 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0404 | G10-WF-016 | 10 | W-H04 | Placement institutional outcome reporting | L3-H-028 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0405 | G10-WF-017 | 10 | W-H05 | Research proposal/project governance and closure | L3-H-029; L3-H-030; L3-H-031 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0406 | G10-WF-018 | 10 | W-H05 | Research Grant governance and closure | L3-H-032 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0407 | G10-WF-019 | 10 | W-H05 | Research ethics and continuing compliance | L3-H-033 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0408 | G10-WF-020 | 10 | W-H05 | Research publication governance and archival | L3-H-034 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0409 | G10-WF-021 | 10 | W-H05 | Research IP disclosure/protection/closure | L3-H-035 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0410 | G10-WF-022 | 10 | W-H05 | Research collaboration lifecycle | L3-H-036 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0411 | G10-WF-023 | 10 | W-H05 | Consultancy delivery and reconciliation | L3-H-037 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0412 | G10-WF-024 | 10 | W-H05 | Research portfolio governance/reporting | L3-H-038 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0413 | G10-WF-025 | 10 | W-H06 | Student-to-Alumni handoff and Alumni relationship establishment | L3-H-039 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0414 | G10-WF-026 | 10 | W-H06 | Alumni membership lifecycle | L3-H-040 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0415 | G10-WF-027 | 10 | W-H06 | Alumni profile/preference governance | L3-H-041 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0416 | G10-WF-028 | 10 | W-H06 | Alumni Chapter lifecycle | L3-H-042 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0417 | G10-WF-029 | 10 | W-H06 | Alumni networking relationship | L3-H-043 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0418 | G10-WF-030 | 10 | W-H06 | Alumni mentoring engagement | L3-H-044 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0419 | G10-WF-031 | 10 | W-H06 | Alumni contribution/engagement lifecycle | L3-H-045 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0420 | G10-WF-032 | 10 | W-H06 | Alumni Recognition governance | L3-H-046 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0421 | G10-WF-033 | 10 | W-H06 | Alumni donation intent to Finance-confirmed contribution | L3-H-047 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0422 | G10-WF-034 | 10 | W-H07 | Dashboard publication/change/retirement/restoration lifecycle | L3-H-048; L3-H-052; L3-H-053 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0423 | G10-WF-035 | 10 | W-H07 | Dashboard Template governance | L3-H-049 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0424 | G10-WF-036 | 10 | W-H07 | Dashboard workspace assignment | L3-H-050 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0425 | G10-WF-037 | 10 | W-H07 | Dashboard sharing/access governance | L3-H-051 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0426 | G10-WF-038 | 10 | W-H07 | Widget Definition governance | L3-H-054 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0427 | G10-WF-039 | 10 | W-H08 | Regulatory Reporting Framework Definition governance | L3-H-055 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0428 | G10-WF-040 | 10 | W-H08 | Reporting Cycle planning, snapshot, approval, submission and closure | L3-H-056; L3-H-057; L3-H-058; L3-H-059; L3-H-060 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0429 | G10-WF-041 | 10 | W-H09 | Accreditation Framework governance | L3-H-061 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0430 | G10-WF-042 | 10 | W-H09 | Accreditation Cycle, evidence, assessment, readiness and SSR handoff | L3-H-062; L3-H-064; L3-H-066; L3-H-069; L3-H-071 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0431 | G10-WF-043 | 10 | W-H09 | Compliance Requirement lifecycle and periodic review | L3-H-063; L3-H-070 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0432 | G10-WF-044 | 10 | W-H09 | Internal Audit lifecycle | L3-H-065 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0433 | G10-WF-045 | 10 | W-H09 | CAPA remediation, verification and closure | L3-H-067 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0434 | G10-WF-046 | 10 | W-H09 | Quality Improvement lifecycle | L3-H-068 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0435 | G10-WF-047 | 10 | W-H10 | External Education Provider lifecycle | L3-H-072 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0436 | G10-WF-048 | 10 | W-H10 | External Connector configuration lifecycle | L3-H-073 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0437 | G10-WF-049 | 10 | W-H10 | External Provider Trust lifecycle | L3-H-074 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0438 | G10-WF-050 | 10 | W-H10 | Synchronization, progress exchange and discrepancy reconciliation | L3-H-075; L3-H-077; L3-H-080 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0439 | G10-WF-051 | 10 | W-H10 | External Enrollment Exchange | L3-H-076 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0440 | G10-WF-052 | 10 | W-H10 | External Credit request to canonical Academic decision | L3-H-078 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0441 | G10-WF-053 | 10 | W-H10 | External Credential Verification | L3-H-079 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0442 | G10-WF-054 | 10 | W-H10 | Governed external education data exchange | L3-H-081 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0443 | G11-WF-001 | 11 | W-I01 | Examination Policy definition, revision, activation and retirement | L3-I-001; L3-I-002; L3-I-003 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0444 | G11-WF-002 | 11 | W-I02 | Examination Session creation and governed revision | L3-I-004; L3-I-005 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0445 | G11-WF-003 | 11 | W-I02 | Duplicate Examination Session resolution | L3-I-006 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0446 | G11-WF-004 | 11 | W-I03 | Examination Schedule planning, revision, publication and withdrawal | L3-I-007; L3-I-008; L3-I-013; L3-I-016 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0447 | G11-WF-005 | 11 | W-I03 | Examination venue allocation and seating arrangement | L3-I-009; L3-I-015 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0448 | G11-WF-006 | 11 | W-I03 | Invigilation assignment and shortage resolution | L3-I-010; L3-I-011 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0449 | G11-WF-007 | 11 | W-I03 | Practical/Viva examiner allocation | L3-I-012 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0450 | G11-WF-008 | 11 | W-I03 | Examination scheduling conflict resolution | L3-I-014 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0451 | G11-WF-009 | 11 | W-I04 | Candidate, assisted, late and administrative registration | L3-I-017; L3-I-018; L3-I-019; L3-I-034 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0452 | G11-WF-010 | 11 | W-I04 | Registration deadline override and reopening | L3-I-020; L3-I-025 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0453 | G11-WF-011 | 11 | W-I04 | Registration amendment, error, duplicate, scope conflict and transfer resolution | L3-I-021; L3-I-022; L3-I-027; L3-I-032; L3-I-033; L3-I-035 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0454 | G11-WF-012 | 11 | W-I04 | Registration withdrawal, reinstatement and opportunity-change withdrawal | L3-I-023; L3-I-024; L3-I-031 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0455 | G11-WF-013 | 11 | W-I04 | Registration rejection/ineligibility decision | L3-I-026 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0456 | G11-WF-014 | 11 | W-I04 | Special-permission and special-arrangement registration | L3-I-028; L3-I-029 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0457 | G11-WF-015 | 11 | W-I04 | Supplementary/backlog/improvement registration | L3-I-030 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0458 | G11-WF-016 | 11 | W-I04 | Registration population finalization | L3-I-036 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0459 | G11-WF-017 | 11 | W-I05 | Hall Ticket issuance and population finalization | L3-I-037; L3-I-042 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0460 | G11-WF-018 | 11 | W-I05 | Hall Ticket withheld-entry decision | L3-I-038 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0461 | G11-WF-019 | 11 | W-I05 | Hall Ticket invalidation following registration withdrawal | L3-I-039 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0462 | G11-WF-020 | 11 | W-I05 | Special-arrangement Hall Ticket governance | L3-I-040 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0463 | G11-WF-021 | 11 | W-I05 | Duplicate/conflicting Hall Ticket resolution | L3-I-041 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0464 | G11-WF-022 | 11 | W-I06 | Practical/Viva conduct, special arrangement, re-conduct and panel viva | L3-I-043; L3-I-044; L3-I-047; L3-I-050 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0465 | G11-WF-023 | 11 | W-I06 | Non-theory entry/non-participation and identity decision | L3-I-045; L3-I-046 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0466 | G11-WF-024 | 11 | W-I06 | Practical resource and batch-allocation conflict resolution | L3-I-048; L3-I-049 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0467 | G11-WF-025 | 11 | W-I06 | Mid-conduct examiner replacement | L3-I-051 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0468 | G11-WF-026 | 11 | W-I06 | Medical interruption and candidate withdrawal during non-theory conduct | L3-I-052; L3-I-053 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0469 | G11-WF-027 | 11 | W-I06 | Non-theory conduct record correction | L3-I-054 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0470 | G11-WF-028 | 11 | W-I07 | Theory conduct, multi-centre conduct, re-conduct and special arrangement | L3-I-055; L3-I-056; L3-I-061; L3-I-062 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0471 | G11-WF-029 | 11 | W-I07 | Theory entry/non-participation and identity decision | L3-I-057; L3-I-058 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0472 | G11-WF-030 | 11 | W-I07 | Theory seating-conflict resolution | L3-I-059 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0473 | G11-WF-031 | 11 | W-I07 | Mid-conduct invigilator replacement | L3-I-060 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0474 | G11-WF-032 | 11 | W-I07 | Question-paper unavailability, compromise and conduct-issue disposition | L3-I-063; L3-I-064; L3-I-065 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0475 | G11-WF-033 | 11 | W-I07 | Answer-script collection/custody discrepancy resolution | L3-I-066; L3-I-067 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0476 | G11-WF-034 | 11 | W-I07 | Medical interruption and candidate withdrawal during theory conduct | L3-I-068; L3-I-069 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0477 | G11-WF-035 | 11 | W-I07 | Theory conduct record correction | L3-I-070 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0478 | G11-WF-036 | 11 | W-I08 | Marks collection, evaluation and final approval | L3-I-071; L3-I-072; L3-I-073 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0479 | G11-WF-037 | 11 | W-I08 | Marks submission exception resolution | L3-I-074 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0480 | G11-WF-038 | 11 | W-I08 | Marks moderation under approved policy | L3-I-075 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0481 | G11-WF-039 | 11 | W-I08 | Incomplete evaluation and withheld approval | L3-I-076 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0482 | G11-WF-040 | 11 | W-I08 | Supplementary/improvement, re-conduct and special-arrangement evaluation | L3-I-077; L3-I-078; L3-I-079 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0483 | G11-WF-041 | 11 | W-I08 | Second evaluation, evaluation-difference and adjudicating evaluation | L3-I-080; L3-I-081; L3-I-082 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0484 | G12-WF-001 | 12 | W-I09 | Result computation, recomputation, moderation and policy-governed computation | L3-I-083; L3-I-084; L3-I-087; L3-I-089; L3-I-090; L3-I-091 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0485 | G12-WF-002 | 12 | W-I09 | Result-computation blocking and governed resumption | L3-I-085; L3-I-086 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0486 | G12-WF-003 | 12 | W-I09 | Approved-marks correction and recomputed result | L3-I-088 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0487 | G12-WF-004 | 12 | W-I09 | Result-processing identity mismatch handoff and resume | L3-I-092 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0488 | G12-WF-005 | 12 | W-I09 | Duplicate/conflicting result-computation reconciliation | L3-I-093; L3-I-094 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0489 | G12-WF-006 | 12 | W-I09 | Malpractice consequence application in result processing | L3-I-095 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0490 | G12-WF-007 | 12 | W-I09 | Examination-exception resolution to post-resolution result | L3-I-096 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0491 | G12-WF-008 | 12 | W-I09 | Individual-result approval blocking and release | L3-I-097; L3-I-098 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0492 | G12-WF-009 | 12 | W-I09 | Result reopening, correction and pre-publication replacement | L3-I-099; L3-I-100 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0493 | G12-WF-010 | 12 | W-I10 | Official result publication | L3-I-101 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0494 | G12-WF-011 | 12 | W-I10 | Publication blocking and withheld-result release | L3-I-102; L3-I-103 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0495 | G12-WF-012 | 12 | W-I10 | Publication delay and delay resolution | L3-I-104; L3-I-105 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0496 | G12-WF-013 | 12 | W-I10 | Pending publication cancellation | L3-I-106 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0497 | G12-WF-014 | 12 | W-I10 | Post-publication correction, replacement, republication and discrepancy correction | L3-I-107; L3-I-108; L3-I-117 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0498 | G12-WF-015 | 12 | W-I10 | Published-result withdrawal and restoration | L3-I-109; L3-I-110 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0499 | G12-WF-016 | 12 | W-I10 | Revised publication after revaluation/rechecking | L3-I-111 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0500 | G12-WF-017 | 12 | W-I10 | Source-governed consequence to official publication | L3-I-112; L3-I-113; L3-I-114; L3-I-115 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0501 | G12-WF-018 | 12 | W-I10 | Publication failure recovery | L3-I-116 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0502 | G12-WF-019 | 12 | W-I11 | Revaluation request, processing and withdrawal | L3-I-118; L3-I-120 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0503 | G12-WF-020 | 12 | W-I11 | Rechecking request, processing and withdrawal | L3-I-119; L3-I-121 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0504 | G12-WF-021 | 12 | W-I11 | Practical revaluation/rechecking | L3-I-122; L3-I-123 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0505 | G12-WF-022 | 12 | W-I11 | Revaluation evaluator conflict-of-interest replacement | L3-I-124 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0506 | G12-WF-023 | 12 | W-I11 | Additional revaluation and adjudication | L3-I-125; L3-I-126 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0507 | G12-WF-024 | 12 | W-I11 | Interrupted revaluation recovery | L3-I-127 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0508 | G12-WF-025 | 12 | W-I11 | Duplicate/conflicting revaluation/rechecking request resolution | L3-I-128; L3-I-129; L3-I-130 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0509 | G12-WF-026 | 12 | W-I11 | Revaluation reopening and finalized-outcome correction | L3-I-131; L3-I-132 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0510 | G12-WF-027 | 12 | W-I12 | Subsequent-examination application eligibility and attempt authorization | L3-I-133 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0511 | G12-WF-028 | 12 | W-I12 | Subsequent-examination scheduling, revision and rescheduling | L3-I-134; L3-I-135; L3-I-136 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0512 | G12-WF-029 | 12 | W-I12 | Subsequent-examination re-conduct | L3-I-137 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0513 | G12-WF-030 | 12 | W-I12 | Duplicate subsequent-examination attempt reconciliation | L3-I-138 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0514 | G12-WF-031 | 12 | W-I12 | Conflicting subsequent-examination result reconciliation | L3-I-139 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0515 | G12-WF-032 | 12 | W-I12 | Opportunity cancellation and restoration | L3-I-140; L3-I-141 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0516 | G12-WF-033 | 12 | W-I12 | Attempt exhaustion and policy-change eligibility determination | L3-I-142; L3-I-143 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0517 | G12-WF-034 | 12 | W-I12 | Subsequent-examination execution coordination | L3-I-144 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0518 | G13-WF-001 | 13 | W-I13 | Completion eligibility evaluation and authoritative re-evaluation | L3-I-145; L3-I-150; L3-I-151; L3-I-152 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0519 | G13-WF-002 | 13 | W-I13 | Incomplete/conflicting completion-input resolution | L3-I-146; L3-I-147 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0520 | G13-WF-003 | 13 | W-I13 | Completion identity/candidate association correction hand-off | L3-I-148; L3-I-160 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0521 | G13-WF-004 | 13 | W-I13 | Duplicate completion evaluation/evidence reconciliation | L3-I-149; L3-I-159 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0522 | G13-WF-005 | 13 | W-I13 | Governed completion/provisional evidence establishment and reissue | L3-I-153; L3-I-154; L3-I-156 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0523 | G13-WF-006 | 13 | W-I13 | Completion evidence correction, withdrawal and restoration | L3-I-155; L3-I-157; L3-I-158 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0524 | G13-WF-007 | 13 | W-I13 | Upstream Examination dependency blocking and resume | L3-I-161 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0525 | G13-WF-008 | 13 | W-I14 | Malpractice case reopening and authorized appeal review | L3-I-162; L3-I-163 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0526 | G13-WF-009 | 13 | W-I14 | Malpractice review decision and restored examination outcome | L3-I-164; L3-I-165; L3-I-166; L3-I-167 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0527 | G13-WF-010 | 13 | W-I14 | Duplicate/conflicting malpractice case or decision reconciliation | L3-I-168; L3-I-169 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0528 | G13-WF-011 | 13 | W-I14 | Malpractice adjudication and penalty determination | L3-I-170 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0529 | G13-WF-012 | 13 | W-I14 | Malpractice case closure and archival | L3-I-171 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0530 | G13-WF-013 | 13 | W-I15 | Examination exception registration and impact classification | L3-I-172; L3-I-173 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0531 | G13-WF-014 | 13 | W-I15 | Contingency suspension, resumption and termination of conduct | L3-I-174; L3-I-175; L3-I-176 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0532 | G13-WF-015 | 13 | W-I15 | Re-conduct authorization and contingency re-examination | L3-I-177; L3-I-178 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0533 | G13-WF-016 | 13 | W-I15 | Contingency decision communication | L3-I-179 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0534 | G13-WF-017 | 13 | W-I15 | Exception escalation and conflicting-decision resolution | L3-I-180; L3-I-181 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0535 | G13-WF-018 | 13 | W-I15 | Exception reopening, review and closure | L3-I-182; L3-I-183; L3-I-184 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0536 | G13-WF-019 | 13 | W-I15 | Contingency postponement and rescheduling | L3-I-185 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0537 | G13-WF-020 | 13 | W-I15 | Examination contingency-plan preparation and authorization | L3-I-186 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0538 | G13-WF-021 | 13 | W-I16 | Examination Session closure and finalization | L3-I-187 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0539 | G13-WF-022 | 13 | W-I16 | Authorized archived-record retrieval | L3-I-188 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0540 | G13-WF-023 | 13 | W-I16 | Retention classification, disposal and legal-hold governance | L3-I-189; L3-I-190; L3-I-191 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0541 | G13-WF-024 | 13 | W-I16 | Examination record archival | L3-I-192 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0542 | G14-WF-001 | 14 | W-J01 | Procurement Plan lifecycle | L3-J-001; L3-J-002; L3-J-003 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0543 | G14-WF-002 | 14 | W-J01 | Competitive/RFP/Tender/Direct sourcing | L3-J-004; L3-J-005; L3-J-006; L3-J-007 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0544 | G14-WF-003 | 14 | W-J01 | Consolidated/split demand sourcing | L3-J-008; L3-J-009 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0545 | G14-WF-004 | 14 | W-J01 | Multi-supplier award | L3-J-010 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0546 | G14-WF-005 | 14 | W-J01 | Rate Contract lifecycle | L3-J-011; L3-J-012; L3-J-013; L3-J-014 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0547 | G14-WF-006 | 14 | W-J01 | Single-source, emergency and external sourcing governance | L3-J-015; L3-J-016; L3-J-017 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0548 | G14-WF-007 | 14 | W-J01 | Insufficient-competition disposition | L3-J-018 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0549 | G14-WF-008 | 14 | W-J01 | Sourcing-event cancellation | L3-J-019 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0550 | G14-WF-009 | 14 | W-J02 | Supplier registration and activation | L3-J-020; L3-J-021 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0551 | G14-WF-010 | 14 | W-J02 | Supplier qualification, renewal, restriction and classification | L3-J-022; L3-J-023; L3-J-024; L3-J-025 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0552 | G14-WF-011 | 14 | W-J02 | Supplier profile maintenance and sensitive change approval | L3-J-026; L3-J-027 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0553 | G14-WF-012 | 14 | W-J02 | Supplier performance/risk assessment | L3-J-028; L3-J-029 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0554 | G14-WF-013 | 14 | W-J02 | Supplier suspension, blacklisting and remediation | L3-J-030; L3-J-031; L3-J-032 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0555 | G14-WF-014 | 14 | W-J02 | Supplier inactivity and relationship closure | L3-J-033; L3-J-034 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0556 | G14-WF-015 | 14 | W-J02 | Duplicate supplier and external-reference reconciliation | L3-J-035; L3-J-036 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0557 | G14-WF-016 | 14 | W-J02 | Supplier representative lifecycle | L3-J-037 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0558 | G14-WF-017 | 14 | W-J03 | Purchase Requisition approval/rejection | L3-J-038; L3-J-039 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0559 | G14-WF-018 | 14 | W-J03 | Purchase Requisition amendment and termination | L3-J-040; L3-J-041 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0560 | G14-WF-019 | 14 | W-J03 | Emergency Purchase Requisition | L3-J-042 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0561 | G14-WF-020 | 14 | W-J04 | Purchase Order issue and supplier acknowledgement | L3-J-043; L3-J-044 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0562 | G14-WF-021 | 14 | W-J04 | Purchase Order amendment/cancellation | L3-J-045; L3-J-046 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0563 | G14-WF-022 | 14 | W-J04 | Purchase Order closure and exceptional closure | L3-J-047; L3-J-048 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0564 | G14-WF-023 | 14 | W-J05 | Goods receipt, inspection and disposition | L3-J-049 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0565 | G14-WF-024 | 14 | W-J05 | Finalized receipt correction | L3-J-050 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0566 | G14-WF-025 | 14 | W-J06 | Item master lifecycle | L3-J-051; L3-J-052 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0567 | G14-WF-026 | 14 | W-J06 | Warehouse/location structure governance | L3-J-053 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0568 | G14-WF-027 | 14 | W-J06 | Accepted receipt to inventory consequence | L3-J-054 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0569 | G14-WF-028 | 14 | W-J06 | Expiry/controlled-stock availability governance | L3-J-055; L3-J-056 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0570 | G14-WF-029 | 14 | W-J06 | Movement/activity to inventory availability and ledger consequence | L3-J-057; L3-J-058 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0571 | G14-WF-030 | 14 | W-J06 | Inventory stock adjustment and discrepancy correction | L3-J-059; L3-J-060 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0572 | G14-WF-031 | 14 | W-J06 | Reorder-threshold publication | L3-J-061 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0573 | G14-WF-032 | 14 | W-J06 | Inventory valuation-reference reconciliation | L3-J-062 | SYSTEM-BACKGROUND-OBSERVABLE | PROHIBITED |
| L6-RWF-0574 | G14-WF-033 | 14 | W-J06 | Disposal/write-off inventory adjustment consequence | L3-J-063 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0575 | G14-WF-034 | 14 | W-J07 | Stock issue and reservation execution | L3-J-064; L3-J-065 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0576 | G14-WF-035 | 14 | W-J07 | Stock return, transfer and distribution | L3-J-066; L3-J-067; L3-J-068 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0577 | G14-WF-036 | 14 | W-J07 | Consumption and custody handover | L3-J-069; L3-J-070 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0578 | G14-WF-037 | 14 | W-J07 | Stock movement exception resolution | L3-J-071 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0579 | G14-WF-038 | 14 | W-J07 | Asset-linked stock custody handoff | L3-J-072 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0580 | G14-WF-039 | 14 | W-J07 | Stock movement cancellation/reversal | L3-J-073 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0581 | G14-WF-040 | 14 | W-J08 | Asset registration, assignment and return | L3-J-074; L3-J-075; L3-J-076 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0582 | G14-WF-041 | 14 | W-J08 | Asset location and transfer | L3-J-077; L3-J-078 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0583 | G14-WF-042 | 14 | W-J08 | Asset condition, missing, maintenance and verification reconciliation | L3-J-079; L3-J-080; L3-J-081; L3-J-082 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0584 | G14-WF-043 | 14 | W-J08 | Asset retirement, disposal handoff and archival | L3-J-083; L3-J-084; L3-J-085 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0585 | G14-WF-044 | 14 | W-J08 | Protected Asset history correction | L3-J-086 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0586 | G14-WF-045 | 14 | W-J09 | Inventory verification audit lifecycle and interrupted audit | L3-J-087; L3-J-088 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0587 | G14-WF-046 | 14 | W-J09 | Inventory variance investigation and adjustment referral | L3-J-089; L3-J-090 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0588 | G14-WF-047 | 14 | W-J09 | Confirmed audit loss/damage to J10 referral | L3-J-091 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0589 | G14-WF-048 | 14 | W-J09 | Asset-linked verification to J08 finding | L3-J-092 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0590 | G14-WF-049 | 14 | W-J09 | Supplemental audit determination | L3-J-093 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |
| L6-RWF-0591 | G14-WF-050 | 14 | W-J10 | Disposal/write-off authorization, rejection and cancellation | L3-J-094; L3-J-095; L3-J-096 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0592 | G14-WF-051 | 14 | W-J10 | Disposal execution and partial execution | L3-J-097; L3-J-098 | INTERACTIVE-WORKFLOW | REQUIRED |
| L6-RWF-0593 | G14-WF-052 | 14 | W-J10 | Loss/write-off, audit-linked and asset-linked disposal | L3-J-099; L3-J-100; L3-J-101 | CROSS-WORKSPACE-HANDOFF | REQUIRED |
| L6-RWF-0594 | G14-WF-053 | 14 | W-J10 | Emergency disposal | L3-J-102 | REVIEW-APPROVAL-WORKFLOW | REQUIRED |
| L6-RWF-0595 | G14-WF-054 | 14 | W-J10 | Disposal exception and supplemental correction | L3-J-103; L3-J-104 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED |

## 8. Master Exit Gate

| Gate | Result |
|---|---|
| 1,084 canonical scenarios represented | PASS |
| Unique primary scenario assignment | PASS |
| Duplicate primary assignment = 0 | PASS |
| 595 workflow identities reconciled | PASS |
| Repeated workspace across groups = 0 | PASS |
| Cross-group invalid merge = 0 | PASS |
| Canonical ownership preserved | PASS |
| Shared governance consumed consistently | PASS |
| Terminology conflicts requiring upstream change | NONE |
| Material upstream blockers | 0 |
| Implementation leakage | NONE |

The canonical Workflow Realization Register may now be populated from this master catalog and the final scenario-level realization rows.
