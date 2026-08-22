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
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H04 — Placement Workflows; applicable locked Student Lifecycle and People & Workforce architecture  
**Referenced By:** Placement exception handling, offer withdrawal, candidate reconsideration, drive governance, internship exceptions, and audit activities

# 2. Scenario Purpose

Govern Placement disruptions that prevent a drive, application, selection, or offer from continuing normally, using only the cancellation, withdrawal, reconsideration, and reopening paths explicitly defined by H04 while preserving all prior Placement history.

# 3. Business Outcome

The affected lifecycle reaches a permitted exception state and any reopening preserves the full prior history

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a Placement Drive, CandidateApplication, selection, PlacementOffer, or related placement activity cannot continue normally.

## 4.2 End Boundary

The frozen end boundary requires the affected lifecycle to reach a permitted cancellation, withdrawal, or reopening state with full prior history preserved. H04 fully defines offer withdrawal and candidate reconsideration/reopening, but does not define cancellation/withdrawal/reopening semantics for every frozen affected lifecycle; therefore the complete cross-aggregate outcome is materially unresolved.

## 4.3 In Scope

- Offer withdrawal before joining confirmation.
- Candidate eligibility reopening after governed offer withdrawal where policy permits.
- Authorized reconsideration and reinstatement of an eliminated candidate.
- Resuming RecruitmentWorkflow from the appropriate stage with history preserved.
- Preserving immutable prior elimination and withdrawal records.
- Identifying unsupported drive/application/internship cancellation branches without inventing states/events.

## 4.4 Out of Scope

- Creating generic cancellation, withdrawal, or reopening states not present in H04.
- Reversing a verified placement completion.
- Changing Student or workforce source records.
- Non-business recovery mechanics.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Student Lifecycle / Section C | Retains Student identity/lifecycle information referenced by Placement exceptions. |
| People & Workforce / Section E | Retains workforce identity/assignment information for Placement staff exercising source-supported responsibilities. |
| Placement / Section H | Owns placement disruption consequences for its Placement aggregates where H04 defines the relevant state, transition, event, and authority. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Placement Officer | Reviews employer withdrawal input, applies offer withdrawal, and participates in candidate reconsideration/reopening where H04 assigns responsibility. | Performs supported exception decisions/actions. |
| Placement Coordinator | Supports CandidateApplication/RecruitmentWorkflow exception handling where H04 assigns the role. | Performs supported placement coordination. |

H04 references an employer request for withdrawal/reconsideration as external input; no employer system actor is assumed.

# 7. Preconditions and Entry Criteria

- The affected Placement aggregate exists with preserved prior history.
- The disruption occurs before a branch's terminal condition where H04 requires it.
- Any reconsideration has explicit authorization.
- Any offer withdrawal occurs before joining confirmation.
- Reopening does not erase prior elimination or withdrawal evidence.

# 8. Trigger

