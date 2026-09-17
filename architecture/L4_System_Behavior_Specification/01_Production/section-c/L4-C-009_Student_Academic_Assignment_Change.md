# Student Academic Assignment Change

## 1. Specification Metadata

**Specification ID:** L4-C-009  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Student Lifecycle  
**Realizes Scenario:** L3-C-009 — Student Academic Assignment Change  
**Primary Business Owner:** Student Lifecycle  
**Version:** 1.0.0  
**Status:** Architecture Approved  
**Owner:** Varad  
**Last Updated:** 2026-09-17  
**Depends On:** L3-C-009; applicable locked C architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

The system shall govern authorized Programme, Batch and/or Section assignment for an Active Student against valid academic structure. The behavior ends when assignment is completed, prior assignment history is preserved and `AcademicAssignmentCompleted` is published.

## 3. Required Business Context

- Student is Active.
- Required academic structure exists.
- Programme changes require authoritative `ProgrammeMigrationApproved` where applicable.
- Student Lifecycle owns academic assignment.
- Academic Delivery participates in academic context but does not own Student assignment lifecycle.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-009-BH-001 | When an academic assignment request is received for an Active Student, the system shall validate the required academic structure before applying the assignment. | L3-C-009 §§7, 9, 12; L2-C02 WF-C02-04 |
| L4-C-009-BH-002 | When Programme assignment is changing, the system shall require the authoritative `ProgrammeMigrationApproved` outcome before applying the Programme association. | L3-C-009 §§7, 9, 12, 13; L2-C04/L2-C02 |
| L4-C-009-BH-003 | When valid Programme, Batch or Section assignments are authorized, the system shall apply the applicable assignment to the StudentContext. | L3-C-009 §§9, 13; L2-C02 |
| L4-C-009-BH-004 | The system shall preserve the previous academic assignment as historical assignment information. | L3-C-009 §§9, 12; L2-C02 |
| L4-C-009-BH-005 | The system shall not make or republish the Programme Migration decision when applying an already-approved migration outcome. | L3-C-009 §§4, 10, 12; L2-C04/L2-C02 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-009-BH-006 | On successful academic assignment completion, the system shall publish the canonical `AcademicAssignmentCompleted` business event under Student Lifecycle ownership. | L3-C-009 §§3, 14; L2-C02 |

## 6. Failure / Rejection / Continuation

- Prevent assignment when the Student is not Active.
- Prevent assignment when required academic structure is invalid.
- Prevent Programme association change without the required authoritative migration outcome.
- Do not absorb enrollment update, Programme Migration decision-making or another distinct lifecycle outcome.
- Preserve assignment history when the change cannot complete.

## 7. Audit & Evidence Behavior

Preserve attributable evidence of the authorized assignment change, applicable academic context, prior assignment and resulting outcome through Student Lifecycle/Audit capabilities.

## 8. Observable Completion Conditions

Completion occurs when authorized assignment is applied against valid academic structure, required migration approval is respected where applicable, prior history is preserved, audit evidence is available and `AcademicAssignmentCompleted` has been published.

## 9. Traceability & Provenance

**Parent:** L3-C-009 — Student Academic Assignment Change  
**Primary verification source:** Authoritative Student Lifecycle L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is Architecture Approved under the W-C02 parent-set validation decision and is not Locked.
