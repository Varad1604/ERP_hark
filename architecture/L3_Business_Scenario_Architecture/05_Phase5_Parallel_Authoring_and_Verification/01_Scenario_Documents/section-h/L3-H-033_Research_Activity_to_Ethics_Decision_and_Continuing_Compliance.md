# Research Activity to Ethics Decision and Continuing Compliance

# 1. Scenario Metadata

**Scenario ID:** L3-H-033  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows

# 2. Scenario Purpose

Govern Research ethics submission, review, decision, renewal, and expiry using only H05-defined Ethics Approval lifecycle semantics while preserving E03 researcher ownership.

# 3. Business Outcome

Approval, rejection, revision, renewal, suspension, or revocation is recorded and governs the research activity

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

Research requiring ethics oversight is proposed or materially changed

## 4.2 End Boundary

H05 supports submission, review, approval, rejection, conditional approval, renewal, and expiry. The frozen outcome additionally requires revision, suspension, revocation, and continuing-compliance governance that H05 does not define.

## 4.3 In Scope

Ethics application submission, authorized review, approval/rejection, approval conditions, renewal, expiry, and evidence preservation.

## 4.4 Out of Scope

Invented ethics revision, suspension, revocation, or continuing-compliance states/events; changes to E03 workforce facts.

# 5. Ownership and Participating Domains

E03 owns faculty identity, research capability, availability, capacity, and eligibility. Research owns the Ethics Approval aggregate and its defined lifecycle.

# 6. Business Actors, Roles and Responsibilities

Principal Investigator submits/renews; Ethics Committee Member reviews; Ethics Committee Chair / Authorized Approver approves; Ethics Committee Chair records rejection where authorized.

# 7. Preconditions and Entry Criteria

An applicable Research Project exists; ethics oversight is required by configured research category; mandatory evidence is available; committee/reviewer configuration is valid.

# 8. Trigger

Research requiring ethics oversight is proposed or materially changed

# 9. Main Business Flow

1. Determine that the Research activity requires ethics oversight.
2. Submit the Ethics application; enter `Submitted`.
3. Assign authorized review and enter `Under Review`.
4. Record review evidence.
5. Record an H05-supported approval or rejection decision.
6. For an approved request, preserve conditions and validity period.
7. Renew within the governed window or allow approval to expire.
8. Stop before any frozen revision, suspension, revocation, or undefined continuing-compliance action.

# 10. Alternate Business Flows

Conditional approval is supported. Rejection is supported. Renewal creates a new approval version with prior versions preserved. Expiry is source-defined.

# 11. Failure, Interruption and Business Recovery

Incomplete application or unauthorized reviewer prevents progression. Unsupported revision/suspension/revocation cannot be represented through a substitute state.

# 12. Business Rules and Decision Constraints

Ethics requirement, committee structure, validity, and renewal window are configurable. Historical review/approval versions remain immutable. E03 remains researcher eligibility owner.

# 13. State and Lifecycle Impact

`Submitted → Under Review → Approved / Rejected`; `Approved → Approved` through renewal, or `Approved → Expired`. No H05 ethics states exist for revision, suspension, or revocation.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H05 defines `EthicsApplicationSubmitted`, `EthicsApplicationReviewed`, `EthicsApprovalGranted`, `EthicsApprovalRenewed`, and `EthicsApprovalExpired`. The detailed Ethics lifecycle uses `EthicsApplicationRejected`, while the consolidated catalogue uses `EthicsApprovalRejected`; L3 does not resolve that naming conflict locally.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Research consumes E03 workforce facts only as authoritative references. No additional participant is introduced.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Only H05-defined PI, Ethics Committee Member, Ethics Committee Chair, and Authorized Approver responsibilities are used.

## 15.2 Separation of Duties, Overrides and Exceptions

No L3 authority is invented for missing frozen lifecycle actions.

## 15.3 Business Audit and Evidence Obligations

Preserve application version, reviewer observations, conditions, decision authority, validity period, renewal history, expiry, and exact source-supported lifecycle evidence.

# 16. Completion Criteria and Postconditions

Supported H05 branches may complete to approval, rejection, renewal, or expiry. The full frozen outcome cannot complete because revision, suspension, revocation, and continuing-compliance lifecycle semantics are missing.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- No canonical ethics revision state/transition/authority/event.
- No ethics suspension or revocation state/transition/authority/event.
- Continuing compliance is described as a capability but has no canonical workflow beyond renewal/expiry.
- Rejection event name conflicts between detailed lifecycle and consolidated event catalogue.

The missing frozen branches materially block the scenario.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
