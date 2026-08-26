# Overdue Borrowing to Fine Calculation

**Scenario ID:** L3-G-137  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, F, H  
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
| Final L3 ID | L3-G-137 |
| Canonical Title | Overdue Borrowing to Fine Calculation |
| Primary Owner | Library Management |
| Participating Domains | G, F, H |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the complete institutional business outcome in which an active library borrowing passes its applicable due date, the borrowing is recognised as overdue, the applicable library fine is calculated under the authoritative fine policy, and the financial/communication consequences are made available to the appropriate participating domains.

The L2 defines the fine workflow as:

`Due Date Exceeded → Fine Calculated → Member Notified → Finance Reference → Payment Status Updated → Fine Closed`.

Fine Management governs operational calculation of library penalties, while accounting and payment processing remain within Finance.

## 3. Business Outcome

An overdue borrowing has an authoritative fine calculation associated with it, the fine is recorded in Library Management, the canonical overdue/fine business outcomes are published where defined, and Finance/H can consume the appropriate business information without acquiring ownership of Library Fine Management.

The scenario does not settle the financial obligation or close the fine.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The due date of an active borrowing is exceeded without a completed return.

### 4.2 End Boundary

The applicable fine has been calculated and recorded, with the resulting business information available for notification and Finance processing.

### 4.3 In Scope

- Identify the overdue borrowing.
- Confirm member and resource.
- Establish due-date exceedance.
- Determine fine applicability.
- Apply authoritative configurable fine rules.
- Calculate the fine.
- Record the fine.
- Preserve the borrowing/fine relationship.
- Publish canonical overdue/fine events where defined.
- Provide financial reference information to Finance.
- Provide notification outcome to H.

### 4.4 Out of Scope

- Resource return.
- Payment execution.
- Accounting settlement.
- Fine waiver approval.
- Membership suspension.
- New borrowing.
- Technical scheduling, queues, APIs, databases or implementation mechanics.

## 5. Ownership and Participating Domains

### Library Management — G

Owns Circulation, Fine Management, overdue determination within library policy, and fine calculation.

### Finance — F

Owns accounting and payment processing. Finance does not become owner of the Library Fine lifecycle merely because it receives a financial reference.

### Communication — H

Owns notification delivery. H does not calculate or own the fine.

## 6. Business Actors, Roles and Responsibilities

### Library Management / Librarian or Circulation Authority

Ensures overdue borrowing is processed under library policy and that the fine outcome is recorded.

### Library Member

Is the subject of the overdue borrowing and fine.

### Finance

Receives the financial reference and manages payment/accounting processing.

### Communication

Delivers the configured member notification.

## 7. Preconditions and Entry Criteria

1. An active borrowing exists.
2. The borrowing has an authoritative due date.
3. The due date has been exceeded.
4. The member and resource can be identified.
5. Applicable fine policy is available.
6. The borrowing has not already been closed by return processing.

## 8. Trigger

The due date of an active borrowing is exceeded.

## 9. Main Business Flow

### Phase A — Establish Overdue Condition

1. Identify the active borrowing.
2. Identify the member.
3. Identify the borrowed resource.
4. Establish the authoritative due date.
5. Determine that the due date has been exceeded.
6. Record the borrowing as subject to overdue processing.
7. Publish the canonical `ResourceOverdue` event where applicable.

### Phase B — Calculate Fine

8. Determine whether a fine applies under the configured library policy.
9. Determine the applicable resource category.
10. Determine the applicable membership category.
11. Apply the configured grace-period rule where one exists.
12. Determine the applicable number of overdue days.
13. Apply institutional fine policies.
14. Calculate the library fine.
15. Record the fine and its applicable category.
16. Publish the canonical `FineCalculated` event where applicable.

### Phase C — Establish Downstream Business Consequences

17. Make the fine information available for member notification.
18. Provide the applicable Finance reference.
19. Preserve the borrowing/fine history.
20. End the scenario with the fine calculated and outstanding unless another authoritative process changes its status.

## 10. Alternate Business Flows

### 10.1 Grace Period Applies

Where configured, the grace period is applied before determining the chargeable overdue period.

No fixed grace period is invented.

