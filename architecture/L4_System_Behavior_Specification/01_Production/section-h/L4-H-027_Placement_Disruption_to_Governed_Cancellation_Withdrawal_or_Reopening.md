# Placement Disruption to Governed Cancellation, Withdrawal, or Reopening

## 1. Specification Metadata
**Specification ID:** L4-H-027  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Placement  
**Realizes Scenario:** L3-H-027  
**Primary Business Owner:** Placement  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-027; L2-H04 — Placement Workflows; `L2-H04_Placement_Lifecycle_Completion_Amendment_2026-08-26.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall route Placement disruptions through the exact H04 cancellation, withdrawal or reopening path applicable to the affected aggregate while preserving prior history and final outcomes.

## 3. Required Business Context
Affected PlacementDrive, CandidateApplication, PlacementOffer or Internship; current state; disruption reason; applicable H04 authority; reopening eligibility and prior history.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-027-BH-001 | When a drive, application, selection or offer cannot continue normally, the system shall resolve the affected H04 aggregate/current state and permit only the exception path valid for that state. | L3-H-027 §§7–9 |
| L4-H-027-BH-002 | PlacementDrive may enter `Cancelled` and reopen only when original recruitment context remains valid and final outcomes are not invalidated; CandidateApplication may enter `Withdrawn` and reopen only after eligibility revalidation while the drive/position remains open. | L3-H-027 §§9–13 |
| L4-H-027-BH-003 | PlacementOffer withdrawal shall follow existing H04 withdrawal/reconsideration rules; Internship may enter `Cancelled` or `Withdrawn` before `Verified` and may reopen only before `Verified`. | L3-H-027 §§9–13 |
| L4-H-027-BH-004 | Reopening shall never erase cancellation, withdrawal or elimination evidence, and no completed/verified final outcome shall be silently reversed. | L3-H-027 §§11–13,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-027-BH-005 | Placement shall publish the applicable `PlacementDriveCancelled`, `PlacementDriveReopened`, `CandidateApplicationWithdrawn`, `CandidateApplicationReopened`, offer-withdrawal/reconsideration, `InternshipWithdrawn`, `InternshipCancelled` and `InternshipReopened` events. | L3-H-027 §14 |

## 6. Failure / Rejection / Continuation
Unauthorized or ineligible reopening leaves the prior exception state authoritative; C/E source lifecycles remain unchanged.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-027-BH-006 | The system shall preserve prior state, disruption reason, decision authority, exception outcome, reopening rationale, eligibility/context revalidation and complete historical lineage. | L3-H-027 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-027-BH-007 | Completion shall require the affected aggregate to reach its permitted exception state or be governedly reopened under locked conditions, with prior history intact and final immutable outcomes preserved. | L3-H-027 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-027 — Placement Disruption to Governed Cancellation, Withdrawal, or Reopening  
**Primary verification source:** Locked H04 lifecycle-completion amendment.

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation.