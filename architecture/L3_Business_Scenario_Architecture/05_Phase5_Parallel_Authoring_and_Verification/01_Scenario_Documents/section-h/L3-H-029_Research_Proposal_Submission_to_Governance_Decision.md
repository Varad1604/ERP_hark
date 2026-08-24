# Research Proposal Submission to Governance Decision

# 1. Scenario Metadata

**Scenario ID:** L3-H-029  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, F, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; applicable locked Finance architecture

# 2. Scenario Purpose

Govern a complete Research Proposal from formal submission through review to an approval, rejection, or revision-required governance decision while preserving Research ownership of proposal state and the upstream ownership of researcher eligibility and financial facts.

# 3. Business Outcome

Review produces approval, rejection, or revision-required outcome with immutable decision evidence

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a researcher submits a complete proposal.

## 4.2 End Boundary

The scenario ends when the proposal reaches `Approved`, `Rejected`, or `Revision Requested` and the review and decision evidence remains immutable and traceable.

## 4.3 In Scope

- Validate proposal completeness and Principal Investigator eligibility.
- Submit the proposal, route it for review, and preserve review evidence.
- Record an approval, rejection, or revision-required decision under configured Research governance.
- Preserve decision history and source-domain references.

## 4.4 Out of Scope

- Proposal withdrawal.
- Research Project execution after approval.
- Finance accounting, budgets, payments, or postings.
- Faculty capability, availability, capacity, eligibility, or workforce lifecycle changes.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| People & Workforce / E | Owns Faculty Research Capability, availability, capacity, eligibility, and workforce identity used by Research. |
| Finance / F | Retains ownership of financial facts and funding context referenced by the proposal. |
| Research / H | Owns Research Proposal state, review records, governance decisions, and Proposal Aggregate events. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Principal Investigator | Submits the complete proposal. |
| Reviewer / Research Committee Member | Records proposal review evidence. |
| Research Committee / Dean / Research Director / Institution-specific Approver | Performs configured approval where authorized. |
| Authorized Approver | Records rejection where authorized. |
| Reviewer / Research Committee | Requests revision where required. |

# 7. Preconditions and Entry Criteria

- The proposal exists in `Draft` and is complete.
- Principal Investigator eligibility is derived from L2-E03 Faculty Workforce.
- Required documents and configured proposal-governance information are available.
- The applicable approval workflow is configured.
- Submission authority is valid.

# 8. Trigger

A researcher submits a complete proposal

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain / Role | Result |
|---|---|---|---|
| 1 | Validate proposal completeness, investigator eligibility, mandatory evidence, and applicable governance configuration. | Research with E/F source facts | Only an eligible complete proposal proceeds. |
| 2 | Submit the proposal. | Principal Investigator / Research | Proposal enters `Submitted`; `ResearchProposalSubmitted` is published. |
| 3 | Assign the governed review and begin review. | Research | Proposal enters `Under Review`; `ProposalReviewStarted` is published. |
| 4 | Record review comments, scores, and evaluation evidence. | Reviewer / Research Committee Member | `ResearchProposalReviewed` records the completed review fact. |
| 5 | Apply the configured governance decision after required review evidence is complete. | Authorized Research governance role | A permitted decision is selected. |
| 6 | Record approval, rejection, or revision-required outcome. | Research | Proposal reaches `Approved`, `Rejected`, or `Revision Requested`. |
| 7 | Preserve the decision route, authority, rationale/comments, and prior proposal/review history. | Research | Immutable governance evidence is available. |

# 10. Alternate Business Flows

- **Revision required:** move `Under Review` to `Revision Requested`; prior review and proposal history remains preserved.
- **Rejected:** record the mandatory rejection reason and move `Under Review` to `Rejected`.
- **Multiple reviews:** complete the institution-configured parallel or sequential review path before the final governance decision.

# 11. Failure, Interruption and Business Recovery

Incomplete proposal information prevents submission. Unauthorized review evidence is not accepted. Missing required approval conditions prevent approval. The proposal remains in the last valid Research-owned lifecycle position until the governed condition is satisfied.

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Effect |
|---|---|---|
| Every proposal has exactly one Principal Investigator. | Research | One PI reference is mandatory. |
| Researcher eligibility is derived from L2-E03. | People & Workforce | Research cannot decide faculty eligibility independently. |
| Submitted proposals are read-only except through governed revision. | Research | Review cannot silently rewrite submitted content. |
| Approval routing is institution configurable. | Research | L3 does not invent an approval route. |
| Rejection reason is mandatory and completed history is immutable. | Research | Rejection evidence is permanent. |
| Revision history remains immutable. | Research | Earlier proposal/review evidence is preserved. |

# 13. State and Lifecycle Impact

`Draft` → `Submitted` → `Under Review`, followed by `Approved`, `Rejected`, or `Revision Requested`. A revision may return through the H05-defined governed revision path. E/F source records are not changed.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H05 consistently defines `ResearchProposalSubmitted`, `ProposalReviewStarted`, `ResearchProposalReviewed`, `ResearchProposalApproved`, and `ResearchProposalRejected`. For the revision request, H05 is internally inconsistent: its detailed transition/event sections use `ResearchProposalRevisionRequested`, while the consolidated event catalogue uses `ProposalRevisionRequested`. The Proposal Aggregate remains the Research publisher; L3 does not choose a replacement canonical name.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Research consumes Faculty eligibility/capability facts from E03 and Finance-owned funding context where required. Neither participation transfers source ownership.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The Principal Investigator submits. H05 names Reviewer/Research Committee roles for review and revision, and Research Committee, Dean, Research Director, institution-specific approver, or Authorized Approver for configured decisions.

## 15.2 Separation of Duties, Overrides and Exceptions

No additional separation-of-duty rule or override is introduced at L3.

## 15.3 Business Audit and Evidence Obligations

Preserve submitter, submission timestamp, reviewer identity, review scores/comments, decision route, decision authority, decision timestamp, rejection rationale or revision comments as applicable, and proposal-state history.

# 16. Completion Criteria and Postconditions

The scenario completes when a valid submitted proposal has completed the required review and reaches `Approved`, `Rejected`, or `Revision Requested` with immutable evidence. Approved proposals become eligible for project initiation; rejected proposals remain terminal under H05; revision-required proposals follow governed revision.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

H05 uses two names for the same proposal-revision-request fact: `ResearchProposalRevisionRequested` in the detailed Proposal lifecycle and `ProposalRevisionRequested` in the consolidated event catalogue. The publisher and lifecycle outcome are unambiguous, so this does not block the business outcome, but the canonical event name requires upstream reconciliation.

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
