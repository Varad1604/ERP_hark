# Student Scholarship Eligibility Determination and Recommendation

**Scenario ID:** L3-C-064  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Scholarship Eligibility  
**Participating Domains:** C  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-C07 — Scholarship Eligibility Workflows; materially applicable upstream scholarship-policy, Student Lifecycle, Person & Identity, academic, document and institution sources  
**Referenced By:** Related Section C scholarship scenarios listed below

## 1. Scenario Metadata
The frozen Section C identity is preserved. Scholarship Eligibility remains the canonical owner. The scenario applies L2-C07 without transferring ownership of upstream facts or downstream financial processing.

## 2. Scenario Purpose
Determine whether a Student is eligible for a defined scholarship opportunity under the authoritative scholarship policy and, where supported by the resulting eligibility, produce the canonical scholarship recommendation.

## 3. Business Outcome
The Student Scholarship Eligibility Record contains an authoritative eligibility outcome for the scholarship opportunity, with an explainable recommendation where applicable and complete decision history preserved.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
A scholarship opportunity is available and a Student's authoritative information is sufficient to evaluate its eligibility criteria.

### 4.2 End Boundary
The eligibility decision and any applicable recommendation are recorded in the Student Scholarship Eligibility Record and published for downstream consumption.

### 4.3 In Scope
- Identify the scholarship definition, policy and Student.
- Gather authoritative eligibility facts from upstream domains.
- Evaluate the Student against the scholarship criteria and policy.
- Record the eligibility decision and explanation.
- Where eligible, determine the applicable scholarship recommendation.
- Preserve the decision history/version.
- Publish the canonical eligibility outcome for downstream consumers.

### 4.4 Out of Scope
- Scholarship financial approval.
- Scholarship allocation or amount calculation.
- Payment/disbursement.
- Student lifecycle change.
- Rewriting upstream academic, identity or document facts.

### 4.5 Related Scenarios
C063 — Scholarship Eligibility Policy Lifecycle Governance; C065 — Conditional Scholarship Eligibility Resolution or Expiry; C066 — Scholarship Eligibility Reassessment; C067 — Scholarship Renewal Evaluation; C071 — Scholarship Eligibility to Financial Aid Processing.

## 5. Ownership and Participating Domains
The **Student Scholarship Eligibility Record** is the canonical aggregate owned by Scholarship Eligibility. Upstream domains retain ownership of identity, student lifecycle, academic, document and policy facts. Downstream financial approval, allocation, disbursement and accounting remain outside Scholarship Eligibility.

## 6. Business Actors, Roles and Responsibilities
- Scholarship Eligibility owns evaluation and the Student Scholarship Eligibility Record.
- Upstream domains remain authoritative for the facts consumed by the evaluation.
- Downstream Finance consumes eligibility outcomes and owns financial processing; it does not re-evaluate eligibility.

No technical access control/technical identity profiles are introduced.

## 7. Preconditions and Entry Criteria
- Student exists in an applicable lifecycle position.
- Scholarship definition/policy is available.
- Required authoritative evidence is available or its absence has a policy-defined eligibility consequence.
- Student Scholarship Eligibility Record can be established or updated under L2-C07.

## 8. Trigger
An eligible scholarship opportunity and applicable Student evaluation context trigger eligibility determination.

## 9. Main Business Flow
1. Resolve the scholarship definition and effective eligibility policy.
2. Resolve authoritative Student identity, lifecycle, academic, document and other applicable facts.
3. Evaluate all applicable criteria.
4. Determine the canonical eligibility outcome.
5. Record the explanation and supporting decision evidence.
6. If the outcome permits recommendation, generate the applicable scholarship recommendation.
7. Preserve the eligibility history/version.
8. Publish the canonical eligibility outcome (`EligibilityGranted`, `EligibilityConditionallyGranted`, or `EligibilityRejected`) using the publisher defined by L2-C07.

## 10. Alternate Business Flows
- Missing required evidence may produce a policy-defined pending/conditional outcome rather than an invented failure.
- If criteria are not satisfied, record `EligibilityRejected`.
- If criteria are satisfied subject to governed conditions, record `EligibilityConditionallyGranted`.
- If all required criteria are satisfied, record `EligibilityGranted`.
- Recommendation does not constitute financial approval or allocation.

## 11. Failure, Interruption and Business Recovery
Eligibility evaluation remains authoritative within C07. Downstream financial domains cannot replace the eligibility decision. Corrections caused by changed upstream facts are handled through the canonical reassessment scenario, not by rewriting historical decisions.

Only business-level recovery is described. No technical retry, queue, rollback or infrastructure behavior is introduced.

## 12. Business Rules and Decision Constraints
L2-C07 establishes eligibility policy evaluation as the canonical C07 responsibility and explicitly separates eligibility from financial processing. The exact criterion-level rules remain those of the applicable scholarship policy; no new eligibility rule is created here.

## 13. State and Lifecycle Impact
Student Scholarship Eligibility Record moves through the exact eligibility lifecycle defined by L2-C07. The scenario does not invent state labels.

No new lifecycle state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration
`EligibilityGranted`, `EligibilityConditionallyGranted`, and `EligibilityRejected` are explicitly identified in L2-C07 as published eligibility outcomes. Their canonical publisher remains Scholarship Eligibility / Student Scholarship Eligibility Record. No additional event name is invented.

Canonical event names and publishers must remain those defined by locked L2-C07. No new event is invented in this L3.

## 15. Business Authorization, Approval and Audit Obligations
Eligibility determination must be explainable, auditable and governed by the applicable scholarship policy. Financial approval is a separate downstream responsibility.

## 16. Completion Criteria and Postconditions
- Applicable scholarship policy identified.
- Authoritative Student facts evaluated.
- Eligibility outcome recorded.
- Explanation/history preserved.
- Recommendation generated where applicable.
- Canonical eligibility outcome published.

## 17. Open Issues, Assumptions and Deferred Questions
**OI-C064-01:** The accessible L2-C07 extract confirms the canonical eligibility outcome vocabulary but does not expose the full criterion catalogue. L3 therefore does not invent criterion-level rules.

**OI-C064-02:** Exact event-envelope/lifecycle vocabulary beyond the named outcome events requires verification against the detailed L2-C07 event/state sections before locking.

**Classification:** PASS WITH ISSUES — business outcome and ownership are source-grounded; detailed policy criteria remain upstream.

## Production Verification Record
- [x] Frozen ID/title/owner/domains/type/priority preserved.
- [x] L2-C07 ownership boundary preserved.
- [x] Financial processing excluded from Scholarship Eligibility.
- [x] No invented technical architecture.
- [x] All 17 mandatory sections present.
- [x] Unverified upstream semantics explicitly recorded rather than invented.
- [x] Status remains Draft under Phase 5 governance.
