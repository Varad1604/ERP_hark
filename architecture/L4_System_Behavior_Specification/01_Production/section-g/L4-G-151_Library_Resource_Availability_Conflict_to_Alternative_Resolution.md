# Library Resource Availability Conflict to Alternative Resolution

## 1. Specification Metadata

**Specification ID:** L4-G-151  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-151 — Library Resource Availability Conflict to Alternative Resolution  
**Primary Business Owner:** Library Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-151; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an authoritative library resource-availability conflict to either a source-supported alternative resolution or entry into the existing reservation/waiting-list path. The conflict itself shall not become a new lifecycle state or event.

## 3. Required Business Context

- A valid library resource request and requester/member context exist.
- The requested resource and authoritative availability condition are identifiable.
- Membership/borrowing/reservation context and applicable configurable policies can be evaluated.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-151-BH-001 | Identify the requested resource/request and establish the authoritative availability conflict without inventing a new `Availability Conflict` lifecycle state. | L3-G-151 §§7–9,13 |
| L4-G-151-BH-002 | Evaluate applicable membership, existing borrowing, reservation limit, resource availability and duplicate-reservation constraints under configurable reservation policy. | L3-G-151 §§9,12 |
| L4-G-151-BH-003 | Where an authorised alternative resource/resolution exists, establish that outcome without inventing a universal substitution/equivalence rule. | L3-G-151 §§3,9–12,17 |
| L4-G-151-BH-004 | Otherwise, where eligible, enter the existing reservation/waiting-list path and preserve queue position, reservation date, priority and notification status as required by that authoritative workflow. | L3-G-151 §§2–3,9,12–13 |
| L4-G-151-BH-005 | Where notification is required, hand the governed outcome to H while G retains ownership of the library decision. | L3-G-151 §§5–6,9,14 |
| L4-G-151-BH-006 | Preserve request, availability determination, selected resolution/reservation path and operational history. | L3-G-151 §§15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-151-BH-007 | The source defines reservation events `LibraryReservationCreated`, `ReservationAvailable`, `ReservationCollected` and `LibraryReservationExpired`; these shall be used only when the corresponding reservation transition actually occurs. No `AlternativeResourceAssigned` or `AvailabilityConflictResolved` event is invented. G is the canonical owner/publisher; H owns notification delivery and C supplies context where required. | L3-G-151 §§5,14,17 |

## 6. Failure / Rejection / Continuation

- Unclear resource identity or availability shall prevent invented resolution.
- Reservation eligibility that cannot be established shall prevent false waiting-list placement.
- If no authorised alternative exists, no substitute rule shall be fabricated.
- Pending notification shall not redefine the underlying library outcome.

## 7. Audit & Evidence Behavior

Preserve resource/request identity, availability determination, eligibility checks, selected alternative or reservation path, queue/reservation information where applicable and notification hand-off.

## 8. Observable Completion Conditions

Completion occurs when an authoritative alternative resolution is established or the request enters the existing reservation/waiting-list boundary under applicable policy, with canonical reservation events used only for transitions that actually occur.

## 9. Traceability & Provenance

**Parent:** L3-G-151 — Library Resource Availability Conflict to Alternative Resolution  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

The upstream source defines no dedicated availability-conflict lifecycle/event and no universal alternative-resource selection policy. Those gaps are preserved. Technical implementation remains deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
