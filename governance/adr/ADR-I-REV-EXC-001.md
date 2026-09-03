# ADR-I-REV-EXC-001 — Revaluation & Rechecking Exceptional Lifecycle Completion

| Attribute | Value |
|---|---|
| ADR ID | `ADR-I-REV-EXC-001` |
| Architecture Level | L2 — Domain Workflow Architecture |
| Domain | Section I — Examination / Revaluation & Rechecking |
| Date | 2026-09-03 |
| Status | **Approved and Incorporated** |
| Change Category | **Minor — controlled workflow/capability extension** |
| Target | `L2-I11_Revaluation_and_Rechecking_Workflows.md` |
| Working Branch | `pallav` |
| Approval Source | Explicit repository-change authorization recorded in the architecture working session on 2026-09-03: “Yes go ahead I authorize” |

## 1. Decision Context

Independent review of the Section I L3 event-remediation population identified six Locked L3 scenarios whose institutional outcomes were present in the frozen L3 discovery/canonicalization lineage but were not sufficiently established by current Locked `L2-I11`:

- `L3-I-120` — Revaluation Request Withdrawal to Closed Revaluation Request;
- `L3-I-121` — Rechecking Request Withdrawal to Closed Rechecking Request;
- `L3-I-124` — Revaluation Evaluator Conflict of Interest to Governed Replacement Evaluation;
- `L3-I-126` — Persistent Revaluation Difference to Adjudicated Revised Marks;
- `L3-I-131` — Revaluation Reopening to Valid Authoritative Outcome; and
- `L3-I-132` — Finalized Revaluation Correction to Traceable Authoritative Outcome.

The frozen Phase 1 inventory independently defines each outcome, and the Phase 4 canonical register records each source candidate as `KEEP`. Current Locked L2-I11 already owns post-publication Revaluation & Rechecking, evaluator assignment within revaluation, outcome verification, revised-marks approval, review lifecycle governance, and downstream result-revision initiation.

The defect is therefore an upstream coverage gap inside the existing L2-I11 owner, not a reason to transfer ownership or manufacture the missing semantics in L3/L4.

## 2. Decision

All six L3 scenario identities are retained. L2-I11 is extended through one controlled amendment containing three shared exception mechanisms:

1. **Review Request Withdrawal** — one governed capability applying to both Revaluation and Rechecking requests before the permitted review-processing boundary.
2. **Exceptional Revaluation Resolution** — evaluator conflict/replacement and policy-required adjudicating revaluation remain inside the L2-I11 Revaluation workflow and reuse its existing ownership, verification and approval chain.
3. **Controlled Review Reopening and Finalized-Outcome Correction** — reopening before revised-marks finalization and correction after finalization but before publication of the resulting revised result are explicitly governed as separate lifecycle paths.

## 3. Canonical Ownership Decisions

1. `L2-I11` remains sole owner of post-publication Revaluation & Rechecking review lifecycle and authoritative review outcome.
2. `L2-I08` continues to own ordinary Marks Collection & Evaluation and does not acquire revaluation ownership.
3. Workforce, identity, organisation, academic-delivery, finance, result-processing and result-publication facts remain owned by their canonical domains when consumed by L2-I11.
4. Evaluator conflict-of-interest governance inside a revaluation constrains the L2-I11-owned revaluation assignment; it does not transfer Employee, Position, Faculty Profile or workforce lifecycle ownership to Examination.
5. L2-I09 remains owner of result recomputation and L2-I10 remains owner of official publication/republication.
6. No L0 or L1 ownership change is required.

## 4. Canonical Lifecycle Decisions

### 4.1 Withdrawal
A Revaluation or Rechecking request may reach `Withdrawn` only while institutional policy permits withdrawal and before the request enters `Review In Progress`. Institutions may configure an earlier cutoff. Withdrawal preserves history and prevents further review processing. Finance-owned fee/refund consequences remain outside L2-I11.

### 4.2 Evaluator conflict and replacement
A conflicted or otherwise invalid evaluator assignment shall not establish an authoritative revaluation outcome. L2-I11 replaces the assignment through its governed Revaluation workflow and continues only with an authorized evaluator. No workforce lifecycle is created.

### 4.3 Persistent difference / adjudication
Where institutional policy requires an additional independent/adjudicating review because valid evaluation outcomes remain outside permitted tolerance, L2-I11 conducts that adjudicating revaluation. The resulting marks still require Review Outcome Verification and Revised Marks Approval.

### 4.4 Reopening before finalization
An authorized institutional review authority may reopen a completed/verified revaluation before revised marks become finally approved for downstream revision. Reopening returns the review to `Review In Progress` at the earliest valid stage required to resolve the authorized issue. Prior evidence remains traceable.

### 4.5 Correction after finalization but before revised-result publication
If an authorized material error is established after the revaluation outcome has been finalized/approved but before L2-I10 publishes the resulting revised official result, L2-I11 may initiate controlled correction. The corrected outcome must again satisfy Review Outcome Verification and Revised Marks Approval. If the revised official result has already been published, correction/republication remains governed by L2-I10.

## 5. Canonical Events

Only three new L2-I11 canonical events are introduced:

| Event | Canonical Publisher | Meaning |
|---|---|---|
| `Review Request Withdrawn` | L2-I11 — Revaluation & Rechecking | A Revaluation or Rechecking request has been validly withdrawn before the permitted review-processing boundary. |
| `Review Reopened` | L2-I11 — Revaluation & Rechecking | An authorized completed/verified revaluation has been returned to active review before final revised-marks approval. |
| `Review Outcome Corrected` | L2-I11 — Revaluation & Rechecking | A previously finalized/approved revaluation outcome has been superseded by an authorized corrected review outcome before revised-result publication. |

Evaluator replacement and adjudicating revaluation reuse existing lifecycle events such as `Revaluation Started`, `Review Completed`, `Review Outcome Verified`, and `Revised Marks Approved`; no scenario-title event is created.

## 6. L3 Disposition

`L3-I-120`, `L3-I-121`, `L3-I-124`, `L3-I-126`, `L3-I-131`, and `L3-I-132` are **KEEP**. This ADR resolves their missing upstream L2 coverage but does not itself re-lock or approve their L3 successor interpretation. Each must be revalidated against the incorporated L2-I11 amendment.

## 7. Repository Synchronization

The controlled change creates the L2-I11 amendment and updates the six-parent disposition from `BLOCKED BY UPSTREAM ARCHITECTURE` to `UPSTREAM RESOLVED — L3 REVALIDATION REQUIRED`.

No dependency-map row is added because this decision creates no new cross-section dependency class or ownership direction. Existing L2-I11 collaboration boundaries remain in force.

No implementation, API, database, UI, infrastructure or technology decision is authorized.
