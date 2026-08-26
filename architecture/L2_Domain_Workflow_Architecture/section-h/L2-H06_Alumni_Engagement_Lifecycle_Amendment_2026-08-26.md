# L2-H06 — Alumni Workflows — Engagement Lifecycle Amendment

| Attribute | Value |
|---|---|
| Target | L2-H06 — Alumni Workflows v1.0 |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-26 |
| Approved Through | ADR-H-ALUMNI-ENGAGEMENT-001 |
| Authority | Explicit Lead Architect remediation authorization recorded 2026-08-26 |

This amendment is canonical L2-H06 architecture. It operates together with `L2-C08_Alumni_Relationship_Boundary_Clarification_2026-08-26.md` and does not transfer ownership of the Alumni Persona or Student-to-Alumni transition from C08.

## 1. Engagement Boundary

H06 begins only after consumption of C08 `AlumniTransitionCompleted`.

H06 owns the Alumni Profile and post-transition engagement aggregates. The phrase **Alumni engagement relationship** means the H06 relationship represented by the Alumni Profile and its governed engagement state. It is distinct from the C08 institutional Alumni Persona/transition fact.

## 2. Alumni Profile and Membership Lifecycle

Canonical Alumni Profile engagement lifecycle:

`Created → Active → Suspended → Active`

Terminal branch:

`Active / Suspended → Closed`

The profile remains historically available after Closed and cannot be used to rewrite the C08 Alumni Persona.

Canonical Alumni Membership lifecycle:

`Requested → Under Review → Active → Closed`

Branches:

- `Under Review → Rejected`
- `Active → Suspended → Active`
- `Active / Suspended → Closed`

A Closed membership may be followed by a new governed membership request where policy permits; the prior membership record remains historical.

Canonical H06 events added:

- `AlumniProfileActivated`
- `AlumniProfileSuspended`
- `AlumniProfileReactivated`
- `AlumniProfileClosed`
- `AlumniMembershipActivated`
- `AlumniMembershipRejected`
- `AlumniMembershipSuspended`
- `AlumniMembershipClosed`

## 3. Alumni Chapter Lifecycle

Canonical Alumni Chapter lifecycle:

`Proposed → Under Review → Approved → Active → Retired`

Branches:

- `Under Review → Rejected`
- `Active → Suspended → Active`
- `Suspended → Retired`

`Retired` is terminal for the chapter record. Historical chapter membership and participation evidence remain preserved.

Canonical H06 events added:

- `AlumniChapterApproved`
- `AlumniChapterActivated`
- `AlumniChapterRejected`
- `AlumniChapterSuspended`
- `AlumniChapterRetired`

## 4. Alumni Network Connection Lifecycle

Canonical Alumni Network Connection lifecycle:

`Requested → Verification Pending → Verified → Closed`

Branches:

- `Verification Pending → Rejected`
- `Verified → Suspended → Verified`
- `Verified / Suspended → Closed`

A network connection is an H06 engagement relationship only. It does not assert employment, academic or identity facts owned elsewhere.

Canonical H06 events added:

- `AlumniNetworkConnectionVerified`
- `AlumniNetworkConnectionRejected`
- `AlumniNetworkConnectionSuspended`
- `AlumniNetworkConnectionClosed`

## 5. Alumni Recognition Lifecycle

Canonical Alumni Recognition lifecycle:

`Nominated → Under Review → Approved → Recognized`

Branches:

- `Under Review → Rejected`
- `Recognized → Withdrawn` only when a governed institutional decision invalidates or rescinds the recognition; the historical recognition record remains preserved.

Recognition may reference authoritative achievements or contributions from other domains but does not re-own those source facts.

Canonical H06 events added:

- `AlumniRecognitionApproved`
- `AlumniRecognized`
- `AlumniRecognitionRejected`
- `AlumniRecognitionWithdrawn`

## 6. Ownership Preservation

- C08 owns Alumni Persona and Student-to-Alumni transition.
- H06 owns Alumni Profile, Membership, Chapter, Network Connection and Recognition lifecycles defined here.
- Finance owns donation execution and financial facts; H06 owns donation intent only.
- Placement owns placement facts; Research owns research facts; H06 may reference them for engagement purposes.

## 7. Downstream Effect

This amendment closes the H06 lifecycle gaps identified by L3-H-039, L3-H-040, L3-H-042, L3-H-043 and L3-H-046 when read together with the C08 boundary clarification.

## 8. Locked Decisions

The engagement boundary, lifecycles, terminal meanings, events and ownership rules in §§1–6 are Locked L2-H06 decisions under ADR-H-ALUMNI-ENGAGEMENT-001.
