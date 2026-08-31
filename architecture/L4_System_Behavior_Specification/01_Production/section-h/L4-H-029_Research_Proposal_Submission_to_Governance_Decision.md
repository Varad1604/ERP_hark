# Research Proposal Submission to Governance Decision

## 1. Specification Metadata
**Specification ID:** L4-H-029  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Research  
**Realizes Scenario:** L3-H-029  
**Primary Business Owner:** Research  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-029; L2-H05 — Research Workflows; E03 workforce facts; Finance-owned funding context  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern a complete Research Proposal from submission through review to `Approved`, `Rejected` or `Revision Requested` while consuming workforce/funding facts without re-owning them.

## 3. Required Business Context
Draft complete proposal; Principal Investigator; E03 eligibility; required evidence; configured Research approval workflow and valid submission authority.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-029-BH-001 | A proposal shall be admitted from `Draft` only when complete, PI eligibility is authoritative from E03, mandatory evidence exists and the configured Research governance route is available. | L3-H-029 §§7–9 |
| L4-H-029-BH-002 | Submission shall move `Draft → Submitted`, review shall move `Submitted → Under Review`, and completed required review evidence shall precede any final governance decision. | L3-H-029 §§9,12–13 |
| L4-H-029-BH-003 | The governed decision shall move `Under Review` to `Approved`, `Rejected` or `Revision Requested`; rejection shall preserve a mandatory reason and revision shall preserve prior immutable submission/review history. | L3-H-029 §§9–13 |
| L4-H-029-BH-004 | Submitted proposal content shall be read-only except through the governed revision path; Research shall not decide faculty eligibility or Finance-owned funding facts. | L3-H-029 §§5,12 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-029-BH-005 | Research shall publish `ResearchProposalSubmitted`, `ProposalReviewStarted`, `ResearchProposalReviewed`, `ResearchProposalApproved` and `ResearchProposalRejected` for their corresponding facts. The revision-request event shall retain the parent-documented unresolved naming inconsistency rather than invent a replacement canonical name. | L3-H-029 §14 |

## 6. Failure / Rejection / Continuation
Incomplete proposals, unauthorized review evidence or missing required approval conditions shall prevent progression and leave the proposal in its last valid Research state.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-029-BH-006 | The system shall preserve submitter/time, reviewer identity, scores/comments, configured decision route, decision authority/time, rejection rationale or revision comments and full state history. | L3-H-029 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-029-BH-007 | Completion shall require required review and a recorded `Approved`, `Rejected` or `Revision Requested` outcome with immutable evidence; E/F source ownership remains unchanged. | L3-H-029 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-029 — Research Proposal Submission to Governance Decision  
**Primary verification source:** L2-H05 — Research Workflows.

## 10. Open Issues / Deferred Design
H05 contains two names for the proposal-revision-request fact. This L4 specification preserves the conflict for upstream reconciliation and does not choose a new canonical name.