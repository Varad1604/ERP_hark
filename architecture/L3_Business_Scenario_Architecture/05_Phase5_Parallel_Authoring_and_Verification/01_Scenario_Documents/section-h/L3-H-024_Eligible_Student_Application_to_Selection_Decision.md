# Eligible Student Application to Selection Decision

# 1. Scenario Metadata

**Scenario ID:** L3-H-024  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Placement  
**Participating Domains:** C, D, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H04 — Placement Workflows; applicable locked Student Lifecycle and Academic Delivery architecture  
**Referenced By:** Placement candidate screening, shortlisting, recruitment rounds, offer generation, placement reporting, and audit activities

# 2. Scenario Purpose

Govern an eligible student's application to an active placement opportunity through registration, eligibility verification, screening, shortlisting, recruitment evaluation, and a Placement-owned selection or elimination outcome without transferring ownership of Student or academic source facts to Placement.

# 3. Business Outcome

Eligibility, assessment, interview, and employer feedback produce an authoritative selection or rejection outcome

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a student applies for an active placement opportunity during the governed registration window.

## 4.2 End Boundary

The scenario ends when the CandidateApplication reaches a governed `Selected` or `Eliminated` position after required eligibility, screening, shortlisting, and recruitment evaluation, with the decision history preserved.

## 4.3 In Scope

- Registering the student's application to one Position.
- Validating placement eligibility and institutional placement policy.
- Screening the application using source-supported academic and position requirements.
- Ranking, shortlisting, and publishing the shortlist where applicable.
- Executing mandatory recruitment rounds and recording evaluations.
- Preserving employer-originated selection feedback as reviewed placement input.
- Recording `CandidateSelected` or `CandidateEliminated` as the Placement-owned selection outcome.

## 4.4 Out of Scope

- Creating or changing Student master/lifecycle information.
- Creating or changing canonical academic records.
- Offer issuance and joining confirmation, governed by L3-H-025.
- Candidate reconsideration after elimination except as an alternate governed branch.
- Employer account/authentication design.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Student Lifecycle / Section C | Owns Student identity/lifecycle information used to establish the candidate context. |
| Academic Delivery / Section D | Provides applicable D-owned academic-delivery information used by Placement eligibility/screening where authoritative. |
| Placement / Section H | Owns CandidateApplication, recruitment screening/shortlisting/progression, recruitment evaluation history, and the final selection/elimination outcome. |

Placement does not become owner of Student or academic source facts.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Student | Applies to a published Position and provides the governed candidate participation input supported by H04. | Initiates the application. |
| Placement Officer | Reviews screening/shortlisting and employer selection input where H04 assigns the role. | Performs Placement-owned review/decision activities. |
| Placement Coordinator | Supports screening, recruitment, and candidate progression where H04 assigns that role. | Performs supported Placement coordination. |

Employer selection input is external to SutraERP and is recorded/reviewed by authorized placement staff under H04; no employer system actor is invented.

# 7. Preconditions and Entry Criteria

- The Placement Position is published and registration is open.
- The Student is active and identifiable from the canonical Student domain.
- Applicable eligibility rules are published.
- The recruitment workflow is assigned to the Position.
- Required source-owned academic information is available from its canonical owner.
- Placement policy applicable to the application is available.

# 8. Trigger

