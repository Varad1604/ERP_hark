# Consultancy Request to Delivered and Financially Reconciled Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-H-037  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, F, H  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.2  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; locked Finance architecture; `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md`; `L2-H05_Consultancy_Cancellation_Governance_Amendment_2026-08-27.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose

Govern the Research-owned Consultancy lifecycle through approval, execution, delivery acceptance, Finance-confirmed reconciliation and closure, or through governed pre-delivery-acceptance cancellation, while preserving Finance ownership of all financial truth.

# 3. Business Outcome

The engagement is approved, delivered, accepted, financially reconciled, and closed or reaches a governed cancellation outcome

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized external consultancy engagement is proposed.

## 4.2 End Boundary

The scenario ends when the Research Consultancy reaches `Closed` after delivery acceptance and authoritative Finance reconciliation confirmation, or reaches `Cancelled` through the locked H05 cancellation branch before `Delivery Accepted`.

## 4.3 In Scope

Consultancy approval, activation, delivery, acceptance, reconciliation-pending boundary, Finance confirmation reference, Research closure, cancellation request/decision before accepted delivery, and immutable history.

## 4.4 Out of Scope

Finance accounting/payment/reconciliation mechanisms, creation of Finance facts, cancellation after `Delivery Accepted`, or implementation details.

# 5. Ownership and Participating Domains

E03 owns workforce capability/eligibility/capacity/availability. Finance owns accounting, receivables, payments, settlement and reconciliation truth. Research owns Consultancy operational lifecycle, delivery evidence and cancellation lifecycle facts.

# 6. Business Actors, Roles and Responsibilities

Research Director is the canonical business authority for strategic Consultancy approval, cancellation and closure decisions where institutional approval is required. Research Administrator administers the process. Principal Investigator / Consultancy Team execute the work and may originate cancellation where policy permits. Finance provides authoritative reconciliation confirmation/reference and remains owner of monetary consequences.

# 7. Preconditions and Entry Criteria

- Authorized external engagement exists.
- Client/agreement context is valid.
- Consultancy approval is complete before execution.
- Team/workforce references are valid.
- Finance confirmation is available when closure depends on financial reconciliation.
- Cancellation may be requested only before `Delivery Accepted`.

# 8. Trigger

An authorized external consultancy engagement is proposed

# 9. Main Business Flow

1. Validate client, agreement, team and authorization context.
2. Complete the governed Research approval and establish the Consultancy in `Approved`.
3. Move to `Active` and execute the engagement.
4. When deliverables are ready, move to `Delivery Submitted` and publish `ResearchConsultancyDeliverySubmitted`.
5. Record governed delivery acceptance, move to `Delivery Accepted`, and publish `ResearchConsultancyDeliveryAccepted`.
6. Move to `Financial Reconciliation Pending` and publish `ResearchConsultancyFinancialReconciliationRequested`.
7. Consume authoritative Finance confirmation/reference that the applicable reconciliation condition is satisfied.
8. Move the Research Consultancy to `Closed` and publish `ResearchConsultancyClosed`.
9. Preserve complete Research and Finance-reference evidence without creating or modifying Finance accounting facts.

# 10. Alternate Business Flows

- `Approved / Active / Delivery Submitted → Cancellation Requested`: record the governed cancellation request and publish `ResearchConsultancyCancellationRequested`.
- If cancellation is approved before `Delivery Accepted`, move `Cancellation Requested → Cancelled` and publish `ResearchConsultancyCancelled`.
- If cancellation is not approved, the Consultancy remains on its applicable current path without erasing prior history.
- Once `Delivery Accepted` has been reached, cancellation is not permitted; normal reconciliation/closure governance applies.

# 11. Failure, Interruption and Business Recovery

Research cannot self-certify Finance reconciliation. Closure is blocked until the authoritative Finance confirmation exists. Invalid cancellation authority or a cancellation request after `Delivery Accepted` does not alter the current state. No branch erases prior execution, delivery or decision evidence.

# 12. Business Rules and Decision Constraints

- Canonical successful progression: `Approved → Active → Delivery Submitted → Delivery Accepted → Financial Reconciliation Pending → Closed`.
- Canonical cancellation branch: `Approved / Active / Delivery Submitted → Cancellation Requested → Cancelled`.
- Cancellation is permitted only before `Delivery Accepted`.
- `Cancelled` is terminal for that Consultancy record.
- Finance exclusively owns accounting/payment/reconciliation truth and any monetary cancellation consequences.
- H05 may store only Finance-confirmed references/statuses.
- Research does not publish Finance-owned payment/accounting facts.

# 13. State and Lifecycle Impact

The Consultancy follows the locked successful lifecycle or the locked pre-delivery cancellation branch. E03 and Finance source lifecycles remain unchanged.

# 14. Business Events and Cross-Domain Collaboration

Research publishes:

- `ResearchConsultancyDeliverySubmitted`
- `ResearchConsultancyDeliveryAccepted`
- `ResearchConsultancyFinancialReconciliationRequested`
- `ResearchConsultancyClosed`
- `ResearchConsultancyCancellationRequested`
- `ResearchConsultancyCancelled`

Finance publishes/owns its own authoritative financial facts; Research consumes confirmation by reference.

# 15. Business Authorization, Approval and Audit Obligations

Research Director owns applicable strategic Consultancy approval/cancellation/closure decisions. Audit evidence preserves approval, execution, delivery submission/acceptance, cancellation request/decision where applicable, Finance reconciliation reference, closure decision and immutable history. Finance evidence remains Finance-owned.

# 16. Completion Criteria and Postconditions

Completion requires either:

- accepted delivery, authoritative Finance reconciliation confirmation where required, and Research-side transition to `Closed`; or
- governed pre-delivery-acceptance transition to `Cancelled` with complete cancellation rationale, authority and history preserved.

Finance ownership remains unchanged and Research does not create financial settlement facts.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former missing Consultancy cancellation lifecycle, authority and event semantics were resolved by locked `L2-H05_Consultancy_Cancellation_Governance_Amendment_2026-08-27.md`. The earlier approval/delivery/reconciliation/closure remediation remains governed by the locked 2026-08-26 H05 amendment.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific questions.
