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
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 Faculty Workforce  
**Referenced By:** Research proposal governance, proposal history, Research Project initiation controls, and research audit

# 2. Scenario Purpose

Govern withdrawal of a submitted Research Proposal while withdrawal remains permitted, preserving prior submission/review history and preventing further progression after withdrawal.

# 3. Business Outcome

The proposal becomes withdrawn, cannot progress further, and its submitted and review history remains auditable

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The frozen scenario begins when a proposer requests withdrawal before final approval or project activation. H05 defines a narrower permitted withdrawal boundary; the conflict is recorded in Section 17.

## 4.2 End Boundary

For H05-permitted states, the scenario ends when the proposal reaches `Withdrawn`, `ResearchProposalWithdrawn` is published, further proposal progression is prohibited, and prior evidence remains immutable.

## 4.3 In Scope

- Validate withdrawal authority and proposal state.
- Record the mandatory withdrawal reason.
- Move an eligible proposal to `Withdrawn`.
- Preserve proposal/review history and publish the canonical withdrawal event.

## 4.4 Out of Scope

- Project suspension, completion, or closure.
- Withdrawal of an Approved proposal unless upstream architecture is changed.
- Inventing a missing withdrawal transition.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| People & Workforce / Section E | Retains canonical proposer/Principal Investigator workforce identity and eligibility. |
| Research / Section H | Owns Research Proposal withdrawal validation, transition, history, and event publication. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Principal Investigator | Requests withdrawal and supplies the reason. | Initiates withdrawal. |
| Delegated authority | May request withdrawal where H05 permits delegation. | Initiates under governed delegation. |

# 7. Preconditions and Entry Criteria

- A submitted Research Proposal exists.
- Requestor authority can be validated.
- A withdrawal reason is available.
- The proposal is in an H05-permitted withdrawal state.

# 8. Trigger

A proposer requests withdrawal before final approval or project activation

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the withdrawal request and identify the proposal/requestor. | Principal Investigator / Research | Request is associated with one proposal. |
| 2 | Validate requestor authority and current proposal state. | Research | Only an upstream-permitted path proceeds. |
| 3 | Record the mandatory withdrawal reason. | Research | Withdrawal evidence is complete. |
| 4 | Transition an eligible proposal to `Withdrawn`. | Research | Proposal lifecycle terminates. |
| 5 | Publish `ResearchProposalWithdrawn` and preserve prior history. | Research | Withdrawal is immutable and auditable. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Under Review | Proposal is `Under Review`. | Move to `Withdrawn` after authority/reason validation. | Review ends with history preserved. | Research |
| Revision Requested | H05 workflow permits withdrawal but its state matrix omits the transition. | Do not choose an interpretation. | Remains unresolved upstream. | Research |
| Approved | Proposal is already `Approved`. | Reject proposal-withdrawal request. | Remains `Approved`. | Research |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Unauthorized requestor | Withdrawal is invalid. | Reject request; preserve current state. | Research |
| Missing reason | Required evidence is incomplete. | Do not transition until reason exists. | Research |
| Approved proposal | H05 forbids withdrawal. | Reject withdrawal; do not invent a transition. | Research |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Withdrawal reason is mandatory. | Research | Withdrawal cannot complete without reason. | L2-H05 §24.7 |
| Requestor must be Principal Investigator or delegated authority. | Research / E source identity | Authority must be validated. | L2-H05 §24.7 |
| Approved proposals cannot be withdrawn. | Research | No `Approved` → `Withdrawn` transition is permitted. | L2-H05 §§24.7, 25.2 |
| Withdrawn proposals cannot be resubmitted or edited. | Research | Withdrawal is terminal. | L2-H05 §§24.7, 25.2 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Research Proposal | `Submitted` / `Under Review` | Authorized withdrawal. | `Withdrawn` | Research | L2-H05 §25.1 |
| Research Proposal | `Revision Requested` | Workflow/state-matrix conflict. | Unresolved. | Research | L2-H05 §§24.7, 25.1 |
| Research Proposal | `Approved` | Withdrawal prohibited. | `Approved` | Research | L2-H05 §§24.7, 25.2 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Research publishes `ResearchProposalWithdrawn` for a valid withdrawal. Relevant proposal history includes `ResearchProposalSubmitted` and `ResearchProposalReviewed` where applicable.

## 14.2 Business Contracts and Cross-Domain Hand-offs

L2-E03 retains workforce identity/eligibility ownership. Proposal withdrawal does not alter workforce state.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Principal Investigator or permitted delegated authority may request withdrawal. No authority may override H05's prohibition on withdrawing an Approved proposal.

## 15.2 Separation of Duties, Overrides and Exceptions

No additional separation-of-duty rule is introduced at L3.

## 15.3 Business Audit and Evidence Obligations

Preserve requestor, reason, timestamp, proposal identity, previous state/history, resulting state, and `ResearchProposalWithdrawn`.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

For the unambiguous H05 path, authority and reason are valid, the proposal moves from `Submitted` or `Under Review` to `Withdrawn`, the canonical event is published, and history is preserved.

## 16.2 Postconditions

A validly withdrawn proposal is terminal and auditable. The full frozen boundary remains unresolved pending Section 17.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Frozen trigger includes withdrawal before project activation, potentially after proposal approval, while H05 says Approved proposals cannot be withdrawn. | Frozen boundary is broader than locked lifecycle. | Clarify frozen boundary or define a governed post-approval/pre-project path upstream. |
| H05 §24.7 permits withdrawal from `Revision Requested`, but §25.1 has no such transition. | Lifecycle authority is internally inconsistent. | Reconcile H05 workflow and state matrix. |

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
