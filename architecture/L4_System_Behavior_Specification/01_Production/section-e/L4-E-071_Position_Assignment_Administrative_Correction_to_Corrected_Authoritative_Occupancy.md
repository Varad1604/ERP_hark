# Position Assignment Administrative Correction to Corrected Authoritative Occupancy

## 1. Specification Metadata

**Specification ID:** L4-E-071  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Position & Assignment  
**Realizes Scenario:** L3-E-071  
**Primary Business Owner:** Position & Assignment  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-E-071 — Position Assignment Administrative Correction to Corrected Authoritative Occupancy; applicable locked L2-E02 Position & Assignment architecture  

## 2. Behavioral Objective & Boundary

Correct an identified administrative error in an existing Position Assignment so the corrected occupancy becomes authoritative while preserving effective dating, version lineage, and historical traceability. Position Definition, Workforce Engagement, authorization policy, payroll, leave, and technical data-repair mechanics remain outside this behavior.

## 3. Required Business Context

- Existing Position Assignment.
- Valid Workforce Engagement.
- Valid Position Definition reference.
- Identified administrative discrepancy.
- Proposed correction satisfying applicable assignment rules.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-071-BH-001 | When an administrative discrepancy is identified, the system shall establish the governed correction context for the affected Position Assignment. | L3-E-071 §§3,7–9 |
| L4-E-071-BH-002 | The system shall validate the proposed correction against Position Definition, Workforce Engagement, organisational placement, effective dating, and Position Assignment rules. | L3-E-071 §§7,9,12 |
| L4-E-071-BH-003 | When the correction is valid, the system shall establish the corrected Position Assignment through the governed assignment evolution model. | L3-E-071 §§3,9,13 |
| L4-E-071-BH-004 | The system shall preserve the prior assignment state and effective history as reconstructable lineage. | L3-E-071 §§3,9,12 |
| L4-E-071-BH-005 | The system shall publish the canonical Position Assignment correction outcome using the frozen event vocabulary where defined. | L3-E-071 §§4.3,14 |

## 5. Cross-Domain & Event Behavior

Position & Assignment remains the canonical publisher of Position Assignment lifecycle events. The exact frozen correction event name shall be taken from the authoritative L2-E02/Stage 3 mapping and shall not be invented. Organisation and Workforce & Engagement remain owners of their referenced facts.

## 6. Failure / Rejection / Continuation

- If the proposed correction violates an assignment invariant, the system shall not apply it.
- If Position Definition is invalid or unavailable, the existing valid assignment shall remain authoritative pending resolution.
- If Workforce Engagement is incompatible, the assignment correction shall not establish invalid occupancy.
- If correction evidence is insufficient, the authoritative assignment shall remain unchanged.

## 7. Audit & Evidence Behavior

Preserve the correction basis, supporting evidence, prior assignment state, corrected state, effective dating, and lineage required for business reconstruction.

## 8. Observable Completion Conditions

- Correction is validated.
- Corrected Position Assignment is authoritative.
- Prior state remains reconstructable.
- Ownership of Position Definition and Workforce Engagement remains unchanged.
- Canonical correction outcome is published where defined.

## 9. Traceability & Provenance

**Parent:** L3-E-071 — Position Assignment Administrative Correction to Corrected Authoritative Occupancy  
**Primary verification source:** L2-E02 Position & Assignment architecture

## 10. Open Issues / Deferred Design

Confirm the exact frozen administrative-correction event and transition terminology from the authoritative L2-E02/Stage 3 mapping before Architecture Approval.
