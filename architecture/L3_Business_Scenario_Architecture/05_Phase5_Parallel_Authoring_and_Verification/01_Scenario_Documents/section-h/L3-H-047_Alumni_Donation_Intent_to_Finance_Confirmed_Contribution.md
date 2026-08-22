# Alumni Donation Intent to Finance-Confirmed Contribution

# 1. Scenario Metadata

**Scenario ID:** L3-H-047  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Alumni  
**Participating Domains:** F, H  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-F00 — Finance Domain Model & Foundation; L2-F02 — Payment Management Architecture; L2-H06 — Alumni Workflows  
**Referenced By:** Alumni fundraising engagement and Finance settlement scenarios

# 2. Scenario Purpose

Coordinate an H06-owned alumni donation intent with Finance-owned monetary settlement while preserving strict separation between alumni relationship engagement and financial authority.

# 3. Business Outcome

Finance authoritatively records the monetary outcome and H06 records the alumni relationship contribution reference

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An alumnus initiates a donation or sponsorship contribution.

## 4.2 End Boundary

The frozen outcome requires a Finance-authoritative monetary outcome followed by an H06 relationship contribution reference. Current locked sources conflict on the consumed Finance event and do not define the H06 post-payment reference update.

## 4.3 In Scope

Donation intent capture, campaign/pledge preferences, H06 event publication, Finance ownership of settlement, and source-preserving receipt of a Finance outcome.

## 4.4 Out of Scope

H06 payment processing/accounting, Finance modification of H06 Donation Intent, locally renamed payment events, or an invented contribution-confirmation lifecycle.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Finance / F | Owns Payment, monetary settlement, financial state, payment events, and accounting consequences. |
| Alumni / H | Owns Donation Intent, campaign participation, donation preferences, and alumni engagement history only. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Alumni | Initiates the donation/sponsorship contribution and supplies H06-owned intent/preferences. | Initiates |
| Alumni Administrator | May submit/update/withdraw H06 Donation Intent where authorized upstream. | Performs H06 administration |

Finance F02 does not define a canonical business actor for the settlement decision; no Finance actor is invented here.

# 7. Preconditions and Entry Criteria

- Active Alumni Membership exists.
- Active fundraising campaign exists.
- Donation Intent is eligible for submission under H06.
- Finance remains the sole owner of monetary settlement and accounting consequences.

# 8. Trigger

An alumnus initiates a donation or sponsorship contribution

# 9. Main Business Flow

1. Receive the alumni donation or sponsorship intent.
2. Capture only H06-owned campaign, pledge, recognition, anonymity, and engagement preferences.
3. Publish the canonical H06 donation-intent business fact.
4. Finance independently receives and governs the monetary settlement under Finance-owned rules.
5. Preserve Finance as authoritative for the payment/settlement outcome.
6. Stop before recording the frozen H06 contribution reference because the current H06–F02 completion contract is contradictory and no H06 post-payment reference action/event is defined.

# 10. Alternate Business Flows

H06 permits Donation Intent update or withdrawal before/according to its defined lifecycle. Finance-owned payment exceptions remain Finance scenarios and do not change H06 ownership.

# 11. Failure, Interruption and Business Recovery

If Finance does not produce an authoritative settlement outcome, H06 must not treat the donation intent as a confirmed monetary contribution. Contract mismatch must be resolved upstream rather than by renaming an event locally.

# 12. Business Rules and Decision Constraints

- H06 captures donation intent only.
- Payment processing remains exclusively Finance-owned.
- Donation Intent must not create accounting entries or store payment transactions.
- Every monetary settlement is represented by the Finance Payment Aggregate.
- Finance financial history remains authoritative and immutable under its rules.

# 13. State and Lifecycle Impact

H06 Donation Intent states are `Draft → Submitted → Updated` with permitted withdrawal/resubmission paths. F02 Payment progresses independently through its Finance-owned lifecycle and settlement states. No H06 `Finance Confirmed Contribution` state is defined.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H06 publishes `DonationIntentSubmitted`, `DonationIntentUpdated`, and `DonationIntentWithdrawn`.

H06 lists a consumed Finance event named `PaymentCompleted`, while locked F02 defines canonical payment events including `PaymentSettlementCompleted` and does not define `PaymentCompleted`.

## 14.2 Business Contracts and Cross-Domain Hand-offs

H06 hands donation intent to Finance without transferring monetary ownership. A canonical Finance-to-H06 event/contract and H06 action for recording the frozen alumni contribution reference are not consistently defined.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Alumni initiate H06 Donation Intent. Finance owns financial settlement decisions; F02 does not define a business actor for that authority.

## 15.2 Separation of Duties, Overrides and Exceptions

H06 shall never infer financial confirmation from intent submission. L3 shall not rename `PaymentSettlementCompleted` to `PaymentCompleted` or invent a bridging event.

## 15.3 Business Audit and Evidence Obligations

Preserve Donation Intent history, campaign/pledge preferences, Finance settlement reference when a valid contract exists, financial ownership boundary, and final relationship contribution evidence.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The frozen outcome can complete only when Finance publishes an authoritative monetary outcome through an approved contract and H06 has a defined action to record the corresponding alumni relationship contribution reference.

## 16.2 Postconditions

Until the contract is resolved, Donation Intent and Finance Payment histories remain independently authoritative and H06 does not claim a confirmed contribution.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- H06 declares consumed Finance event `PaymentCompleted`; locked F02 defines canonical `PaymentSettlementCompleted` and no `PaymentCompleted`.
- F02's multi-domain contracts do not define Alumni/H06 as a payment-outcome consumer.
- H06 defines no command, state transition, or canonical event for recording the frozen Finance-confirmed alumni relationship contribution reference.
- The frozen final outcome therefore depends on a missing/contradictory mandatory cross-domain contract.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream architecture must resolve the Finance-to-H06 payment outcome contract and define the H06 contribution-reference completion action without transferring Finance ownership.
