# L2-I11 — Revaluation & Rechecking — Exception Lifecycle and Adjudication Amendment

| Attribute | Value |
|---|---|
| Target | `L2-I11_Revaluation_and_Rechecking_Workflows.md` |
| Amendment Version | 1.1.0 |
| Status | **Locked** |
| Effective Date | 2026-09-03 |
| Approved Through | `ADR-I-REV-EXC-001` |
| Authority | Explicit repository-change authorization recorded 2026-09-03 |
| Change Category | Minor — controlled workflow/capability extension |

This amendment is part of canonical L2-I11 architecture. The original Locked L2-I11 document and this amendment shall be read together. Where this amendment governs withdrawal, evaluator-conflict replacement, adjudicating revaluation, reopening, or pre-publication correction of a finalized review outcome, this amendment is authoritative for that subject. Unaffected L2-I11 content remains authoritative.

---

## 1. Purpose

Complete the missing exception lifecycle required to govern six frozen Section I business-scenario outcomes without transferring ownership or introducing implementation design.

The amendment adds three shared business mechanisms:

1. Review Request Withdrawal;
2. Exceptional Revaluation Resolution; and
3. Controlled Review Reopening / Finalized-Outcome Correction.

---

## 2. Preserved Ownership and Boundary

- Revaluation & Rechecking remains owner of the post-publication review request, review process and authoritative review outcome.
- L2-I08 continues to own ordinary Marks Collection & Evaluation; revaluation remains outside I08 scope.
- L2-I09 continues to own Result Processing & Computation.
- L2-I10 continues to own official Result Publication, replacement, republication and withdrawal of published results.
- Finance continues to own fees, payments, waivers and refunds.
- People & Workforce continues to own workforce identity, employment, position and faculty workforce information.
- L2-I11 may consume authoritative evaluator/person/workforce facts required to validate assignment, but does not own those facts.

No L0 or L1 change is made.

---

## 3. New Capability — Review Request Withdrawal

### 3.1 Purpose

Govern voluntary withdrawal of an accepted Revaluation or Rechecking request before the permitted processing boundary.

### 3.2 Entry conditions

Withdrawal may proceed only when:

- the review request exists and is authoritative;
- the requester is authorized under institutional review policy;
- the request has not entered `Review In Progress`; and
- no earlier policy-defined withdrawal cutoff has passed.

### 3.3 Business flow

1. Receive a withdrawal request for an existing Revaluation or Rechecking request.
2. Verify request identity, requester authority, current review state and withdrawal-window eligibility.
3. Determine whether withdrawal is still permitted under authoritative Examination Policy.
4. If permitted, transition the review request to `Withdrawn`.
5. Preserve prior request, eligibility, fee-verification, approval and withdrawal evidence.
6. Publish `Review Request Withdrawn`.
7. Stop further Revaluation/Rechecking processing for that request.

### 3.4 Rejection / alternate outcome

If the request has entered `Review In Progress`, or policy otherwise prohibits withdrawal, the withdrawal request is denied and the existing review continues under its authoritative state.

A denied withdrawal does not create a new review state.

### 3.5 Finance boundary

Withdrawal may create a Finance-owned refund/forfeiture consequence where policy requires it. L2-I11 records only the review withdrawal; it does not calculate, approve or execute a financial transaction.

---

## 4. New Capability — Exceptional Revaluation Resolution

### 4.1 Evaluator independence and conflict constraint

Every revaluation evaluator assignment must satisfy applicable institutional requirements for qualification, independence, confidentiality and conflict of interest.

If an assigned evaluator is conflicted, unavailable, ineligible, or otherwise unable to provide a valid independent revaluation:

1. the invalid assignment is prevented from producing an authoritative review outcome;
2. the Revaluation Officer or other policy-authorized L2-I11 authority records the assignment exception;
3. authoritative person/workforce eligibility facts are obtained from their owning domain where required;
4. L2-I11 establishes a valid replacement evaluator assignment under the existing Revaluation workflow; and
5. revaluation continues without changing ownership of workforce facts.

No new evaluator-replacement lifecycle aggregate or workforce event is created by L2-I11.

### 4.2 Persistent revaluation difference

Where institutional policy defines a permitted difference/tolerance and valid revaluation outcomes remain outside that boundary:

