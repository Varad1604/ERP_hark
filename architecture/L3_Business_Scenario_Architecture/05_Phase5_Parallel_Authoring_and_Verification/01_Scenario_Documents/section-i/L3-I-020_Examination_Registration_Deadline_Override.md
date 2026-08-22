# Examination Registration Deadline Override

**Scenario ID:** L3-I-020  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination Registration  
**Participating Domains:** A, C, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Locked L1 Business Domain Architecture; L2-I04 Examination Registration Workflows; L2-I01 Examination Policy & Configuration; L2-I02 Examination Session Management; L2-I03 Examination Planning & Scheduling; authoritative Platform Authorization and Audit architecture  
**Referenced By:** Downstream examination registration consumers; None yet for this L3 scenario

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-I-020 |
| Canonical Title | Examination Registration Deadline Override |
| Primary Owner | Examination Registration |
| Participating Domains | A, C, I |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs an authorised institutional exception to an examination registration deadline when an examination registration is otherwise subject to a closed or expired approved registration window.

The purpose is to ensure that a deadline exception is handled as a controlled business decision within Examination Registration governance, with appropriate authority, justification, auditability, and preservation of registration integrity.

## 3. Business Outcome

An authorised examination registration deadline exception is established for the applicable registration context, allowing the affected registration activity to proceed under the approved exception while preserving the authoritative Examination Registration governance and audit history.

## 4. Scenario Boundary and Scope

**Start Boundary:** An examination registration requires consideration after the applicable approved registration period has closed or the registration deadline has otherwise passed.

**In Scope:**
- Recognition of the exceptional deadline circumstance.
- Validation that an institutional exception may be considered.
- Authorised review and decision on the exception.
- Recording the business justification and outcome.
- Allowing the registration workflow to proceed only when the exception is approved.
- Preserving auditability and historical traceability.

**Out of Scope:**
- Definition or revision of institution-wide examination policy.
- Creation or revision of the normal registration window.
- Candidate academic eligibility rules owned by participating domains.
- Financial eligibility rules owned by Finance.
- Hall ticket generation or examination conduct.
- Technical access control or implementation mechanisms.

**End Boundary:** The deadline exception has been approved or rejected and the registration workflow has a clear governed business disposition.

## 5. Ownership and Participating Domains

**Primary Owner — Examination Registration:** Owns the business governance of the Examination Registration and the exception outcome within the registration workflow.

**Platform Foundation (A):** Provides authoritative institutional authorization and audit capabilities consumed by the business workflow; it does not own the registration exception decision.

**Student (C):** Provides the affected candidate context and participates in the registration outcome as applicable; Student ownership remains with the Student domain.

**Examination (I):** Owns Examination Registration governance and the resulting business disposition.

## 6. Business Actors, Roles and Responsibilities

- **Examination Administrator:** Manages examination registration operations and the exceptional registration case.
- **Controller of Examination:** Governs examination registration policies and approvals and provides the required institutional authority for exceptional administrative action where applicable.
- **Candidate (Student):** Is the candidate whose registration is affected by the deadline exception.
- **Authorised Institutional Officer:** Performs a controlled administrative override only where the required institutional authority exists.

No technical role or permission model is defined at L3.

## 7. Preconditions and Entry Criteria

- An Examination Session is Active.
- An Examination Schedule has been officially published.
- The candidate and registration context are identifiable.
- The applicable registration window or deadline has passed or is otherwise closed.
- The requested action falls within an institutional process that permits an authorised exception.
- The affected registration remains subject to Examination Registration eligibility and policy requirements.
- The required institutional authority for an exceptional override is available.

## 8. Trigger

A candidate or institutional registration case requires examination registration after the approved registration period and an authorised institutional exception is requested for consideration.

## 9. Main Business Flow

1. The exceptional deadline circumstance is identified within the Examination Registration context.
2. The applicable registration context, candidate, examination session, and registration requirements are established.
3. The registration case is reviewed against the governing registration-window and exception requirements.
4. The authorised institutional decision-maker considers the exceptional request.
5. The business justification and decision are recorded as part of the registration governance.
6. When the exception is approved, the Examination Registration workflow is permitted to proceed under the approved exception.
7. The resulting registration activity remains subject to the applicable eligibility, review, approval, confirmation, and locking lifecycle requirements.
8. The exception and resulting registration activity are preserved in the registration audit history.

## 10. Alternate Business Flows

### 10.1 Exception Approved Before Registration Completion

