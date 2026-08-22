# Placement Requirement to Published Opportunity and Closed Drive

# 1. Scenario Metadata

**Scenario ID:** L3-H-023  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Placement  
**Participating Domains:** C, E, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H04 — Placement Workflows; applicable locked Student Lifecycle and People & Workforce architecture  
**Referenced By:** Placement opportunity publication, candidate participation, recruitment execution, placement completion, drive closure, and institutional placement reporting

# 2. Scenario Purpose

Govern a complete institutional placement drive from an approved employer requirement through opportunity publication, recruitment execution, outcome preservation, and formal drive closure while Placement retains ownership of placement aggregates and source domains retain Student and workforce information.

# 3. Business Outcome

The governed opportunity is published, executed, and formally closed with outcomes preserved

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an approved employer requirement initiates a placement drive or opportunity.

## 4.2 End Boundary

The scenario ends when the placement opportunity has been published and executed, all outstanding recruitment and offer activity required for closure has been resolved, the Placement Drive reaches `Closed`, and placement outcomes remain preserved.

## 4.3 In Scope

- Recording the approved employer requirement as a Placement-owned drive/opportunity context.
- Creating and configuring the Placement Drive and one or more Positions.
- Configuring and publishing eligibility requirements before registration.
- Publishing the governed placement opportunity.
- Opening and closing candidate registration according to the Placement Drive lifecycle.
- Executing screening, shortlisting, recruitment rounds, offer processing, and completion activity required by the drive.
- Reviewing completion, generating placement statistics, and formally closing the drive.
- Preserving historical drive and outcome evidence.

## 4.4 Out of Scope

- Employer verification, which is governed by L3-H-022.
- Candidate-specific application-to-selection detail, governed by L3-H-024.
- Candidate offer and joining outcome detail, governed by L3-H-025.
- Canonical Student, academic, or workforce record maintenance.
- Enterprise notification delivery.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Student Lifecycle / Section C | Retains canonical Student identity/lifecycle information used for placement participation. |
| People & Workforce / Section E | Retains Placement Officer/Placement Coordinator workforce identity and assignment information. |
| Placement / Section H | Owns Employer, PlacementDrive, Position, CandidateApplication, RecruitmentWorkflow, PlacementOffer, Internship, and JoiningConfirmation placement lifecycles and the final drive-closure outcome. |

Participation does not transfer Student or workforce ownership to Placement.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Placement Officer | Creates and governs drive/position configuration, placement decisions, offer processing, and closure activities supported by H04. | Performs and reviews Placement-owned business actions. |
| Placement Coordinator | Supports Placement Drive, Position, CandidateApplication, and RecruitmentWorkflow activities where H04 assigns that role. | Performs supported placement coordination. |
| Student | Applies to an eligible published position and participates in the governed recruitment process. | Initiates candidate participation when registration is open. |

Employer-originated requirements and recruitment inputs are recorded by authorized placement staff; H04 explicitly does not assume an employer user account.

# 7. Preconditions and Entry Criteria

- The employer is eligible to participate under Placement governance.
- The initiating employer requirement has been approved outside or within the governed placement process sufficiently to start a drive/opportunity.
- Placement staff required by H04 are authorized.
- Student and placement-staff source information remains available from its canonical owner.
- The drive can be associated with exactly one employer.
- At least one Position can be created and configured before publication.

# 8. Trigger

An approved employer requirement initiates a placement drive or opportunity

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create a Placement Drive for the eligible employer and record the employer association and schedule. | Placement Officer / Placement | The drive enters `Draft` with its employer and schedule context. |
| 2 | Create one or more Positions and configure compensation, hiring capacity, eligibility, and required recruitment workflow. | Placement Officer / Placement Coordinator / Placement | The opportunity is fully configured for publication. |
| 3 | Publish the eligibility rules, Position, and Placement Drive according to H04 governance. | Placement | The governed placement opportunity becomes published. |
| 4 | Open candidate registration, accept eligible applications, then close registration for recruitment execution. | Student / Placement | Candidate participation is established under Placement policy. |
| 5 | Execute screening, shortlisting, and mandatory recruitment stages; preserve advancement and elimination outcomes. | Placement Officer / Placement Coordinator / Placement | Recruitment reaches governed candidate outcomes. |
| 6 | Resolve offer-processing and joining activity required for drive completion. | Placement Officer / Placement | Outstanding placement outcomes are resolved sufficiently for closure. |
| 7 | Review drive completion, generate placement statistics, archive the completed drive where applicable, and close the Placement Drive. | Placement Officer / Placement | `PlacementDriveClosed` records the formal end of the drive. |
| 8 | Preserve historical placement, offer, recruitment, joining, and closure evidence. | Placement | The closed drive remains immutable and traceable. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| No eligible candidates | Screening/eligibility produces no valid participants. | Preserve the governed no-candidate outcome and proceed to completion review when no recruitment work remains. | Drive may close with no successful placement outcome. | Placement |
| Candidate elimination | A candidate fails a governed recruitment stage. | Record `CandidateEliminated`; continue the drive for remaining candidates. | The drive continues without rewriting the eliminated candidate's history. | Placement |
| Candidate reconsideration | Authorized reconsideration is approved before closure. | Reinstate the candidate and resume the RecruitmentWorkflow from the appropriate stage. | Recruitment continues with preserved prior history. | Placement |
| Offer withdrawal | A valid employer withdrawal is recorded before joining confirmation. | Apply the governed offer-withdrawal path and reopen candidate eligibility where policy permits. | Drive completion proceeds using the updated placement outcomes. | Placement |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Published drive still has unresolved offers or recruitment rounds | Formal closure criteria are not satisfied. | Do not close the drive until outstanding placement activity is resolved under H04. | Placement |
| Required eligibility or recruitment configuration is incomplete | Opportunity cannot safely proceed. | Correct the Placement-owned configuration before registration/recruitment continues. | Placement |
| Candidate history requires authorized reconsideration | Prior elimination cannot be silently overwritten. | Use H04 reconsideration/reinstatement and preserve prior immutable history. | Placement |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every Placement Drive belongs to exactly one employer. | Placement | Drive identity and employer association remain singular. | L2-H04 §17.2 |
| Every drive contains at least one Position before governed publication. | Placement | Opportunity publication requires defined positions. | L2-H04 §§17.2, 19.3 |
| Position eligibility rules must exist before publication and recruitment workflow must be assigned before registrations begin. | Placement | Registration cannot open on an ungoverned Position. | L2-H04 §17.3 |
| Registration occurs only during the configured registration period and students must satisfy mandatory eligibility and placement policy. | Placement | Only governed candidate applications enter the drive. | L2-H04 §19.6 |
| Mandatory recruitment stages cannot be skipped. | Placement | Recruitment execution follows the configured workflow. | L2-H04 §§17.4, 17.5, 19.10 |
| Closed drives are immutable and historical records cannot be deleted. | Placement | Formal closure preserves outcomes and history. | L2-H04 §19.16 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| PlacementDrive | `Draft` | Published, registration opened/closed, recruitment executed, completion reviewed, and closed. | `Closed` | Placement | L2-H04 §§17.2, 19.16 |
| Position | `Draft` / `Configured` | Eligibility is configured and the Position is published for participation. | `Published` or later governed Position state | Placement | L2-H04 §17.3 |
| CandidateApplication | `Registered` onward | Applications progress through eligibility, screening, recruitment, and selection/elimination. | Placement-owned candidate outcome | Placement | L2-H04 §17.4 |
| PlacementOffer / JoiningConfirmation | Applicable offer/joining states | Outstanding placement outcomes are resolved before drive closure. | Governed offer/joining outcome | Placement | L2-H04 §§17.6, 17.8 |

