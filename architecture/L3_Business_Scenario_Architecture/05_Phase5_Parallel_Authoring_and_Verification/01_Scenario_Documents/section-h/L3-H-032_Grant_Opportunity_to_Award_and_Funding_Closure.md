# Grant Opportunity to Award and Funding Closure

# 1. Scenario Metadata

**Scenario ID:** L3-H-032  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, F, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; locked Finance architecture

# 2. Scenario Purpose

Govern a Research Grant through application, verified award, Research-side allocation/utilization metadata, reporting, and closure while preserving Finance as the authority for accounting and financial consequences.

# 3. Business Outcome

The award, funding use, reporting, amendment, and closure outcomes are governed and reconciled with Finance

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An eligible funding opportunity or approved grant application initiates grant governance

## 4.2 End Boundary

H05 can reach a closed Research Grant after award, allocation, utilization, and reporting. The frozen outcome additionally requires governed amendment and Finance reconciliation, neither of which has a complete canonical H05/F contract.

## 4.3 In Scope

Grant application, award registration, allocation metadata, utilization metadata, reporting/compliance evidence, Research Grant closure, and Finance ownership preservation.

## 4.4 Out of Scope

Finance ledger/payment/posting activity; invented Grant amendment states/events; invented Finance reconciliation contracts.

# 5. Ownership and Participating Domains

E03 owns PI eligibility and workforce facts. Finance owns budgets, ledgers, accounting, payments, posting, tax, and other monetary consequences. Research owns the Grant lifecycle and research-specific funding metadata.

# 6. Business Actors, Roles and Responsibilities

Principal Investigator applies and may provide Research-side utilization evidence. Research Office records the award and closes the Grant. Research Office and Finance Officer participate in funding allocation as H05 specifies.

# 7. Preconditions and Entry Criteria

Eligible project or approved proposal; valid PI eligibility; active configured funding scheme; complete budget/application context; Finance-owned source facts available where required.

# 8. Trigger

An eligible funding opportunity or approved grant application initiates grant governance

# 9. Main Business Flow

1. Validate grant eligibility, funding scheme, budget context, and PI eligibility.
2. Submit the Research Grant application and enter `Proposed`.
3. Record a verified funding-agency award and enter `Awarded`.
4. Record approved allocation within the awarded amount and enter `Active`.
5. Record Research-side utilization metadata against valid allocations.
6. Complete required reporting/compliance evidence.
7. Close the Grant when H05 closure validation is satisfied.
8. Stop before asserting amendment or Finance-reconciled completion because the required upstream semantics are not defined.

# 10. Alternate Business Flows

Partial awards and phased allocations are supported provided allocations do not exceed the award. Premature closure is rejected. A required award amendment cannot be authored because H05 contains no amendment workflow/state/event.

# 11. Failure, Interruption and Business Recovery

Unverified award data blocks award registration; over-allocation and invalid utilization are rejected; unresolved reporting blocks closure. Missing amendment or Finance-reconciliation semantics require architecture resolution.

# 12. Business Rules and Decision Constraints

Funding source must be configured; allocations cannot exceed award; utilization references approved allocation; Finance remains accounting authority; closure requires reporting obligations; award amount changes require a governed amendment workflow that H05 references but does not define.

# 13. State and Lifecycle Impact

Research Grant: `Proposed → Awarded → Active → Closed`; utilization remains within `Active`. No canonical amendment lifecycle is defined.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Detailed H05 Grant sections use `ResearchGrantApplied`, `GrantAwarded`, `FundingAllocated`, `FundingUtilizationRecorded`, and `ResearchGrantClosed`. The consolidated event catalogue instead names the application fact `GrantApplicationSubmitted`; the Research Grant publisher is unambiguous, but the application-event name is inconsistent.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Research exchanges grant/funding metadata with Finance while Finance retains authoritative financial records. H05 does not define the exact Finance reconciliation return fact required by the frozen end boundary.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

PI applies; Research Office records award and closure; Research Office and Finance Officer participate in allocation under H05.

## 15.2 Separation of Duties, Overrides and Exceptions

No L3 override may bypass award limits, create financial postings, or invent amendment/reconciliation authority.

## 15.3 Business Audit and Evidence Obligations

Preserve application, award evidence, award amount, allocation history/authority, utilization references, reporting/compliance evidence, closure evidence, and applicable Grant events.

# 16. Completion Criteria and Postconditions

The ordinary H05 path can close a Grant after governed award/allocation/utilization/reporting. The full frozen business outcome cannot complete until amendment and Finance-reconciliation semantics are canonical.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- H05 states award amounts may change through a governed amendment workflow, but defines no amendment command, lifecycle transition, authority, or event.
- The frozen outcome requires reconciliation with Finance, but no canonical Finance reconciliation completion contract/event or acceptance criterion is defined.
- H05 uses both `ResearchGrantApplied` and `GrantApplicationSubmitted` for the application fact.

The first two gaps materially prevent the frozen end boundary.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
