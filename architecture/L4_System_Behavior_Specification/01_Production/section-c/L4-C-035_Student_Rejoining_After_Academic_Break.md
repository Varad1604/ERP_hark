# Student Rejoining After Academic Break

## 1. Specification Metadata

**Specification ID:** L4-C-035  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Academic Progression  
**Realizes Scenario:** L3-C-035 — Student Rejoining After Academic Break  
**Primary Business Owner:** Academic Progression  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-08-31  
**Depends On:** L3-C-035; L2-C04 Student Academic Progression Workflows; L2-C03 Student Academic Registration Workflows; L2-C02 Student Lifecycle and Activation  
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
| L4-C-035-BH-003 | The system shall support mapping preserved academic credits to the current curriculum before rejoining approval. | L3-C-035 §§2, 9 |
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
**Primary verification source:** L2-C04 — Student Academic Progression Workflows.  
**Cross-domain verification:** L2-C03 for subsequent registration ownership.

## 10. Open Issues / Deferred Design

Exact rejoining eligibility, semester-placement rules and detailed registration hand-off remain institution-specific/upstream or downstream matters. Technical interfaces, persistence, transport, retry and IAM design are deferred.

## Canonical Validation Note

This specification is a Draft L4 artifact and is not Architecture Approved or Locked.
