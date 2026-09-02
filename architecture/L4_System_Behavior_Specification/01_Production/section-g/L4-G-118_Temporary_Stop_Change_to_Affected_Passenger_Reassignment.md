# Temporary Stop Change to Affected Passenger Reassignment

## 1. Specification Metadata

**Specification ID:** L4-G-118  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-118 — Temporary Stop Change to Affected Passenger Reassignment  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-118; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern a temporary stop change and the resulting effective reassignment of affected transport allocations while preserving passenger identity ownership and prior allocation history.

## 3. Required Business Context

- Affected route, stop and active transport allocations are authoritative.
- Passenger identity/context remains owned by C or E as applicable.
- Transport Management owns stop and allocation changes.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-118-BH-001 | Identify the affected route, stop and active passenger allocations. | L3-G-118 §§3, 7, 9 |
| L4-G-118-BH-002 | Establish the authorised temporary stop arrangement after applicable operational assessment and approval. | L3-G-118 §§9, 12, 15 |
| L4-G-118-BH-003 | Reassign only affected Transport Allocations to the applicable temporary stop arrangement using authoritative passenger context. | L3-G-118 §§3, 9, 12, 16 |
| L4-G-118-BH-004 | Leave unaffected allocations unchanged and prevent reassignment to an operationally unsuitable temporary stop. | L3-G-118 §§10–12 |
| L4-G-118-BH-005 | Preserve previous stop/allocation context and effective temporary reassignment history. | L3-G-118 §§12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-118-BH-006 | For each authoritative Transport Allocation update produced by the reassignment, Transport Management shall publish `TransportAllocationUpdated`. No `TemporaryStopChanged` or `PassengerReassigned` event shall be invented. | L3-G-118 §§14, 16–17 |

## 6. Failure / Rejection / Continuation

- An unauthorised or unsuitable temporary stop arrangement shall not become effective.
- Passenger reassignment shall not complete where authoritative passenger context cannot be confirmed.
- A permanent route revision shall follow its separate scenario.

## 7. Audit & Evidence Behavior

Preserve affected stop/route, temporary arrangement, authority, affected allocations, prior allocation context and effective reassignment evidence.

## 8. Observable Completion Conditions

The temporary stop arrangement is effective, affected allocations use the authorised temporary stop context, applicable `TransportAllocationUpdated` events are published, and prior allocation history remains traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-118 — Temporary Stop Change to Affected Passenger Reassignment  
**Primary verification source:** L3-G-118 §§3, 9–17; authoritative L2-G04 Transport Management workflow architecture.

## 10. Open Issues / Deferred Design

No dedicated temporary-stop/passenger-reassignment event is defined upstream. Only exact event-bearing transitions established by G04 are used. Technical implementation remains deferred to L5/L6.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
