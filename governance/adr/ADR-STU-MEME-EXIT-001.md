# ADR-STU-MEME-EXIT-001 — NEP/MEME Intermediate Qualification Exit

| Attribute | Value |
|---|---|
| Status | Approved |
| Date | 2026-08-22 |
| Architecture Level | L2 — Business Workflow Architecture |
| Domain | Section C — Student Lifecycle / Qualification Completion & Student Exit |
| Decision Authority | Lead Architect authorization recorded in the architecture working session |
| Trigger | L3 Phase 5 architecture review of L3-C-041 |

## 1. Context

Locked L1 establishes Multiple Entry Multiple Exit (MEME) as an NEP 2020 capability of Student Lifecycle and defines L1-12 Qualification Completion / Student Exit as the owner of qualification exit. L1 permits qualification outcomes including Certificate, Diploma, Advanced Diploma, Degree, and Degree with Research, states that MEME is governed by institutional policy, and requires credits to remain preserved.

Locked L2-C05 owns Qualification Completion & Student Exit but does not explicitly describe the intermediate MEME qualification-exit path needed by canonical scenario L3-C-041.

L3 may not invent the missing qualification threshold, exit authority, event, or lifecycle semantics. The required business semantics therefore belong upstream in L2-C05.

## 2. Decision

L2-C05 explicitly supports an **NEP/MEME Intermediate Qualification Exit** as a qualification-completion pathway.

A student may receive an intermediate qualification and conclude the current Student Context when all of the following are true:

1. the student is eligible to exercise an applicable MEME exit under the institution's approved policy and regulatory framework;
2. the Academic Credit Ledger and other authoritative academic evidence satisfy the policy-defined threshold for the applicable qualification outcome;
3. the existing Qualification Completion eligibility, institutional-clearance, and approval obligations applicable to that qualification outcome are satisfied; and
4. the authorized Qualification Completion authority approves the award.

The exact credit threshold, academic-period threshold, qualification mapping, re-entry eligibility, credit validity, and credit-expiry policy are **not fixed by L2**. They remain institution/regulation-configured business policy as required by locked L1.

## 3. Qualification Outcomes

The pathway may result only in a qualification outcome permitted by the higher-authority L1 architecture:

- Certificate
- Diploma
- Advanced Diploma
- Degree
- Degree with Research

For an intermediate MEME exit, the applicable outcome is determined by the authoritative policy and the student's verified academic position. L2 does not hard-code a year-to-award or credit-count-to-award mapping.

## 4. Canonical Ownership

Qualification Completion & Student Exit remains the sole canonical owner of:

- qualification-completion eligibility for the exit pathway;
- qualification approval;
- qualification award;
- current Student Context exit completion within the L2-C05 boundary; and
- the canonical qualification and exit events described below.

Academic Progression remains owner of the Academic Credit Ledger and progression facts consumed by L2-C05. The amendment does not transfer Academic Credit Ledger ownership to Qualification Completion.

## 5. Canonical Lifecycle and Events

The MEME intermediate qualification exit uses the existing L2-C05 qualification-completion lifecycle. No parallel qualification lifecycle is created.

Applicable progression is:

`Eligibility Evaluation → Degree/Qualification Audit → Institutional Clearance → Approval Pending → Qualification Completion Approved → Qualification Awarded → Academic Record Finalized → Student Exit Completed`

The existing event vocabulary is reused:

- `QualificationEligibilityEvaluated`
- `DegreeAuditCompleted`
- `InstitutionalClearanceCompleted`
- `QualificationCompletionApproved`
- `QualificationAwarded`
- `AcademicRecordFinalized`
- `StudentExitCompleted`

No separate `IntermediateQualificationAwarded` or MEME-specific canonical event is introduced.

## 6. Authority

Existing L2-C05 authority rules apply:

- Qualification Completion Evaluation — Academic Administrator
- Institutional Clearance — Authorized Organizational Units
- Qualification Completion Approval — Academic Authorities
- Degree/Qualification Conferment — Registrar / Authorized Academic Authority
- Student Exit Approval — Authorized Institutional Authority

The exact institution-specific approval chain remains policy-configured within the existing L2-C05 governance model.

## 7. Credit Preservation and Re-entry Boundary

A MEME exit does not erase earned academic credits. The Academic Credit Ledger remains authoritative and credits remain preserved subject to the institution's approved credit-validity/expiry policy.

This ADR does not create or transfer ownership of a future re-entry workflow. Re-entry eligibility is policy-governed under locked L1 and must be consumed by the appropriate Student Lifecycle workflow when re-entry is performed.

## 8. Downstream Boundaries

- Certificate generation remains downstream Certificate Management responsibility.
- Transcript generation remains downstream Transcript Management responsibility.
- Alumni transition remains downstream of the authoritative qualification/exit outcome and follows existing L2-C05/L2-C08 collaboration.
- Examination supplies evidence only and does not award the qualification.

## 9. Implementation in Architecture Repository

This ADR is incorporated through:

`architecture/L2_Domain_Workflow_Architecture/section-c/L2-C05_Qualification_Completion_and_Student_Exit_Workflows_Amendment_2026-08-22.md`

The amendment has precedence only for the MEME intermediate qualification-exit subject. All unaffected L2-C05 content remains authoritative.
