# Persistent Revaluation Difference to Adjudicated Revised Marks

## 1. Specification Metadata

**Specification ID:** L4-I-126
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Examination — Revaluation & Rechecking
**Realizes Scenario:** L3-I-126
**Primary Business Owner:** Revaluation & Rechecking
**Version:** 1.0.1
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-04
**Depends On:** L3-I-126 — canonical parent interpreted through the Locked Section I v3 successor baseline; L2-I11 — Locked primary verification source and Locked exception amendment; `Section_I_L3_Event_Integrity_and_Boundary_Remediation_Amendment_v3_2026-09-03.md` — Architecture Approved and Locked authoritative corrective overlay; L4 Governance and Architecture Standard; L4 Validation Standard
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP must realize **Persistent Revaluation Difference to Adjudicated Revised Marks** by requiring the source-defined adjudicating revaluation when valid revaluation outcomes remain outside the permitted tolerance and authoritative Examination Policy requires additional adjudication.

The adjudicating activity remains within L2-I11 Revaluation & Rechecking. The lifecycle follows `Review In Progress` → `Review Completed` → `Outcome Verified` → `Revised Marks Approved` for a revised-marks outcome. It is not transferred to L2-I08 and does not create a separate adjudication lifecycle.

## 3. Required Business Context

- Authoritative valid revaluation outcomes whose difference remains outside the source-defined permitted tolerance.
- Authoritative Examination Policy requiring adjudicating/additional revaluation in that condition.
- Existing L2-I11 review context and source-supported evaluator/authority conditions.
- Prior revaluation evidence required to establish the adjudication basis.

Only authoritative business context identified by the approved successor baseline and Locked L2-I11 amendment may be used. L4 does not invent a tolerance, adjudicator, event or separate workflow.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-126-BH-001 | When valid revaluation outcomes remain outside the permitted tolerance and authoritative Examination Policy requires adjudication, the system shall prevent revised marks from becoming final until the required adjudicating revaluation is completed. | L3-I-126 successor interpretation; L2-I11 `BR-I11-EXC-005` |
| L4-I-126-BH-002 | The system shall preserve the differing valid revaluation outcomes and the authoritative policy basis that requires adjudication. | L3-I-126 successor interpretation; L2-I11 `BR-I11-EXC-005` |
| L4-I-126-BH-003 | The system shall keep the adjudicating revaluation inside the existing L2-I11 Revaluation workflow and `Review In Progress`; it shall not transfer the activity to L2-I08 or create a separate adjudication workflow. | L3-I-126 successor interpretation; Section I v3 §5.4 |
| L4-I-126-BH-004 | The system shall establish adjudicated revised marks only after the review reaches `Review Completed`, the outcome reaches `Outcome Verified`, and revised marks receive the source-defined approval where required. | L3-I-126 successor interpretation; Section I v3 §5.4 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-126-BH-005 | When the adjudicating revaluation commences, the system shall publish `Revaluation Started` under the canonical L2-I11 publisher. | L3-I-126 successor interpretation; Section I v3 §5.4 |
| L4-I-126-BH-006 | When the adjudicating review completes, the system shall publish `Review Completed` under the canonical L2-I11 publisher. | L3-I-126 successor interpretation; Section I v3 §5.4 |
| L4-I-126-BH-007 | When the adjudicated outcome is verified, the system shall publish `Review Outcome Verified` under the canonical L2-I11 publisher. | L3-I-126 successor interpretation; Section I v3 §5.4 |
| L4-I-126-BH-008 | When revised marks are authoritatively approved, the system shall publish `Revised Marks Approved` under the canonical L2-I11 publisher. | L3-I-126 successor interpretation; Section I v3 §5.4 |
| L4-I-126-BH-009 | The system shall not publish `Adjudication Completed` or any other adjudication-specific event not defined by the Locked source architecture. | L3-I-126 successor interpretation; Section I v3 §§5.4, 6 |

## 6. Failure / Rejection / Continuation

- If authoritative policy does not require adjudication, the system shall not create an adjudicating review path through this scenario.
- If the required adjudicating revaluation is incomplete, revised marks shall not become final through this scenario.
- If the adjudicated outcome has not been verified, progression to revised-marks approval shall remain blocked.
- The last valid review position and all prior valid revaluation evidence shall be preserved while a required condition remains unresolved.

## 7. Audit & Evidence Behavior

The system shall preserve the differing revaluation outcomes, policy/tolerance basis, adjudicating revaluation commencement, review completion, outcome verification, revised-marks approval where applicable, and the resulting adjudicated marks. Evidence shall remain attributable to L2-I11 Revaluation & Rechecking.

## 8. Observable Completion Conditions

This L4 behavior is complete when:

- the authoritative policy condition requiring adjudication was established;
- the adjudicating revaluation remained inside L2-I11;
- the review progressed through `Review In Progress`, `Review Completed`, `Outcome Verified` and `Revised Marks Approved` where required;
- the four canonical L2-I11 events were published at their supported transitions; and
- no adjudication-specific event or L2-I08 ownership transfer was introduced.

## 9. Traceability & Provenance

**Parent:** L3-I-126 — Persistent Revaluation Difference to Adjudicated Revised Marks
**Authoritative successor overlay:** `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/Section_I_L3_Event_Integrity_and_Boundary_Remediation_Amendment_v3_2026-09-03.md`
**Primary verification source:** L2-I11 — Locked Revaluation & Rechecking workflow architecture and Locked exception amendment

Every BH retains L3-I-126 as its single parent. The Locked successor overlay establishes the adjudicating revaluation semantics without changing the parent identity or owner.

## 10. Open Issues / Deferred Design

No upstream architecture blocker remains for this specification. It remains in `Review` pending L4 Architecture Approval.

API/interface design, persistence, event transport, technical access control, infrastructure and UI/UX remain deferred.