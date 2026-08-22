# Student Lifecycle Upstream L2 Architecture Approval Record — 2026-08-22

| Attribute | Value |
|---|---|
| Record Type | Architecture Review Decision / Repository Update Authorization |
| Date | 2026-08-22 |
| Branch | review |
| Status | Approved and Incorporated |
| Approval Source | Explicit Lead Architect / repository-authority approval recorded in the architecture working session: “Please do the upstream chnages I authrize them” |

## 1. Purpose

This record closes the decision gate for the Student Lifecycle upstream architecture remediation identified during Level 3 Phase 5 review. It records approval of the repository-consistent resolutions and authorizes their incorporation into Locked Level 2 architecture.

This approval is limited to the three conflicts/gaps documented below. It does not authorize unrelated architectural invention or implementation-level design.

## 2. Approved ADR Decisions

| ADR | Decision | Incorporated L2 Effect |
|---|---|---|
| ADR-STU-ADM-EXP-001 | Canonicalize Admission expiry as `Admission Initiated → Expired`; retain terminal `Expired` and canonical Admissions event `AdmissionExpired`. | Locked L2-C01 amendment v1.1.0. |
| ADR-STU-PROG-EVT-001 | Canonicalize progression outcome events as `ConditionalPromotionGranted`, `AcademicProbationAssigned`, and `AcademicDetentionApplied`; supersede earlier synonyms as canonical publications. | Locked L2-C04 amendment v1.1.1. |
| ADR-STU-MEME-EXIT-001 | Explicitly support NEP/MEME intermediate qualification exit using L1-permitted qualification outcomes, policy-governed thresholds, existing L2-C05 lifecycle/events, and preserved Academic Credit Ledger ownership. | Locked L2-C05 amendment v1.1.0. |

## 3. Canonical Amendment Files

The following files are authoritative parts of their target L2 documents. Where they explicitly conflict with an earlier target version, the amendment has precedence for the amended subject only; all unaffected target-document content remains authoritative.

- `architecture/L2_Domain_Workflow_Architecture/section-c/L2-C01_Admissions_Workflows_Amendment_2026-08-22.md`
- `architecture/L2_Domain_Workflow_Architecture/section-c/L2-C04_Student_Academic_Progression_Workflows_Amendment_2026-08-22.md`
- `architecture/L2_Domain_Workflow_Architecture/section-c/L2-C05_Qualification_Completion_and_Student_Exit_Workflows_Amendment_2026-08-22.md`

## 4. Canonical Ownership Decisions

1. Admissions remains the sole owner of Admission expiry and publisher of `AdmissionExpired`.
2. Academic Progression remains the canonical publisher of the three standardized progression outcome events.
3. Qualification Completion & Student Exit owns the MEME qualification decision, award, record finalization, and exit completion.
4. Academic Progression retains ownership of the Academic Credit Ledger; the MEME qualification pathway consumes the ledger and does not take ownership of it.
5. Examination remains owner of examination evidence only and does not award the qualification.
6. Certificate and Transcript generation remain downstream responsibilities and are not transferred to L2-C05.

## 5. Event Decisions

- `AdmissionExpired` — Admissions.
- `ConditionalPromotionGranted` — Academic Progression.
- `AcademicProbationAssigned` — Academic Progression.
- `AcademicDetentionApplied` — Academic Progression.
- `QualificationEligibilityEvaluated` — Qualification Completion Aggregate.
- `DegreeAuditCompleted` — Qualification Completion Aggregate.
- `InstitutionalClearanceCompleted` — Qualification Completion Aggregate.
- `QualificationCompletionApproved` — Qualification Completion Aggregate.
- `QualificationAwarded` — Qualification Completion Aggregate.
- `AcademicRecordFinalized` — Qualification Completion Aggregate.
- `StudentExitCompleted` — Qualification Completion Aggregate.

The superseded C04 names `StudentConditionallyProgressed`, `StudentPlacedOnProbation`, `StudentOnProbation`, and `StudentDetained` are historical aliases only and are not separate canonical events.

## 6. MEME Policy Boundary

This approval deliberately does not fix numeric credit thresholds, semester/year thresholds, qualification mappings, re-entry windows, credit-validity durations, or credit-expiry durations. Locked L1 requires these to remain institution/regulation-configured policy.

Allowed qualification outcomes remain those already established by locked L1: Certificate, Diploma, Advanced Diploma, Degree, and Degree with Research.

## 7. Downstream Synchronization Record

The approved decisions have been synchronized into the directly affected Phase 5 L3 scenarios on `review`:

- `L3-C-006` — Admission Offer Expiry.
- `L3-C-027` — Student Academic Progression Decision.
- `L3-C-030` — Academic Probation Management and Resolution.
- `L3-C-031` — Academic Detention and Repeat Determination.
- `L3-C-038` — Progression Decision Revision After Re-evaluation or Appeal.
- `L3-C-041` — NEP/MEME Intermediate Qualification Exit.

`L3-C-029` was reviewed against the amendment and does not publish a superseded canonical event, so no semantic event-name change was required.

## 8. Repository Update Boundary

This decision record covers only the three Student Lifecycle upstream L2 remediations described above and their necessary downstream synchronization. It does not approve, lock, or certify the full Level 3 Phase 5 scenario set.