A student applies for an active placement opportunity

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Register the student's application to the published Position and preserve the source Student reference. | Student / Placement | One CandidateApplication represents one Student applying for one Position. |
| 2 | Validate mandatory eligibility and institutional placement policy using authoritative source information. | Placement using C/D source facts | The application becomes eligible or is stopped from progressing. |
| 3 | Execute the governed candidate-screening process, including academic eligibility, position requirement matching, and manual review where required. | Placement Officer / Placement Coordinator / Placement | `CandidateScreened` records the governed screening outcome. |
| 4 | Rank screened candidates, apply shortlisting criteria, obtain the H04-supported shortlist approval, and publish the shortlist. | Placement Officer / Placement | Shortlisted candidates become eligible for recruitment rounds. |
| 5 | Execute mandatory recruitment rounds sequentially and record evaluations and advancement decisions. | Placement Officer / Placement Coordinator / Placement | Candidate progression remains traceable across required stages. |
| 6 | If the candidate fails a governed recruitment decision, record `CandidateEliminated`. | Placement | The CandidateApplication reaches an elimination/rejection position. |
| 7 | If the candidate successfully completes required stages and employer selection is finalized, record the Placement-owned selected outcome and `CandidateSelected`. | Placement Officer / Placement | The CandidateApplication reaches `Selected`. |
| 8 | Preserve all eligibility, screening, shortlisting, evaluation, advancement, elimination/selection, and decision evidence. | Placement | The authoritative placement decision is auditable. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Eligibility failure | Mandatory eligibility or placement policy is not satisfied. | Do not progress the application into recruitment. | No unauthorized candidate progresses. | Placement |
| Candidate not shortlisted | Screening/ranking does not satisfy shortlist criteria. | Preserve the screening/ranking outcome without entering recruitment rounds. | Candidate does not progress to recruitment. | Placement |
| Candidate eliminated in recruitment | Required stage result fails advancement criteria. | Publish/record the H04 elimination outcome. | CandidateApplication reaches `Eliminated`. | Placement |
| Authorized reconsideration | A previously eliminated candidate is authorized for reconsideration before process closure. | Use `CandidateReconsiderationRequested`, `CandidateReinstated`, and `RecruitmentWorkflowResumed`. | Recruitment resumes with prior elimination history preserved. | Placement |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required source academic information is unavailable | Eligibility/screening cannot be authoritatively completed. | Do not invent eligibility; hold progression until authoritative information is available. | Placement / applicable source domain |
| Recruitment evaluation is incomplete | Candidate progression cannot be decided. | Complete the governed evaluation before advancement/elimination. | Placement |
| Employer selection input conflicts with completed mandatory stages or hiring capacity | Final selection cannot safely be recorded. | Preserve prior recruitment outcomes and require governed Placement review. | Placement |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| One CandidateApplication belongs to one Student and one Position. | Placement | Application identity remains singular. | L2-H04 §17.4 |
| Students must satisfy all mandatory eligibility criteria and institutional placement policy before registration progresses. | Placement using source facts | Ineligible applications do not enter recruitment. | L2-H04 §19.6 |
| Every registered candidate undergoes screening; screening decisions are traceable and auditable. | Placement | Screening precedes shortlist/recruitment. | L2-H04 §19.7 |
| Only screened candidates may be shortlisted and published shortlists are immutable. | Placement | Shortlisting preserves decision history. | L2-H04 §19.8 |
| Mandatory recruitment stages cannot be skipped. | Placement | Candidate progression follows the configured workflow. | L2-H04 §§17.5, 19.10 |
| Eliminated candidates cannot progress without authorized reconsideration. | Placement | Reopening follows the exact H04 reconsideration path. | L2-H04 §§17.4, 19.11 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| CandidateApplication | `Registered` | Eligibility validation, screening, shortlisting, and recruitment progression occur. | `Selected` or `Eliminated` | Placement | L2-H04 §17.4 |
| RecruitmentWorkflow | `Draft` / `In Progress` | Required stages execute and evaluations determine advancement/elimination. | `Completed` for completed stage sequence, or resumed if authorized reconsideration occurs. | Placement | L2-H04 §17.5 |
| Source Student / academic facts | Current canonical states | Read/consumed for eligibility and screening only. | Unchanged by Placement. | C / applicable academic owner | `DEPENDENCY_MAP.md`; H04 ownership boundary |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Applicable exact Placement events include `CandidateRegistrationInitiated`, `CandidateEligibilityValidated`, `CandidateRegistered`, `CandidateApplicationCreated`, `CandidateScreeningStarted`, `AcademicEligibilityValidated`, `PlacementPolicyValidated`, `CandidateScreened`, `CandidateRankingGenerated`, `CandidateShortlisted`, `ShortlistApproved`, `ShortlistPublished`, `RecruitmentRoundStarted`, `CandidateEvaluated`, `RoundResultsSubmitted`, `CandidateAdvanced`, `CandidateEliminated`, `RecruitmentRoundCompleted`, and `CandidateSelected`.

Placement remains the publisher of Placement-owned candidate and recruitment events.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Placement consumes canonical Student and applicable academic information for eligibility/screening without modifying those source records. Employer selection feedback is an external business input reviewed by Placement staff as H04 defines; no new employer-system contract or actor is invented at L3.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The Student initiates an application. Placement Officer/Coordinator performs the Placement-owned screening/recruitment responsibilities assigned in H04. H04 requires explicit authorization for reconsideration and states employer selection is recorded/reviewed by placement staff.

## 15.2 Separation of Duties, Overrides and Exceptions

Manual screening may override an AI recommendation according to H04, while published shortlist history and prior elimination history remain preserved. Reconsideration cannot bypass authorization.

## 15.3 Business Audit and Evidence Obligations

Audit evidence must preserve Student/Position references, eligibility result, policy validation, screening criteria/outcome, ranking, shortlist approval/publication, recruitment stage evaluations, advancement/elimination decisions, employer selection input, final selected/eliminated position, and timestamps/actors where defined.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The application is tied to one Student and one Position.
- Mandatory eligibility and policy validation are complete.
- Required screening and shortlist/recruitment stages are complete.
- The final Placement-owned candidate outcome is recorded as `Selected` or `Eliminated`.
- Decision history remains traceable and auditable.

## 16.2 Postconditions

- Placement owns the candidate/recruitment outcome but not the Student or academic source facts.
- A selected candidate may proceed to offer generation.
- An eliminated candidate may proceed only through governed reconsideration if later authorized.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| H04 lists Section D as providing academic records/results/eligibility and consumes `ExaminationResultPublished`, while the current `DEPENDENCY_MAP.md` states Section D does not own official results and Section I owns official examination results. | L3 must not transfer official-result ownership to D. This Draft relies only on source-owned academic information that D may authoritatively provide and does not assign official-result publishing to D. | Reconcile H04's stale result-ownership/dependency wording with the current locked ownership map. |
| H04 records employer selection as an external input reviewed by Placement staff but does not define a canonical employer-feedback event/contract. | The selection decision is still recordable through Placement's `CandidateSelected` outcome, but L3 cannot invent a separate employer-feedback event. | Define a formal employer-feedback hand-off upstream only if cross-domain event semantics are required. |

**Architecture assessment:** `PASS WITH ISSUES`

The CandidateApplication and RecruitmentWorkflow lifecycles fully support a source-grounded application-to-selection/elimination scenario. The ownership wording and employer-feedback contract gaps do not block the final Placement-owned decision.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
