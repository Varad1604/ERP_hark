# Research Activity to Ethics Decision and Continuing Compliance

# 1. Scenario Metadata

**Scenario ID:** L3-H-033  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Review  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose

Govern Research ethics submission, review, revision, approval, rejection, suspension, reinstatement, revocation and closure while preserving E03 researcher ownership.

# 3. Business Outcome

Approval, rejection, revision, renewal, suspension, or revocation is recorded and governs the research activity

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

Research requiring ethics oversight is proposed or materially changed.

## 4.2 End Boundary

The scenario ends when the Ethics Approval reaches an authoritative governed outcome and continuing-compliance obligations are reflected through the locked H05 lifecycle.

## 4.3 In Scope

Ethics submission, review, revision-required branch, approval/rejection, suspension/reinstatement, revocation, closure, and preservation of immutable decision evidence.

## 4.4 Out of Scope

Changes to E03 workforce facts and any implementation mechanism.

# 5. Ownership and Participating Domains

E03 owns faculty identity, research capability, availability, capacity and eligibility. Research owns the Ethics Approval aggregate and its lifecycle/events.

# 6. Business Actors, Roles and Responsibilities

Principal Investigator submits/revises the ethics application. Existing Ethics Committee Member / Chair / Authorized Approver responsibilities remain as defined in H05. Research Director is the canonical business authority for strategic suspension, revocation and closure decisions where institutional approval is required.

# 7. Preconditions and Entry Criteria

- The Research activity requires ethics oversight.
- Mandatory evidence exists.
- Applicable committee/reviewer configuration is valid.
- Researcher/workforce references remain authoritative in E03.

# 8. Trigger

Research requiring ethics oversight is proposed or materially changed

# 9. Main Business Flow

1. Submit the Ethics application and enter `Submitted`.
2. Move to `Under Review` and record governed review evidence.
3. If revision is required, move `Under Review → Revision Required`, publish `EthicsRevisionRequired`, and return a corrected submission to `Submitted`.
4. If rejected, move `Under Review → Rejected` and preserve decision history.
5. If approved, move `Under Review → Approved` and preserve approval conditions/evidence.
6. If continuing-compliance risk requires temporary restriction, move `Approved → Suspended` and publish `EthicsApprovalSuspended`.
7. If compliance is restored, move `Suspended → Approved` and publish `EthicsApprovalReinstated`.
8. If the approval is formally revoked, move `Approved / Suspended → Revoked` and publish `EthicsApprovalRevoked`.
9. When the governed research activity ends and no continuing ethics obligation remains, move `Approved → Closed` and publish `EthicsApprovalClosed`.

# 10. Alternate Business Flows

- Revision Required returns to a new governed submission while prior review history remains preserved.
- Rejected is a terminal outcome for that review decision.
- Revoked is terminal for that Ethics Approval record; future approval requires a new governed submission.

# 11. Failure, Interruption and Business Recovery

Incomplete evidence or unauthorized review prevents progression. Suspension preserves approval history and may be reinstated only through governed compliance restoration. Revocation does not erase prior approval evidence.

# 12. Business Rules and Decision Constraints

- Ethics lifecycle is owned by H05.
- Revision, suspension, reinstatement, revocation and closure follow only the locked H05 transitions.
- Revoked is terminal for that approval record.
- Workforce facts remain E03-owned.
- Historical review/approval evidence is immutable.

# 13. State and Lifecycle Impact

Canonical Ethics Approval lifecycle:

`Submitted → Under Review → Approved`

Branches:

- `Under Review → Revision Required → Submitted`
- `Under Review → Rejected`
- `Approved → Suspended → Approved`
- `Approved / Suspended → Revoked`
- `Approved → Closed`

# 14. Business Events and Cross-Domain Collaboration

Canonical H05 events include:

- `EthicsRevisionRequired`
- `EthicsApprovalSuspended`
- `EthicsApprovalReinstated`
- `EthicsApprovalRevoked`
- `EthicsApprovalClosed`

Existing submission/review/approval/rejection facts remain Research-owned. E03 workforce facts are consumed by reference only.

# 15. Business Authorization, Approval and Audit Obligations

Existing ethics review/approval authorities remain as defined by H05. Research Director owns the amendment-defined strategic suspension/revocation/closure decisions where institutional approval is required. Evidence preserves application version, reviewers, conditions, decision authority, suspension/reinstatement/revocation/closure rationale and complete history.

# 16. Completion Criteria and Postconditions

Completion occurs when the Ethics Approval reaches the applicable approved, rejected, suspended, reinstated, revoked or closed outcome and required evidence is preserved. The resulting ethics state governs the Research activity without changing E03 workforce ownership.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former missing revision, suspension, revocation and continuing-compliance lifecycle semantics were resolved by locked `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific questions.