### 10.2 No Fine Is Due

If policy determines that no fine applies, no unsupported financial amount is created. The overdue outcome remains governed by circulation policy.

### 10.3 Member Category Changes the Calculation

The configured membership-category rule is applied. No local category-specific amount is invented.

### 10.4 Resource Category Changes the Calculation

The configured resource-category rule is applied.

### 10.5 Borrowing Is Returned Before Fine Processing Completes

The return workflow remains authoritative for the borrowing's final circulation outcome. The fine calculation follows the applicable return/overdue policy.

### 10.6 Existing Fine Already Recorded

A duplicate fine must not be created where the authoritative business record already represents the same overdue obligation.

## 11. Failure, Interruption and Business Recovery

### 11.1 Due-Date Exceedance Cannot Be Established

Fine calculation does not proceed until the authoritative due date and current borrowing condition are established.

### 11.2 Applicable Policy Cannot Be Established

The fine is not calculated using an invented default. The business decision remains pending authoritative policy.

### 11.3 Fine Calculation Cannot Be Determined

The overdue condition remains known, but the fine outcome is not finalised until the applicable calculation inputs are available.

### 11.4 Finance Processing Is Unavailable

The calculated Library Fine remains owned by G. Finance processing is a downstream business collaboration and does not redefine the library fine outcome.

### 11.5 Notification Is Unavailable

The business notification requirement remains with H; no technical retry or queue behaviour is specified.

## 12. Business Rules and Decision Constraints

Fine calculation may consider:

- Resource Category.
- Membership Category.
- Grace Period.
- Number of Days.
- Institutional Policies.

Calculation rules remain configurable.

Fine categories include examples such as:

- Overdue Fine.
- Lost Resource.
- Damaged Resource.
- Replacement Charge.
- Administrative Penalty.

Categories remain configurable.

Accounting and payment remain within Finance.

No fixed monetary amount, grace period, day-rate, cap or penalty threshold is invented.

## 13. State and Lifecycle Impact

The Fine lifecycle in the L2 state transition matrix is:

`Calculated → Pending → Paid / Waived → Closed`.

This scenario establishes the **Calculated** condition and makes the fine available to subsequent Pending/payment/waiver processing.

The borrowing itself remains governed by the Circulation lifecycle:

`Issued → Active → Renewed → Returned → Closed`.

This scenario does not close the borrowing.

## 14. Business Events and Cross-Domain Collaboration

The canonical business events applicable to this outcome are:

- **`ResourceOverdue`** — due date exceeded.
- **`FineCalculated`** — fine generated.

Library Management is the canonical publisher.

The business event is distinct from the Finance reference and notification hand-off.

### Cross-domain collaboration

- **F:** receives the financial reference and performs accounting/payment processing.
- **H:** receives the business notification outcome and performs communication.
- **G:** remains owner and publisher.

No `PaymentReceived`, `MemberNotified`, or other downstream technical/business event is invented as a G publisher.

## 15. Business Authorization, Approval and Audit Obligations

Fine calculation must follow the configured library operational authorization.

The fine calculation, applicable policy inputs, resulting fine category/amount, overdue basis, member/resource relationship and financial reference must remain auditable.

Fine waiver is a separate process and may require administrative review, approval, reason recording and audit history.

No technical access control/IAM implementation is defined.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Active borrowing identified.
- Due date established.
- Overdue condition established.
- `ResourceOverdue` published where applicable.
- Fine applicability evaluated.
- Configured calculation inputs evaluated.
- Fine calculated.
- `FineCalculated` published where applicable.
- Finance reference established.
- Notification information made available.
- History preserved.

### Postconditions

- A Library Fine is in Calculated condition.
- The fine remains subject to subsequent Pending/Paid/Waived/Closed processing.
- Finance owns payment/accounting.
- H owns notification delivery.
- The borrowing remains governed by Circulation.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

No material ownership conflict identified.

### Assumptions

- Exact fine calculation values remain configurable.
- The canonical events `ResourceOverdue` and `FineCalculated` are used only as defined by the authoritative event catalogue.
- Payment and waiver are separate downstream outcomes.

### Deferred Questions

None material to the frozen scenario outcome.
