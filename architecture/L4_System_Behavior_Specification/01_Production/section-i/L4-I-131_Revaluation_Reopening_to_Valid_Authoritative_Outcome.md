# Revaluation Reopening to Valid Authoritative Outcome

## 1. Specification Metadata

**Specification ID:** L4-I-131
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Examination — Revaluation & Rechecking
**Realizes Scenario:** L3-I-131
**Primary Business Owner:** Revaluation & Rechecking
**Version:** 1.1.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-04
**Depends On:** L3-I-131 — canonical parent interpreted through the Locked Section I v3 successor baseline; L2-I11 — Locked primary verification source and Locked exception amendment; `Section_I_L3_Event_Integrity_and_Boundary_Remediation_Amendment_v3_2026-09-03.md` — Architecture Approved and Locked authoritative corrective overlay; L4 Governance and Architecture Standard; L4 Validation Standard
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP must realize **Revaluation Reopening to Valid Authoritative Outcome** by recognizing an authorized reopening of a completed/verified revaluation, returning the review to the earliest valid review stage required to cure the authorized defect, and supporting a newly verified authoritative outcome without erasing prior review evidence.

The behavior begins only after `Review Completed` / `Outcome Verified`, when a policy-authorized institutional review authority establishes a material authorized basis for reopening, and before final revised-marks approval. It ends when the reopened review has been completed and its outcome reverified, with revised-marks approval occurring only where the reopened review produces revised marks requiring approval.

## 3. Required Business Context

- An authoritative revaluation review that has reached `Review Completed` / `Outcome Verified`.
- A material authorized reopening basis.
- A policy-authorized institutional review authority.
- The review has not passed the final revised-marks approval boundary that would make this reopening path inapplicable.
- Candidate representation may supply evidence but does not itself constitute reopening authority.

Only authoritative context defined by the approved successor baseline and Locked L2-I11 amendment may be used. No new approver, reopening authority, lifecycle state or event is introduced by L4.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-131-BH-001 | The system shall prevent reopening unless a policy-authorized institutional review authority and a material authorized reopening basis exist after `Review Completed` / `Outcome Verified` and before final revised-marks approval. | L3-I-131 successor interpretation; L2-I11 `BR-I11-EXC-006` |
| L4-I-131-BH-002 | When reopening becomes authoritative, the system shall return the review lifecycle to `Review In Progress` at the earliest valid stage required to cure the authorized defect. | L3-I-131 successor interpretation; L2-I11 `BR-I11-EXC-006` |
| L4-I-131-BH-003 | The system shall preserve the prior completed and verified review evidence as historical evidence and shall not replace it with the reopened review position. | L3-I-131 successor interpretation; L2-I11 `BR-I11-EXC-007` |
| L4-I-131-BH-004 | The system shall require the reopened review to complete and its outcome to be reverified before a new authoritative review outcome is established. | L3-I-131 successor interpretation; L2-I11 reopening lifecycle |
| L4-I-131-BH-005 | The system shall recognize `Revised Marks Approved` only where the reopened review produces revised marks that require approval under the source-defined review lifecycle. | L3-I-131 successor interpretation; Section I v3 §5.5 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-131-BH-006 | When authorized reopening becomes authoritative, the system shall publish `Review Reopened` under the canonical L2-I11 publisher. | L3-I-131 successor interpretation; Section I v3 §5.5; L2-I11 exception amendment |
| L4-I-131-BH-007 | When the reopened review completes, the system shall publish `Review Completed` under the canonical L2-I11 publisher. | L3-I-131 successor interpretation; Section I v3 §5.5 |
| L4-I-131-BH-008 | When the reopened outcome is revalidated, the system shall publish `Review Outcome Verified` under the canonical L2-I11 publisher. | L3-I-131 successor interpretation; Section I v3 §5.5 |
| L4-I-131-BH-009 | The system shall not invent a candidate-requested reopening event or transfer reopening authority to a candidate, another domain, or the system itself. | L3-I-131 successor interpretation; L2-I11 `BR-I11-EXC-006` |

## 6. Failure / Rejection / Continuation

- A candidate representation without the required institutional reopening authority shall not reopen the review.
- If the material authorized basis is absent, the existing verified review position shall remain authoritative.
- If final revised-marks approval has already crossed the source-defined reopening boundary, this scenario shall not establish reopening.
- Once validly reopened, progression to a new authoritative outcome shall remain blocked until the reopened review completes and its outcome is reverified.

## 7. Audit & Evidence Behavior

The system shall preserve the prior completed/verified outcome, the authorized reopening basis, the responsible institutional review authority, the `Review Reopened` occurrence, the return to `Review In Progress`, evidence considered during the reopened review, and the resulting completed and reverified outcome. Historical evidence shall remain traceable.

## 8. Observable Completion Conditions

This L4 behavior is complete when:

- the reopening authority and material basis were validated;
- an authorized reopening returned the review to `Review In Progress`, or an invalid reopening attempt preserved the existing authoritative position;
- prior review evidence remained historically traceable;
- the reopened review, when admitted, reached `Review Completed` and `Outcome Verified` again;
- required canonical events were published by L2-I11; and
- no downstream result publication/republication outcome was absorbed.

## 9. Traceability & Provenance

**Parent:** L3-I-131 — Revaluation Reopening to Valid Authoritative Outcome
**Authoritative successor overlay:** `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/Section_I_L3_Event_Integrity_and_Boundary_Remediation_Amendment_v3_2026-09-03.md`
**Primary verification source:** L2-I11 — Locked Revaluation & Rechecking workflow architecture and Locked exception amendment

Every BH retains L3-I-131 as its single parent. The Locked successor overlay supplies the previously missing reopening authority, lifecycle transition and canonical `Review Reopened` event without changing scenario identity or ownership.

## 10. Open Issues / Deferred Design

No upstream architecture blocker remains for this specification. It remains in `Review` pending L4 Architecture Approval under the L4 governance and validation standards.

API/interface design, persistence, event transport, technical access control, infrastructure and UI/UX remain deferred.