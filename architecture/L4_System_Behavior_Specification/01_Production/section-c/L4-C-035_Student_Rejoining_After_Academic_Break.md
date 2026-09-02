# Student Rejoining After Academic Break

## 1. Specification Metadata

**Specification ID:** L4-C-035  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Student Lifecycle  
**Realizes Scenario:** L3-C-035 — Student Rejoining After Academic Break  
**Primary Business Owner:** Academic Progression  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-C-035; applicable locked C architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall support restoration of academic progression after an approved Academic Break by verifying eligibility, considering curriculum changes, mapping preserved credits, establishing academic placement and approving rejoining. The behavior ends when registration eligibility is restored; registration execution remains C03-owned.

## 3. Required Business Context

- Approved Academic Break record.
- Current curriculum information.
- Preserved academic credits and history.
- Academic-period/semester placement information.
- Institutional rejoining rules.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-035-BH-001 | The system shall verify that an approved Academic Break exists and that the Student is eligible to seek rejoining. | L3-C-035 §§3, 7–9 |
| L4-C-035-BH-002 | The system shall consider curriculum changes occurring during the Academic Break before approving rejoining. | L3-C-035 §§2, 9, 12 |
| L4-C-035-BH-003 | the system shall govern mapping preserved academic credits to the current curriculum before rejoining approval. | L3-C-035 §§2, 9 |
| L4-C-035-BH-004 | The system shall establish the applicable semester/academic-period placement under institutional rules. | L3-C-035 §§9, 12 |
| L4-C-035-BH-005 | On successful evaluation, the system shall establish the canonical **Rejoining** position and restore eligibility for Academic Registration. | L3-C-035 §§3, 9, 13 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-035-BH-006 | On approved rejoining, the system shall publish `StudentRejoined` from the Student Rejoining aggregate within Academic Progression. | L3-C-035 §14 |
| L4-C-035-BH-007 | The system shall make restored registration eligibility available to Academic Registration without transferring rejoining decision ownership. | L3-C-035 §§3, 5, 14 |

## 6. Failure / Rejection / Continuation

- If no approved Academic Break exists, the system shall not initiate governed rejoining.
- If eligibility is not satisfied, the system shall not approve rejoining and shall preserve the Academic Break position.
- If curriculum/credit mapping is incomplete, rejoining shall remain unresolved until valid mapping is established.
- If academic placement cannot be established, rejoining shall remain unresolved.
- Once approved, registration shall proceed through C03's own workflow.

## 7. Audit & Evidence Behavior

The system shall preserve the approved break reference, eligibility determination, curriculum and credit mapping, academic placement, rejoining approval and resulting registration eligibility.

## 8. Observable Completion Conditions

Completion requires approved break confirmed, eligibility verified, curriculum changes considered, credits mapped, academic placement established, Rejoining position established, registration eligibility restored, and `StudentRejoined` published. Registration completion is outside scope.

## 9. Traceability & Provenance

**Parent:** L3-C-035 — Student Rejoining After Academic Break  
**Primary verification source:** Authoritative Student Lifecycle L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
