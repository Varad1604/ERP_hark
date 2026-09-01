# L2-H08 — Submission Execution and Closure Governance — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-H08 — NEP, NAAC & AISHE Reporting Workflows |
| Amendment Version | 1.2.0 |
| Status | Locked |
| Effective Date | 2026-09-01 |
| Approved Through | ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-002 |

## 1. Submission execution authority

The **Regulatory Reporting Submission Authority** is the canonical institutional business authority for executing an externally due H08 submission after the Submission Package is institutionally Approved and eligible for submission.

This authority does not approve the package content and does not own the external Regulatory Authority's decision.

## 2. Correction and closure authority

The **Regulatory Reporting Governance Authority** remains the canonical H08 governance authority and owns final Reporting Cycle closure when all reporting obligations and submission-history conditions are complete.

The Regulatory Reporting Submission Authority may initiate the governed correction/resubmission path when an institutional submission action is required. Corrections to source-owned facts remain with their canonical source owners.

## 3. Rejection-event normalization

Institutional package rejection and external regulatory rejection are distinct facts:

- `SubmissionPackageRejected` — institutional H08 package-approval rejection, published by H08 for the Submission Package.
- `SubmissionRejectedByAuthority` — external Regulatory Authority rejection recorded by H08 for the governed Submission Record/submission outcome.

Historical uses of the ambiguous `SubmissionRejected` label shall be interpreted according to context and shall not be used as a new canonical event for future L3/L4 derivation.

Existing external-outcome events including `SubmissionAccepted`, `RegulatoryRemarksReceived` and `ResubmissionRequested` remain unchanged.

## 4. Closure semantics

Reporting Cycle correction/closure continues to preserve immutable Reporting Snapshots and Submission Records. Closure may proceed only after required obligations are complete; successful terminal progression remains `Accepted → Closed → Archived` where applicable.

## 5. Downstream effect

This amendment resolves the actor/authority and rejection-event ambiguity recorded by L3-H-059 and L3-H-060.
