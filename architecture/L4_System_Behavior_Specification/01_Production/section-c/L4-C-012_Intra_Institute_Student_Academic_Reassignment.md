# Intra Institute Student Academic Reassignment

## 1. Specification Metadata

**Specification ID:** L4-C-012  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Student Lifecycle  
**Realizes Scenario:** L3-C-012 — Intra Institute Student Academic Reassignment  
**Primary Business Owner:** Student Lifecycle  
**Version:** 1.0.1  
**Status:** Architecture Approved  
**Owner:** Varad  
**Last Updated:** 2026-09-17  
**Depends On:** L3-C-012; applicable locked C architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

The system shall govern an authorized same-Institute, non-Programme Student Transfer outcome, preserving prior assignment history and applying the destination organisational/placement context. The behavior ends after `Transferred → Active` completion and the canonical transfer fact is published.

## 3. Required Business Context

- Existing Student and StudentContext.
- Same-Institute destination.
- Non-Programme reassignment.
- Authoritative Student Transfer Request or Administrative Approval.
- Student Lifecycle owns Student and StudentContext.
- Institution & Structure supplies authoritative institutional context.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-012-BH-001 | The system shall distinguish an authorized Transfer workflow from ordinary Academic Assignment maintenance. | L3-C-012 §§4, 12; L2-C02 |
| L4-C-012-BH-002 | The system shall validate that the destination remains within the same Institute and is non-Programme. | L3-C-012 §§4, 7, 9; L2-C02 |
| L4-C-012-BH-003 | The system shall validate the destination organisational/placement context against authoritative institutional structure. | L3-C-012 §§5, 9, 12; L2-B02 |
| L4-C-012-BH-004 | The system shall apply the authorized reassignment, preserve prior assignment history, and progress the Student through `Transferred` to `Active`. | L3-C-012 §§3, 9, 13, 16; L2-C02 |
| L4-C-012-BH-005 | The system shall not perform Programme Migration or absorb ordinary Academic Assignment. | L3-C-012 §§4, 10, 12; L2-C02/L2-C04 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-012-BH-006 | On successful transfer completion, the system shall publish the canonical `StudentTransferred` business fact under Student Lifecycle ownership. | L3-C-012 §§9, 14; L2-C02 |
| L4-C-012-BH-007 | When validating a requested destination, the system shall use the authoritative Institution & Structure organisational context and shall not modify that source-owned context as part of the Student transfer. | L3-C-012 §§5, 9, 14; L2-B02/L2-C02 |

## 6. Failure / Rejection / Continuation

- Reject destinations outside the same Institute.
- Reject Programme-changing requests from this behavior.
- Route ordinary assignment maintenance to WF-C02-04.
- Preserve prior assignment when transfer cannot complete.

## 7. Audit & Evidence Behavior

Preserve transfer authorization, source/destination context, prior assignment, lifecycle progression and completed transfer evidence through applicable Student Lifecycle/Audit capabilities.

## 8. Observable Completion Conditions

The destination context is valid, the authorized reassignment is applied, prior history is preserved, the Student completes `Transferred → Active`, and `StudentTransferred` is published.

## 9. Traceability & Provenance

**Parent:** L3-C-012 — Intra Institute Student Academic Reassignment  
**Primary verification source:** Authoritative Student Lifecycle L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is Architecture Approved under the W-C02 parent-set validation decision and is not Locked.
