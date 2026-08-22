# Library Resource Issue to Active Borrowing

**Scenario ID:** L3-G-131  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, C, E, F, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-131 |
| Canonical Title | Library Resource Issue to Active Borrowing |
| Primary Owner | Library Management |
| Participating Domains | G, C, E, F, H |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the complete institutional outcome in which an eligible library member requests a library resource, the member and resource are validated against the applicable borrowing policy, the issue is approved, and an active borrowing is established.

Library Management owns circulation and borrowing. Student identity/status remains owned by Section C, personnel identity remains owned by Section E, Finance remains authoritative for accounting/payment, and Section H owns notification delivery. Participation does not transfer circulation ownership.

The L2 defines the circulation workflow as:

`Member Verified → Copy Available → Issue Approved → Borrowing Active → Renewal / Return → Circulation Closed`.

It also defines borrowing validation using Active Membership, Borrowing Limit, Outstanding Dues, Resource Availability, Reservation Priority and Administrative Restrictions.

## 3. Business Outcome

A valid Library Member has been approved to borrow an eligible Library Copy and the borrowing is Active, with the applicable due date and borrowing history established.

The physical copy is no longer available for an ordinary new issue while the borrowing is Active.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A library member requests issue of an available Library Copy.

### 4.2 End Boundary

The issue is approved and the borrowing reaches the canonical Active condition.

### 4.3 In Scope

- Member identification and verification.
- Active membership validation.
- Copy identification and availability.
- Borrowing-policy validation.
- Outstanding-dues/restriction assessment where applicable.
- Reservation-priority assessment.
- Issue approval.
- Borrowing establishment.
- Due-date determination under policy.
- Circulation history.
- `ResourceIssued` business event.
- Notification hand-off where required.

### 4.4 Out of Scope

- Membership creation/activation.
- Student identity changes.
- Employee lifecycle.
- Financial accounting/payment.
- Fine calculation itself.
- Reservation creation.
- Physical copy registration.
- Technical authentication, APIs, databases, queues or UI.

## 5. Ownership and Participating Domains

### Library Management — G

Owns Library Copy, Circulation and Borrowing operations.

### Student Lifecycle — C

Provides authoritative student identity/status where the member is a student. C does not own the borrowing.

### People & Workforce — E

Provides authoritative personnel context where the member is faculty/staff. E does not own the borrowing.

### Finance — F

Owns accounting/payment processing. Library Management may use outstanding dues as a borrowing validation condition, but does not execute financial accounting.

### Communication — H

Consumes business events for governed notifications. H owns delivery, not circulation.

## 6. Business Actors, Roles and Responsibilities

### Librarian / Circulation Desk Operator

Verifies the member, validates borrowing eligibility, and performs or authorises the issue according to configured authority.

### Library Member

Requests the resource and receives the borrowed resource after approval.

### C Authority

Provides student identity/status context where applicable.

### E Authority

Provides personnel context where applicable.

### F Finance

Provides accounting/payment status where relevant to borrowing validation; does not approve the library issue.

### H Communication

Consumes the issue outcome for applicable notification.

The G05 authorization matrix identifies Librarian / Circulation Desk as typical authorization for issuing resources.

## 7. Preconditions and Entry Criteria

1. The member can be identified.
2. The member has an Active Membership.
3. A valid Library Copy exists.
4. The copy is Available for the requested circulation outcome.
5. The applicable borrowing policies are available.
6. Required reservation/priority information can be evaluated.
7. Required circulation authority is available.

## 8. Trigger

A library member requests issue of an available Library Copy.

## 9. Main Business Flow

### Phase A — Verify Member

1. Identify the member.
2. Confirm that the member has an Active Membership.
3. Establish applicable member category and institutional context where required.
4. Determine the member's borrowing eligibility under the configured policy.
5. Check borrowing limits.
6. Consider outstanding dues where configured.
7. Consider administrative restrictions where configured.

### Phase B — Verify Resource

8. Identify the requested Library Copy.
9. Confirm that the copy is Available.
10. Verify that the resource is eligible for borrowing.
11. Evaluate reservation priority where a reservation exists.
12. Confirm that the requested issue does not violate a configured restriction.

### Phase C — Approve and Establish Borrowing

13. Complete the issue approval.
14. Establish the borrowing record.
15. Record the issue date/time and applicable due date under the configured borrowing policy.
16. Move the borrowing to Active.
17. Move the physical copy from Available into its issued/active-borrowing operational condition.
18. Preserve the circulation history.
19. Publish `ResourceIssued`.

### Phase D — Complete

20. Make the Active borrowing available to subsequent renewal/return workflows.
21. Provide the issue outcome to H where a governed notification is required.
22. End the scenario.

