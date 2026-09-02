# Position Assignment Rollback to Restored Valid Organisational State

## 1. Specification Metadata

**Specification ID:** L4-E-072  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-072 — Position Assignment Rollback to Restored Valid Organisational State  
**Primary Business Owner:** Position & Assignment  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-072; L2-E02 — Position & Assignment; L2-E01 — Workforce & Engagement  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Restore a Position Assignment to a previously valid governed organisational state while preserving assignment lineage. This is a business restoration within Position & Assignment, not technical rollback or modification of Workforce Engagement or Position Definition.

## 3. Required Business Context

- Existing Position Assignment requiring restoration.
- Authoritative historical assignment lineage.
- Valid current Workforce Engagement and Position Definition context.
- Required business authority for restoration.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-072-BH-001 | The system shall establish the restoration need only from an authoritative finding that the current assignment state is invalid or incorrect. | L3-E-072 §§3, 7–9 |
| L4-E-072-BH-002 | The system shall identify the prior valid assignment state from authoritative preserved assignment history and shall not invent a restoration target. | L3-E-072 §§9, 11–12 |
| L4-E-072-BH-003 | The system shall validate the historical target against current Position Definition, Workforce Engagement and Position Assignment invariants. | L3-E-072 §§7, 9, 11–12 |
| L4-E-072-BH-004 | If the target remains valid and required business authority is satisfied, the system shall restore the Position Assignment through its governed lifecycle/version lineage. | L3-E-072 §§3, 9, 13, 15–16 |
| L4-E-072-BH-005 | The system shall preserve the prior invalid or superseded assignment position as immutable historical lineage. | L3-E-072 §§9, 12–13, 16 |
| L4-E-072-BH-006 | The system shall not restore a state that would require unauthorized changes to Workforce Engagement or Position Definition. | L3-E-072 §§4.4, 10–13 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-072-BH-007 | Position & Assignment shall consume authoritative Workforce Engagement and Position Definition context while retaining sole ownership of the assignment restoration. | L3-E-072 §§5, 13–14 |
| L4-E-072-BH-008 | The restored authoritative occupancy fact shall be available to dependent consumers under Position & Assignment ownership. | L3-E-072 §§3, 14, 16 |
| L4-E-072-BH-009 | The system shall not invent the rollback/restoration event name because the Locked parent requires that exact vocabulary to be confirmed against the authoritative Stage-3/L2-E02 mapping. | L3-E-072 §§14.1, 17 |

## 6. Failure / Rejection / Continuation

- If no valid historical target exists, restoration shall not proceed.
- If the historical Position Definition is no longer valid, invalid occupancy shall not be recreated.
- If Workforce Engagement is incompatible, the workforce-side condition must be resolved by its owner before restoration.
- If an assignment invariant is violated, the current authoritative assignment remains unchanged.

## 7. Audit & Evidence Behavior

The restoration finding, historical target, validation result, authority, resulting assignment and complete lineage shall remain traceable.

## 8. Observable Completion Conditions

A valid prior state is established and validated, the Position Assignment is restored through its governed lifecycle, historical lineage remains intact, and Workforce Engagement and Position Definition ownership remain unchanged.

## 9. Traceability & Provenance

**Parent:** L3-E-072 — Position Assignment Rollback to Restored Valid Organisational State  
**Primary verification source:** L3-E-072 §§3, 5, 9, 11–17 and authoritative L2-E02 architecture.

## 10. Open Issues / Deferred Design

The exact restoration transition/event vocabulary remains an upstream confirmation item. No event name or new state is invented at L4. Technical rollback, persistence and recovery mechanics remain deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
