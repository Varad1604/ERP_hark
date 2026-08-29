# Submitted Research Proposal to Governed Withdrawal

# 1. Scenario Metadata

**Scenario ID:** L3-H-030  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose

Govern withdrawal of a Research Proposal before project establishment while preserving all prior submission, review and approval evidence.

# 3. Business Outcome

The proposal becomes withdrawn, cannot progress further, and its submitted and review history remains auditable

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A proposer requests withdrawal before final approval or project activation.

## 4.2 End Boundary

The scenario ends when an eligible Proposal reaches `Withdrawn` through the governed withdrawal path and all prior evidence remains immutable.

## 4.3 In Scope

- Withdrawal from `Submitted`, `Under Review`, or eligible `Approved` before project establishment.
- Withdrawal-request authority and Research approval governance.
- Immutable proposal history and withdrawal events.

## 4.4 Out of Scope

- Withdrawal after a Research Project has been established.
- Project suspension/termination after project establishment.
- Workforce lifecycle changes.

# 5. Ownership and Participating Domains

E03 retains Principal Investigator workforce identity/eligibility. Research owns Proposal withdrawal, Proposal state, withdrawal evidence and Research Proposal events.

# 6. Business Actors, Roles and Responsibilities

The Principal Investigator or other authorized proposal owner may request withdrawal. The Research Director is the canonical business authority for the governed withdrawal decision where institutional approval is required. Research Administrator records the governed decision but does not replace the Research Director's authority.

# 7. Preconditions and Entry Criteria

- A Research Proposal exists in `Submitted`, `Under Review`, or eligible `Approved` state.
- For an Approved proposal, no Research Project or other irreversible downstream fact has yet been established.
- Requestor authority is valid.
- Withdrawal reason is available.

# 8. Trigger

A proposer requests withdrawal before final approval or project activation

# 9. Main Business Flow

1. Receive the withdrawal request and identify the Proposal.
2. Validate requestor authority, reason and current Proposal state.
3. If the Proposal is `Submitted`, `Under Review`, or eligible `Approved` before project establishment, move it to `Withdrawal Requested`.
4. Record the governed withdrawal decision.
5. Move the Proposal to `Withdrawn` and publish `ResearchProposalWithdrawn`.
6. Preserve all prior submission, review, approval and withdrawal evidence.
7. Prevent further Proposal progression from the withdrawn record.

# 10. Alternate Business Flows

- Ineligible request or invalid authority: reject the withdrawal request without changing Proposal state.
- Approved proposal after project establishment: Proposal withdrawal is not used; the Research Project lifecycle governs later suspension/termination.

# 11. Failure, Interruption and Business Recovery

A withdrawal request that fails authority or lifecycle validation does not alter the Proposal. Withdrawal never deletes or rewrites earlier Proposal evidence.

# 12. Business Rules and Decision Constraints

- Canonical branch: `Submitted / Under Review / Approved → Withdrawal Requested → Withdrawn`.
- Approved-proposal withdrawal is allowed only before project establishment or another irreversible downstream boundary.
- Once a Research Project exists, project lifecycle controls apply instead of Proposal withdrawal.
- Withdrawn proposal history is immutable.

# 13. State and Lifecycle Impact

Eligible Proposal states move through `Withdrawal Requested → Withdrawn`. The withdrawn Proposal is terminal for that proposal record and cannot continue normal progression.

# 14. Business Events and Cross-Domain Collaboration

Research publishes `ResearchProposalWithdrawalRequested` and `ResearchProposalWithdrawn`. E03 workforce facts remain unchanged and are consumed by reference only.

# 15. Business Authorization, Approval and Audit Obligations

The authorized Proposal owner initiates withdrawal; the Research Director owns the institutional withdrawal decision where required. Evidence preserves actor, authority basis, reason, prior state, decision, resulting state and complete historical Proposal lineage.

# 16. Completion Criteria and Postconditions

Completion requires an eligible Proposal to reach `Withdrawn`, publication of the applicable Research events, preservation of prior history, and prevention of further Proposal progression. Workforce ownership remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former conflict over Approved-proposal withdrawal and the missing governed withdrawal branch was resolved by locked `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific questions.
