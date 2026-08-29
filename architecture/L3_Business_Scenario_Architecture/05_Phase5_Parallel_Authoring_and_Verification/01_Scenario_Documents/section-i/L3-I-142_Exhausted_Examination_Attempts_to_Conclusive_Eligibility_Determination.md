# Exhausted Examination Attempts to Conclusive Eligibility Determination

**Scenario ID:** L3-I-142  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Supplementary, Backlog & Improvement Examination  
**Participating Domains:** C — Student Lifecycle; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L2-I12 Supplementary / Backlog / Improvement Examination Execution; L2-I01 Examination Policy & Configuration; applicable Student Lifecycle eligibility context  
**Referenced By:** Downstream Examination and Student Lifecycle workflows as applicable

## 1. Scenario Metadata

The frozen Stage 3 canonical semantics are preserved without reclassification. This scenario is owned by Supplementary, Backlog & Improvement Examination and is Critical/P1.

The final ID and exact Phase 5 filename are preserved.

## 2. Scenario Purpose

Determine the candidate's authoritative subsequent-examination eligibility when the permitted attempt opportunity has been exhausted, so that no further Supplementary, Backlog, or Improvement attempt is admitted without an applicable governing policy basis.

## 3. Business Outcome

A conclusive eligibility position is established for the affected candidate and examination scope. The exhausted attempt history remains traceable and any downstream academic consequence is based on the authoritative eligibility outcome.

## 4. Scenario Boundary and Scope

**Start Boundary:** The candidate's subsequent-examination attempt history reaches the point at which the applicable permitted-attempt policy may be exhausted.

**In Scope:** establish candidate and examination context; establish authoritative attempt history; apply the applicable attempt-limit/eligibility policy; determine whether eligibility is exhausted or another permitted route remains; record the authoritative eligibility position; and route downstream consequences.

**Out of Scope:** redefining attempt policy, altering Student Lifecycle academic status, changing examination results, or creating an alternative attempt lifecycle.

**End Boundary:** A conclusive eligibility determination exists for the affected subsequent-examination scope.

## 5. Ownership and Participating Domains

Supplementary, Backlog & Improvement Examination owns the subsequent-examination eligibility and attempt administration.

Examination Policy & Configuration owns the applicable examination policy/configuration.

Student Lifecycle provides authoritative student academic context and consumes examination outcomes where applicable.

The scenario does not transfer policy or Student Lifecycle ownership to L3.

## 6. Business Actors, Roles and Responsibilities

- Supplementary Examination Officer: reviews the candidate's attempt position.
- Examination Administrator: coordinates the eligibility determination.
- Examination Policy authority: provides the applicable attempt policy.
- Student Lifecycle: supplies authoritative student context and consumes the resulting academic fact where required.

No new authority is introduced.

## 7. Preconditions and Entry Criteria

- Candidate context is established.
- The relevant Supplementary, Backlog or Improvement examination scope is identified.
- Prior attempt history is available and traceable.
- The applicable attempt/eligibility policy is available.
- The current examination eligibility position has not already been conclusively closed.

## 8. Trigger

The authoritative attempt history reaches the point where the applicable permitted-attempt rule must determine whether the candidate remains eligible.

## 9. Main Business Flow

1. Identify the candidate, examination category, subject/scope and relevant academic context.
2. Establish the authoritative sequence of prior subsequent-examination attempts.
3. Confirm that the attempts being counted are valid attempts under the applicable examination policy.
4. Determine the permitted-attempt position under the authoritative policy.
5. If a permitted attempt remains, preserve the candidate as eligible and continue through the normal subsequent-examination lifecycle.
6. If the permitted attempts are exhausted, establish the conclusive ineligible/exhausted position through L2-I12.
7. Preserve the attempt history and eligibility basis.
8. Publish the applicable canonical eligibility/attempt lifecycle fact.
9. Route the resulting eligibility position to downstream workflows that require it.
10. End when the candidate has one authoritative eligibility position for the affected scope.

## 10. Alternate Business Flows

**Attempt remains available:** the candidate continues through the existing eligibility/application path.

**Different examination scope:** an exhausted position for one subject/scope does not automatically exhaust unrelated permitted opportunities.

**Policy exception applies:** where the governing policy explicitly permits an exceptional route, that route is evaluated under the owning policy/approval workflow rather than bypassing the attempt policy.

**Historical attempt correction:** if an earlier attempt is found not to be a valid counted attempt, the authoritative history is corrected through the owning workflow before the final eligibility position is determined.

## 11. Failure, Interruption and Business Recovery

**Attempt history is incomplete:** no conclusive exhaustion decision is made until the authoritative history is established.

**Policy cannot be determined:** the scenario remains unresolved; no local attempt limit is invented.

**Conflicting attempt records:** the canonical attempt/registration workflows resolve the underlying record conflict before exhaustion is concluded.

**Candidate context conflict:** Student Lifecycle remains authoritative for its own student context.

**Exceptional decision required:** the case is routed to the applicable authorized examination policy/exception path; the scenario does not create an informal override.

## 12. Business Rules and Decision Constraints

- Attempt eligibility is governed by the authoritative examination policy.
- Only valid attempts count toward the applicable permitted-attempt position.
- Exhaustion for one examination scope does not automatically create exhaustion for unrelated scopes.
- Supplementary/Backlog/Improvement attempt history remains uniquely identifiable and traceable.
- The scenario applies policy; it does not redefine policy.
- Student Lifecycle remains the canonical owner of student academic state.

## 13. State and Lifecycle Impact

The subsequent-examination attempt lifecycle remains the L2-I12 lifecycle. No new `AttemptsExhausted` lifecycle state is introduced unless already defined upstream.

The scenario establishes the eligibility outcome at the business boundary; it does not replace the attempt lifecycle with a separate eligibility state machine.

## 14. Business Events and Cross-Domain Collaboration

L2-I12 remains the canonical publisher for subsequent-examination eligibility and attempt lifecycle facts. The accessible authoritative material does not provide a dedicated event name specifically for `AttemptsExhausted`; therefore no new event name is invented.

Student Lifecycle consumes applicable examination facts through its canonical cross-domain collaboration boundary.

## 15. Business Authorization, Approval and Audit Obligations

- Attempt-limit determination follows the authoritative examination policy and its configured approval/exception rules.
- Any exceptional eligibility decision must use the authority defined upstream.
- The attempt history, policy basis and final eligibility determination must be auditable.
- No technical authorization mechanism is specified.

## 16. Completion Criteria and Postconditions

**Complete when:** candidate and scope are established; valid attempt history is reconciled; authoritative policy is applied; the candidate is conclusively eligible for a remaining permitted attempt or conclusively ineligible because the permitted attempts are exhausted; the decision basis is traceable; and applicable downstream facts are published.

**Postconditions:** one authoritative eligibility position exists; no additional attempt may proceed unless the governing policy independently permits it; historical attempts remain preserved.

## 17. Open Issues, Assumptions and Deferred Questions

1. The frozen Stage 3 title establishes the conclusive eligibility outcome, but it does not itself define the numerical attempt limit. That value remains upstream policy-owned.
2. The accessible L2-I12 evidence confirms the subsequent-examination attempt lifecycle and policy-governed attempt limits, but a dedicated exhausted-attempt event name is not available here.
3. If the repository's detailed L2 policy defines an exceptional attempt route, that route must be consumed rather than recreated in this scenario.
