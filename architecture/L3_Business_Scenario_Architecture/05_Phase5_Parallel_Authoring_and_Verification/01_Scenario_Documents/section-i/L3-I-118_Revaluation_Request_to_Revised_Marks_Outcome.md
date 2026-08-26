# 1. Scenario Metadata

# Revaluation Request to Revised Marks Outcome

**Scenario ID:** L3-I-118  
**Primary Owner:** Revaluation & Rechecking  
**Participating Domains:** Person & Identity, Finance, Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.1  
**Author:** Pallav  
**Previous Architecture Approval:** ACR-I-2026-08-21-002 — Approved by Vikram Desai, Enterprise Architecture Governance Lead, 21 August 2026; Conditions: None  
**Previous Post-Approval Lock:** Authorized 21 August 2026 under ACR-I-2026-08-21-002; superseded by corrected v1.1  
**Architecture Re-review:** Architecture Approved on 2026-08-26 after correction of participating-domain, actor, rule and event mappings; repository lock remains a separate governance action  
**Canonical Source Candidate:** CAN-L3-I-123  
**Primary L2 Source:** L2-I11  
**Last Updated:** 2026-08-26

# 2. Scenario Purpose

This scenario governs **revaluation request to revised marks outcome** as one complete institutional business outcome. It applies frozen semantics and locked L2-I11 without redefining upstream architecture.

# 3. Business Outcome

The scenario concludes when an authorized and verified revaluation establishes an approved **Revised Marks Outcome** as the authoritative Revaluation & Rechecking result, with required evidence and downstream hand-offs preserved.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario starts when an identifiable Revaluation Request exists against an officially published examination result and source-defined entry conditions are available.

## 4.2 End Boundary

The scenario ends when the revaluation outcome has been verified and the applicable revised marks have been institutionally approved, or the request reaches a source-supported governed terminal outcome without revising marks.

## 4.3 In Scope

- Verification of published-result, eligibility, application-window and fee conditions where applicable.
- Governed request approval.
- Retrieval of authorized assessment evidence.
- Revaluation under approved institutional regulations.
- Review-outcome verification.
- Approval of revised marks where marks change.
- Preservation of evidence and source-supported events/hand-offs.

## 4.4 Out of Scope

- Rechecking as a distinct administrative-verification scenario except where needed to preserve shared L2-I11 boundaries.
- Grade, grade-point, credit-point, SGPA, CGPA, standing or classification computation.
- Official result publication or republication.
- Ownership of Person & Identity or Finance facts.
- Technical solution, data, interface, access-control, infrastructure or recovery design.

## 4.5 Related Scenarios — Optional

Downstream Result Processing & Computation and Result Publication scenarios may consume the approved revised marks outcome without ownership transfer.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Revaluation & Rechecking | Owns the Revaluation Request, review lifecycle, review outcome and approved revised marks as the authoritative post-publication review outcome. |
| Person & Identity | Provides canonically owned candidate/evaluator/authority identity and role information where required. |
| Finance | Provides authoritative fee/payment confirmation where institutional policy requires a review fee. |
| Result Processing & Computation | Performs downstream academic recomputation where approved revised marks affect academic results; does not own the review decision. |
| Result Publication | Owns official result publication/republication; does not own the revaluation decision. |

The Primary Owner retains the review outcome; participating domains retain their canonical responsibilities.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility |
|---|---|
| Candidate | Submits the Revaluation Request against an officially published result where eligible. |
| Examination Administrator | Administers request registration, validation and governed review processing. |
| Revaluation Officer | Coordinates the revaluation process and its institutional evidence. |
| Subject Evaluator | Performs authorized independent academic reassessment under institutional regulations. |
| Controller of Examination | Exercises review/approval authority where assigned by institutional policy, including approval of revised marks where applicable. |
| Academic Registrar | Participates in institutional review governance where assigned by policy. |
| Department Head | Participates in department-level review/approval where assigned by policy. |
| Academic Committee | Participates in collective institutional review/approval where required. |

These actors are taken from locked L2-I11. Exact institutional approval routing remains configurable and is not replaced by a locally invented fixed sequence.

# 7. Preconditions and Entry Criteria

- The referenced examination result is officially published.
- The Revaluation Request satisfies the applicable institutional eligibility policy or follows an authorized exception path.
- The request is within an approved application window or has authorized exceptional approval.
- Required fee/payment confirmation is available where policy requires payment.
- Required assessment evidence can be authoritatively identified and retrieved.
- Required review and approval authority is available.
- No unresolved ownership, identity, scope or lifecycle conflict prevents progression.

# 8. Trigger

**Candidate Revaluation Request against an officially published examination result.**

