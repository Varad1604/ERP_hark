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
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; DEPENDENCY_MAP.md; applicable locked L2 Examination workflows  
**Referenced By:** Applicable downstream Examination workflows

## 2. Scenario Purpose
Determine whether a candidate is eligible to apply for a subsequent examination opportunity and, where the application satisfies the applicable academic, financial and examination conditions, authorize the candidate's examination attempt for institutional processing. The scenario governs subsequent-examination administration without redefining academic eligibility rules, financial truth, or Examination Registration ownership.

## 3. Business Outcome
An eligible subsequent-examination application becomes an authorized examination attempt, with the applicable Examination Registration initiated through L2-I04 and the attempt remaining fully traceable. If the candidate is not eligible or a mandatory condition is not satisfied, the application does not become an authorized attempt.

## 4. Scenario Boundary and Scope
**Start Boundary:** A supplementary, backlog or improvement examination opportunity is available and a candidate seeks to apply.

**In Scope:** published-result eligibility; candidate academic context; attempt-limit verification; applicable financial verification; application review; examination-scope selection; registration initiation; attempt authorization; audit evidence.

**Out of Scope:** examination-policy definition; Student Lifecycle state changes; financial truth; Examination Registration ownership; Examination Session creation; schedule creation; Hall Ticket issuance; conduct; evaluation; official result publication.

**End Boundary:** The application is authorized as a subsequent-examination attempt with registration initiated, or is conclusively rejected/withheld.

## 5. Ownership and Participating Domains
**Supplementary, Backlog & Improvement Examination:** owns subsequent-examination administration, eligibility verification, application handling, fee verification, session allocation and attempt lifecycle management.

**Student Lifecycle:** provides authoritative candidate and academic context.

**Finance:** provides authoritative financial eligibility/status where applicable.

**Examination Registration (L2-I04):** owns the Examination Registration aggregate and lifecycle for all registration categories, including Supplementary, Backlog and Improvement. L2-I12 initiates registration but does not own it.

## 6. Business Actors, Roles and Responsibilities
- **Candidate:** submits the subsequent-examination application within permitted scope.
- **Examination Administrator:** manages application review and subsequent-examination administration.
- **Supplementary Examination Officer:** verifies eligibility, prior attempts and application conditions.
- **Controller of Examination:** provides governance/approval where required.
- **Student Lifecycle:** supplies authoritative academic context.
- **Finance:** supplies financial eligibility/status where applicable.
- **Examination Registration:** establishes the canonical registration through L2-I04.

## 7. Preconditions and Entry Criteria
- A subsequent examination opportunity is institutionally available.
- Candidate Student Lifecycle context is authoritative.
- Relevant official examination result information is published.
- Applicable Supplementary/Backlog/Improvement policy is active.
- Prior attempts can be established.
- Applicable financial conditions can be determined.
- The application is within the permitted opportunity or an approved exception applies.

## 8. Trigger
A candidate submits an application for a subsequent examination opportunity.

## 9. Main Business Flow
1. Identify the candidate and requested subsequent-examination opportunity.
2. Establish authoritative academic context and published result information relevant to eligibility.
3. Determine eligibility under the applicable examination policy.
4. Establish prior attempts and verify that another attempt is permitted.
5. Determine applicable financial eligibility/status through Finance.
6. Validate the application and requested subject/examination scope.
7. Obtain required institutional examination approval.
8. Authorize the application for registration.
9. Initiate Examination Registration through L2-I04 for the applicable registration category.
10. Establish the attempt within the L2-I12 lifecycle.
11. Preserve the eligibility decision, approval evidence and attempt history.
12. Publish applicable canonical subsequent-examination lifecycle events.
13. Hand the authorized outcome to downstream scheduling and examination workflows.

## 10. Alternate Business Flows
### 10.1 Supplementary Eligibility
An eligible uncleared subject proceeds as a supplementary attempt.

