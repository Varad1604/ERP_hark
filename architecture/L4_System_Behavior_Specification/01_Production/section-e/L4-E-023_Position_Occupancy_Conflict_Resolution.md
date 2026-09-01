# Position Occupancy Conflict Resolution

## 1. Specification Metadata

**Specification ID:** L4-E-023  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Position & Assignment  
**Realizes Scenario:** L3-E-023  
**Primary Business Owner:** Position & Assignment  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-E-023 — Position Occupancy Conflict Resolution; L2-E02 — Position & Assignment Workflows  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP shall identify and govern Position Assignment occupancy conflicts so that no ambiguous organisational occupancy state becomes operational.

L4 begins when an occupancy or related assignment conflict is identified. L4 ends when a deterministic governed resolution is authoritative, or the conflicting state remains non-operational pending resolution.

Technical conflict-handling mechanisms remain outside this specification.

## 3. Required Business Context

- Conflicting Position Assignment occupancy condition.
- Applicable Position Assignment and organisational constraints.
- Resolution precedence defined by governed policy.
- Authoritative organisational and workforce context where required.
- Applicable override authorization where permitted.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-023-BH-001 | When a Position Assignment occupancy conflict is identified, the system shall evaluate the conflicting business condition against applicable Position Assignment and organisational constraints before allowing the state to become operational. | L3-E-023 §§4, 8, 9, 12 |
| L4-E-023-BH-002 | The system shall prevent an ambiguous Position Assignment occupancy state from becoming operational. | L3-E-023 §§3, 9, 12 |
| L4-E-023-BH-003 | When multiple valid resolution outcomes exist, the system shall apply the authoritative resolution precedence defined by governed policy. | L3-E-023 §§9, 10, 12 |
| L4-E-023-BH-004 | Where a permitted manual override is authorised, the system shall require the applicable governed authority before accepting the override outcome. | L3-E-023 §§10, 12, 15 |
| L4-E-023-BH-005 | The system shall preserve the authoritative business position and conflict-resolution history when a conflict cannot be resolved immediately. | L3-E-023 §§3, 11, 16 |

## 5. Cross-Domain & Event Behavior

The L3 parent requires publication of applicable canonical domain events for completed business facts but does not identify a distinct conflict-resolution event name. No event name is invented. Organisation and Workforce & Engagement remain authoritative for their respective facts.

## 6. Failure / Rejection / Continuation

When a conflict cannot be validly resolved, the system shall prevent the conflicting state from becoming operational and shall retain the applicable existing authoritative business state where one exists.

A permitted override shall proceed only where the governing architecture allows it and required business governance is satisfied.

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable evidence sufficient to demonstrate:

- the conflicting Position Assignment condition;
- applicable business constraints considered;
- resolution precedence applied;
- any authorised override decision and its authority basis;
- resulting authoritative occupancy outcome; and
- historical resolution lineage.

## 8. Observable Completion Conditions

The behavior is complete when either:

- the conflict has been resolved deterministically and the resulting organisational occupancy state is authoritative; or
- the conflict remains explicitly non-operational with the last valid business position preserved until governed resolution occurs.

No ambiguous occupancy state may be operational at completion.

## 9. Traceability & Provenance

**Parent:** L3-E-023 — Position Occupancy Conflict Resolution  
**Primary verification source:** L2-E02 — Position & Assignment Workflows  

The L3 parent explicitly limits conflict-resolution behavior to business outcomes and preserves upstream ownership and policy precedence.

## 10. Open Issues / Deferred Design

No L4 business-behavior issue is identified in the approved parent. Technical conflict algorithms, persistence, APIs, transaction handling, queues, retry/rollback mechanisms, infrastructure, and UI remain downstream concerns.

## Canonical Validation Note

This specification is a Draft and requires applicable L4 validation and governance review before Architecture Approval or Locking.
