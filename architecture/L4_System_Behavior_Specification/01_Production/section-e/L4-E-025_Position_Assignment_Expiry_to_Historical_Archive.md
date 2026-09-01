# Position Assignment Expiry to Historical Archive

## 1. Specification Metadata

**Specification ID:** L4-E-025  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Position & Assignment  
**Realizes Scenario:** L3-E-025  
**Primary Business Owner:** Position & Assignment  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-E-025 — Position Assignment Expiry to Historical Archive; L2-E02 — Position & Assignment Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern Position Assignment expiry and subsequent historical archival while preserving immutable assignment lineage and preventing archived assignments from participating in operational processing.

L4 begins when the authoritative expiry condition is reached for an eligible Position Assignment. L4 ends when the assignment reaches the authoritative Archived outcome.

Assignment suspension, separate assignment changes, and technical archival mechanisms remain outside this specification.

## 3. Required Business Context

- Active Position Assignment.
- Authoritative expiry condition/date.
- Archive eligibility after expiry.
- Applicable lifecycle governance.
- Assignment history and effective-period context.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-025-BH-001 | When the authoritative expiry condition is reached, the system shall validate the Position Assignment's eligibility for expiry before applying the expiry outcome. | L3-E-025 §§7, 9 |
| L4-E-025-BH-002 | On valid expiry, the system shall transition the Position Assignment from Active to Expired and recognise `PositionAssignmentExpired` as the canonical business fact. | L3-E-025 §§9, 12, 13, 14 |
| L4-E-025-BH-003 | The system shall prevent an Expired Position Assignment from participating in subsequent operational processing. | L3-E-025 §§3, 11, 12, 16 |
| L4-E-025-BH-004 | When archive eligibility is satisfied, the system shall transition the Position Assignment from Expired to Archived and recognise `PositionAssignmentArchived`. | L3-E-025 §§9, 12, 13, 14 |
| L4-E-025-BH-005 | The system shall preserve immutable assignment history and lineage across expiry and archival. | L3-E-025 §§3, 9, 11, 12, 16 |
| L4-E-025-BH-006 | When an assignment is not yet eligible for expiry or archival, the system shall retain the current authoritative lifecycle position. | L3-E-025 §§10, 11 |

## 5. Cross-Domain & Event Behavior

The system shall recognise the canonical lifecycle events:

- `PositionAssignmentExpired`
- `PositionAssignmentArchived`

The canonical publisher remains Position & Assignment. Downstream consumers may use the completed business facts without acquiring lifecycle ownership. fileciteturn176file0L2-L2

## 6. Failure / Rejection / Continuation

When expiry conditions are not satisfied, the system shall retain the current authoritative Position Assignment state. When archive eligibility is not satisfied after expiry, the system shall retain the Expired historical position and shall not assert Archived.

An extension before expiry shall follow the canonical extension path rather than the expiry path.

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- expiry condition/date;
- lifecycle position before and after expiry;
- archive eligibility determination;
- applicable governance basis;
- canonical expiry and archival business facts; and
- immutable assignment lineage.

## 8. Observable Completion Conditions

The behavior is complete when:

- the Position Assignment is Expired and later Archived when eligible;
- the assignment no longer participates in operational processing;
- assignment history remains immutable and reconstructable; and
- both canonical lifecycle facts have been recognised/made available.

## 9. Traceability & Provenance

**Parent:** L3-E-025 — Position Assignment Expiry to Historical Archive  
**Primary verification source:** L2-E02 — Position & Assignment Workflows  

Canonical workflows identified by the parent: W-015 — Assignment Expiry and W-021 — Archive Position Assignment.

## 10. Open Issues / Deferred Design

No L4 business-behavior issue is identified in the approved parent. Technical archival storage, interfaces, access controls, transport, retry/rollback, infrastructure, and UI remain downstream concerns.

## Canonical Validation Note

This specification is a Draft and requires applicable L4 validation and governance review before Architecture Approval or Locking.
