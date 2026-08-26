# L2-C05 — Qualification Completion & Student Exit Workflows — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-C05 — Qualification Completion & Student Exit Workflows |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-22 |
| Approved Through | ADR-STU-MEME-EXIT-001 |
| Authority | Lead Architect approval recorded 2026-08-22 |

This amendment is part of the canonical L2-C05 architecture. Where this amendment conflicts with the original L2-C05 document, this amendment is authoritative for the MEME intermediate qualification-exit subject only. Unaffected L2-C05 content remains unchanged.

## 1. MEME Intermediate Qualification Exit

Qualification Completion & Student Exit explicitly supports an NEP/MEME intermediate qualification-exit pathway.

A student may complete the current Student Context with an applicable intermediate qualification only when:

- the institution's approved MEME policy permits exit at the student's authoritative academic position;
- the Academic Credit Ledger and other authoritative academic evidence satisfy the policy-defined requirements for an allowed qualification outcome;
- applicable institutional clearance is complete;
- the required Qualification Completion approval is granted; and
- the qualification award and Student Exit are conclusively recorded.

## 2. Allowed Qualification Outcomes

Consistent with locked L1, the qualification outcome must be one of:

- Certificate
- Diploma
- Advanced Diploma
- Degree
- Degree with Research

The applicable outcome is determined by institution/regulation-configured policy and verified academic evidence. No fixed credit count, academic year, semester, or duration-to-award mapping is introduced by this amendment.

## 3. Canonical Ownership

Qualification Completion & Student Exit remains the canonical owner of the qualification decision, approval, award, academic-record finalization, and exit completion.

Academic Progression remains the owner of the Academic Credit Ledger and progression facts. L2-C05 consumes those authoritative facts and does not directly modify the Academic Credit Ledger.

Examination remains the owner of examination evidence and does not make the institutional qualification decision.

## 4. Canonical Lifecycle

The MEME pathway uses the existing L2-C05 qualification-completion lifecycle rather than creating a parallel lifecycle:

`Eligibility Evaluation → Degree/Qualification Audit → Institutional Clearance → Approval Pending → Qualification Completion Approved → Qualification Awarded → Academic Record Finalized → Student Exit Completed`

For this amendment, references to the existing Degree Audit capability apply to the qualification-completion audit appropriate to the policy-selected qualification outcome; this does not create a second audit aggregate or lifecycle.

## 5. Canonical Events

Existing L2-C05 events are reused:

| Outcome | Canonical Event | Canonical Publisher |
|---|---|---|
| Qualification eligibility evaluated | `QualificationEligibilityEvaluated` | Qualification Completion Aggregate |
| Qualification audit completed | `DegreeAuditCompleted` | Qualification Completion Aggregate |
| Institutional clearance completed | `InstitutionalClearanceCompleted` | Qualification Completion Aggregate |
| Qualification completion approved | `QualificationCompletionApproved` | Qualification Completion Aggregate |
| Qualification awarded | `QualificationAwarded` | Qualification Completion Aggregate |
| Academic record finalized | `AcademicRecordFinalized` | Qualification Completion Aggregate |
| Student exit completed | `StudentExitCompleted` | Qualification Completion Aggregate |

No MEME-specific duplicate event is introduced.

## 6. Business Authority

Existing L2-C05 authority rules govern the pathway:

| Business Action | Canonical Responsibility |
|---|---|
| Qualification Completion Evaluation | Academic Administrator |
| Institutional Clearance | Authorized Organizational Units |
| Qualification Completion Approval | Academic Authorities |
| Qualification Conferment / Award | Registrar / Authorized Academic Authority |
| Student Exit Approval | Authorized Institutional Authority |

Institution-specific approval chains remain policy-configured.

## 7. Credit Preservation and Re-entry

A MEME exit does not erase earned academic credits. The Academic Credit Ledger remains authoritative and preserves earned credits subject to institution-configured credit-validity and credit-expiry policy established under locked L1.

Re-entry eligibility is policy-governed, but execution of a later re-entry remains outside this amendment and must occur through the appropriate Student Lifecycle workflow. This amendment does not create a re-entry lifecycle or owner.

## 8. Downstream Boundaries

- Certificate generation remains owned by Certificate Management.
- Transcript generation remains owned by Transcript Management.
- Alumni transition remains downstream of the authoritative qualification/exit outcome.
- This amendment does not alter any non-MEME exit pathway.

## 9. Downstream Rule

L3-C-041 may now consume this pathway without inventing a qualification threshold, a new qualification state, a new event, or a new approval authority. Downstream architecture must preserve the ownership, policy boundary, lifecycle, and event publishers defined here.
