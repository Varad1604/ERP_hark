# Grant Opportunity to Award and Funding Closure

# 1. Scenario Metadata

**Scenario ID:** L3-H-032  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, F, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; locked Finance architecture; `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose

Govern a Research Grant through application, award, amendment, utilization, reporting and closure while preserving Finance as the authority for accounting and financial reconciliation facts.

# 3. Business Outcome

The award, funding use, reporting, amendment, and closure outcomes are governed and reconciled with Finance

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An eligible funding opportunity or approved grant application initiates grant governance.

## 4.2 End Boundary

The scenario ends when the Research Grant is governedly closed after research obligations are complete and authoritative Finance reconciliation confirmation has been consumed where required.

## 4.3 In Scope

Grant application, award, amendment, Research-side funding metadata, utilization/reporting evidence, closure and Finance-confirmation reference.

## 4.4 Out of Scope

Finance ledger/payment/posting activity and any Research-owned mutation of Finance facts.

# 5. Ownership and Participating Domains

E03 owns PI workforce facts. Finance owns accounting, budgets, payments, postings, tax and reconciliation facts. Research owns the Grant lifecycle and Research-side funding metadata.

# 6. Business Actors, Roles and Responsibilities

Principal Investigator applies and supplies Research-side evidence. Research Director is the canonical business authority for strategic grant amendment/closure decisions where institutional approval is required. Research Administrator records governed Research decisions. Finance supplies authoritative reconciliation confirmation/reference where required.

# 7. Preconditions and Entry Criteria

- Eligible project/proposal and PI exist.
- Applicable funding scheme and award evidence exist.
- Research-side obligations and reporting requirements are known.
- Finance confirmation is available when closure depends on financial reconciliation.

# 8. Trigger

An eligible funding opportunity or approved grant application initiates grant governance

# 9. Main Business Flow

1. Validate funding opportunity, PI eligibility, award and Research governance context.
2. Establish the governed Research Grant and record the award.
3. Move the Grant to `Active` when Research-side activation conditions are satisfied.
4. Where an amendment is required, move `Active → Amendment Pending` and publish `ResearchGrantAmendmentRequested`.
5. Complete the applicable Research approval and consume any required authoritative Finance confirmation.
6. Publish `ResearchGrantAmendmentApproved` and return the Grant to `Active`.
7. Preserve Research-side utilization/reporting/compliance evidence while Finance remains owner of monetary facts.
8. When Research obligations are complete, move to `Closure Pending` and publish `ResearchGrantClosureInitiated`.
9. Consume authoritative Finance reconciliation confirmation/reference where financial reconciliation is required.
10. Move the Grant to `Closed` and publish `ResearchGrantClosed`.

# 10. Alternate Business Flows

- Amendment is not approved: the Grant does not return to Active through the amendment branch.
- Closure is deferred while Research obligations, sponsor evidence or required Finance reconciliation confirmation remain incomplete.

# 11. Failure, Interruption and Business Recovery

Missing or invalid award information prevents Grant progression. Over-allocation or invalid utilization is rejected under existing H05 rules. Closure remains pending until all Research obligations and required authoritative Finance confirmation are satisfied.

# 12. Business Rules and Decision Constraints

- Research owns Grant lifecycle and Research-side funding metadata.
- Finance exclusively owns accounting, budget, payment, posting and reconciliation truth.
- Active Grant may enter `Amendment Pending` and return to Active only after applicable Research approval and required Finance confirmation.
- Grant closure requires completion of Research obligations and authoritative Finance confirmation where reconciliation is required.
- H05 stores only the Finance-confirmed reference/status; it does not create Finance facts.

# 13. State and Lifecycle Impact

Canonical Grant lifecycle for this scenario:

`Awarded → Active → Closure Pending → Closed`

Amendment branch:

`Active → Amendment Pending → Active`

# 14. Business Events and Cross-Domain Collaboration

Research publishes:

- `ResearchGrantAmendmentRequested`
- `ResearchGrantAmendmentApproved`
- `ResearchGrantClosureInitiated`
- `ResearchGrantClosed`

Finance remains semantic owner/publisher of its own reconciliation/accounting facts; Research consumes confirmation by reference.

# 15. Business Authorization, Approval and Audit Obligations

Research Director owns the applicable Research approval decisions. Audit evidence preserves application/award, amendment request/decision, Research obligations, Finance confirmation reference, closure decision and immutable Grant history.

# 16. Completion Criteria and Postconditions

Completion requires the Grant to reach `Closed`, required Research obligations to be satisfied, applicable sponsor evidence to be recorded, and authoritative Finance reconciliation confirmation to be consumed where required. Finance ownership remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former missing Grant amendment lifecycle and Finance-reconciliation closure boundary were resolved by locked `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26.

Historical application-event vocabulary differences do not alter the Grant amendment/closure outcome addressed here and shall not be used to create a second semantic publisher.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific questions.