This is a business occurrence, not an implementation action.

# 9. Main Business Flow

1. The Candidate submits the Revaluation Request against an officially published result.
2. Revaluation & Rechecking verifies eligibility, applicable application-window conditions and other source-defined entry criteria.
3. Where policy requires a fee, Finance provides authoritative payment/fee confirmation without transferring Finance ownership.
4. The request is approved according to institutional review policy before academic reassessment begins.
5. Authorized assessment evidence is identified and retrieved without modifying the source assessment record.
6. An authorized Subject Evaluator performs the governed academic reassessment under institutional regulations.
7. Revaluation & Rechecking verifies the completed review outcome, including required documentation and policy compliance.
8. Where the review changes marks, authorized institutional authority approves the revised marks.
9. Approved revised marks become the authoritative Revaluation & Rechecking outcome for downstream use.
10. Required evidence and exact source-supported business events/hand-offs are preserved.
11. Where revised marks require downstream academic recomputation or republication, the appropriate downstream owner receives the approved revised marks outcome; this scenario does not perform those downstream outcomes.

# 10. Alternate Business Flows

## 10.1 Request Ineligible

An ineligible request is rejected under institutional review policy with an attributable business reason. No academic reassessment occurs.

## 10.2 Review Window Closed

A request outside the permitted window follows the institution's authorized late-submission/exception path or is rejected according to policy.

## 10.3 Required Fee Not Verified

Where a review fee is mandatory and Finance cannot confirm the required payment, the request remains pending or is rejected according to institutional policy. Revaluation & Rechecking does not create Finance truth.

## 10.4 Answer Script / Assessment Evidence Unavailable

The request follows the source-defined exception procedure until authorized assessment evidence becomes available or an institutional terminal decision is reached.

## 10.5 No Marks Change

Where the governed reassessment confirms the existing marks, the review concludes with a verified no-change outcome. No revised marks are introduced.

# 11. Failure, Interruption and Business Recovery

- Progression stops when required authority, evidence or authoritative input is unavailable.
- Requests failing mandatory eligibility do not proceed to revaluation unless an authorized exception path applies.
- An unavailable evaluator or assessment record leaves the request pending under institutional exception governance.
- Unapproved revised marks do not become authoritative and do not alter academic records.
- Disputed external facts return to their canonical owner.
- The last valid review position and evidence are preserved until the blocking condition is resolved.
- Technical exception and recovery mechanisms are outside L3.

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Scenario Effect | Owner / Source |
|---|---|---|
| Only officially published results are eligible for Revaluation or Rechecking. | Prevents review of provisional/draft results. | L2-I11 §9.1 |
| Every request must satisfy institutional eligibility requirements before acceptance. | Ineligible requests cannot proceed to review. | L2-I11 §9.2 |
| Requests must fall within approved institutional windows unless exceptional approval is granted. | Controls timing and exception handling. | L2-I11 §9.3 |
| Revaluation is an authorized academic reassessment and only authorized evaluators may perform it. | Prevents unauthorized academic reassessment. | L2-I11 §9.5 |
| Revised marks are not authoritative until approved by authorized institutional authority. | Prevents unofficial outcomes changing academic records. | L2-I11 §9.6 |
| Revaluation & Rechecking does not compute grades, grade points, credit points, SGPA, CGPA, standing or classification. | Preserves Result Processing & Computation ownership. | L2-I11 §9.7 |
| Revaluation & Rechecking does not publish official results. | Preserves Result Publication ownership. | L2-I11 §9.8 |
| Significant review activity must remain auditable and traceable. | Preserves review evidence and history. | L2-I11 §9.9 |

No local threshold, deadline, formula, fixed approval hierarchy, state or rule is created.

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry Condition | Scenario Impact | Resulting Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Revaluation Request / Review | Revaluation Request submitted against an officially published result | Eligibility verification, request approval, evidence retrieval, revaluation, outcome verification and revised-marks approval where applicable | Review completed with approved revised marks, verified no-change outcome, or another source-supported terminal disposition | Revaluation & Rechecking | L2-I11 §§5, 8, 9 |

No separate downstream Result Processing or Result Publication lifecycle state is created by this scenario.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

### Published Events

The following exact L2-I11 events are material to this revaluation scenario where their corresponding business transition occurs:

