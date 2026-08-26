# L2-H09 — Accreditation and Compliance Workflows — Lifecycle Amendment

| Attribute | Value |
|---|---|
| Target | L2-H09 — Accreditation and Compliance Workflows v1.0 |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-26 |
| Approved Through | ADR-H-ACCREDITATION-GOV-001 |
| Authority | Explicit Lead Architect remediation authorization recorded 2026-08-26 |

This amendment is canonical L2-H09 architecture. It completes governance states required by frozen L3 scenarios while preserving source-domain ownership of operational evidence.

## 1. Accreditation and Compliance Governance Authority

The **Accreditation and Compliance Governance Authority** is the canonical institutional business authority for approval of assessment plans, governed compliance waivers, acceptance/challenge of external assessment outcomes, CAPA closure/reopening decisions, and discontinuation/closure of quality-improvement initiatives where H09 requires institutional authority.

This authority governs H09 compliance decisions only and cannot change operational evidence owned by source domains.

## 2. Accreditation Assessment Plan Lifecycle

Canonical Assessment Plan lifecycle:

`Draft → Under Review → Approved → Active → Completed → Closed`

Branches:

- `Under Review → Revision Required → Draft`
- `Under Review → Rejected`
- `Active → Suspended → Active`
- `Active / Suspended → Cancelled` where the accreditation cycle is formally cancelled.

Canonical events added:

- `AccreditationAssessmentPlanApproved`
- `AccreditationAssessmentPlanRevisionRequired`
- `AccreditationAssessmentPlanRejected`
- `AccreditationAssessmentPlanActivated`
- `AccreditationAssessmentPlanClosed`

## 3. Compliance Requirement Lifecycle

Canonical Compliance Requirement lifecycle:

`Identified → Applicable → Active → Satisfied → Closed`

Governed alternative outcomes:

- `Applicable / Active → Waiver Requested → Waived`
- `Active → Noncompliant → Active` after governed remediation restores compliance
- `Applicable / Active / Satisfied → Retired` when the governing obligation is formally withdrawn or superseded.

A waiver requires explicit authority and recorded rationale/evidence. `Waived` does not mean the external obligation ceased to exist; it records the institution's governed treatment for the applicable scope. `Retired` means the requirement version is no longer current; history remains immutable.

Canonical events added:

- `ComplianceWaiverRequested`
- `ComplianceRequirementWaived`
- `ComplianceRequirementRetired`
- `ComplianceRequirementSatisfied`

## 4. External Assessment Outcome

An external assessment notice/result is recorded as authoritative external evidence by reference. H09 governs the institution's response to that evidence.

Canonical H09 response lifecycle:

`Received → Under Review → Accepted`

Branch:

`Under Review → Challenged → Resolved → Accepted / Replaced`

`Replaced` means a later authoritative external outcome supersedes the prior outcome for current-use purposes; the prior record remains historical.

Canonical events added:

- `ExternalAssessmentOutcomeAccepted`
- `ExternalAssessmentOutcomeChallenged`
- `ExternalAssessmentChallengeResolved`
- `ExternalAssessmentOutcomeReplaced`

## 5. CAPA Lifecycle and Reopening

Canonical Corrective and Preventive Action (CAPA) lifecycle:

`Opened → Action Planned → In Progress → Verification Pending → Verified → Closed`

If post-closure evidence demonstrates that the corrective outcome did not hold or the underlying finding remains unresolved, the Accreditation and Compliance Governance Authority may authorize:

`Closed → Reopened → In Progress`

Reopening preserves the original closure evidence and records the new decision/reason; it does not erase history.

Canonical events added:

- `CAPAVerified`
- `CAPAClosed`
- `CAPAReopened`

## 6. Quality Improvement Lifecycle

Canonical Quality Improvement Initiative lifecycle:

`Proposed → Approved → Active → Outcome Verification → Verified → Closed`

Governed branch:

`Proposed / Approved / Active → Discontinued`

Discontinuation requires recorded institutional rationale and preserves all prior evidence. A discontinued initiative does not imply that the underlying quality obligation is satisfied.

Canonical events added:

- `QualityImprovementApproved`
- `QualityImprovementOutcomeVerified`
- `QualityImprovementClosed`
- `QualityImprovementDiscontinued`

## 7. Evidence Ownership

H09 may assemble, map and govern compliance/accreditation evidence but shall not become the canonical owner of operational facts from Student, Academic, People, Finance, Examination, Research, Campus Operations or other source domains. Corrections to source facts remain with the source owner.

## 8. Downstream Effect

This amendment resolves the lifecycle/authority gaps identified by L3-H-062, L3-H-063, L3-H-066, L3-H-067 and L3-H-068.

## 9. Locked Decisions

The authority, lifecycles, events and evidence boundary in §§1–7 are Locked L2-H09 decisions under ADR-H-ACCREDITATION-GOV-001.
