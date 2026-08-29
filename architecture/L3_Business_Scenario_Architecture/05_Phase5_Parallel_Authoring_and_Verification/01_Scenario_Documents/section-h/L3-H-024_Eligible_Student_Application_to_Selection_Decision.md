# Eligible Student Application to Selection Decision

# 1. Scenario Metadata

**Scenario ID:** L3-H-024  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Placement  
**Participating Domains:** C, D, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H04 — Placement Workflows; L2-C02 — Student Lifecycle & Activation Workflows; applicable locked Section D Academic Delivery sources including L2-D05 — Academic Monitoring Workflows  
**Referenced By:** Placement offer and joining management

# 2. Scenario Purpose

Govern an eligible Student's application through Placement-owned eligibility, screening, shortlisting and recruitment decisions to an authoritative Placement selection or rejection outcome without changing Student or academic source facts.

# 3. Business Outcome

Eligibility, assessment, interview, and employer feedback produce an authoritative selection or rejection outcome

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a student applies for an active placement opportunity.

## 4.2 End Boundary

The scenario ends when H04 records the CandidateApplication as selected or eliminated/rejected after required eligibility, screening, shortlisting and recruitment evaluation, with complete decision history.

## 4.3 In Scope

Application registration; eligibility/policy validation; screening; ranking/shortlisting; mandatory recruitment rounds; employer selection input; selected/eliminated Placement outcome.

## 4.4 Out of Scope

Student lifecycle changes, creation/modification of academic or official examination results, offer/joining processing (H025), and employer authentication.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Student Lifecycle / C | Owns Student identity/lifecycle context. |
| Academic Delivery / D | Supplies only D-owned academic-delivery/monitoring facts where relevant. |
| Placement / H | Owns CandidateApplication, screening, recruitment progression and final Placement selection/elimination outcome. |

Official examination results remain Section I-owned under `DEPENDENCY_MAP.md`; this scenario does not reassign them to D.

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Student | Applies to the published Position. |
| Placement Officer | Reviews/decides Placement screening, shortlist and selection activity where H04 assigns authority. |
| Placement Coordinator | Supports H04 screening and recruitment activity. |

Employer selection input is external and recorded/reviewed by authorized Placement staff.

# 7. Preconditions and Entry Criteria

- Position is published and registration is open.
- Student is active under C02.
- Published Placement eligibility rules and recruitment workflow exist.
- Required source-owned academic information is available from its canonical owner.

# 8. Trigger

A student applies for an active placement opportunity

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Register one CandidateApplication for one Student and one Position. | H with C reference | Governed application exists. |
| 2 | Validate mandatory eligibility and institutional Placement policy using authoritative source facts. | H with C/D facts as applicable | Ineligible applications do not progress. |
| 3 | Complete H04 screening and preserve screening criteria/outcome. | H | `CandidateScreened` records screening completion. |
| 4 | Rank, approve and publish the shortlist where applicable. | H | Shortlisted candidates enter recruitment. |
| 5 | Execute mandatory recruitment rounds sequentially and record evaluations/advancement. | H | Candidate progression is traceable. |
| 6 | Record `CandidateEliminated` when required or `CandidateSelected` after successful required stages and employer selection input. | H | Authoritative Placement selection/rejection outcome exists. |

# 10. Alternate Business Flows

Eligibility failure prevents progression; a screened candidate may not be shortlisted; a candidate may be eliminated during recruitment; an eliminated candidate may return only through H04's authorized reconsideration/reinstatement path with prior history preserved.

# 11. Failure, Interruption and Business Recovery

If required authoritative academic facts are unavailable, Placement must not invent eligibility. Incomplete evaluation prevents advancement. Conflicting selection input is held for governed Placement review without rewriting completed stages.

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Effect |
|---|---|---|
| One CandidateApplication belongs to one Student and one Position. | H | Application identity is singular. |
| Mandatory eligibility and Placement policy must be satisfied before progression. | H using source facts | Ineligible candidates do not enter recruitment. |
| Every candidate undergoes screening; published shortlist history is immutable. | H | Screening/shortlisting remain auditable. |
| Mandatory recruitment stages cannot be skipped. | H | Selection follows configured recruitment governance. |
| Eliminated candidates cannot progress without authorized reconsideration. | H | Reopening is controlled and historical. |

# 13. State and Lifecycle Impact

CandidateApplication progresses from `Registered` through H04 eligibility/screening/shortlisting/recruitment positions to `Selected` or `Eliminated`. RecruitmentWorkflow progresses through required stages. C02 Student and source academic lifecycles remain unchanged.

# 14. Business Events and Cross-Domain Collaboration

Applicable exact Placement events include `CandidateRegistrationInitiated`, `CandidateEligibilityValidated`, `CandidateRegistered`, `CandidateApplicationCreated`, `CandidateScreeningStarted`, `AcademicEligibilityValidated`, `PlacementPolicyValidated`, `CandidateScreened`, `CandidateRankingGenerated`, `CandidateShortlisted`, `ShortlistApproved`, `ShortlistPublished`, `RecruitmentRoundStarted`, `CandidateEvaluated`, `RoundResultsSubmitted`, `CandidateAdvanced`, `CandidateEliminated`, `RecruitmentRoundCompleted`, and `CandidateSelected`. H publishes these Placement facts.

# 15. Business Authorization, Approval and Audit Obligations

Student initiates the application. Placement Officer/Coordinator perform only H04-assigned actions; candidate reconsideration requires explicit authorization. Audit evidence preserves Student/Position references, source evidence used, eligibility/policy results, screening/ranking, shortlist approval/publication, recruitment evaluations, employer selection input, and final candidate outcome.

# 16. Completion Criteria and Postconditions

Mandatory eligibility and recruitment activity are complete; `Selected` or `Eliminated` is recorded; decision history is auditable; Placement owns only the candidate/recruitment outcome; C/D source facts remain source-owned.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| H04 lists Examination Results as Section D-owned/consumed, but `DEPENDENCY_MAP.md` locks official results to Section I and explicitly says D does not own them. | L3 cannot use D as publisher/owner of official examination results. | Correct H04's stale dependency wording upstream. |
| H04 records employer selection input but defines no canonical employer-feedback event/contract. | Final Placement selection remains recordable, but the external feedback hand-off is not independently event-defined. | Define it upstream only if a formal cross-domain event is required. |

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