### 10.2 Backlog Eligibility
An eligible backlog subject proceeds within the permitted attempt framework.

### 10.3 Improvement Eligibility
An eligible improvement request proceeds without treating the prior result as erroneous.

### 10.4 Financial Condition Requires Resolution
The application remains pending the authoritative Finance outcome and does not bypass the condition.

### 10.5 Candidate Is Not Eligible
The application is rejected or withheld under the applicable policy; no attempt or registration is authorized.

## 11. Failure, Interruption and Business Recovery
### 11.1 Published Result Missing
Eligibility is not inferred; the application remains unresolved until the authoritative result exists.

### 11.2 Student Context Conflict
Student Lifecycle remains authoritative for its own context; examination administration does not create a replacement academic status.

### 11.3 Attempt Limit Exhausted
No further attempt is authorized unless a separately governed policy exception permits one.

### 11.4 Financial Eligibility Cannot Be Established
The application does not become authorized on assumption; Finance remains authoritative.

### 11.5 Eligibility Changes Before Registration
The application is re-evaluated against the current authoritative position.

### 11.6 Registration Not Established
The application approval alone does not authorize downstream examination participation when the required L2-I04 registration lifecycle has not been established.

## 12. Business Rules and Decision Constraints
- Subsequent-examination eligibility is determined under institutional examination policy.
- Supplementary, Backlog and Improvement are categories within the unified L2-I12 administration framework.
- Eligibility relies on authoritative published results and applicable academic regulations.
- Attempt limits must be respected.
- Applicable financial conditions remain Finance-owned.
- Examination Registration remains owned by L2-I04.
- Administrative application approval does not itself create an Examination Registration.
- Downstream participation requires the required registration lifecycle position.
- Attempt history remains complete and traceable.
- No new eligibility, attempt or registration state is created.

## 13. State and Lifecycle Impact
L2-I12's subsequent-examination governance begins with **Eligible** and progresses through **Applied** and **Registered**, followed by the execution stages defined by L2-I12. Attempt authorization is expressed within that existing lifecycle; no new `Authorized` state is invented.

The Examination Registration lifecycle remains the L2-I04 lifecycle.

## 14. Business Events and Cross-Domain Collaboration
L2-I12 identifies:
- **Supplementary Eligibility Confirmed**
- **Supplementary Registration Completed**
- **Examination Scheduled**
- **Hall Ticket Initiated**
- **Examination Conduct Started**
- **Supplementary Evaluation Initiated**
- **Supplementary Result Processing Initiated**
- **Attempt Closed**

L2-I04 remains the publisher of Examination Registration lifecycle events. No separate `AttemptAuthorized` event is invented.

Student Lifecycle and Finance provide authoritative facts without receiving ownership of the attempt.

## 15. Business Authorization, Approval and Audit Obligations
- Eligibility follows approved examination policy.
- Required institutional approval is obtained.
- Financial eligibility remains a Finance-owned decision.
- Application, eligibility basis, attempt history, approval and registration initiation are auditable.
- Any exception to attempt limits requires an authoritative institutional decision.
- Academic eligibility, financial eligibility and examination administration remain separated.

## 16. Completion Criteria and Postconditions
**Completion Criteria:** eligibility established; applicable financial condition satisfied; required approval complete; application authorized; L2-I04 registration initiated; attempt history preserved; applicable lifecycle events published.

**Authorized Outcome:** the candidate has an authorized subsequent-examination attempt within L2-I12 and downstream scheduling may proceed.

**Rejected/Withheld Outcome:** no subsequent attempt is authorized and the decision remains traceable.

## 17. Open Issues, Assumptions and Deferred Questions
1. L2-I12 explicitly owns subsequent-examination administration while L2-I04 owns the registration aggregate and lifecycle; this scenario preserves that split.
2. The available L2-I12 material does not define a dedicated `Attempt Authorized` event; none is invented.
3. Institution-specific eligibility, attempt and fee values remain policy/configuration governed and are not defined locally.
