# Subsequent Examination Application, Eligibility and Attempt Authorization

**Scenario ID:** L3-I-133  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Supplementary, Backlog & Improvement Examination  
**Participating Domains:** C — Student Lifecycle; F — Finance; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-22  
**Depends On:** Locked L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-I12 Supplementary / Backlog / Improvement Examination Execution; L2-I04 Examination Registration; applicable result and Finance workflows  
**Referenced By:** Applicable downstream Examination workflows

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-I-133` |
| Canonical Title | Subsequent Examination Application, Eligibility and Attempt Authorization |
| Primary Owner | Supplementary, Backlog & Improvement Examination |
| Participating Domains | C, F, I |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Draft |
| Authoritative L2 | `L2-I12` — Supplementary / Backlog / Improvement Examination Execution Workflows |

The locked I-RC-03 ownership resolution is controlling: L2-I04 owns the Examination Registration aggregate and lifecycle for every category; L2-I12 owns subsequent-examination administration and attempt lifecycle management and initiates registration through I04.

## 2. Scenario Purpose

Determine whether a candidate is eligible for a supplementary, backlog, or improvement examination opportunity and, where all applicable conditions are satisfied, authorise the subsequent-examination attempt for governed institutional processing without redefining academic eligibility, Finance truth, or Examination Registration ownership.

## 3. Business Outcome

An eligible subsequent-examination application becomes an authorised examination attempt under I12, with the required Examination Registration initiated through I04. If an authoritative condition is not satisfied, no attempt is authorised.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
A subsequent examination opportunity is available and a candidate applies.

### 4.2 End Boundary
The application is authorised as a governed attempt with required registration initiation, or is rejected/withheld with the decision preserved.

### 4.3 In Scope
- Published-result eligibility.
- Candidate academic context.
- Attempt-limit verification.
- Applicable financial verification.
- Application review and approval.
- Registration initiation through I04.
- Attempt lifecycle establishment and audit evidence.

### 4.4 Out of Scope
- Examination-policy definition.
- Student Lifecycle state changes.
- Finance transaction processing or financial ownership.
- Examination Registration lifecycle ownership.
- Session/schedule creation, Hall Ticket issuance, conduct, evaluation, or official result publication.

### 4.5 Related Scenarios — Optional
Downstream scheduling, Hall Ticket, conduct, evaluation, and result workflows proceed only through their canonical owners.

## 5. Ownership and Participating Domains

- **L2-I12:** owns supplementary/backlog/improvement administration, eligibility verification, application handling, fee verification coordination, session allocation, and attempt lifecycle management.
- **L2-I04:** owns Examination Registration and its lifecycle for all registration categories.
- **Student Lifecycle:** provides authoritative candidate/academic context.
- **Finance:** provides authoritative financial eligibility/status where applicable.

## 6. Business Actors, Roles and Responsibilities

- Candidate — submits the application.
- Examination Administrator / Supplementary Examination Officer — administer and review the request.
- Controller of Examination / applicable examination authority — provides governed approval where required.
- Student Lifecycle — supplies authoritative academic context.
- Finance — supplies authoritative financial status.
- Examination Registration (I04) — establishes the canonical registration when initiated.

## 7. Preconditions and Entry Criteria

- A subsequent examination opportunity is institutionally available.
- Candidate context is authoritative.
- Relevant official examination result is published.
- Applicable supplementary/backlog/improvement policy is active.
- Prior attempts can be established.
- Applicable financial condition can be determined.

## 8. Trigger

A candidate submits an application for a subsequent examination opportunity.

## 9. Main Business Flow

1. Identify candidate and requested opportunity.
2. Consume authoritative academic context and published result information.
3. Determine eligibility under the applicable examination policy.
4. Verify prior attempts and permitted attempt limits.
5. Consume applicable Finance eligibility/status.
6. Validate application and requested examination scope.
7. Obtain required examination approval.
8. Authorise the application for governed registration initiation.
9. Initiate Examination Registration through L2-I04.
10. Establish the attempt within the I12 lifecycle.
11. Preserve decision, approval, and attempt history.
12. Publish applicable I12 lifecycle events and hand off to downstream canonical workflows.

## 10. Alternate Business Flows

- Supplementary, Backlog, and Improvement eligibility follow their configured policy variants within the unified I12 framework.
- Unsatisfied Finance condition remains Finance-owned and prevents bypass.
- Ineligible candidate receives a governed rejected/withheld outcome.
- An authorised policy exception may alter eligibility only through the owning policy authority.

## 11. Failure, Interruption and Business Recovery

Missing published results, conflicting Student context, exhausted attempt limits, unresolved Finance eligibility, or failure to establish the required I04 registration prevents downstream participation. Each deficiency is resolved through its canonical owner; I12 does not invent substitute facts.

## 12. Business Rules and Decision Constraints

- Subsequent-examination eligibility follows institutional examination policy.
- Supplementary, Backlog, and Improvement operate within the I12 administration framework.
- I04 owns Examination Registration for all categories.
- Application approval alone does not create or own a registration.
- Finance retains financial truth.
- Attempt history must remain complete and traceable.
- No new eligibility, attempt, or registration state is introduced.

## 13. State and Lifecycle Impact

I12's subsequent-examination governance begins with `Eligible` and progresses through `Applied` and `Registered`, followed by the governed execution stages. Attempt authorisation is expressed within that lifecycle; no separate `Authorized` state is invented. The Examination Registration lifecycle remains exclusively I04-owned.

## 14. Business Events and Cross-Domain Collaboration

L2-I12 identifies lifecycle facts including Supplementary Eligibility Confirmed, Supplementary Registration Completed, Examination Scheduled, Hall Ticket Initiated, Examination Conduct Started, Supplementary Evaluation Initiated, Supplementary Result Processing Initiated, and Attempt Closed.

L2-I04 remains the canonical publisher of Examination Registration lifecycle events. No separate `AttemptAuthorized` event is invented. Student Lifecycle and Finance supply authoritative facts without acquiring attempt ownership.

## 15. Business Authorization, Approval and Audit Obligations

Eligibility and approval follow authoritative examination policy. Application, eligibility basis, Finance condition where applicable, attempt history, approval, and registration initiation must remain auditable. Any exception to attempt limits requires an authoritative institutional decision.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Eligibility established.
- Applicable financial condition satisfied.
- Required approval complete.
- Application authorised for the subsequent attempt.
- I04 registration initiated as required.
- Attempt history and applicable lifecycle facts preserved.

### 16.2 Postconditions
An authorised subsequent-examination attempt exists under I12 and downstream processing may proceed through canonical owners. A rejected/withheld application creates no authorised attempt.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
No registration-ownership conflict remains: the locked I-RC-03 resolution explicitly assigns Examination Registration to I04 and subsequent-examination administration to I12.

### 17.2 Assumptions
Institution-specific eligibility, attempt-limit, fee, and approval values remain policy/configuration governed.

### 17.3 Deferred Questions
Technical orchestration and implementation mechanics remain outside L3.
