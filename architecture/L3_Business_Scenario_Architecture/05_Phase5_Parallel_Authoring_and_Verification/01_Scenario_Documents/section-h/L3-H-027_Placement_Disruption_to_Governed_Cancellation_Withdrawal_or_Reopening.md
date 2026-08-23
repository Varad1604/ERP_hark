# Placement Disruption to Governed Cancellation, Withdrawal, or Reopening

# 1. Scenario Metadata

**Scenario ID:** L3-H-027  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Placement  
**Participating Domains:** C, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H04 — Placement Workflows; L2-C02 — Student Lifecycle & Activation Workflows; L2-E01 — Workforce & Engagement Workflows  
**Referenced By:** Placement exception handling and audit

# 2. Scenario Purpose

Govern a Placement disruption using only exception paths that H04 explicitly defines, while preserving all prior lifecycle history and refusing to generalize one aggregate's exception mechanics into another.

# 3. Business Outcome

The affected lifecycle reaches a permitted exception state and any reopening preserves the full prior history

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a drive, application, selection, or offer cannot continue normally.

## 4.2 End Boundary

H04 explicitly supports offer withdrawal and candidate reconsideration/reinstatement. It does not define PlacementDrive cancellation/reopening, CandidateApplication withdrawal/cancellation, or Internship cancellation/withdrawal lifecycle semantics. Because the frozen trigger spans multiple aggregates, the complete frozen outcome is materially unresolved.

## 4.3 In Scope

Offer withdrawal; candidate eligibility reopening after withdrawal; authorized candidate reconsideration/reinstatement; RecruitmentWorkflow resumption; preservation of prior history; identification of unsupported branches.

## 4.4 Out of Scope

Invented generic cancellation/reopening states, reversal of completed Placement, Student/workforce changes, and implementation-level recovery mechanics.

# 5. Ownership and Participating Domains

C owns Student facts; E owns workforce facts; H owns exception consequences only for H04 Placement aggregates where a canonical state/transition/event exists.

# 6. Business Actors, Roles and Responsibilities

Placement Officer reviews/approves supported offer withdrawal and candidate reconsideration. Placement Coordinator supports candidate/recruitment activity where H04 permits. Employer requests remain external inputs recorded by Placement staff.

# 7. Preconditions and Entry Criteria

Affected Placement aggregate exists; prior history is preserved; reconsideration is explicitly authorized; offer withdrawal occurs before joining confirmation; reopening never deletes earlier elimination/withdrawal evidence.

# 8. Trigger

A drive, application, selection, or offer cannot continue normally

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Identify the affected Placement aggregate/current state and determine whether H04 defines an exception path. | H | Only canonical exceptions proceed. |
| 2 | For a valid offer withdrawal, apply the governed withdrawal path. | H | Offer becomes `Withdrawn`. |
| 3 | Where policy permits, reopen candidate eligibility after withdrawal. | H | Candidate eligibility is reopened with prior history. |
| 4 | For an eliminated candidate, verify reconsideration authority, reinstate the candidate, and resume recruitment from the appropriate stage. | H | Recruitment reopens with elimination history preserved. |
| 5 | For drive/application/internship disruption lacking a canonical H04 path, preserve current state and escalate the gap. | H | **BLOCKED:** no invented exception state is created. |

# 10. Alternate Business Flows

- Offer withdrawal: supported before joining confirmation.
- Candidate reconsideration: supported with explicit authorization.
- PlacementDrive cancellation/reopening: undefined.
- CandidateApplication withdrawal/cancellation: mentioned in scope but no lifecycle state/transition/event.
- Internship cancellation/withdrawal: listed as failures but no lifecycle semantics.

# 11. Failure, Interruption and Business Recovery

Unauthorized reconsideration leaves elimination in force. Withdrawal after joining confirmation is rejected. Unsupported disruption paths preserve current state/history until upstream architecture defines a valid exception outcome.

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Effect |
|---|---|---|
| Eliminated candidates cannot progress without authorized reconsideration. | H | Reopening is explicit and governed. |
| Reinstatement preserves complete prior history. | H | Elimination is not erased. |
| Offer withdrawal is permitted only before joining confirmation; withdrawn offers remain auditable. | H | Withdrawal is bounded and historical. |
| Candidate eligibility reopening after withdrawal follows institutional Placement policy. | H | Reopening is policy-governed. |
| Closed drives are immutable. | H | L3 cannot invent drive reopening. |

# 13. State and Lifecycle Impact

PlacementOffer may become `Withdrawn`; CandidateApplication may be reinstated through the H04 reconsideration workflow; RecruitmentWorkflow may resume from `Completed` to `In Progress`. PlacementDrive cancellation/reopening and Internship cancellation/withdrawal are undefined. C/E lifecycles are unchanged.

# 14. Business Events and Cross-Domain Collaboration

Supported exact events include `OfferWithdrawalRequested`, `PlacementOfferWithdrawn`, `CandidateEligibilityReopened`, `CandidateReconsiderationRequested`, `CandidateReinstated`, and `RecruitmentWorkflowResumed`. No canonical PlacementDrive-cancel, CandidateApplication-withdraw/cancel, or Internship-cancel/withdraw event exists.

# 15. Business Authorization, Approval and Audit Obligations

Candidate reconsideration requires authorized personnel; offer withdrawal requires H04 Placement review. Missing exception branches have no authority to assign. Evidence preserves prior state, disruption reason/input, approval where defined, resulting supported state, reopened workflow/eligibility, and immutable prior history.

# 16. Completion Criteria and Postconditions

Supported offer-withdrawal and candidate-reconsideration branches can complete. The frozen cross-aggregate scenario cannot fully complete until drive/application/internship exception lifecycles are defined upstream.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| No PlacementDrive cancellation/reopening state, transition, event or authority. | Frozen drive-disruption branch cannot complete. | Define upstream. |
| CandidateApplication scope mentions withdrawal but its lifecycle has no withdrawal/cancel semantics. | Frozen application-disruption branch cannot complete. | Define upstream. |
| Internship failure scenarios mention cancellation/withdrawal but no lifecycle semantics exist. | Internship disruption cannot complete canonically. | Define upstream. |

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
