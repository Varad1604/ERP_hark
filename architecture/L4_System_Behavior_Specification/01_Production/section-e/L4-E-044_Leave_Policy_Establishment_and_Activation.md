# L4-E-044 — Leave Policy Establishment and Activation

**Scenario ID:** L4-E-044  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-044 — Leave Policy Establishment and Activation  
**Primary Owner:** Leave Management  
**Participating Domains:** B, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata
Derived from locked L3-E-044. Leave Management is the canonical owner of the Leave Policy lifecycle.

## 2. System Behavior Purpose
Specify observable behavior for validating, approving, and activating a governed leave policy.

## 3. System Behavior Outcome
A validated and approved leave policy becomes active and effective according to its governed version and effective-dating rules.

## 4. System Behavior Scope and Boundary
### 4.1 Start Boundary
A leave policy draft enters the validation and activation workflow.
### 4.2 End Boundary
Required validation and approval are complete and the policy is activated.
### 4.3 In Scope
Policy establishment, rule validation, conflict handling, approval, activation, effective dating, and evidence preservation.
### 4.4 Out of Scope
Leave requests, leave balances, accrual, entitlement, ledger transactions, and employee leave decisions.

## 5. Ownership and Participating Domains
Leave Management owns the Leave Policy Aggregate, policy rules, policy version lifecycle, and activation decision. Governance participants contribute to validation/approval where required.

## 6. Business Actors, Roles and Responsibilities
Leave Management manages the policy lifecycle; rule reviewers validate policy rules; approving authority establishes approval where required.

## 7. Preconditions and Entry Criteria
- Draft leave policy exists.
- Applicable rules are available.
- Required governance authority is established.
- Version/effective-date rules can be evaluated.

## 8. Trigger
A leave policy draft enters policy validation.

## 9. Main System Behavior
1. Establish the policy draft.
2. Validate policy rules.
3. Identify conflicts.
4. Resolve identified conflicts through governance and return to validation where required.
5. Obtain required approval.
6. Activate the approved policy.
7. Preserve version and effective-date evidence.
8. Publish the authoritative policy-activation business outcome where defined upstream.

## 10. Alternate System Behaviors
- Validation conflict prevents progression until resolved.
- Approval refusal prevents activation.
- Future-effective activation remains governed by its effective date.

## 11. Failure, Interruption and Business Recovery
A failed validation retains the governed pre-activation state until corrected. A rejected approval prevents activation. No technical recovery behavior is defined.

## 12. Business Rules and Decision Constraints
- Leave Management owns policy decisions and activation.
- Lifecycle follows Draft → UnderReview → Approved → Scheduled → Active → Superseded → Archived.
- Version history and effective dating are preserved.
- External domains cannot directly mutate the Leave Policy Aggregate.

## 13. State and Lifecycle Impact
The Leave Policy advances through its authoritative lifecycle toward Active. No new local state is introduced.

## 14. Business Events and Cross-Domain Collaboration
The locked upstream evidence requires a policy-activation business event but does not expose its canonical identifier. No event name is invented.

## 15. Business Authorization, Approval and Audit Obligations
Validation findings, conflict resolution, approval, activation, version and effective-date evidence remain attributable and auditable.

## 16. Completion Criteria and Postconditions
- Rules validated.
- Conflicts resolved where applicable.
- Required approval recorded.
- Policy activated.
- Effective version established.

## 17. Open Issues, Assumptions and Deferred Questions
Canonical policy-activation event identifier requires confirmation before Architecture Approval/Lock.

**Authoring Classification:** PASS WITH ISSUES — source-grounded Draft.
