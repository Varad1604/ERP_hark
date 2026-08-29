# Research Collaboration Request to Active or Closed Agreement

# 1. Scenario Metadata

**Scenario ID:** L3-H-036  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, F, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; locked Finance architecture; `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose

Govern an institutional Research Collaboration through request, review, approval/rejection, activation, amendment, termination or closure while preserving external agreement, workforce and Finance ownership boundaries.

# 3. Business Outcome

Due diligence and approval establish, reject, amend, terminate, or close a governed collaboration

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An internal or external party proposes research collaboration.

## 4.2 End Boundary

The scenario ends when the Collaboration reaches an authoritative governed outcome: Rejected, Active, Terminated or Closed, with amendment history preserved where applicable.

## 4.3 In Scope

Partner/agreement validation, request/review, approval/rejection, activation, amendment, termination, closure and Research-owned collaboration evidence.

## 4.4 Out of Scope

Ownership of external legal agreements, E03 workforce facts, or Finance monetary/accounting facts.

# 5. Ownership and Participating Domains

E03 owns workforce facts. Finance owns monetary/accounting consequences. Research owns the Research Collaboration relationship, its lifecycle, Research activities and Research events. External agreement/legal facts remain authoritative references.

# 6. Business Actors, Roles and Responsibilities

Research Office administers the Collaboration process. Principal Investigator participates in Research collaboration activity. Research Director is the canonical business authority for strategic approval, rejection, amendment, termination and closure decisions where institutional approval is required.

# 7. Preconditions and Entry Criteria

- Partner organizations and agreement context are identified.
- Required due-diligence evidence is available.
- Participants are valid.
- Applicable institutional approval requirements are known.

# 8. Trigger

An internal or external party proposes research collaboration

# 9. Main Business Flow

1. Record the Collaboration request in `Requested`.
2. Validate partner, agreement and participant context.
3. Move to `Under Review` and complete institutional review.
4. If rejected, move to `Rejected` and publish `ResearchCollaborationRejected`.
5. If approved, move to `Approved` and then `Active`, publishing `ResearchCollaborationActivated` when the relationship becomes active.
6. If an amendment is required, move `Active → Amendment Pending`; after governed approval, return to `Active` and publish `ResearchCollaborationAmendmentApproved`.
7. If the collaboration is terminated, move `Active / Amendment Pending → Terminated` and publish `ResearchCollaborationTerminated`.
8. When the collaboration completes normally, move `Active → Closed` and publish `ResearchCollaborationClosed`.
9. Preserve immutable agreement references, decisions, activities and lifecycle history.

# 10. Alternate Business Flows

- `Under Review → Rejected`.
- `Active → Amendment Pending → Active` after governed amendment approval.
- `Active / Amendment Pending → Terminated`.

# 11. Failure, Interruption and Business Recovery

Unverified partner/agreement context prevents approval. Rejected, Closed and Terminated collaboration records remain historical and immutable. A later collaboration requires a new governed relationship rather than rewriting the old one.

# 12. Business Rules and Decision Constraints

- Canonical lifecycle: `Requested → Under Review → Approved → Active → Closed`.
- Rejected, amendment and termination branches follow only the locked H05 transitions.
- Research governs only the Research collaboration relationship.
- Finance and E03 facts remain externally owned.
- External legal agreement facts remain references to their authoritative evidence.

# 13. State and Lifecycle Impact

Canonical Collaboration lifecycle:

`Requested → Under Review → Approved → Active → Closed`

Branches:

- `Under Review → Rejected`
- `Active → Amendment Pending → Active`
- `Active / Amendment Pending → Terminated`

# 14. Business Events and Cross-Domain Collaboration

Research publishes:

- `ResearchCollaborationRejected`
- `ResearchCollaborationActivated`
- `ResearchCollaborationAmendmentApproved`
- `ResearchCollaborationTerminated`
- `ResearchCollaborationClosed`

E03 and Finance facts remain consumed by reference only.

# 15. Business Authorization, Approval and Audit Obligations

Research Director owns the applicable strategic collaboration decisions; Research Office records/administers them. Evidence preserves partner/agreement references, review, decision authority, amendment history, termination/closure rationale and complete lifecycle history.

# 16. Completion Criteria and Postconditions

Completion occurs when the collaboration reaches Rejected, Active, Terminated or Closed according to the governed branch and all evidence is preserved. External agreement, E03 and Finance ownership remain unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former missing collaboration rejection, amendment, termination and closure semantics were resolved by locked `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26.

Finance participation does not imply a collaboration-specific Finance-owned outcome unless a Finance fact is actually required; Research does not invent such a fact.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific questions.
