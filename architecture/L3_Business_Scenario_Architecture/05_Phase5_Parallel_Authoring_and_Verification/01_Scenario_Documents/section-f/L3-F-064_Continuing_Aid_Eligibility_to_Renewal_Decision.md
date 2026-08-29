# Continuing Aid Eligibility to Renewal Decision

**Scenario ID:** L3-F-064  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Scholarship & Financial Aid  
**Participating Domains:** C, F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-F11 Scholarship & Financial Aid; L2-C07 Scholarship Eligibility Workflows; applicable Student Lifecycle and Academic sources  
**Referenced By:** Student Charge & Fee Management; Financial Reporting; Audit; Executive Dashboards

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-F-064 |
| Canonical Title | Continuing Aid Eligibility to Renewal Decision |
| Primary Owner | Scholarship & Financial Aid |
| Participating Domains | C, F |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern the institutional renewal decision for continuing financial assistance when a new or continuing eligibility outcome is available.

Scholarship & Financial Aid owns the renewal decision and Renewal Record. Eligibility policy remains owned by the authoritative eligibility context, and historical renewal decisions remain immutable.

## 3. Business Outcome

A continuing-assistance eligibility outcome is evaluated as an authoritative input to the financial-assistance renewal process, a governed renewal decision is recorded, and—when renewal is approved—the renewed assistance becomes a new business fact without overwriting prior assistance history.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when continuing assistance requires a renewal decision and an authoritative eligibility outcome is available.

### 4.2 End Boundary

The scenario ends when the renewal decision has been recorded as either an approved renewal business fact or a rejected renewal outcome, with prior renewal history preserved.

### 4.3 In Scope

- Consumption of continuing eligibility outcome.
- Renewal evaluation under the applicable Renewal Policy.
- Renewal approval or rejection.
- Creation of a new Renewal Record.
- Activation/continuation of assistance when the canonical lifecycle permits it.
- Publication of `FinancialAidRenewed` or `RenewalRejected`.

### 4.4 Out of Scope

- Eligibility policy evaluation.
- Academic evaluation.
- Student ownership.
- New charge creation.
- Receivable creation.
- Payment execution.
- Treasury.
- Accounting.
- Technical scheduling or notification mechanisms.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Ownership |
|---|---|---|
| C — Scholarship Eligibility / Student contexts | Provides authoritative continuing eligibility facts. | Eligibility and student concepts remain externally owned. |
| F — Scholarship & Financial Aid | Owns Renewal Record, renewal decision and Financial Aid lifecycle. | Canonical renewal ownership. |
| F — Student Charge & Fee Management | Consumes renewed assistance facts and determines its own charge consequences. | Charges remain F01-owned. |

L2-F11 states that renewals create new business facts and that historical renewal records remain immutable.

## 6. Business Actors, Roles and Responsibilities

L2-F11 records that its required Actors section is absent. No renewal officer, reviewer, scholarship committee member, or other human role is invented.

Responsibility is capability-based: eligibility context supplies the authoritative eligibility outcome; Scholarship & Financial Aid governs renewal.

## 7. Preconditions and Entry Criteria

- Existing assistance is active and eligible for renewal consideration.
- An authoritative continuing eligibility outcome is available.
- Applicable renewal policy can be applied.
- The existing assistance history is available.
- Renewal can be recorded without overwriting previous records.

L2-F11 explicitly requires Renewal to have an active assistance record and creates a new Renewal Record.

## 8. Trigger

The active assistance reaches the business point at which its continuation requires a governed renewal decision and the authoritative eligibility context is available.

## 9. Main Business Flow

| Step | Business Behaviour | Owner | Result |
|---|---|---|---|
| 1 | Obtain the authoritative continuing eligibility outcome. | Eligibility context | Current eligibility fact is available. |
| 2 | Confirm that the existing assistance is active and eligible for renewal consideration. | Scholarship & Financial Aid | Renewal is permitted for evaluation. |
| 3 | Apply the applicable Renewal Policy to the authoritative eligibility outcome and existing assistance context. | Scholarship & Financial Aid | A renewal decision is reached. |
| 4 | Record the renewal decision. | Scholarship & Financial Aid | A new Renewal Record becomes part of the Financial Aid Aggregate history. |
| 5 | If approved, continue/renew the assistance through the canonical lifecycle. | Scholarship & Financial Aid | Renewed assistance becomes a new business fact. |
| 6 | Publish `FinancialAidRenewed` after successful renewal. | Scholarship & Financial Aid | Downstream consumers receive the authoritative renewal fact. |
| 7 | If rejected, record the rejection outcome and publish `RenewalRejected`. | Scholarship & Financial Aid | Renewal rejection becomes a historical business fact. |
| 8 | Downstream consumers independently determine any resulting financial consequences. | Downstream Finance domains | Ownership remains with the consuming domain. |

