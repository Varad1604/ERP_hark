# Consultancy Request to Delivered and Financially Reconciled Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-H-037  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, F, H  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Review  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; locked Finance architecture; `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose

Govern the Research-owned Consultancy lifecycle through approval, execution, delivery acceptance, Finance-confirmed reconciliation and closure while preserving Finance ownership of all financial truth.

# 3. Business Outcome

The engagement is approved, delivered, accepted, financially reconciled, and closed or reaches a governed cancellation outcome

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized external consultancy engagement is proposed.

## 4.2 End Boundary

The scenario ends when the Research Consultancy reaches `Closed` after delivery acceptance and authoritative Finance reconciliation confirmation, or reaches another source-supported governed outcome without transferring financial ownership.

## 4.3 In Scope

Consultancy approval, activation, delivery, acceptance, reconciliation-pending boundary, Finance confirmation reference and Research closure.

## 4.4 Out of Scope

Finance accounting/payment/reconciliation mechanisms, creation of Finance facts, or implementation details.

# 5. Ownership and Participating Domains

E03 owns workforce capability/eligibility/capacity/availability. Finance owns accounting, receivables, payments, settlement and reconciliation truth. Research owns Consultancy operational lifecycle and delivery evidence.

# 6. Business Actors, Roles and Responsibilities

Research Director is the canonical business authority for strategic Consultancy approval and closure decisions where institutional approval is required. Research Administrator administers the process. Principal Investigator / Consultancy Team execute the work. Finance provides the authoritative reconciliation confirmation/reference.

# 7. Preconditions and Entry Criteria

- Authorized external engagement exists.
- Client/agreement context is valid.
- Consultancy approval is complete before execution.
- Team/workforce references are valid.
- Finance confirmation is available when closure depends on financial reconciliation.

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

- Missing/invalid delivery evidence prevents acceptance.
- Missing Finance reconciliation confirmation keeps the Consultancy in `Financial Reconciliation Pending`.
- Any cancellation outcome must use only source-supported H05 lifecycle semantics; no generic cancellation state is invented beyond the locked amendment scope.

# 11. Failure, Interruption and Business Recovery

Research cannot self-certify Finance reconciliation. Closure is blocked until the authoritative Finance confirmation exists. Delivery or approval failures keep the Consultancy from progressing to later states.

# 12. Business Rules and Decision Constraints

- Canonical Research-side progression: `Approved → Active → Delivery Submitted → Delivery Accepted → Financial Reconciliation Pending → Closed`.
- Finance exclusively owns accounting/payment/reconciliation truth.
- H05 may store only a Finance-confirmed reference/status.
- Research does not publish Finance-owned payment/accounting facts.

# 13. State and Lifecycle Impact

The Consultancy follows the locked Research-side lifecycle above. Closure is permitted only after Finance-confirmed reconciliation where applicable. E03 and Finance source lifecycles remain unchanged.

# 14. Business Events and Cross-Domain Collaboration

Research publishes:

- `ResearchConsultancyDeliverySubmitted`
- `ResearchConsultancyDeliveryAccepted`
- `ResearchConsultancyFinancialReconciliationRequested`
- `ResearchConsultancyClosed`

Finance publishes/owns its own authoritative financial facts; Research consumes confirmation by reference.

# 15. Business Authorization, Approval and Audit Obligations

Research Director owns applicable strategic Consultancy decisions. Audit evidence preserves approval, execution, delivery submission/acceptance, Finance reconciliation reference, closure decision and immutable history. Finance evidence remains Finance-owned.

# 16. Completion Criteria and Postconditions

Completion requires accepted delivery, authoritative Finance reconciliation confirmation where required, and Research-side transition to `Closed`. Finance ownership remains unchanged and Research does not create financial settlement facts.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former missing Consultancy approval/delivery-acceptance/reconciliation/closure semantics were resolved by locked `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26.

No cancellation state is introduced beyond source-supported H05 semantics; the frozen cancellation wording does not authorize invention of a new lifecycle branch.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific questions.
