# Position Assignment Rollback to Restored Valid Organisational State

## 1. Specification Metadata

**Specification ID:** L4-E-072  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Position & Assignment  
**Realizes Scenario:** L3-E-072  
**Primary Business Owner:** Position & Assignment  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-E-072 — Position Assignment Rollback to Restored Valid Organisational State; applicable locked L2-E02 Position & Assignment architecture  

## 2. Behavioral Objective & Boundary

Restore a Position Assignment to a previously valid governed organisational state while preserving historical lineage. This is a business restoration, not technical rollback, database recovery, or reversal of unrelated domains.

## 3. Required Business Context

- Existing Position Assignment.
- Authoritative prior valid assignment state.
- Valid Workforce Engagement.
- Valid Position Definition context.
- Required business authority for restoration.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-072-BH-001 | When an authoritative finding requires restoration, the system shall establish the restoration context for the affected Position Assignment. | L3-E-072 §§3,7–9 |
| L4-E-072-BH-002 | The system shall identify the authoritative prior valid assignment state from preserved assignment history. | L3-E-072 §§3,9,12 |
| L4-E-072-BH-003 | The system shall validate the restoration target against current Position Definition, Workforce Engagement, and assignment rules. | L3-E-072 §§7,9,12 |
| L4-E-072-BH-004 | When the target remains valid and authority is satisfied, the system shall restore the Position Assignment through its governed business lifecycle. | L3-E-072 §§3,9,13 |
| L4-E-072-BH-005 | The system shall preserve the invalid or superseded state as immutable historical lineage. | L3-E-072 §§3,9,12 |
| L4-E-072-BH-006 | The system shall publish the canonical restored-state outcome using the frozen event vocabulary where defined. | L3-E-072 §§4.3,14 |

## 5. Cross-Domain & Event Behavior

Position & Assignment remains the canonical publisher of assignment restoration events. The exact frozen restoration event name shall be taken from authoritative Stage 3/L2-E02 mapping. Workforce Engagement and Organisation remain owners of their respective referenced facts.

## 6. Failure / Rejection / Continuation

- If no valid historical state can be established, the system shall not invent one.
- If the historical Position Definition is no longer valid, the system shall not recreate invalid occupancy.
- If Workforce Engagement is incompatible, restoration shall not proceed until the workforce-side condition is resolved.
- If restoration violates an assignment invariant, the current authoritative state shall remain unchanged.

## 7. Audit & Evidence Behavior

Preserve the restoration basis, target prior state, authority, validation outcome, resulting assignment state, and complete assignment lineage.

## 8. Observable Completion Conditions

- Valid restoration target is established.
- Target satisfies current assignment rules.
- Position Assignment is authoritative in the restored state.
- Prior invalid/superseded state remains traceable.
- Cross-domain ownership remains unchanged.

## 9. Traceability & Provenance

**Parent:** L3-E-072 — Position Assignment Rollback to Restored Valid Organisational State  
**Primary verification source:** L2-E02 Position & Assignment architecture

## 10. Open Issues / Deferred Design

Confirm the exact frozen restoration lifecycle transition and event vocabulary before Architecture Approval.