## 10. Alternate Business Flows

### 10.1 Membership Is Not Active

The issue is not approved. The member must first satisfy the applicable membership lifecycle requirements.

### 10.2 Borrowing Limit Exceeded

The issue is rejected because the configured maximum borrowing condition is not satisfied.

### 10.3 Outstanding Dues Restrict Borrowing

Where configured policy makes outstanding dues a borrowing restriction, the issue is not approved until the applicable business condition is resolved.

Finance remains owner of payment/accounting; G does not calculate or settle the payment in this scenario.

### 10.4 Resource Is Not Available

The issue does not proceed. If the member wants the resource, the applicable reservation workflow may be used separately.

### 10.5 Reservation Priority Applies

The issue is evaluated against the configured reservation priority. The reservation workflow remains separately owned by G05.

### 10.6 Resource Is Restricted

Where the resource is governed by a configured restricted-resource policy, the issue is rejected unless the member satisfies the applicable configured authority/condition.

### 10.7 Issue Approval Is Rejected

The Library Copy remains under its existing authoritative operational condition and no Active borrowing is established.

## 11. Failure, Interruption and Business Recovery

### 11.1 Member Eligibility Cannot Be Established

The issue is not approved until authoritative membership/identity context is available.

### 11.2 Copy Availability Cannot Be Established

The issue is not approved until the copy's authoritative operational condition is known.

### 11.3 Business Invariant Is Violated

The transaction is rejected because G05 states that violation of business invariants rejects the transaction. Every active borrowing must reference one valid Library Member, and every Circulation transaction must reference one active Library Copy.

### 11.4 Notification Is Required

The business issue outcome is handed to H. Notification delivery is outside this scenario.

No technical retry, queue, rollback, or infrastructure behaviour is specified.

## 12. Business Rules and Decision Constraints

Borrowing validation may consider:

- Active Membership.
- Borrowing Limit.
- Outstanding Dues.
- Resource Availability.
- Reservation Priority.
- Administrative Restrictions.

Borrowing policies may define:

- Maximum Borrowings.
- Borrowing Duration.
- Renewal Eligibility.
- Reservation Priority.
- Restricted Resources.

Policy definitions remain configurable.

The following invariants also apply:

- Every active borrowing references one valid Library Member.
- Every Circulation transaction references one active Library Copy.
- Borrowing history is never deleted.
- Every lifecycle transition is audited.
- Finance owns accounting/payment processing.

No local borrowing limit, duration, fine threshold, or restriction is invented.

## 13. State and Lifecycle Impact

The G05 Circulation lifecycle is:

`Issued → Active → Returned → Closed`

This scenario establishes the Active borrowing condition following issue approval.

The Library Copy participates in its own lifecycle:

`Added → Available → Issued → Returned → Withdrawn`

The scenario therefore connects two authoritative business outcomes without merging their ownership.

No new state such as `Borrowing Pending`, `Issue Processing`, or `Loan Active` is invented.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Event

The canonical event is:

**`ResourceIssued` — Library resource issued.**

The event is published by Library Management.

No `BorrowingActivated` event is invented.

### 14.2 Business Collaboration

- C supplies authoritative student context where applicable.
- E supplies authoritative personnel context where applicable.
- F provides/consumes financial status information where applicable without transferring accounting ownership.
- H consumes the issue business event for notification.
- G remains the canonical owner and publisher.

## 15. Business Authorization, Approval and Audit Obligations

The G05 authorization matrix identifies **Librarian / Circulation Desk** as typical authorization for issuing resources, while authorization policies remain configurable.

The issue decision, eligibility outcome, borrowing details, due date and circulation history must remain auditable.

No technical access control/IAM implementation is defined.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Member verified.
- Active Membership confirmed.
- Borrowing eligibility validated.
- Library Copy identified and available.
- Applicable restrictions/reservation priority evaluated.
- Issue approved.
- Active borrowing established.
- Due date established.
- Copy moved out of ordinary availability.
- `ResourceIssued` published.
- Circulation history preserved.

### Postconditions

- Borrowing is Active.
- Member is the valid borrower for the circulation transaction.
- Library Copy is no longer ordinarily Available for another issue.
- Borrowing may proceed to Renewal or Return.
- Historical circulation information remains preserved.
- H may notify the relevant stakeholder.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

No material ownership, lifecycle or publisher conflict identified.

### Assumptions

- Exact borrowing limits and durations remain configurable.
- Exact notification requirement remains policy driven.
- Reservation priority is evaluated according to the configured reservation policy.

### Deferred Questions

None material to the frozen scenario outcome.
