# L2-H04 — Placement Workflows — Lifecycle Completion Amendment

| Attribute | Value |
|---|---|
| Target | L2-H04 — Placement Workflows v1.0 |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-26 |
| Approved Through | ADR-H-PLACEMENT-LIFECYCLE-001 |
| Authority | Explicit Lead Architect remediation authorization recorded 2026-08-26 |

This amendment is canonical L2-H04 architecture. It completes lifecycle and authority semantics already implied by H04 capabilities without changing aggregate ownership. Unaffected H04 content remains unchanged.

## 1. Canonical Business Authorities

Existing H04 ownership remains authoritative:

- Placement Officer is the primary business authority for Employer, PlacementOffer, Internship and JoiningConfirmation decisions.
- Placement Officer / Placement Coordinator remains the business authority for PlacementDrive, Position, CandidateApplication and RecruitmentWorkflow decisions defined by H04.
- Employer-originated facts are recorded through the governed H04 process; no employer account or technical role is introduced.

## 2. Employer Participation Lifecycle Completion

The canonical Employer participation progression is completed as:

`Registered → Verification Pending → Verified → Active → Retired`

Governed branches include:

- `Verification Pending → Rejected`
- `Active → Suspended → Active`
- `Suspended → Retired`

`Active` means the verified employer is currently eligible to participate in governed placement activity. `Retired` is terminal for that employer participation record; historical recruitment evidence remains preserved.

Canonical H04 events added:

- `EmployerActivated`
- `EmployerSuspended`
- `EmployerReactivated`
- `EmployerRetired`

## 3. Placement Offer Expiry and Joining Outcome

The existing PlacementOffer lifecycle is extended to make H04's already-declared Offer Expiry capability explicit.

A published offer may reach one of the governed response outcomes:

- Accepted
- Rejected
- Withdrawn
- Expired

`Expired` means the response period ended without a valid acceptance and the offer is no longer actionable. Expiry preserves the offer record and does not imply candidate rejection by the institution.

An accepted offer does not by itself prove joining. Joining remains governed by the separate H04 JoiningConfirmation aggregate. A placement outcome is complete only when the applicable joining evidence has been verified under the JoiningConfirmation lifecycle.

Canonical event added:

- `PlacementOfferExpired`

## 4. Internship Evaluation and Verified Completion

The Internship lifecycle is completed as:

`Published → Candidate Registered → Offered → Accepted → Started → Completed → Evaluated → Verified`

Governed exceptional outcomes include `Withdrawn` and `Cancelled` before Verified completion where policy permits.

- `Completed` records operational completion evidence.
- `Evaluated` records the governed assessment of completion evidence.
- `Verified` is the final successful H04 internship-completion outcome.

Canonical events added:

- `InternshipEvaluated`
- `InternshipCompletionVerified`
- `InternshipWithdrawn`
- `InternshipCancelled`

Existing `InternshipCompleted` remains the event for operational completion and is not redefined as verified completion.

## 5. Disruption, Withdrawal, Cancellation and Reopening

H04 explicitly permits the following governed recovery paths where institutional placement policy and the aggregate's current state allow them:

### PlacementDrive
- A drive may be `Cancelled` before normal closure.
- A cancelled drive may be `Reopened` only by Placement Officer / Placement Coordinator when the original recruitment context remains valid and reopening does not invalidate already final outcomes.

### CandidateApplication
- A candidate may reach `Withdrawn` from an eligible non-final participation state.
- A withdrawn application may be `Reopened` only through governed reconsideration when the relevant position/drive remains open and eligibility is revalidated.

### Internship
- An internship may be `Cancelled` or `Withdrawn` before verified completion.
- Reopening creates a governed resumption of the same internship only when prior completion has not been verified; Verified internships are immutable historical outcomes.

Canonical events added:

- `PlacementDriveCancelled`
- `PlacementDriveReopened`
- `CandidateApplicationWithdrawn`
- `CandidateApplicationReopened`
- `InternshipReopened`

## 6. Ownership Boundaries

- H04 owns all Placement aggregates and events defined above.
- Section C retains Student identity and Student lifecycle facts.
- Section D retains academic facts used for eligibility.
- Section E retains People/Employee facts.
- H04 consumes upstream facts by reference and shall not write them.

## 7. Downstream Effect

This amendment closes the upstream lifecycle gaps identified by L3-H-022, L3-H-025, L3-H-026 and L3-H-027.

## 8. Locked Decisions

The authority, lifecycle transitions, terminal meanings, recovery rules, events and ownership boundaries in §§1–6 are Locked L2-H04 decisions under ADR-H-PLACEMENT-LIFECYCLE-001.
