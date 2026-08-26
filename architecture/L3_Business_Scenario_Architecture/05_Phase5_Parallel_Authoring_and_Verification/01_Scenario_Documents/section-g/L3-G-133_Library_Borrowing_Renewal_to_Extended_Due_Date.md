# Library Borrowing Renewal to Extended Due Date

**Scenario ID:** L3-G-133  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, C, H  
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
| Final L3 ID | L3-G-133 |
| Canonical Title | Library Borrowing Renewal to Extended Due Date |
| Primary Owner | Library Management |
| Participating Domains | G, C, H |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the complete outcome in which an Active borrowing is reviewed for renewal, the member and resource satisfy the configured renewal conditions, the renewal is approved, and the borrowing receives an extended due date.

The L2 explicitly defines renewal eligibility using:

- No Reservation Exists.
- Renewal Limit Not Exceeded.
- Membership Active.
- Resource Eligible.

Renewal rules remain configurable.

The scenario does not create a new borrowing or reset the borrowing history. It extends the existing borrowing according to policy.

## 3. Business Outcome

An eligible Active borrowing is renewed and receives an extended due date, while the borrowing remains governed by the applicable membership, reservation, resource and renewal policies.

The renewal is recorded in immutable borrowing history and the Library Copy remains associated with the same active borrowing.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An Active borrowing is submitted for renewal.

### 4.2 End Boundary

The renewal is approved and the borrowing's due date is extended according to the effective renewal policy.

### 4.3 In Scope

- Identify Active borrowing.
- Verify member/membership.
- Check reservation existence.
- Check renewal limit.
- Verify resource eligibility.
- Apply configured renewal policy.
- Extend due date.
- Record renewal history.
- Publish `ResourceRenewed`.
- Notify through H where applicable.

### 4.4 Out of Scope

- New borrowing creation.
- Reservation creation.
- Membership activation.
- Fine calculation/payment.
- Resource return.
- Student/personnel lifecycle.
- Technical implementation.

## 5. Ownership and Participating Domains

### Library Management — G

Owns circulation, borrowing, renewal policy application and Library Copy operational management.

### Student Lifecycle — C

Provides authoritative student context where applicable. C does not own borrowing or renewal.

### Communication — H

Consumes the renewal outcome for governed notification.

The frozen scenario includes C because student context may participate in eligibility; this does not transfer borrowing ownership.

## 6. Business Actors, Roles and Responsibilities

### Librarian / Circulation Desk Operator

Processes or authorises renewal according to configured library authority.

### Library Member

Requests renewal.

### C Authority

Provides student context where applicable.

### H

Consumes the renewal event for notification where required.

The exact operational authorization remains configurable.

## 7. Preconditions and Entry Criteria

1. An Active borrowing exists.
2. The member has an Active Membership.
3. The Library Copy remains associated with the borrowing.
4. The renewal request can be evaluated against configured renewal rules.
5. The member has not exceeded the configured renewal limit.
6. No reservation exists where the policy prohibits renewal.
7. The resource remains eligible for renewal.

## 8. Trigger

A member requests renewal of an Active borrowing.

## 9. Main Business Flow

### Phase A — Identify the Borrowing

1. Identify the Active borrowing.
2. Confirm the member.
3. Confirm the associated Library Copy.
4. Confirm that the borrowing is still within the applicable renewal conditions.

### Phase B — Validate Renewal Eligibility

5. Confirm that the member has an Active Membership.
6. Determine whether a reservation exists for the resource.
7. Apply the configured reservation rule.
8. Determine whether the renewal limit has been exceeded.
9. Determine whether the resource remains eligible for renewal.
10. Apply any other configured renewal policy conditions.

### Phase C — Approve Renewal

11. Complete the configured renewal approval.
12. Determine the extended due date according to the effective borrowing policy.
13. Record the renewal against the existing borrowing.
14. Preserve prior borrowing history.
15. Publish `ResourceRenewed`.

