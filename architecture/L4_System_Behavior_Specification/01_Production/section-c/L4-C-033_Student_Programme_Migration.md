# Student Programme Migration

## 1. Specification Metadata

**Specification ID:** L4-C-033  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Student Lifecycle  
**Realizes Scenario:** L3-C-033 — Student Programme Migration  
**Primary Business Owner:** Academic Progression  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-C-033; applicable locked C architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall support an authoritative Programme Migration decision with eligibility, destination curriculum mapping and credit mapping completed before approval. The behavior ends at the approved migration outcome; C02 remains responsible for applying the Programme association update.

## 3. Required Business Context

- Valid existing Student Programme association.
- Authoritative destination Programme and curriculum information.
- Applicable migration eligibility and institutional policy.
- Relevant academic credit position and, where required, destination capacity information.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-033-BH-001 | The system shall evaluate whether Programme-changing mobility is permitted for the Student under the governed process. | L3-C-033 §§2, 7–9 |
| L4-C-033-BH-002 | Where destination capacity/seat validation is required, the system shall consider the authoritative destination availability condition before approval. | L3-C-033 §§4.3, 9, 12 |
| L4-C-033-BH-003 | the system shall govern mapping the Student's current academic position to the destination curriculum before migration approval. | L3-C-033 §§2, 9, 12 |
| L4-C-033-BH-004 | the system shall govern mapping recognised academic credits to the destination Programme before migration approval. | L3-C-033 §§9, 12 |
| L4-C-033-BH-005 | The system shall establish the migration decision only after the required eligibility and mappings are complete. | L3-C-033 §§3, 9–12 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-033-BH-006 | On approved Programme Migration, the system shall publish the canonical `ProgrammeMigrationApproved` outcome from Academic Progression. | L3-C-033 §14; L2-C04 §7.4–7.7 |
| L4-C-033-BH-007 | The system shall make the approved migration outcome available for Student Lifecycle to apply its own Programme association update without re-evaluating the migration decision. | L3-C-033 §§3, 5, 14 |

## 6. Failure / Rejection / Continuation

- If eligibility fails, the system shall not approve migration and shall preserve the existing Programme association as authoritative.
- If curriculum mapping is invalid or incomplete, migration shall not be approved until governed mapping is complete.
- If required destination capacity is unavailable, migration shall not be approved.
- If approved migration has not yet been applied by C02, the migration decision remains complete while the downstream lifecycle update remains separately owned.

## 7. Audit & Evidence Behavior

The system shall preserve migration eligibility evidence, destination Programme/curriculum mapping, credit mapping, approval outcome and material historical academic context.

## 8. Observable Completion Conditions

Approved completion requires eligibility evaluated, required mappings complete, the migration decision established, and `ProgrammeMigrationApproved` published. Rejected/deferred completion preserves the existing Programme association and records the governed reason or unresolved mapping/eligibility condition.

## 9. Traceability & Provenance

**Parent:** L3-C-033 — Student Programme Migration  
**Primary verification source:** Authoritative Student Lifecycle L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
