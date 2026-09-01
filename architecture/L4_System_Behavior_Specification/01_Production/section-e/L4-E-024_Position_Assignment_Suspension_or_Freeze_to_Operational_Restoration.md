# Position Assignment Suspension or Freeze to Operational Restoration

## 1. Specification Metadata

**Specification ID:** L4-E-024  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Position & Assignment  
**Realizes Scenario:** L3-E-024  
**Primary Business Owner:** Position & Assignment  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-E-024 — Position Assignment Suspension or Freeze to Operational Restoration; L2-E02 — Position & Assignment Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern suspension or freeze of a Position Assignment and its authorised restoration to an operational state while preserving assignment ownership and history.

L4 begins when an authorised suspension/freeze or restoration condition exists for a Position Assignment. L4 ends when the governed suspended/frozen outcome or restored Active outcome is authoritative.

## 3. Required Business Context

- Current Position Assignment lifecycle position.
- Authorised governance decision.
- Applicable suspension/freeze or restoration condition.
- Effective-period context.
- Authoritative organisational and workforce context where required.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-024-BH-001 | The system shall validate the current Position Assignment lifecycle position before applying a permitted suspension, freeze, or restoration transition. | L3-E-024 §§7, 9, 12, 13 |
| L4-E-024-BH-002 | The system shall prevent a suspension, freeze, or restoration transition when the current state or governance conditions do not permit the requested transition. | L3-E-024 §§9, 11, 12 |
| L4-E-024-BH-003 | When suspension is authorised, the system shall move the Position Assignment through the canonical Active → Suspended transition and recognise the corresponding canonical business event. | L3-E-024 §§10, 13, 14 |
| L4-E-024-BH-004 | When freeze is authorised, the system shall move the Position Assignment through the canonical Active → Frozen transition and recognise the corresponding canonical business event. | L3-E-024 §§10, 13, 14 |
| L4-E-024-BH-005 | When restoration is authorised, the system shall restore the Position Assignment through the applicable canonical Suspended → Active or Frozen → Active transition. | L3-E-024 §§10, 13 |
| L4-E-024-BH-006 | The system shall preserve effective dating and historical lineage for every suspension, freeze, or restoration outcome. | L3-E-024 §§3, 9, 11, 16 |

## 5. Cross-Domain & Event Behavior

The system shall recognise and make available the canonical events associated with the applicable lifecycle path:

- `PositionAssignmentSuspended`
- `PositionAssignmentReactivated`
- `AssignmentFrozen`
- `AssignmentUnfrozen`

The canonical publisher remains Position & Assignment / Position Assignment. Cross-domain context from Workforce & Engagement and Organisation does not transfer ownership. fileciteturn175file0L2-L2

## 6. Failure / Rejection / Continuation

When the requested transition is not valid for the current lifecycle state or governance conditions are not satisfied, the system shall retain the current authoritative Position Assignment state and shall not assert the requested outcome.

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable evidence of:

- current lifecycle state;
- authorisation basis;
- requested transition;
- resulting lifecycle position;
- effective timing; and
- applicable canonical event/fact.

## 8. Observable Completion Conditions

The behavior is complete when:

- the Position Assignment is in the requested canonical suspended, frozen, or Active restored state;
- the correct lifecycle path has been applied;
- history and effective dating are preserved; and
- the applicable canonical business event has been recognised/made available.

## 9. Traceability & Provenance

**Parent:** L3-E-024 — Position Assignment Suspension or Freeze to Operational Restoration  
**Primary verification source:** L2-E02 — Position & Assignment Workflows  

The L3 parent explicitly identifies W-014 and W-018 and their canonical state/event paths.

## 10. Open Issues / Deferred Design

No L4 business-behavior issue is identified in the approved parent. Technical transaction, persistence, transport, IAM, retry/rollback, infrastructure, and UI behavior remain downstream concerns.

## Canonical Validation Note

This specification is a Draft and requires applicable L4 validation and governance review before Architecture Approval or Locking.