### Phase D — Complete

16. Treat the new due date as the effective due date.
17. Keep the borrowing Active.
18. Make the renewed outcome available to subsequent return/renewal processing.
19. Provide the renewal outcome to H where notification is required.
20. End the scenario.

## 10. Alternate Business Flows

### 10.1 Reservation Exists

Where configured policy prohibits renewal when a reservation exists, renewal is rejected and the existing due date remains authoritative.

The reservation itself remains governed by Reservation Management.

### 10.2 Renewal Limit Exceeded

The renewal is rejected and the current due date remains unchanged.

### 10.3 Membership Is No Longer Active

Renewal is rejected because the L2 requires Active Membership for renewal.

### 10.4 Resource Is Not Eligible

The renewal is rejected because the resource is not eligible under configured policy.

### 10.5 Administrative Restriction Applies

The renewal is rejected or requires configured review according to the applicable library policy.

### 10.6 Renewal Approved With Extended Due Date

The existing borrowing remains Active and the due date is extended according to policy.

## 11. Failure, Interruption and Business Recovery

### 11.1 Eligibility Cannot Be Established

The renewal does not become effective until the authoritative business conditions can be evaluated.

### 11.2 Reservation Status Cannot Be Established

Renewal is not approved until the applicable reservation condition is known.

### 11.3 Renewal Decision Is Rejected

The existing borrowing and due date remain authoritative.

### 11.4 Historical Information Is Required

Borrowing history is preserved. Renewal does not erase the prior due date or prior circulation history.

No technical retry, queue, rollback or infrastructure behaviour is introduced.

## 12. Business Rules and Decision Constraints

Renewals may be permitted when:

- No Reservation Exists.
- Renewal Limit Not Exceeded.
- Membership Active.
- Resource Eligible.

Renewal rules remain configurable. Borrowing policies may define maximum borrowings, borrowing duration, renewal eligibility, reservation priority and restricted resources.

The scenario does not invent:

- a fixed renewal count;
- a fixed extension duration;
- a grace period;
- a specific reservation priority;
- a special member category exception.

Those remain policy/configuration concerns.

## 13. State and Lifecycle Impact

The borrowing remains in the Active circulation condition.

The scenario changes the effective due-date information associated with the Active borrowing; it does not introduce a new lifecycle state.

The circulation lifecycle remains:

`Issued → Active → Returned → Closed`

No `Renewed` lifecycle state is invented.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Event

The canonical event is:

**`ResourceRenewed` — Borrowing renewed.**

Library Management is the publisher.

No `DueDateExtended` event is invented.

### 14.2 Business Collaboration

- C provides student context where applicable.
- H consumes the renewal outcome for governed communication.
- Reservation Management supplies the reservation condition where required.
- Library Management remains the owner and publisher of the borrowing/renewal outcome.

## 15. Business Authorization, Approval and Audit Obligations

Renewal follows the configured library authorization policy. The applicable operator must be authorised to perform circulation activity.

The renewal decision, previous and effective due-date information, eligibility outcome and renewal history must remain auditable.

No technical access control/IAM mechanism is defined.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Active borrowing identified.
- Active Membership confirmed.
- Reservation condition evaluated.
- Renewal limit evaluated.
- Resource eligibility evaluated.
- Required approval completed.
- Due date extended according to policy.
- `ResourceRenewed` published.
- Renewal history preserved.

### Postconditions

- Borrowing remains Active.
- New due date is authoritative.
- Previous circulation history remains preserved.
- Resource remains associated with the same borrowing.
- Subsequent return/renewal processing uses the effective due date.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

No material ownership, lifecycle or publisher conflict identified.

### Assumptions

- Exact renewal limits and extension periods are configurable.
- Reservation priority is governed by the configured reservation policy.
- Notification requirements remain policy driven.

### Deferred Questions

None material to the frozen scenario outcome.
