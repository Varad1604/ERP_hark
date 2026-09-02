# Transport Stop Creation to Active Pickup/Drop Location

## 1. Specification Metadata

**Specification ID:** L4-G-096  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-096 — Transport Stop Creation to Active Pickup/Drop Location  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-096; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall establish and activate a valid Transport Stop as an operational pickup/drop location under Transport Management ownership.

## 3. Required Business Context

- Transport Management owns the Transport Stop lifecycle.
- Location context and intended operational use are identifiable.
- Applicable stop validation and authorization rules are available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-096-BH-001 | Establish the proposed Transport Stop within Transport Management ownership. | L3-G-096 §§2–5 |
| L4-G-096-BH-002 | Validate the stop location and required pickup/drop business context before activation. | L3-G-096 §§7, 9, 12 |
| L4-G-096-BH-003 | Establish the required route association before the stop can become Active. | L3-G-096 §§3, 7, 9, 16 |
| L4-G-096-BH-004 | Apply required operational review or authorization where the configured transport workflow requires it. | L3-G-096 §§6, 15 |
| L4-G-096-BH-005 | Transition the Transport Stop to Active only after location verification, route assignment, and operational validation are complete. | L3-G-096 §§9, 13, 16 |
| L4-G-096-BH-006 | Preserve the stop's establishment, assignment, activation history and evidence. | L3-G-096 §§12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-096-BH-007 | Transport Management shall preserve the source-defined `StopCreated` and `StopAssigned` facts where those milestones occur. The parent does not define a separate activation event; this specification shall not invent one. | L3-G-096 §§14, 17 |

## 6. Failure / Rejection / Continuation

- Incomplete stop information, unverifiable location, missing route association, or failed operational validation shall prevent activation.
- If the proposed location is no longer required, the stop shall remain non-active without inventing a new closure state.

## 7. Audit & Evidence Behavior

Preserve stop definition, location verification, route association, validation, authorization where applicable, activation, and historical evidence.

## 8. Observable Completion Conditions

The stop is established, location-verified, route-associated, operationally validated, Active for authorised pickup/drop use, and its history remains traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-096 — Transport Stop Creation to Active Pickup/Drop Location  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

The exact event representing the Active transition is unresolved upstream; this specification deliberately does not invent one. Technical APIs, persistence, UI, geographic implementation, and retry mechanics are deferred to later architecture levels.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