Where the exception is approved while the registration remains otherwise eligible and capable of proceeding, registration continues through the applicable Examination Registration workflow.

### 10.2 Exception Applies to an Administrative Registration Case

Where the exceptional case is managed institutionally rather than initiated directly by the candidate, the authorised institutional process may establish the exception while preserving the candidate-centric registration context.

### 10.3 Exception Considered Under Configured Institutional Policy

Where institutional policy provides a configured exceptional registration category or window, the registration proceeds according to that approved policy without changing the canonical registration workflow.

## 11. Failure, Interruption and Business Recovery

### 11.1 Exception Not Authorised

If the required institutional authority is not established, the deadline exception shall not be approved and the registration shall not proceed outside the approved registration window.

### 11.2 Exception Rejected

If the authorised decision-maker rejects the exception, the registration remains governed by the closed registration window and the case receives a rejected or otherwise non-progressing business disposition.

### 11.3 Eligibility or Policy Requirement Not Satisfied

Approval of a deadline exception does not remove other applicable registration eligibility or policy requirements. If those requirements are not satisfied, the registration shall not progress to confirmation.

### 11.4 Required Business Evidence Not Established

If the required business justification for the exceptional action is not established, the override shall not be completed until the institutional governance requirement is satisfied.

## 12. Business Rules and Decision Constraints

- Registration normally occurs only within an approved institutional registration window.
- Registration outside an approved window requires an authorised institutional exception.
- Exceptional administrative overrides require appropriate authority.
- Every override shall record business justification.
- Every override shall generate complete audit records and preserve historical traceability.
- A deadline exception does not transfer ownership of registration, student, policy, curriculum, or other upstream business information.
- An exception does not bypass applicable registration eligibility, approval, confirmation, or locking requirements.
- Institutional configuration may govern exception behaviour without changing the canonical registration workflow.

## 13. State and Lifecycle Impact

The scenario affects the Examination Registration lifecycle at the registration-window and registration progression boundary.

The approved exception permits an otherwise late registration activity to enter or continue the governed registration lifecycle. It does not create a new lifecycle state or rename an existing state.

Where the registration subsequently satisfies the applicable workflow requirements, it may progress through Registration Review, Registration Approval, Registration Confirmation, and later Registration Locking. If the exception is rejected, the registration does not progress outside the approved window.

## 14. Business Events and Cross-Domain Collaboration

The Examination Registration domain remains the canonical publisher for registration lifecycle events.

Material events include:

- **Registration Initiated** — Examination Registration.
- **Eligibility Validated** — Examination Registration.
- **Registration Submitted** — Examination Registration.
- **Registration Approved** — Examination Registration, when the registration receives institutional approval.
- **Registration Confirmed** — Examination Registration, when the registration becomes operationally valid.
- **Registration Locked** — Examination Registration, when registration is frozen for examination operations.

The exceptional administrative override is an audit-governed business action. L2-I04 does not define a separate canonical override event, so no new event is invented here.

Cross-domain collaboration may consume candidate and institutional information while ownership remains with the originating domain.

## 15. Business Authorization, Approval and Audit Obligations

- The deadline exception requires appropriate institutional authority.
- The authorised institutional officer or governing examination authority must act within the authority defined upstream.
- The exception decision must have a recorded business justification.
- Every significant registration activity and administrative override must be auditable.
- Audit history must preserve regulatory compliance, institutional governance, historical traceability, and administrative accountability.
- No technical access control, permission code, IAM mechanism, or enforcement design is defined at L3.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- The deadline exception has an explicit authorised business decision.
- The decision and business justification are recorded.
- The registration case has a clear governed disposition.
- If approved, the registration is permitted to continue only through the applicable Examination Registration lifecycle.
- Audit history is preserved.

**Postconditions:**
- An approved exception permits the affected registration to proceed under the authorised business decision; or
- a rejected exception leaves the registration governed by the applicable closed registration window.
- No upstream ownership or lifecycle definition is changed.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-I04 defines exceptional administrative override requirements but does not specify a dedicated override lifecycle state or a distinct canonical override event. This scenario therefore uses the existing registration lifecycle and existing registration events without introducing new architecture.
2. The exact institutional officer responsible for every type of deadline exception is not defined in L2-I04; the scenario therefore uses the source-supported term **Authorised Institutional Officer** and does not assign a new role.
3. Any institution-specific deadline-exception authority or approval threshold remains governed by the applicable Examination Policy and institutional configuration.
