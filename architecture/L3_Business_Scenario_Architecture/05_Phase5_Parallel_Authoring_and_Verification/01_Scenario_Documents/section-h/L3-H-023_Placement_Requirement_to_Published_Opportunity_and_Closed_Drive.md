# Placement Requirement to Published Opportunity and Closed Drive

# 1. Scenario Metadata

**Scenario ID:** L3-H-023  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Placement  
**Participating Domains:** C, E, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H04 — Placement Workflows; L2-C02 — Student Lifecycle & Activation Workflows; L2-E01 — Workforce & Engagement Workflows  
**Referenced By:** Candidate selection, offer/joining, placement reporting

# 2. Scenario Purpose

Govern one complete Placement Drive from an approved employer requirement through opportunity publication, recruitment execution, completion review, and formal closure while Student and workforce facts remain source-owned.

# 3. Business Outcome

The governed opportunity is published, executed, and formally closed with outcomes preserved

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an approved employer requirement initiates a placement drive or opportunity.

## 4.2 End Boundary

The scenario ends when required drive activity is complete, outstanding recruitment/offers are resolved, Placement statistics are generated, and the Placement Drive reaches `Closed` with historical outcomes preserved.

## 4.3 In Scope

Drive/Position creation and publication; eligibility configuration; governed candidate participation and recruitment execution at summary level; completion review; statistics; closure.

## 4.4 Out of Scope

Employer onboarding (H022), detailed candidate selection (H024), detailed offer/joining (H025), Student/workforce lifecycle changes, and notification delivery.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Student Lifecycle / C | Owns canonical Student identity/lifecycle facts referenced by Placement. |
| People & Workforce / E | Owns workforce relationships used for Placement Officer/Coordinator context. |
| Placement / H | Owns PlacementDrive, Position and related Placement lifecycle/outcomes culminating in drive closure. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Placement Officer | Creates/governs drives, positions and closure activities. |
| Placement Coordinator | Supports drive, position, screening and recruitment activity where H04 permits. |
| Student | Participates in an eligible published opportunity. |

Employer-originated requirements are external inputs recorded by authorized Placement staff; H04 assumes no employer account.

# 7. Preconditions and Entry Criteria

- Employer is eligible under Placement governance.
- The employer requirement is already approved for placement use.
- Authorized Placement staff are available.
- Student/workforce source references remain valid.
- At least one Position can be configured before publication.

# 8. Trigger

An approved employer requirement initiates a placement drive or opportunity

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Create the Placement Drive for one eligible employer and configure its schedule. | H | Drive exists in governed pre-publication state. |
| 2 | Create at least one Position and configure hiring, eligibility, and recruitment requirements. | H | Opportunity is ready for publication. |
| 3 | Publish eligibility, Position, and Placement Drive. | H | Governed opportunity becomes available. |
| 4 | Open/close registration and execute required screening, shortlisting, and recruitment activity. | H with C references | Candidate outcomes are governed and traceable. |
| 5 | Resolve outstanding offer/joining activity needed for drive completion. | H | Closure preconditions are satisfied. |
| 6 | Review completion, generate Placement statistics, and close/archive the drive according to H04. | H | `PlacementDriveClosed` records formal closure. |

# 10. Alternate Business Flows

No eligible candidates, candidate elimination, authorized reconsideration, and offer withdrawal may occur under their H04-defined paths. None changes Student/workforce ownership or permits deletion of prior Placement history.

# 11. Failure, Interruption and Business Recovery

A drive cannot close while recruitment rounds or offers required for completion remain unresolved. Incomplete Placement configuration is corrected within H04 before the drive progresses. Historical candidate outcomes are never silently rewritten.

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Effect |
|---|---|---|
| Every drive belongs to one employer and contains at least one Position before publication. | H | Drive/opportunity integrity is preserved. |
| Eligibility must be configured before Position publication/registration. | H | Ungoverned participation is prevented. |
| Mandatory recruitment stages cannot be skipped. | H | Recruitment follows the configured Placement workflow. |
| Closed drives are immutable and historical records cannot be deleted. | H | Formal closure preserves evidence. |

# 13. State and Lifecycle Impact

PlacementDrive progresses through its H04 lifecycle toward `Closed`; Position progresses to `Published` and later governed states. The H04 lifecycle diagram includes `Offer Processing`, but the transition table goes from `Recruitment Active` directly to `Completed`, creating a non-blocking L2 inconsistency. C02 Student and E01 workforce lifecycles are unchanged.

# 14. Business Events and Cross-Domain Collaboration

Applicable Placement events include `PlacementDriveCreated`, `DriveScheduleConfigured`, `EmployerAssignedToDrive`, `PositionCreated`, `EligibilityPublished`, `PositionPublished`, `PlacementDrivePublished`, `DriveRegistrationOpened`, `DriveRegistrationClosed`, candidate/recruitment outcome events, `PlacementStatisticsGenerated`, `PlacementDriveArchived`, and `PlacementDriveClosed`. Placement publishes its events; C/E facts retain their source owners.

# 15. Business Authorization, Approval and Audit Obligations

H04 assigns drive/position/closure actions to Placement Officer and, where specified, Placement Coordinator. Student participation is limited by published eligibility and registration rules. Audit evidence preserves employer association, drive schedule, Position/eligibility configuration, recruitment outcomes, statistics, closing authority, and closure time.

# 16. Completion Criteria and Postconditions

The opportunity is published; required recruitment/offer work is concluded; statistics and hiring summary are available; the Placement Drive is `Closed`; historical Placement outcomes are preserved; C/E ownership is unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Frozen trigger uses an “approved employer requirement”, but H04 defines no Employer Requirement aggregate/approval lifecycle. | L3 can start from the already-approved requirement but cannot author its approval. | Define an Employer Requirement lifecycle only if upstream governance requires one. |
| PlacementDrive diagram contains `Offer Processing`, while its transition table does not. | Drive-state detail is internally inconsistent but closure remains defined. | Reconcile the H04 diagram/table upstream. |

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
