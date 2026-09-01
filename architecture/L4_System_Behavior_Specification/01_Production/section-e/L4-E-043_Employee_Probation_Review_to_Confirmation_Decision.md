# L4-E-043 — Employee Probation Review to Confirmation Decision

**Scenario ID:** L4-E-043  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-043 — Employee Probation Review to Confirmation Decision  
**Primary Owner:** HR  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-043. HR owns the probation and confirmation case; Workforce & Engagement owns resulting employment lifecycle facts.

## 2. System Behavior Purpose
Specify observable behavior for processing probation review evidence through a governed confirmation decision.

## 3. System Behavior Outcome
The probation review case reaches a recorded confirmation or non-confirmation decision with required review and approval evidence preserved.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
A probation review case becomes due and enters the HR workflow.
### 4.2 End Boundary
Required review, appraisal, and approval are complete and the confirmation decision is recorded.
### 4.3 In Scope
Review initiation, appraisal capture, review evaluation, decision routing, approval decision, case outcome recording, and evidence preservation.
### 4.4 Out of Scope
Authoritative employment probation/confirmation state transition, employee master data, and technical implementation.

## 5. Ownership and Participating Domains
HR owns the probation/confirmation case and decision record. Workforce & Engagement owns any resulting employment lifecycle state.

## 6. Business Actors, Roles and Responsibilities
HR case owner coordinates the review; reviewing authority evaluates the case; approving authority records the governed decision; Workforce & Engagement receives any required employment-state consequence.

## 7. Preconditions and Entry Criteria
- Workforce engagement is identifiable.
- Probation/confirmation case exists or is due.
- Review and approval requirements are available.

## 8. Trigger
A probation review becomes due.

## 9. Main System Behavior
1. Establish or advance the review case.
2. Capture required appraisal/review evidence.
3. Determine the review outcome.
4. Route for required decision authority.
5. Record confirmation or non-confirmation decision.
6. Preserve the HR case outcome and evidence.
7. Where an employment-state consequence is required, hand the decision to Workforce & Engagement without transferring ownership.

## 10. Alternate System Behaviors
- Incomplete evidence prevents a final decision.
- A non-confirmation outcome is recorded without claiming confirmation.
- Additional review may be required before a final decision.

## 11. Failure, Interruption and Business Recovery
Incomplete review evidence leaves the case pending. An adverse approval outcome records the non-confirmation decision. Employment consequences remain with Workforce & Engagement. No technical recovery mechanics are defined.

## 12. Business Rules and Decision Constraints
- HR owns the probation/confirmation case.
- Employment lifecycle facts remain owned by Workforce & Engagement.
- Approval decisions are attributable.
- Workflow history is preserved as business history.

## 13. State and Lifecycle Impact
The HR case advances through review and decision. No new employment lifecycle state is created by this L4 behavior.

## 14. Business Events and Cross-Domain Collaboration
The locked L3 does not expose a dedicated canonical probation/confirmation event identifier. No event name is invented. Any authoritative employment-state event remains published by Workforce & Engagement.

## 15. Business Authorization, Approval and Audit Obligations
Review evidence, appraisal outcome, approval authority, decision, actor, and timestamp remain traceable.

## 16. Completion Criteria and Postconditions
- Review completed.
- Appraisal/review evidence recorded.
- Required approval recorded.
- Confirmation/non-confirmation decision established.
- Any employment-state consequence handed to the authoritative workforce owner.

## 17. Open Issues, Assumptions and Deferred Questions
Dedicated canonical probation/confirmation event identifier remains to be confirmed before Architecture Approval/Lock.

**Authoring Classification:** PASS WITH ISSUES — source-grounded Draft.
