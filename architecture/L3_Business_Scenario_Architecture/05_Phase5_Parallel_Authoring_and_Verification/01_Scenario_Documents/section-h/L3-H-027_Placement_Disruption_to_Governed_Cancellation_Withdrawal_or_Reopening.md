# Placement Disruption to Governed Cancellation, Withdrawal, or Reopening

# 1. Scenario Metadata

**Scenario ID:** L3-H-027  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Placement  
**Participating Domains:** C, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Review  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H04 — Placement Workflows; L2-C02 — Student Lifecycle & Activation Workflows; L2-E01 — Workforce & Engagement Workflows; `L2-H04_Placement_Lifecycle_Completion_Amendment_2026-08-26.md`  
**Referenced By:** Placement exception handling and audit; L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose

Govern a Placement disruption through H04-defined cancellation, withdrawal and reopening paths while preserving all prior lifecycle history and keeping exception ownership within Placement.

# 3. Business Outcome

The affected lifecycle reaches a permitted exception state and any reopening preserves the full prior history

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a drive, application, selection, offer or internship cannot continue normally.

## 4.2 End Boundary

The scenario ends when the affected H04 aggregate reaches its permitted exception outcome or is governedly reopened without erasing prior history.

## 4.3 In Scope

PlacementDrive cancellation/reopening; CandidateApplication withdrawal/reopening; offer withdrawal and candidate eligibility reopening; Internship cancellation/withdrawal/reopening; preservation of prior history.

## 4.4 Out of Scope

Reversal of verified final outcomes, Student/workforce lifecycle changes, and technical retry/rollback mechanisms.

# 5. Ownership and Participating Domains

C owns Student facts; E owns workforce facts; H owns the Placement aggregates and the exception/recovery paths defined by H04. No exception branch transfers ownership to C or E.

# 6. Business Actors, Roles and Responsibilities

Placement Officer / Placement Coordinator are the canonical business authorities for PlacementDrive cancellation/reopening and related Placement exception decisions where H04 assigns them. Placement Officer governs CandidateApplication, offer and Internship decisions under applicable H04 policy. Employer-originated facts remain external inputs recorded through the governed Placement process.

# 7. Preconditions and Entry Criteria

- The affected Placement aggregate exists.
- Its current state permits the requested exception branch.
- Prior history is preserved.
- Reopening is explicitly authorized and does not invalidate already-final outcomes.

# 8. Trigger

A drive, application, selection, or offer cannot continue normally

# 9. Main Business Flow

1. Identify the affected H04 aggregate and current state.
2. Determine the applicable governed exception path.
3. For PlacementDrive disruption, cancel the drive where permitted and publish `PlacementDriveCancelled`.
4. Reopen a cancelled drive only when the original recruitment context remains valid and reopening does not invalidate final outcomes; publish `PlacementDriveReopened`.
5. For CandidateApplication withdrawal, move the application to `Withdrawn` where permitted and publish `CandidateApplicationWithdrawn`.
6. Reopen a withdrawn application only after eligibility is revalidated and the relevant drive/position remains open; publish `CandidateApplicationReopened`.
7. For offer withdrawal, follow the existing governed offer-withdrawal path and reopen candidate eligibility where policy permits.
8. For Internship disruption before Verified completion, move to `Cancelled` or `Withdrawn` and publish the applicable event.
9. Reopen the same Internship only where prior completion has not been Verified; publish `InternshipReopened`.
10. Preserve complete prior state, decision and reopened-history evidence.

# 10. Alternate Business Flows

- Unauthorized reopening is rejected and the prior exception state remains authoritative.
- Verified Internship completion is immutable and cannot be reopened.
- PlacementDrive reopening is denied when the original recruitment context is no longer valid or final outcomes would be invalidated.
- CandidateApplication reopening is denied when eligibility cannot be revalidated or the position/drive is no longer open.

# 11. Failure, Interruption and Business Recovery

Exception transitions preserve the prior history. Reopening does not erase cancellation, withdrawal or elimination evidence. No completed/verified final outcome is silently reversed.

# 12. Business Rules and Decision Constraints

- PlacementDrive may be Cancelled before normal closure and reopened only under the locked H04 conditions.
- CandidateApplication may be Withdrawn from an eligible non-final state and reopened only after governed reconsideration and eligibility revalidation.
- Internship may be Cancelled or Withdrawn before Verified completion and reopened only before Verified.
- Existing offer-withdrawal and candidate-reconsideration semantics remain authoritative.
- All reopened paths preserve complete prior history.

# 13. State and Lifecycle Impact

Applicable exception states include:

- PlacementDrive: `Cancelled`, with governed `Reopened` continuation where permitted.
- CandidateApplication: `Withdrawn`, with governed reopening to the applicable active participation path.
- PlacementOffer: `Withdrawn` under existing H04 rules.
- Internship: `Cancelled` or `Withdrawn`, with governed reopening only before `Verified`.

C/E lifecycles remain unchanged.

# 14. Business Events and Cross-Domain Collaboration

Canonical H04 events include:

- `PlacementDriveCancelled`
- `PlacementDriveReopened`
- `CandidateApplicationWithdrawn`
- `CandidateApplicationReopened`
- existing offer-withdrawal/reconsideration events
- `InternshipWithdrawn`
- `InternshipCancelled`
- `InternshipReopened`

Placement remains semantic publisher.

# 15. Business Authorization, Approval and Audit Obligations

Placement Officer / Placement Coordinator own the applicable H04 exception decisions according to the aggregate and policy. Evidence preserves prior state, disruption reason, decision authority, exception outcome, reopening rationale, eligibility/context revalidation and complete historical lineage.

# 16. Completion Criteria and Postconditions

Completion occurs when the affected Placement aggregate reaches its permitted exception state or is governedly reopened under the locked conditions. Prior history remains intact, final immutable outcomes are not reversed, and C/E source ownership remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former missing PlacementDrive, CandidateApplication and Internship cancellation/withdrawal/reopening semantics were resolved by locked `L2-H04_Placement_Lifecycle_Completion_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific questions.