A drive, application, selection, or offer cannot continue normally

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the affected Placement aggregate and its current lifecycle position without altering prior history. | Placement | Exception scope is established. |
| 2 | Determine whether H04 defines a permitted exception path for that aggregate/state. | Placement | Only canonical exception behaviour proceeds. |
| 3 | For a valid PlacementOffer withdrawal, review the external employer request and apply `PlacementOfferWithdrawn`. | Placement Officer / Placement | Offer reaches the supported `Withdrawn` state. |
| 4 | Where policy permits after withdrawal, apply `CandidateEligibilityReopened`. | Placement | Candidate eligibility reopens with prior offer history preserved. |
| 5 | For an eliminated candidate with authorized reconsideration, record the reconsideration request, reinstate the candidate, and resume recruitment from the appropriate stage. | Placement Officer / Placement Coordinator / Placement | Candidate/recruitment lifecycle reopens with elimination history preserved. |
| 6 | For a disrupted drive/application/internship branch lacking an H04 cancellation/reopening definition, stop short of inventing a lifecycle outcome and record the unresolved architecture gap. | Placement | Unsupported branches remain governed gaps rather than false states. |
| 7 | Preserve all prior and resulting exception evidence. | Placement | Supported exception outcomes remain auditable. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Offer withdrawal | Employer withdrawal is valid and joining is not confirmed. | Apply `OfferWithdrawalRequested` / `PlacementOfferWithdrawn`; reopen eligibility if allowed. | Supported withdrawal/reopening outcome. | Placement |
| Candidate reconsideration | Candidate is `Eliminated` and reconsideration is authorized. | Apply `CandidateReconsiderationRequested`, `CandidateReinstated`, and `RecruitmentWorkflowResumed`. | Supported candidate reopening with history. | Placement |
| Placement Drive cancellation | Drive cannot continue before closure. | H04 defines no PlacementDrive `Cancelled` state/transition/event. | **Blocked branch.** | Placement |
| Candidate application withdrawal/cancellation | Application cannot continue. | H04 scope mentions application withdrawal but no CandidateApplication withdrawal/cancel state, transition, or event exists. | **Blocked branch.** | Placement |
| Internship cancellation/withdrawal | Internship cannot continue. | H04 lists cancellation/withdrawal as failure scenarios but has no corresponding lifecycle semantics. | **Blocked branch.** | Placement |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Reconsideration is unauthorized | Eliminated candidate cannot progress. | Keep prior elimination in force. | Placement |
| Offer withdrawal occurs after joining confirmation | Withdrawal precondition fails. | Do not apply the H04 withdrawal path. | Placement |
| Disrupted lifecycle has no H04 exception state | Frozen cross-aggregate outcome cannot be completed safely. | Preserve current state/history and escalate upstream rather than inventing cancellation/reopening semantics. | Placement |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Eliminated candidates cannot progress without authorized reconsideration. | Placement | Reopening is permitted only through H04 reconsideration. | L2-H04 §§17.4, 19.11 |
| Reinstatement preserves complete historical records and prior elimination remains immutable. | Placement | Reopening never rewrites history. | L2-H04 §19.11 |
| Offer withdrawal is permitted only before joining confirmation and withdrawn offers remain auditable. | Placement | Supported withdrawal is bounded by the offer lifecycle. | L2-H04 §§17.6, 19.14 |
| Candidate eligibility reopening after withdrawal follows institutional placement policy. | Placement | Reopening is policy-governed. | L2-H04 §19.14 |
| Closed drives cannot be modified. | Placement | Drive closure cannot be locally “reopened” without an upstream lifecycle definition. | L2-H04 §§17.2, 19.16 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| PlacementOffer | `Published` before joining confirmation | Governed withdrawal may occur. | `Withdrawn` | Placement | L2-H04 §17.6 |
| CandidateApplication | `Eliminated` | Authorized reconsideration may reinstate the candidate. | Reinstated/reopened recruitment position as defined by H04 events/workflow | Placement | L2-H04 §§17.4, 19.11 |
| RecruitmentWorkflow | `Completed` after an elimination/review point | Authorized candidate reinstatement may resume workflow. | `In Progress` | Placement | L2-H04 §17.5 |
| PlacementDrive | Active pre-closure state | Frozen scenario may require cancellation, but no cancellation state exists. | **Undefined for cancellation** | Placement | L2-H04 §17.2 |
| Internship | Active state | H04 names cancellation/withdrawal failures but no corresponding state/transition exists. | **Undefined for cancellation/withdrawal** | Placement | L2-H04 §17.7 / §19.17 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Supported exact exception events include `OfferWithdrawalRequested`, `PlacementOfferWithdrawn`, `CandidateEligibilityReopened`, `CandidateReconsiderationRequested`, `CandidateReinstated`, and `RecruitmentWorkflowResumed`.

H04 defines no PlacementDrive cancellation event, CandidateApplication withdrawal/cancellation event, or Internship cancellation/withdrawal event.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Employer-originated withdrawal/reconsideration input is recorded by Placement staff as H04 defines. Student/workforce source ownership remains unchanged. Unsupported disruption paths are not assigned invented cross-domain contracts.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

H04 requires authorized personnel for candidate reconsideration and Placement review/approval for offer withdrawal. The missing generic drive/application/internship cancellation/reopening paths have no canonical authority to assign.

## 15.2 Separation of Duties, Overrides and Exceptions

Prior elimination and withdrawal history remains immutable. Reconsideration and withdrawal cannot silently erase earlier outcomes.

## 15.3 Business Audit and Evidence Obligations

Evidence must preserve affected aggregate identity, prior state, disruption reason/input, withdrawal/reconsideration request, decision authority where defined, resulting supported state, reopened eligibility/workflow position, and immutable prior history.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

Supported branches complete when a PlacementOffer is withdrawn and/or candidate eligibility is reopened under policy, or when an eliminated candidate is authorized, reinstated, and recruitment resumes with full history. The frozen scenario cannot be completed for disrupted drives, application withdrawal/cancellation, or Internship cancellation/withdrawal because H04 lacks canonical lifecycle semantics for those branches.

## 16.2 Postconditions

- Supported exception history remains preserved.
- No unsupported lifecycle is locally invented.
- Placement retains ownership of Placement exception consequences.
- Student/workforce source ownership remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| H04 defines no PlacementDrive `Cancelled` state, cancellation transition, event, or reopening path. | The frozen trigger explicitly includes a drive that cannot continue, and the final outcome requires a permitted exception state/reopening. | Define governed PlacementDrive cancellation/reopening semantics upstream if required. |
| H04's stated scope includes application withdrawal, but CandidateApplication has no withdrawal/cancellation state, transition, or event. | The application-disruption branch cannot reach a canonical exception outcome. | Define CandidateApplication withdrawal/cancellation semantics upstream. |
| H04 lists Internship cancellation and candidate withdrawal as failure scenarios but defines no corresponding Internship state, transition, or event. | Internship disruption cannot reach a canonical exception outcome. | Define Internship cancellation/withdrawal semantics upstream. |
| H04 fully supports offer withdrawal and candidate reconsideration, but those mechanisms do not cover every aggregate named by the frozen trigger. | L3 cannot generalize one aggregate's exception mechanics into another aggregate's lifecycle. | Resolve the missing per-aggregate exception architecture rather than applying a generic L3 rule. |

**Architecture assessment:** `BLOCKED`

Only the offer-withdrawal and candidate-reconsideration/reopening branches are source-grounded. The frozen cross-aggregate cancellation/withdrawal/reopening outcome is materially incomplete upstream.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
