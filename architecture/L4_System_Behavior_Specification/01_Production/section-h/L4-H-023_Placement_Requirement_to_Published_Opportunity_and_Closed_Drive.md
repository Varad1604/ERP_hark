# Placement Requirement to Published Opportunity and Closed Drive

## 1. Specification Metadata
**Specification ID:** L4-H-023  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Placement  
**Realizes Scenario:** L3-H-023  
**Primary Business Owner:** Placement  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-023; L2-H04 — Placement Workflows; applicable C/E references  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall turn an already-approved employer requirement into a governed published Placement opportunity, execute the required recruitment work and close the Placement Drive while preserving source-domain ownership and historical Placement outcomes.

## 3. Required Business Context
Eligible Employer; already-approved employer requirement; Placement Officer/Coordinator authority; valid Student/workforce references; at least one Position; configured eligibility and recruitment requirements.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-023-BH-001 | On an approved employer requirement, Placement shall create one governed Placement Drive for one eligible Employer and at least one Position before publication. | L3-H-023 §§7–9,12 |
| L4-H-023-BH-002 | Eligibility and recruitment requirements shall be configured before publication/registration; mandatory recruitment stages shall not be skipped. | L3-H-023 §§9,12 |
| L4-H-023-BH-003 | Placement shall publish the governed Position/Drive, open and close registration, preserve screening/shortlisting/recruitment outcomes, and resolve outstanding offer/joining work required for closure. | L3-H-023 §§9–11 |
| L4-H-023-BH-004 | The Placement Drive shall reach `Closed` only after required recruitment/offer work is concluded and Placement statistics/hiring summary are available; a closed drive shall be immutable and its historical records shall not be deleted. | L3-H-023 §§11–13,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-023-BH-005 | Placement shall publish the applicable H04 drive/position lifecycle events including `PlacementDriveCreated`, `PositionCreated`, `EligibilityPublished`, `PositionPublished`, `PlacementDrivePublished`, `DriveRegistrationOpened`, `DriveRegistrationClosed`, `PlacementStatisticsGenerated`, `PlacementDriveArchived` and `PlacementDriveClosed` as their corresponding facts occur. | L3-H-023 §14 |

## 6. Failure / Rejection / Continuation
Incomplete configuration or unresolved mandatory recruitment/offer work prevents closure; candidate history shall not be silently rewritten.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-023-BH-006 | The system shall preserve Employer association, drive schedule, Position/eligibility configuration, recruitment outcomes, statistics, closing authority and closure time. | L3-H-023 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-023-BH-007 | Completion shall require a published opportunity, concluded required recruitment/offer work, available statistics/hiring summary, Placement Drive `Closed`, preserved history and unchanged C/E ownership. | L3-H-023 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-023 — Placement Requirement to Published Opportunity and Closed Drive  
**Primary verification source:** L2-H04 — Placement Workflows.

## 10. Open Issues / Deferred Design
The parent starts from an already-approved Employer requirement because H04 defines no Employer Requirement approval aggregate, and H04 has a non-blocking Drive lifecycle diagram/table inconsistency; L4 does not resolve either.