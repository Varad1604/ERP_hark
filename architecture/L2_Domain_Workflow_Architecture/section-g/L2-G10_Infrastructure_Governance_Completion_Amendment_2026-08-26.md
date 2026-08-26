# L2-G10 — Infrastructure Management Workflows — Governance Completion Amendment

| Attribute | Value |
|---|---|
| Target | L2-G10 — Infrastructure Management Workflows and locked 2026-08-26 registration amendment |
| Amendment Version | 1.2.0 |
| Status | Locked |
| Effective Date | 2026-08-26 |
| Approved Through | ADR-G-INFRA-GOV-001 |
| Authority | Explicit Lead Architect remediation authorization recorded 2026-08-26 |

This amendment is canonical L2-G10 architecture. It resolves recorded cross-domain association, registration-event, capability, capacity, availability and reservation-lifecycle ambiguities without changing B03 institutional-campus ownership or G10 physical-infrastructure ownership. Unaffected G10 decisions remain unchanged.

## 1. B03 ↔ G10 Campus Reference Boundary

B03 remains the sole owner of `InstitutionalCampus` identity, institutional status and Institution-to-Campus governance. G10 remains the sole owner of `Physical Campus` identity, physical hierarchy and physical lifecycle.

The canonical persisted cross-domain association is owned by G10 as a reference carried by the G10 Physical Campus to the B03 `InstitutionalCampus` identity. This preserves the locked relationship direction already stated in B03.

For business navigation and consumption, B03 and other authorized consumers may resolve the corresponding G10 Physical Campus through the G10 Physical Campus Summary Contract using the B03 `InstitutionalCampus` identity as the governing reference. Such reverse resolution is a read/reference capability only; B03 does not persist or own a duplicate Physical Campus association.

The **Infrastructure Administrator** is the G10 business actor authorized to establish or replace the Physical Campus → InstitutionalCampus reference where infrastructure administration is permitted. The **Institution Administrator** is responsible for ensuring that the referenced InstitutionalCampus is valid for the institution and institutional context. Neither actor may transfer aggregate ownership.

Canonical event added:

- `PhysicalCampusInstitutionalReferenceChanged` — published only by G10 when an established Physical Campus changes its referenced B03 InstitutionalCampus identity through a governed correction/reassociation.

Initial Physical Campus establishment continues to use the canonical registration event already locked by the earlier G10 amendment; no duplicate association event is required merely because the initial reference is present at establishment.

## 2. Structural Registration Event Vocabulary

For G10 structural registration, the canonical establishment events are:

- `BuildingCreated`
- `FloorCreated`
- `SpaceCreated`

These are the canonical completed business facts for creation/registration of those physical aggregates. Earlier occurrences of `BuildingRegistered`, `FloorRegistered`, or `SpaceRegistered` that describe the same completed establishment fact are legacy vocabulary and shall not be used as separate semantic events.

The canonical initial lifecycle position for a newly established Building, Floor or Space is `Proposed`, unless an explicitly configured G10 workflow defines a stricter pre-Proposed preparation position that is not externally authoritative. Registration itself does not make the aggregate Operational.

## 3. Space Operational Capability Governance

G10 owns Space Operational Capability semantics and assignment.

A capability assignment is an attribute-level governed change on an existing Space; it does not create a separate aggregate lifecycle and does not change the Space lifecycle.

Canonical event:

- `SpaceOperationalCapabilitiesUpdated`

The canonical G10 Space Summary Contract is extended to include the current authoritative set of Operational Capabilities for the Space. Authorized consumers may use those facts for their own eligibility decisions but do not acquire ownership of the capability or Space.

Infrastructure Administrator remains the business actor for the governed capability assignment under applicable authorization and configuration rules.

## 4. Capacity Event Vocabulary

`CapacityUpdated` is the sole canonical G10 event for establishment or change of an authoritative Capacity Profile on physical infrastructure.

Earlier `CapacityChanged` and `RoomCapacityChanged` occurrences describing the same generic Capacity Profile change are legacy vocabulary and shall not be treated as separate semantic events.

A Capacity Profile becomes authoritative through the governed capacity workflow; no separate lifecycle state named `Active` is required. The frozen business phrase “activation” means that the effective Capacity Profile is now the authoritative current capacity fact.

## 5. Availability Event Vocabulary

Availability remains independent from physical-infrastructure lifecycle.

Canonical G10 events are:

- `InfrastructureAvailabilityEstablished` — the first authoritative Availability Profile is established for a physical resource.
- `InfrastructureAvailabilityChanged` — an existing authoritative availability condition changes, including restriction, suspension in the business sense, restoration, scheduled closure, partial closure, or other source-defined availability change.

These events publish the resulting availability fact, not a physical lifecycle transition. Existing lifecycle events such as `InfrastructureActivated` or `InfrastructureDeactivated` shall not substitute for availability events.

No separate Availability Profile lifecycle state named `Active` is required. The frozen business phrase “activation” means that an authoritative Availability Profile is in effect.

## 6. Reservation Cancellation and Closure

The canonical Reservation lifecycle is clarified as:

`Requested → Confirmed → Active → Completed → Archived`

Governed cancellation branch:

`Requested / Confirmed / Active → Cancelled → Archived`

`Cancelled` is the canonical terminal business outcome for a reservation ended by cancellation before normal completion. `Archived` remains the historical preservation position after either Completed or Cancelled.

For scenario language, a Reservation is **closed** when it has reached either `Completed` or `Cancelled` and no longer reserves the resource. `Closed` is therefore a business completion meaning, not an additional lifecycle state.

`ReservationCancelled` remains the sole canonical cancellation event. Cancellation releases the reservation constraint; the resource becomes `Available` only if no other authoritative availability restriction applies.

## 7. Ownership and Event-Publisher Integrity

- B03 exclusively publishes B03 InstitutionalCampus lifecycle facts.
- G10 exclusively publishes the G10 events defined in this amendment.
- Section A supplies authorization, event and audit capabilities without becoming semantic publisher of G10 facts.
- Academic, Examination, Facilities and other consumers may react to G10 availability/capability/capacity facts but cannot mutate G10-owned state.

## 8. Downstream Effect

This amendment resolves the upstream blockers identified by L3-G-279, L3-G-282, L3-G-283, L3-G-287, L3-G-288, L3-G-289, L3-G-290 and L3-G-293. It also provides the event/lifecycle normalization required for neighboring G10 scenarios that referenced the same inconsistent vocabulary.

## 9. Locked Decisions

The cross-domain reference direction, actors, structural event vocabulary, capability contract/event, capacity event, availability events, Reservation cancellation/closure meaning and publisher boundaries in §§1–7 are Locked L2 decisions under ADR-G-INFRA-GOV-001.