H04's PlacementDrive lifecycle diagram includes `Offer Processing`, but its transition table does not define transitions into or out of that state. This does not prevent use of the separately defined PlacementOffer/JoiningConfirmation lifecycles for drive closure, but the inconsistency is recorded in Section 17.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Applicable exact Placement events include `PlacementDriveCreated`, `DriveScheduleConfigured`, `EmployerAssignedToDrive`, `PositionCreated`, `EligibilityPublished`, `PositionPublished`, `PlacementDrivePublished`, `DriveRegistrationOpened`, `DriveRegistrationClosed`, candidate/recruitment events, `PlacementStatisticsGenerated`, `PlacementDriveArchived`, and `PlacementDriveClosed`.

Placement remains the publisher of Placement-owned events. Student and workforce source domains retain ownership of their own canonical facts/events.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Placement consumes Student and workforce information as governed source-domain information and uses H04-owned placement lifecycles to execute the drive. Employer-originated business inputs are recorded by authorized placement staff as H04 specifies. No source-domain business entity is modified by Placement.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Placement Officer and Placement Coordinator authority is used only where H04 explicitly assigns those roles. Student participation is limited to published/open opportunities and applicable eligibility rules.

## 15.2 Separation of Duties, Overrides and Exceptions

Candidate reconsideration requires explicit authorization. Closed-drive history and published shortlist history cannot be silently overwritten.

## 15.3 Business Audit and Evidence Obligations

Evidence must preserve drive creator/closing authority, employer association, schedule, position and eligibility configuration, candidate/recruitment outcomes, offer/joining outcomes, placement statistics, closure timestamp, and immutable historical records.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The governed opportunity was published.
- Registration and recruitment execution are complete.
- Outstanding offers and recruitment rounds required for closure are resolved.
- Placement statistics and hiring summary are available.
- The Placement Drive reaches `Closed`.
- Historical outcomes remain preserved and traceable.

## 16.2 Postconditions

- Closed drive records are immutable.
- Placement remains owner of placement-specific lifecycle/history.
- Student and workforce source ownership is unchanged.
- Applicable placement events are available for downstream authorized consumers.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The frozen trigger begins with an “approved employer requirement”, while H04 does not define a first-class Employer Requirement aggregate, approval lifecycle, or dedicated requirement-approval event. H04 instead states employer-originated actions are recorded by authorized placement staff. | The scenario can begin once the approved external/institutional requirement exists, but L3 cannot invent a separate requirement lifecycle or approver. | Define a canonical Employer Requirement lifecycle only if one is required upstream. |
| H04's PlacementDrive lifecycle diagram includes `Offer Processing`, but its transition table does not define entry/exit transitions for that state. | The detailed offer and joining aggregates still allow outstanding outcomes to be resolved before closure, but the drive-state model is internally incomplete. | Reconcile the PlacementDrive lifecycle diagram and transition table upstream. |
| H04's top aggregate ownership matrix omits some publish/registration lifecycle actions/events that appear in its detailed catalogues. | L3 can use the exact detailed H04 events, but the L2 summary matrix is incomplete. | Align the H04 aggregate summary with its detailed lifecycle/event catalogues. |

**Architecture assessment:** `PASS WITH ISSUES`

The published-opportunity through formal-drive-closure outcome is fully achievable from locked H04 workflows without changing source-domain ownership. The issues above are non-blocking inconsistencies/gaps that do not prevent a source-grounded Draft.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