L2-F11 defines renewal evaluation, renewal approval, renewal lifecycle and renewal history as Scholarship & Financial Aid responsibilities.

## 10. Alternate Business Flows

### A1 — Eligibility No Longer Supports Continuation

If the authoritative eligibility outcome does not support continuation, renewal is not approved. The prior assistance and renewal history remain immutable.

### A2 — Conditional Eligibility

If eligibility is conditional, Scholarship & Financial Aid consumes the authoritative condition and applies its own financial-assistance policy. It does not re-evaluate eligibility policy.

### A3 — Renewal Rejected

A rejected renewal is recorded as a new business fact. The previous renewal history remains unchanged.

### A4 — Active Assistance Not Available

If the assistance is not active, the renewal invariant is not satisfied. No renewal is created through this scenario.

### A5 — Existing Renewal History

A subsequent renewal creates a new Renewal Record. It does not amend or replace an earlier renewal.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| Continuing eligibility is unavailable | Do not make a new renewal decision from invented or stale eligibility. |
| Assistance is not active | Do not create a renewal; preserve current historical state. |
| Renewal violates an invariant | Reject the renewal business action and preserve Aggregate consistency. |
| Renewal is rejected | Record the rejection as a business fact; preserve previous history. |
| Downstream consumer has not reacted | Preserve the authoritative renewal fact; downstream recovery remains its own responsibility. |

L2-F11 requires Aggregate invariants to be preserved and historical records never to be overwritten.

## 12. Business Rules and Decision Constraints

- Renewal requires an active assistance record.
- Renewal creates a new Renewal Record.
- Previous renewals remain immutable.
- Eligibility outcomes are externally evaluated.
- Authorization outcomes are externally determined.
- Historical information is never deleted, overwritten, reassigned or repurposed.
- A renewal is a new business fact, not an alteration of prior history.
- Financial consequences of renewed assistance remain owned by downstream financial domains.

These rules are directly established by L2-F11.

## 13. State and Lifecycle Impact

Relevant Financial Aid lifecycle:

`... → Active → Renewed`

L2-F11 permits:

`Active → Renewed`

and then the renewed assistance continues through the defined lifecycle. Renewal is a canonical state transition and creates a new Renewal Record.

No local `Renewal Pending`, `Eligibility Review`, or `Renewal Approved` state is invented.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Events

- `FinancialAidRenewed`
- `RenewalRejected`

The Financial Aid Aggregate is the canonical publisher. Events represent completed business facts and preserve immutable history.

### 14.2 Cross-Domain Hand-offs

| From | To | Business Fact | Purpose |
|---|---|---|---|
| Eligibility context | Scholarship & Financial Aid | Continuing eligibility outcome | Provides authoritative eligibility input. |
| Scholarship & Financial Aid | Student Charge & Fee Management | `FinancialAidRenewed` | Enables F01 to determine its own renewed-charge consequence. |
| Scholarship & Financial Aid | Reporting / Audit | Renewal facts | Supports historical reconstruction and reporting. |

Events remain distinct from synchronous business contracts.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

Authorization remains externally governed. No human renewal approver is invented.

### 15.2 Approval

Renewal approval is a Financial Aid business outcome, but its authorization policy is externally governed.

### 15.3 Audit Evidence

Renewal history must preserve:

- assistance identity;
- eligibility outcome reference;
- renewal decision;
- effective period;
- renewal history;
- prior immutable renewal facts;
- resulting published event.

L2-F11 explicitly requires renewal history and immutable historical business records.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

For an approved renewal:

- authoritative continuing eligibility outcome is available;
- active assistance satisfies renewal prerequisites;
- renewal decision is approved;
- new Renewal Record is created;
- renewed assistance follows the canonical lifecycle;
- `FinancialAidRenewed` is published.

For a rejected renewal:

- renewal rejection is recorded;
- `RenewalRejected` is published;
- previous renewal history remains unchanged.

### 16.2 Postconditions

- A new renewal business fact exists.
- Previous renewal records remain immutable.
- Financial assistance ownership remains with Scholarship & Financial Aid.
- Downstream charge and receivable ownership remains unchanged.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

L2-F11 defines renewal behaviour but does not define a human actor catalogue or a detailed institution-specific renewal policy. Those are therefore not invented in this scenario.

### 17.2 Assumptions

No assumption overrides locked architecture.

### 17.3 Deferred Questions

Any future detailed definition of renewal authorization, human roles, or institution-specific eligibility criteria must come from the applicable upstream authorization/eligibility architecture and must not be created locally in L3.