1. L2-I11 determines whether policy requires an additional independent/adjudicating revaluation;
2. an authorized adjudicating evaluator or academic review arrangement is established under institutional policy;
3. the adjudicating revaluation is performed as part of the L2-I11 Revaluation workflow;
4. the resulting recommendation enters `Review Outcome Verification`;
5. where marks change, `Revised Marks Approval` remains mandatory before downstream revision; and
6. existing events `Revaluation Started`, `Review Completed`, `Review Outcome Verified`, and `Revised Marks Approved` are used as applicable.

No new `Adjudication Completed` or similar event is created merely to mirror an L3 scenario title.

---

## 5. New Capability — Controlled Review Reopening

### 5.1 Scope

Reopening applies to a Revaluation review that has completed or been verified but has **not yet reached final revised-marks approval for downstream result revision**.

### 5.2 Authorized reasons

Reopening requires an authoritative institutional basis such as:

- material evidence omission;
- invalid evaluator assignment;
- identified review-procedure defect;
- authoritative source correction affecting the review; or
- another policy-defined material defect.

The amendment does not create a candidate right to reopen a review. Candidate representations may provide evidence, but reopening itself requires authorized institutional review authority.

### 5.3 Business flow

1. Receive an authoritative reopening decision with its material basis.
2. Verify the current review position is eligible for reopening.
3. Preserve the previously completed/verified review evidence and outcome as historical evidence.
4. Transition the review back to `Review In Progress` at the earliest valid stage required to resolve the defect.
5. Publish `Review Reopened`.
6. Complete the required revaluation activity.
7. Reapply `Review Outcome Verification` and, where applicable, `Revised Marks Approval`.
8. Continue the ordinary L2-I11 lifecycle toward downstream result revision where required.

---

## 6. New Capability — Finalized Review Outcome Correction Before Revised-Result Publication

### 6.1 Scope boundary

This correction applies when:

- a Revaluation outcome has been finalized/approved by L2-I11;
- an authorized material error is subsequently established; and
- L2-I10 has **not yet published the resulting revised official result**.

If the revised result is already published, correction/republication is governed by L2-I10 and this L2-I11 amendment shall not override that ownership.

### 6.2 Business flow

1. Receive an authorized correction decision and authoritative evidence of the material error.
2. Verify that the correction is still inside the L2-I11 pre-publication correction boundary.
3. Preserve the prior finalized review outcome and approval history as immutable historical evidence.
4. Reopen only the review activity necessary to resolve the error.
5. Establish the corrected review recommendation.
6. Reapply `Review Outcome Verification`.
7. Reapply `Revised Marks Approval` where revised marks are affected.
8. Publish `Review Outcome Corrected` when the corrected outcome becomes the authoritative L2-I11 review outcome.
9. Where a prior downstream result-revision initiation exists, supply the superseding authoritative review outcome to L2-I09/L2-I10 while preserving prior revision history.

---

## 7. Lifecycle Amendment

The canonical Review Request lifecycle is extended with:

```text
Submitted
  -> Eligibility Verified
  -> Approved
  -> Withdrawn                 [only before Review In Progress, if policy permits]

Approved
  -> Review In Progress
  -> Review Completed
  -> Outcome Verified
  -> Revised Marks Approved    [where applicable]
  -> Result Revision Initiated [where applicable]
  -> Closed
  -> Archived

Review Completed / Outcome Verified
  -> Review In Progress        [authorized reopening before final revised-marks approval]

Finalized / Revised Marks Approved
  -> Correction Review         [authorized material correction before revised-result publication]
  -> Outcome Verified
  -> Revised Marks Approved
  -> Result Revision Initiated [where applicable]
```

### Canonical state interpretation

- `Withdrawn` is a terminal review-request state for that request.
- `Review In Progress`, `Review Completed`, `Outcome Verified`, `Revised Marks Approved`, `Closed`, and `Archived` retain their existing L2-I11 meaning.
- `Correction Review` is an explicit controlled lifecycle position used only for pre-publication correction of a finalized L2-I11 review outcome.
- Reopening does not erase the prior completed/verified history.
- Correction does not overwrite the prior finalized outcome history.

---

## 8. Business Rules Added

