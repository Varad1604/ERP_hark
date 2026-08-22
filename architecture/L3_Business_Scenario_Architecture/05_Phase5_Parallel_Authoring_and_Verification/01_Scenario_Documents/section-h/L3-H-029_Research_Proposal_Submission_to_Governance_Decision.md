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
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 Faculty Workforce; applicable locked Finance architecture  
**Referenced By:** Research project initiation, research governance, grant/funding activity, research audit, and institutional research portfolio management

# 2. Scenario Purpose

Govern a complete submitted research proposal through institutional review to an approval, rejection, or revision-required decision while preserving Research ownership of proposal governance and the source ownership of workforce and financial information.

# 3. Business Outcome

Review produces approval, rejection, or revision-required outcome with immutable decision evidence

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a researcher submits a complete proposal.

## 4.2 End Boundary

The scenario ends when the submitted proposal reaches one governed decision outcome: `Approved`, `Rejected`, or `Revision Requested`, and the review/decision evidence required by H05 is preserved.

## 4.3 In Scope

- Validating proposal completeness and investigator eligibility before submission.
- Submitting the Research Proposal and making the submitted proposal read-only except through governed revision.
- Assigning/performing proposal review.
- Recording review comments and scores.
- Producing approval, rejection, or revision-required outcome under configured institutional governance.
- Preserving decision, reviewer, approval-route, reason, and revision evidence.

## 4.4 Out of Scope

- Proposal withdrawal, governed by L3-H-030.
- Research Project execution after approval, governed by L3-H-031.
- Grant lifecycle after proposal/project eligibility, governed by L3-H-032.
- Faculty eligibility ownership or Finance accounting ownership.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| People & Workforce / Section E | Owns Faculty Research Capability, Faculty Availability, Faculty Capacity, Faculty Eligibility, and workforce identity used to validate the Principal Investigator. |
| Finance / Section F | Retains ownership of funding schemes, budgets, accounting, and financial facts referenced where proposal funding information is relevant. |
| Research / Section H | Owns the Research Proposal aggregate, review records, proposal approval/rejection/revision lifecycle, and Research proposal events. |

Cross-domain validation does not transfer workforce or Finance ownership to Research.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Principal Investigator | Submits the complete Research Proposal. | Initiates formal proposal governance. |
| Reviewer / Research Committee Member | Evaluates the submitted proposal and records governed review evidence. | Reviews. |
| Research Committee / Dean / Research Director / Institution-specific Approver | Makes or participates in the configured approval decision. | Approves according to H05-configured hierarchy. |
| Authorized Approver | Records a governed rejection decision where applicable. | Rejects. |
| Reviewer / Research Committee | Requests revision where proposal changes are required. | Returns proposal for revision. |

# 7. Preconditions and Entry Criteria

- The Research Proposal exists in `Draft` and is complete for submission.
- The Principal Investigator eligibility is derived from L2-E03 Faculty Workforce.
- Mandatory proposal information and required document references are available.
- Applicable research category, ethics, funding, and approval-policy validations are satisfied sufficiently for submission.
- The submitter is authorized under the Research proposal workflow.

# 8. Trigger

