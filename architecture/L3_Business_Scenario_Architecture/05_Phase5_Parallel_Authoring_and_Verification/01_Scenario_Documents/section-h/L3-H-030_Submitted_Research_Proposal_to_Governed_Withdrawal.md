# Submitted Research Proposal to Governed Withdrawal

# 1. Scenario Metadata

**Scenario ID:** L3-H-030  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows

# 2. Scenario Purpose

Govern withdrawal of a Research Proposal while withdrawal remains valid under the locked Proposal lifecycle, preserving all prior submission and review evidence.

# 3. Business Outcome

The proposal becomes withdrawn, cannot progress further, and its submitted and review history remains auditable

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A proposer requests withdrawal before final approval or project activation

## 4.2 End Boundary

For source-supported entry states, the end boundary is `Withdrawn`, with `ResearchProposalWithdrawn` published and prior history preserved. The frozen trigger is broader than the locked H05 lifecycle and is materially unresolved.

## 4.3 In Scope

- Validate requestor authority, withdrawal reason, and current Proposal state.
- Withdraw a source-supported `Submitted` or `Under Review` proposal.
- Preserve submission/review history and terminate further proposal progression.

## 4.4 Out of Scope

- Inventing withdrawal from `Approved`.
- Inventing the missing `Revision Requested → Withdrawn` transition.
- Research Project suspension or closure.
- Workforce lifecycle changes.

# 5. Ownership and Participating Domains

E03 retains Principal Investigator workforce identity/eligibility. Research owns Proposal withdrawal, Proposal state, withdrawal evidence, and `ResearchProposalWithdrawn`.

# 6. Business Actors, Roles and Responsibilities

The Principal Investigator may request withdrawal. H05 also permits delegated authority and exceptional institutional-administrator withdrawal, subject to the Research withdrawal rules.

# 7. Preconditions and Entry Criteria

A submitted Proposal exists; requestor authority is valid; withdrawal reason is supplied; the current state must have a source-defined withdrawal transition.

# 8. Trigger

A proposer requests withdrawal before final approval or project activation

# 9. Main Business Flow

1. Receive the request and identify the Proposal.
2. Validate requestor authority, mandatory reason, and current state.
3. If the Proposal is `Submitted` or `Under Review`, transition it to `Withdrawn`.
4. Publish `ResearchProposalWithdrawn`.
5. Preserve all submission, review, withdrawal, actor, reason, and timestamp evidence.
6. Stop if the request relies on a post-approval/pre-project or `Revision Requested` path not consistently defined by H05.

# 10. Alternate Business Flows

- `Submitted` → `Withdrawn`: source-supported.
- `Under Review` → `Withdrawn`: source-supported.
- `Revision Requested`: RM-07 says withdrawal is permitted, but the state-transition matrix omits the transition.
- `Approved`: H05 explicitly prohibits withdrawal.

# 11. Failure, Interruption and Business Recovery

Unauthorized requests and requests without a reason are rejected without changing Proposal state. A request against an unsupported lifecycle position remains unresolved rather than being mapped to another state.

# 12. Business Rules and Decision Constraints

Withdrawal reason is mandatory. The requestor must be the Principal Investigator or permitted delegated authority. Approved proposals cannot be withdrawn. Withdrawn proposals cannot be edited or resubmitted.

# 13. State and Lifecycle Impact

Source-supported transitions are `Submitted → Withdrawn` and `Under Review → Withdrawn`. H05 does not define `Approved → Withdrawn`; its RM-07 prose and state matrix conflict on `Revision Requested → Withdrawn`.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Research publishes `ResearchProposalWithdrawn` for a valid withdrawal. Existing submission/review events remain historical facts.

## 14.2 Business Contracts and Cross-Domain Hand-offs

E03 workforce ownership is unchanged. Withdrawal does not create a workforce-domain state change.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Principal Investigator or permitted delegated authority initiates the standard withdrawal. No L3 actor may override the locked prohibition on Approved-proposal withdrawal.

## 15.2 Separation of Duties, Overrides and Exceptions

No additional L3 override is introduced.

## 15.3 Business Audit and Evidence Obligations

Preserve requestor, authority basis, reason, request time, previous state, submission/review history, resulting state, and `ResearchProposalWithdrawn`.

# 16. Completion Criteria and Postconditions

The supported branch completes when a valid `Submitted` or `Under Review` proposal becomes `Withdrawn`, cannot progress further, and its history remains auditable. The full frozen boundary cannot be claimed while Section 17 remains unresolved.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- The frozen trigger allows withdrawal “before final approval or project activation,” which can include an Approved proposal before project creation, while H05 explicitly prohibits withdrawal after approval.
- RM-07 lists `Revision Requested` as withdrawable, but the authoritative Proposal state-transition matrix does not define that transition.

These gaps materially affect the entry boundary and lifecycle.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