| Event | Canonical Publisher | Published When | Material Purpose |
|---|---|---|---|
| Revaluation Requested | Revaluation & Rechecking | A governed Revaluation Request is registered. | Records initiation of the review request. |
| Review Eligibility Verified | Revaluation & Rechecking | Eligibility verification completes successfully. | Records eligibility completion. |
| Review Fee Verified | Revaluation & Rechecking | Required Finance-owned payment confirmation has been successfully verified where applicable. | Records the review-domain verification outcome without transferring Finance ownership. |
| Review Request Approved | Revaluation & Rechecking | The request is institutionally approved to proceed. | Records governed approval to begin academic review. |
| Answer Script Retrieved | Revaluation & Rechecking | Authorized assessment evidence is made available for review. | Records review-evidence readiness. |
| Revaluation Started | Revaluation & Rechecking | Authorized academic reassessment begins. | Records start of academic reassessment. |
| Review Completed | Revaluation & Rechecking | The reassessment is completed. | Records completion of the review activity. |
| Review Outcome Verified | Revaluation & Rechecking | Institutional outcome verification is complete. | Records verified review result. |
| Revised Marks Approved | Revaluation & Rechecking | Revised marks are authorized by institutional authority. | Establishes the authoritative revised marks review outcome. |
| Result Revision Initiated | Revaluation & Rechecking | Approved revised marks require downstream result revision. | Initiates downstream processing without performing it. |
| Review Closed | Revaluation & Rechecking | The review lifecycle is formally closed. | Records terminal closure where applicable. |

### Consumed Events

Material upstream events defined by locked L2-I11 may include:

| Event | Canonical Publisher | Business Purpose |
|---|---|---|
| Published Result Available | Result Publication | Establishes the officially published result eligible for review. |
| Examination Policy Updated | Examination Policy & Configuration | Supplies authoritative review regulations. |
| Academic Calendar Updated | Academic Calendar Domain | Supplies authoritative review-window/timeline context where applicable. |
| Candidate Information Updated | Person & Identity | Supplies authoritative candidate identity information. |
| Fee Payment Confirmed | Finance | Supplies authoritative payment confirmation where required. |

Only exact source-supported event names are used; business capabilities such as `Revaluation Request Management` or `Revaluation Eligibility` are not treated as events.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider | Consumer | Business Hand-off |
|---|---|---|
| Result Publication | Revaluation & Rechecking | Officially published result and authoritative publication context. |
| Person & Identity | Revaluation & Rechecking | Canonically owned candidate/evaluator/authority identity and role information required for review. |
| Finance | Revaluation & Rechecking | Authoritative fee/payment confirmation where policy requires it. |
| Marks Collection & Evaluation | Revaluation & Rechecking | Authorized assessment evidence required for academic reassessment. |
| Revaluation & Rechecking | Result Processing & Computation | Approved revised marks where downstream recomputation is required. |
| Revaluation & Rechecking | Result Publication | Approved revised outcome / result-revision initiation where republication is required. |

These are business hand-offs only. Ownership remains with the originating domain.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Candidate initiation, request administration, evaluator reassessment, outcome verification and revised-marks approval remain distinct where L2-I11 distinguishes them. Exact approval routing may be configured by institution using the source-supported actors and authority model; no fixed local hierarchy is invented.

## 15.2 Separation of Duties, Overrides and Exceptions

Any administrative override or exceptional review path requires authorized approval, recorded justification and complete audit history as required by L2-I11. No technical permission code is defined here.

## 15.3 Business Audit and Evidence Obligations

The institution must retain the published-result reference, request identity, eligibility basis, application-window result, applicable fee verification, request approval, assessment evidence, evaluator/review evidence, outcome verification, prior marks, proposed/revised marks where applicable, approval authority, final review outcome and downstream initiation evidence.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The Revaluation Request was validly governed against an officially published result.
- Applicable eligibility, window and fee conditions were resolved.
- Required request approval and authorized academic reassessment were completed.
- The review outcome was institutionally verified.
- Any changed marks became authoritative only after authorized revised-marks approval.
- Exact source-supported review events and evidence were preserved where their transitions occurred.
- Any required downstream result revision was initiated without absorbing Result Processing or Result Publication ownership.

## 16.2 Postconditions

- Revaluation & Rechecking owns the authoritative completed review outcome.
- Approved revised marks, where produced, are authoritative input for downstream Result Processing & Computation.
- Official result publication/republication remains owned by Result Publication.
- Person & Identity retains identity/role information ownership.
- Finance retains ownership of fee/payment truth.
- Prior result and review evidence remain historically traceable.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No scenario-specific material conflict remains after the 2026-08-26 correction of participating-domain, actor, rule and event mappings.

## 17.2 Assumptions

No assumption overrides locked architecture. Referenced locked sources are assumed unchanged.

## 17.3 Deferred Questions

System behaviour, technical integrations, persistence, access-control implementation, notification channels, computation design and user-experience design are deferred to later architecture levels.