**BR-I11-EXC-001 — Withdrawal Boundary**  
A Revaluation or Rechecking request may be withdrawn only before `Review In Progress` and only while institutional policy permits withdrawal.

**BR-I11-EXC-002 — Withdrawal History Preservation**  
Withdrawal shall preserve the original request and all completed eligibility, fee, approval and audit evidence.

**BR-I11-EXC-003 — Evaluator Independence**  
A conflicted or otherwise invalid evaluator assignment shall not establish an authoritative revaluation outcome.

**BR-I11-EXC-004 — Replacement Ownership Boundary**  
L2-I11 owns the review assignment decision; authoritative workforce/person facts used to validate an evaluator remain owned by their canonical source domain.

**BR-I11-EXC-005 — Adjudicating Revaluation**  
An additional/adjudicating revaluation may occur only where authoritative Examination Policy requires it for unresolved evaluation difference.

**BR-I11-EXC-006 — Reopening Authority**  
A completed/verified review may be reopened only by policy-authorized institutional review authority and only before final revised-marks approval.

**BR-I11-EXC-007 — Reopening Traceability**  
Reopening shall preserve the prior review outcome, evidence, authority and reason.

**BR-I11-EXC-008 — Correction Boundary**  
A finalized L2-I11 review outcome may be corrected by L2-I11 only before publication of the resulting revised official result.

**BR-I11-EXC-009 — Correction Revalidation**  
A corrected review outcome must undergo Review Outcome Verification and Revised Marks Approval where marks are affected.

**BR-I11-EXC-010 — Publication Ownership**  
Once the resulting revised official result has been published, official-result correction/republication remains owned by L2-I10.

---

## 9. Events Published — Amendment

The following canonical L2-I11 events are added:

- `Review Request Withdrawn`
- `Review Reopened`
- `Review Outcome Corrected`

### Publication semantics

| Event | Published When | Canonical Publisher |
|---|---|---|
| `Review Request Withdrawn` | A valid Revaluation or Rechecking request reaches `Withdrawn` before review execution. | L2-I11 — Revaluation & Rechecking |
| `Review Reopened` | An authorized completed/verified Revaluation returns to active review within the permitted reopening boundary. | L2-I11 — Revaluation & Rechecking |
| `Review Outcome Corrected` | A corrected L2-I11 review outcome supersedes a prior finalized outcome before revised-result publication. | L2-I11 — Revaluation & Rechecking |

Existing canonical events and publishers remain unchanged.

---

## 10. Downstream Traceability

This amendment supplies missing upstream architecture for:

- `L3-I-120` — Revaluation Request Withdrawal to Closed Revaluation Request;
- `L3-I-121` — Rechecking Request Withdrawal to Closed Rechecking Request;
- `L3-I-124` — Revaluation Evaluator Conflict of Interest to Governed Replacement Evaluation;
- `L3-I-126` — Persistent Revaluation Difference to Adjudicated Revised Marks;
- `L3-I-131` — Revaluation Reopening to Valid Authoritative Outcome; and
- `L3-I-132` — Finalized Revaluation Correction to Traceable Authoritative Outcome.

These parents require controlled L3 revalidation before their successor interpretation or dependent L4 specifications may advance.

---

## 11. Dependency and Architecture-Level Review

No new foundational or runtime dependency class is introduced. Existing L2-I11 collaborations with Person & Identity, Organisation, Finance, Academic Calendar, Result Processing, Result Publication and Platform Foundation remain unchanged.

The amendment defines business workflow, rules, lifecycle and events only. APIs, database structures, services, UI, IAM implementation, infrastructure and technical recovery mechanisms remain outside L2.

---

## 12. Locked Decisions

1. All six affected L3 scenario outcomes remain within Revaluation & Rechecking ownership.
2. Review withdrawal is one shared L2 mechanism for Revaluation and Rechecking, not two duplicate workflows.
3. Evaluator conflict resolution and adjudicating revaluation are exception paths inside the existing Revaluation workflow.
4. Reopening and finalized-outcome correction are distinct lifecycle boundaries.
5. Only the three events in §9 are newly created by this amendment.
6. No L1 ownership change is required.
7. No automatic L3 or L4 approval follows from this amendment.
