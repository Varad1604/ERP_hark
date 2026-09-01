# ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-002 — Remaining Dhruv L4 Parent Blockers

**Status:** Approved — Authorized for incorporation  
**Date:** 2026-09-01  
**Authority:** User-authorized controlled upstream remediation in SutraERP_Level_3 project  
**Architecture Scope:** L2/L3 corrections required to remove the ten remaining Dhruv L4 parent-readiness blockers identified after the 2026-09-01 HF-10 remediation

## 1. Decision

The ten remaining parent-readiness blockers shall be resolved at their canonical owning architecture level. L4 shall consume these decisions and shall not invent authority, lifecycle, event identity, publisher, ownership or recovery semantics.

| Scenario | Locked resolution |
|---|---|
| L3-F-085 | Finance Foundation owns Financial Rule governance. `Finance Financial Rule Governance Authority` is the canonical approval authority. Financial Rule lifecycle events are published by Finance Foundation for approval, activation, deprecation and archival. |
| L3-H-049 | `DashboardTemplateRetired` is the canonical retirement event for the H07 Dashboard Template lifecycle. `DashboardTemplateArchived` is historical catalogue wording and shall not be used as a competing canonical event. |
| L3-H-052 | `DashboardVersionPublished` is the canonical H07 Dashboard Version publication event. `DashboardPublished` remains the Dashboard aggregate publication event; `DashboardVersionLocked` records immutability after publication and is not a substitute for the version-publication event. |
| L3-H-059 | `Regulatory Reporting Submission Authority` is the H08 institutional authority for executing governed external submissions. External regulatory rejection is represented by `SubmissionRejectedByAuthority`; institutional package rejection remains `SubmissionPackageRejected`. |
| L3-H-060 | `Regulatory Reporting Governance Authority` owns final Reporting Cycle closure; `Regulatory Reporting Submission Authority` may initiate correction/resubmission where institutional action is required. External regulatory rejection uses `SubmissionRejectedByAuthority`. |
| L3-H-061 | `Under Review` is the canonical Accreditation Framework review-state label. Historical `Review` wording denotes the same lifecycle position and shall not create a competing state. |
| L3-H-070 | `Compliance Review Completed` means evidence evaluation is complete; it does not close the Compliance Review. The review may then progress through `Action Assigned → Closed`. `Compliant` is a governed compliance-status outcome for an Active requirement when BR-H09-035 is satisfied and is not a replacement lifecycle state for the Compliance Requirement aggregate. |
| L3-H-072 | External Provider and External Education Provider Trust remain separate H10 aggregates/lifecycles. Provider onboarding rejection is the External Provider fact `ProviderOnboardingRejected`, published by the External Provider aggregate. Provider Trust lifecycle semantics remain those of the locked 2026-08-26 H10 amendment. |
| L3-H-075 | H10 synchronization recovery is a business recovery progression, not a technical queue/retry design. Canonical business events remain the existing synchronization/recovery events; `SynchronizationRecoveryDeferred` represents governed deferral to reconciliation. No new quarantine lifecycle state is introduced for Synchronization Job. |
| L3-H-080 | Reconciliation Record progression is canonically `Detected → Assigned → Resolved → Verified → Closed`, with `Escalated` as the unresolved governed branch. These states correspond to the existing canonical reconciliation events and do not authorize source-domain mutation. |

## 2. Ownership safeguards

- No L1 ownership changes are introduced.
- Existing Finance, H07, H08, H09 and H10 ownership boundaries remain unchanged.
- External authorities remain external sources of decisions/evidence; SutraERP records those outcomes without owning the external institution.
- Source-domain facts remain owned by their canonical source domains.
- No technical implementation mechanism is defined by this ADR.

## 3. Incorporation

The resolutions are incorporated by the accompanying owner-domain L2 amendments and the targeted L3/L4 parent-readiness revalidation record dated 2026-09-01.

## 4. Prohibition

This ADR does not authorize modification of `main`, full-corpus relock, implementation design, or redesign outside the ten listed identities.
