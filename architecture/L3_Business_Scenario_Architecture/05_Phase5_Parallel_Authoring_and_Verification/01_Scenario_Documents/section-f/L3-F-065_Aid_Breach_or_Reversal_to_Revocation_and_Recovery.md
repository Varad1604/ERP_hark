# Aid Breach or Reversal to Revocation and Recovery

**Scenario ID:** L3-F-065  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Scholarship & Financial Aid  
**Participating Domains:** C, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-F11 — Scholarship & Financial Aid; L2-C07 — Scholarship Eligibility Workflows; applicable Student Charge & Fee Management and Accounts Receivable architecture  
**Referenced By:** Student Charge & Fee Management; Accounts Receivable; Financial Reporting; Audit; Executive Dashboards

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-F-065 |
| Canonical Title | Aid Breach or Reversal to Revocation and Recovery |
| Primary Owner | Scholarship & Financial Aid |
| Participating Domains | C, F |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern the institutional response when an active financial-aid outcome is subject to a breach or authoritative reversal condition requiring revocation and recovery.

Scholarship & Financial Aid owns the financial-assistance revocation decision and preserves the historical assistance record. It does not retroactively rewrite prior approval or allocation facts and does not assume ownership of downstream charge, receivable, payment, treasury, or accounting consequences.

## 3. Business Outcome

An authoritative breach or reversal condition results in a governed revocation of the affected financial assistance, the revocation becomes an immutable new business fact, historical approvals and allocations remain reconstructable, and downstream financial domains receive the authoritative revocation fact to determine their own consequences.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an authoritative business condition establishes that active financial assistance must be revoked.

### 4.2 End Boundary

The scenario ends when the Financial Aid Aggregate has recorded the canonical revocation outcome and published `FinancialAidRevoked`, with historical assistance facts preserved.

### 4.3 In Scope

- Receipt of an authoritative breach/reversal condition.
- Determination that the affected assistance requires revocation under the applicable Financial Aid policy.
- Revocation of the Financial Aid Aggregate's active assistance.
- Preservation of prior approval and allocation history.
- Publication of `FinancialAidRevoked`.
- Downstream business consumption of the revocation fact.
- Business-level recovery of affected assistance without altering historical facts.

### 4.4 Out of Scope

- Re-evaluation of eligibility policy.
- Creation or modification of Student Charges.
- Receivable creation or adjustment.
- Payment collection or refund execution.
- Treasury operations.
- General Ledger posting.
- Technical retries, queues, rollback, APIs, databases, or infrastructure.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Canonical Ownership |
|---|---|---|
| Scholarship & Financial Aid | Owns Financial Aid Aggregate, revocation decision and revocation history. | Financial assistance remains F11-owned. |
| Student Lifecycle / Eligibility | Provides authoritative eligibility outcomes where the breach originates from eligibility. | Eligibility remains externally owned. |
| Student Charge & Fee Management | Determines charge consequences of revoked assistance. | Student Charge remains F01-owned. |
| Accounts Receivable | Determines receivable consequences arising from charge processing. | Receivable remains F07-owned. |

L2-F11 explicitly states that Scholarship & Financial Aid owns institutional financial assistance while Student Charges, Accounts Receivable, Treasury and General Ledger remain external.

## 6. Business Actors, Roles and Responsibilities

L2-F11 has no authoritative Actors section. No breach investigator, scholarship officer, approver, finance officer, or recovery officer is invented.

Business responsibility is capability-based: the authoritative breach/reversal condition originates from its canonical owner; Scholarship & Financial Aid governs the revocation of financial assistance.

## 7. Preconditions and Entry Criteria

- The affected assistance exists as a canonical Financial Aid Aggregate.
- The assistance is in an active business position permitting revocation.
- An authoritative breach, reversal, or equivalent business condition exists.
- The revocation is permitted by the applicable Financial Aid lifecycle and policy.
- Historical approval and allocation facts are available for preservation.

L2-F11 defines `RevokeFinancialAid`, requires historical allocations to remain preserved after revocation, and treats revocation as a new business fact.

## 8. Trigger

An authoritative breach or reversal condition establishes that existing financial assistance is no longer valid for continued use.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Receive the authoritative breach or reversal business fact. | Canonical originating domain | The condition requiring review is established. |
| 2 | Identify the affected Financial Aid Aggregate and its active assistance history. | Scholarship & Financial Aid | The canonical assistance record is identified. |
| 3 | Apply the applicable revocation policy and determine the financial-assistance revocation outcome. | Scholarship & Financial Aid | Revocation is governed within F11 ownership. |
| 4 | Revoke the financial assistance through the canonical Financial Aid Aggregate lifecycle. | Scholarship & Financial Aid | Assistance enters the canonical revoked position. |
| 5 | Preserve prior approval, Benefit Allocation, Funding Source and utilization history. | Scholarship & Financial Aid | Historical facts remain immutable and reconstructable. |
| 6 | Publish `FinancialAidRevoked` after the successful business transition. | Scholarship & Financial Aid | Revocation becomes an immutable enterprise business fact. |
| 7 | Student Charge & Fee Management consumes the revocation fact and determines whether charges require its own adjustment. | Student Charge & Fee Management | Charge ownership remains with F01. |
| 8 | Accounts Receivable independently determines any resulting receivable consequence. | Accounts Receivable | Receivable ownership remains with F07. |
| 9 | Financial Reporting and Audit consumers use the revocation fact for historical and financial reconstruction. | Reporting / Audit | Institutional history remains traceable. |