A researcher submits a complete proposal

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate proposal completeness, required evidence, investigator eligibility, and applicable proposal governance conditions. | Principal Investigator / Research with source-domain validation | Only a complete, eligible proposal proceeds. |
| 2 | Submit the Research Proposal and enter `Submitted`. | Principal Investigator / Research | `ResearchProposalSubmitted` records formal submission. |
| 3 | Route the submitted proposal into the configured institutional review process and begin review. | Research | Proposal enters governed review. |
| 4 | Record reviewer evaluation, comments, and scores without changing proposal ownership. | Reviewer / Research Committee Member / Research | Immutable review evidence is established. |
| 5 | Apply the configured governance decision path after required review evidence is complete. | Research governance roles | Proposal is ready for a permitted decision. |
| 6 | Approve, reject, or request revision according to the authoritative H05 proposal lifecycle. | Authorized Research governance role | Proposal reaches `Approved`, `Rejected`, or `Revision Requested`. |
| 7 | Preserve decision authority, decision reason or revision comments, review history, and applicable Research events. | Research | Decision evidence is immutable and traceable. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Revision required | Review identifies changes required before a final decision. | Record revision comments and move `Under Review` → `Revision Requested`. | Revision-required outcome is recorded; prior proposal/review history remains preserved. | Research |
| Proposal rejected | Authorized governance decides not to approve. | Record mandatory rejection reason and move `Under Review` → `Rejected`. | Proposal lifecycle completes as rejected. | Research |
| Multiple configured reviews | Institutional governance requires multiple or sequential reviewers. | Complete the configured review sequence before final decision. | Decision remains based on preserved review evidence. | Research |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Proposal is incomplete | Formal review cannot begin safely. | Reject the submission request and retain the proposal in its permitted pre-submission state. | Research |
| Reviewer is not authorized or assigned | Review evidence is invalid. | Do not accept the review; resolve reviewer assignment under Research governance. | Research |
| Required approval criteria are incomplete | Approval cannot be issued. | Keep the proposal in the governed review path until required approval conditions are satisfied. | Research |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every proposal has exactly one Principal Investigator. | Research | Submission must retain one authoritative PI reference. | L2-H05 PR-02 |
| Principal Investigator eligibility is derived from L2-E03 Faculty Workforce. | People & Workforce | Research does not determine workforce eligibility independently. | L2-H05 §§4, 24.1, 25.5 |
| Submitted proposals become read-only except through governed revision. | Research | Review cannot silently rewrite the submitted proposal. | L2-H05 §24.2 |
| Approval workflow is institution configurable and required approvals must be satisfied. | Research | Decision follows configured governance rather than a locally invented route. | L2-H05 PR-04; §24.4 |
| Rejection reason is mandatory and rejection is immutable. | Research | Rejected outcome retains permanent rationale. | L2-H05 §24.6 |
| Revision history remains immutable. | Research | Revision-required outcome preserves prior review/proposal versions. | L2-H05 §24.5 |
| Every approval decision generates immutable audit evidence. | Research / Section A audit infrastructure | Decision evidence must be preserved. | L2-H05 PR-08 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Research Proposal | `Draft` | Submission moves proposal to `Submitted`, then review moves it to `Under Review`. | `Under Review` before decision. | Research | L2-H05 §25.1 |
| Research Proposal | `Under Review` | Governance decision is recorded. | `Approved`, `Rejected`, or `Revision Requested`. | Research | L2-H05 §§24.4–24.6, 25.1 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Applicable exact Research events are `ResearchProposalSubmitted`, `ProposalReviewStarted`, `ResearchProposalReviewed`, `ResearchProposalApproved`, `ResearchProposalRejected`, and `ResearchProposalRevisionRequested`. Research is the publisher of these Proposal Aggregate events.

H05 also consumes source-domain validation facts such as `FacultyVerified` and funding/configuration facts where applicable; those sources retain their canonical ownership.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Research references workforce eligibility from L2-E03 and Finance-owned funding information when proposal funding context requires it. Research stores proposal funding request metadata only and does not assume Finance accounting ownership.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The Principal Investigator submits the proposal. Reviewer/Research Committee roles review it. H05 permits approval through Research Committee, Dean, Research Director, or institution-specific approver according to configured hierarchy; rejection uses an Authorized Approver; revision may be requested by Reviewer or Research Committee.

## 15.2 Separation of Duties, Overrides and Exceptions

No additional separation-of-duty rule is defined by H05 beyond configured authorization and approval hierarchy. L3 does not invent one.

## 15.3 Business Audit and Evidence Obligations

Preserve submitter, submission timestamp, reviewer identity, review scores/comments, approval route, decision authority, decision timestamp, rejection reason where applicable, revision comments/history where applicable, proposal state transitions, and published Research proposal events.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The proposal was formally submitted.
- Required review evidence was recorded.
- A permitted governance decision was made.
- The proposal is `Approved`, `Rejected`, or `Revision Requested`.
- Decision and review evidence is preserved.
- Workforce and financial source ownership is unchanged.

## 16.2 Postconditions

- `Approved` proposals become eligible for Research Project initiation.
- `Rejected` proposals are immutable and cannot directly become Approved.
- `Revision Requested` proposals may return through the governed revision path while prior history remains preserved.
- Applicable Research events are available for downstream governed workflows.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No material scenario-specific architecture issue prevents this Draft. H05 explicitly defines the relevant actors, proposal states, decision rules, events, and audit evidence.

**Architecture assessment:** `PASS`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No deferred questions.
