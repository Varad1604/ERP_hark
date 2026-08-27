# Research Activity to Ethics Decision and Continuing Compliance

# 1. Scenario Metadata

**Scenario ID:** L3-H-033  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Review  
**Version:** 1.2  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation; pre-approval validation; independent Architecture Approval review

# 2. Scenario Purpose

Govern Research ethics submission, review, revision, approval, renewal, rejection, suspension, reinstatement, revocation and closure while preserving E03 researcher ownership and immutable ethics-decision history.

# 3. Business Outcome

Approval, rejection, revision, renewal, suspension, or revocation is recorded and governs the research activity

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

Research requiring ethics oversight is proposed, materially changed, or requires governed renewal of an existing approval.

## 4.2 End Boundary

The scenario ends when the Ethics Approval reaches the applicable authoritative governed outcome and continuing-compliance obligations are reflected through the locked H05 ethics lifecycle and renewal/versioning rules.

## 4.3 In Scope

Ethics submission, review, revision-required branch, approval/rejection, governed renewal, suspension/reinstatement, revocation, closure, validity governance, and preservation of immutable decision/version evidence.

## 4.4 Out of Scope

Changes to E03 workforce facts; invention of a new `Renewed` lifecycle state; and any implementation mechanism.

# 5. Ownership and Participating Domains

E03 owns faculty identity, research capability, availability, capacity and eligibility. Research owns the Ethics Approval aggregate, its H05 lifecycle, renewal/version history and Research ethics events.

# 6. Business Actors, Roles and Responsibilities

Principal Investigator submits/revises the ethics application and may initiate governed renewal according to H05 policy. Existing Ethics Committee Member / Chair / Authorized Approver responsibilities remain as defined in H05. Research Director is the canonical business authority for strategic suspension, revocation and closure decisions where institutional approval is required.

# 7. Preconditions and Entry Criteria

- The Research activity requires ethics oversight.
- Mandatory evidence exists.
- Applicable committee/reviewer configuration is valid.
- Researcher/workforce references remain authoritative in E03.
- For renewal, an existing approval is valid and the configured renewal window/eligibility conditions are satisfied.

# 8. Trigger

Research requiring ethics oversight is proposed or materially changed, or an existing approval reaches its governed renewal window.

# 9. Main Business Flow

1. Submit the Ethics application and enter `Submitted`.
2. Move to `Under Review` and record governed review evidence.
3. If revision is required, move `Under Review → Revision Required`, publish `EthicsRevisionRequired`, and return a corrected submission to `Submitted`.
4. If rejected, move `Under Review → Rejected` and preserve decision history.
5. If approved, move `Under Review → Approved` and preserve approval conditions, validity period and evidence.
6. When an existing approval enters its governed renewal window, validate renewal eligibility and review the renewal under the source-defined H05 renewal process.
7. On approved renewal, extend the governed approval validity through a successor approval version, preserve prior approval history, and publish `EthicsApprovalRenewed`. Renewal does not introduce a separate `Renewed` lifecycle state or overwrite the predecessor approval evidence.
8. If continuing-compliance risk requires temporary restriction, move `Approved → Suspended` and publish `EthicsApprovalSuspended`.
9. If compliance is restored, move `Suspended → Approved` and publish `EthicsApprovalReinstated`.
10. If the approval is formally revoked, move `Approved / Suspended → Revoked` and publish `EthicsApprovalRevoked`.
11. When the governed research activity ends and no continuing ethics obligation remains, move `Approved → Closed` and publish `EthicsApprovalClosed`.

# 10. Alternate Business Flows

- Revision Required returns to a new governed submission while prior review history remains preserved.
- Renewal is rejected or cannot proceed when the existing approval or renewal-window conditions fail; the authoritative existing approval/validity position remains governed by H05 and no renewal event is published.
- Administrative extension may occur only where base-H05 renewal policy explicitly permits it and remains part of the governed renewal evidence.
- Rejected is a terminal outcome for that review decision.
- Revoked is terminal for that Ethics Approval record; future approval requires a new governed submission.

# 11. Failure, Interruption and Business Recovery

Incomplete evidence or unauthorized review prevents progression. Invalid or out-of-window renewal does not extend approval validity. Suspension preserves approval history and may be reinstated only through governed compliance restoration. Revocation does not erase prior approval or renewal evidence.

# 12. Business Rules and Decision Constraints

- Ethics lifecycle is owned by H05.
- Revision, suspension, reinstatement, revocation and closure follow only locked H05 transitions.
- Renewal timing and eligibility are institution-configurable under the locked base H05 workflow.
- Renewal requires a valid existing approval and satisfaction of the configured renewal window.
- Approved renewal extends validity through a successor approval version while preserving historical approvals.
- `EthicsApprovalRenewed` is the canonical renewal event.
- Renewal does not create a new `Renewed` lifecycle state.
- Revoked is terminal for that approval record.
- Workforce facts remain E03-owned.
- Historical review, approval and renewal evidence is immutable.

# 13. State and Lifecycle Impact

Canonical Ethics Approval lifecycle:

`Submitted → Under Review → Approved`

Branches:

- `Under Review → Revision Required → Submitted`
- `Under Review → Rejected`
- `Approved → Suspended → Approved`
- `Approved / Suspended → Revoked`
- `Approved → Closed`

Governed renewal extends the validity of an eligible Approved ethics decision through a successor approval version while preserving predecessor history. It does not add a parallel lifecycle state.

# 14. Business Events and Cross-Domain Collaboration

Canonical H05 events used in this scenario include:

- `EthicsApprovalRenewed`
- `EthicsRevisionRequired`
- `EthicsApprovalSuspended`
- `EthicsApprovalReinstated`
- `EthicsApprovalRevoked`
- `EthicsApprovalClosed`

Existing submission/review/approval/rejection facts remain Research-owned. E03 workforce facts are consumed by reference only.

# 15. Business Authorization, Approval and Audit Obligations

Existing ethics review/approval/renewal authorities remain as defined by H05. Research Director owns the amendment-defined strategic suspension/revocation/closure decisions where institutional approval is required. Evidence preserves application/approval version, validity period, renewal request and eligibility, reviewers, conditions, renewal/approval decision, suspension/reinstatement/revocation/closure rationale and complete history.

# 16. Completion Criteria and Postconditions

Completion occurs when the Ethics Approval reaches the applicable approved, rejected, renewed-validity, suspended, reinstated, revoked or closed business outcome and required evidence is preserved. A renewed approval retains immutable predecessor history and current validity according to H05 policy. The resulting ethics position governs the Research activity without changing E03 workforce ownership.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blockers

The former missing revision, suspension, revocation and continuing-compliance lifecycle semantics were resolved by locked `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md`.

The pre-approval finding that the synchronized file omitted the frozen renewal outcome was corrected on 2026-08-27 using the already-locked base-H05 Ethics renewal workflow and canonical event `EthicsApprovalRenewed`. No new state, authority or event was invented.

**Current architecture assessment:** `READY FOR INDEPENDENT ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific questions.