L2-F11 defines `FinancialAidRevoked` as the canonical revocation event and maps it to Student Charge & Fee Management and Reporting as typical consumers.

## 10. Alternate Business Flows

### A1 — Breach Does Not Require Revocation

If the authoritative business condition does not satisfy the applicable revocation policy, the Financial Aid Aggregate is not revoked. No unsupported revocation event is published.

### A2 — Assistance Is Already Revoked

The historical revocation remains the authoritative fact. A second revocation does not rewrite the historical record.

### A3 — Assistance Is Suspended Instead

If the authoritative condition requires restriction or suspension rather than revocation, the applicable canonical suspension/restriction lifecycle is used instead. This scenario does not convert a restriction into a revocation without an authoritative decision.

### A4 — Downstream Charge Consequence Is Required

Student Charge & Fee Management independently determines whether previously applied assistance affects a charge. Scholarship & Financial Aid does not directly modify Student Charges.

### A5 — Recovery Requires a New Assistance Decision

If a later authoritative condition permits assistance again, recovery occurs through the applicable new/reinstatement business lifecycle. Historical revocation remains immutable.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| No authoritative breach/reversal condition exists | Do not revoke the assistance. |
| Revocation violates an Aggregate invariant | Reject the revocation business action and preserve Aggregate consistency. |
| Assistance is already revoked | Preserve the existing revocation fact; do not duplicate or rewrite history. |
| Downstream charge consequence remains unresolved | Preserve `FinancialAidRevoked`; F01 governs its own charge resolution. |
| A later condition restores eligibility/assistance | Establish the appropriate new business fact without deleting the prior revocation. |

L2-F11 requires business-rule violations to reject the command, preserves historical records, and states that corrections occur through new business facts.

## 12. Business Rules and Decision Constraints

- Revoked assistance preserves historical allocations.
- Revocation creates a new business fact.
- Historical records are never deleted.
- Historical information is never overwritten, reassigned or repurposed.
- Authorization outcomes are externally determined.
- Eligibility outcomes are externally evaluated.
- The Financial Aid Aggregate remains the sole authoritative consistency boundary for institutional financial assistance.
- Student Charges remain externally owned.
- Receivables remain externally owned.
- Revocation does not itself create or modify a financial obligation.
- Downstream domains independently determine their own consequences.

These rules are directly defined by L2-F11's Suspension & Revocation, Governance and Historical invariants.

## 13. State and Lifecycle Impact

The scenario applies the canonical Financial Aid lifecycle command:

`RevokeFinancialAid`

The relevant lifecycle progression is from an active assistance position to the canonical revoked outcome.

No new `Breach`, `Recovery Pending`, or `Reversal Pending` state is invented.

The prior `Approved`, `Benefit Allocated`, `Funding Source Assigned`, and utilization history remain historical facts.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Event

**`FinancialAidRevoked`**

The Financial Aid Aggregate is the canonical publisher. The event represents the completed revocation business fact and becomes part of permanent enterprise history.

### 14.2 Cross-Domain Hand-offs

| From | To | Business Fact | Purpose |
|---|---|---|---|
| Canonical originating domain | Scholarship & Financial Aid | Breach/reversal condition | Establishes the authoritative reason for review. |
| Scholarship & Financial Aid | Student Charge & Fee Management | `FinancialAidRevoked` | Enables F01 to determine charge consequences. |
| Scholarship & Financial Aid | Accounts Receivable | `FinancialAidRevoked` where relevant | Provides context for downstream receivable processing. |
| Scholarship & Financial Aid | Reporting / Audit | `FinancialAidRevoked` | Preserves financial history and evidence. |

Events are distinct from synchronous business contracts or hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

L2-F11 states that authorization outcomes are externally determined. No revocation authority or human role is invented.

### 15.2 Approval / Separation of Duties

No scenario-specific separation-of-duty rule is defined in L2-F11. None is created locally.

### 15.3 Audit and Evidence

The revocation record must remain traceable to:

- affected Financial Aid Aggregate;
- authoritative breach/reversal condition;
- prior approval decision;
- Benefit Allocation;
- Funding Source;
- relevant utilization;
- revocation outcome;
- `FinancialAidRevoked` event;
- downstream references where applicable.

L2-F11 requires permanent, immutable and reconstructable business history.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- An authoritative breach/reversal condition exists.
- Revocation is permitted by applicable Financial Aid policy.
- The Financial Aid Aggregate records the revocation.
- Historical approval and allocation facts remain intact.
- `FinancialAidRevoked` is published.
- Downstream consumers have the authoritative business fact from which they determine their own consequences.

### 16.2 Postconditions

- Financial assistance is revoked.
- Historical assistance facts remain immutable.
- Revocation is permanently reconstructable.
- Student Charge ownership remains with F01.
- Receivable ownership remains with F07.
- No technical or implementation mechanism is introduced into the business scenario.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

The frozen title uses “Breach or Reversal,” but L2-F11 does not define a dedicated `AidBreach` or `FinancialAidReversed` event. Therefore this scenario uses the canonical `FinancialAidRevoked` outcome and treats breach/reversal as the authoritative business condition that leads to revocation, without inventing a new event.

### 17.2 Assumptions

No assumption overrides locked architecture.

### 17.3 Deferred Questions

If an upstream architecture later distinguishes different breach/reversal authorities or requires a separate reversal lifecycle, that distinction must be established upstream before additional L3 states or events are introduced.
