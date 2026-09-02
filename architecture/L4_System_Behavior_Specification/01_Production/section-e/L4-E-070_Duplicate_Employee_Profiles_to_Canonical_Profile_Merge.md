# Duplicate Employee Profiles to Canonical Profile Merge

## 1. Specification Metadata

**Specification ID:** L4-E-070  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-070 — Duplicate Employee Profiles to Canonical Profile Merge  
**Primary Business Owner:** Employee Profile  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-070; L2-E07 — Employee Profile; L2-A02 — Person & Identity  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Resolve duplicate EmployeeProfile records representing the same employee into one canonical EmployeeProfile while preserving profile history and the separate canonical ownership of Person and Identity. This behavior does not merge Person, Identity, Workforce Engagement, Position Assignment, payroll or leave records.

## 3. Required Business Context

- At least two EmployeeProfile records exist.
- Business evidence indicates that the candidate profiles may represent the same employee.
- Person and Identity context is resolvable under Platform Foundation ownership.
- A governed canonical-profile selection can be established.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-070-BH-001 | The system shall establish the duplicate-resolution context for the candidate EmployeeProfile records. | L3-E-070 §§3, 7–9 |
| L4-E-070-BH-002 | The system shall verify from authoritative business evidence that the candidate profiles represent the same employee before any merge resolution is applied. | L3-E-070 §§7, 9, 11–12 |
| L4-E-070-BH-003 | If the duplicate relationship is not confirmed, the system shall keep the profiles independently governed and shall not merge them. | L3-E-070 §§10–11 |
| L4-E-070-BH-004 | When the duplicate relationship is confirmed, the system shall establish exactly one EmployeeProfile as canonical through the governed resolution decision. | L3-E-070 §§3, 9, 12, 16 |
| L4-E-070-BH-005 | The system shall reconcile only valid EmployeeProfile-owned information into the selected canonical profile and shall not transfer Person or Identity ownership. | L3-E-070 §§5, 9, 12–13 |
| L4-E-070-BH-006 | The system shall apply the upstream-defined resolved lifecycle treatment to the duplicate profile records without inventing a terminal state. | L3-E-070 §§9, 13, 17 |
| L4-E-070-BH-007 | The system shall preserve duplicate evidence, canonical selection, reconciled information and prior profile history so the resolution remains reconstructable. | L3-E-070 §§9, 12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-070-BH-008 | The canonical Person and Identity references shall remain owned by Platform Foundation and unchanged by the EmployeeProfile merge unless a separate Person/Identity scenario is invoked. | L3-E-070 §§5, 13–14 |
| L4-E-070-BH-009 | The system shall make the resolved canonical EmployeeProfile identity and profile facts available to dependent consumers under Employee Profile ownership. | L3-E-070 §§3–5, 14.2, 16 |
| L4-E-070-BH-010 | The system shall not invent a duplicate-profile merge event name because the Locked parent records the exact frozen merge event vocabulary as unresolved upstream. | L3-E-070 §§14.1, 17 |

## 6. Failure / Rejection / Continuation

- If duplicate evidence is insufficient, the profiles remain separate.
- If the canonical profile cannot be determined, resolution remains pending governed review.
- If conflicting profile information cannot be authoritatively resolved, no unsupported value is selected.
- If Person/Identity ownership would be changed, the profile merge stops and the issue is routed to Platform Foundation.

## 7. Audit & Evidence Behavior

The system shall preserve the duplicate evidence, verification outcome, canonical-profile decision, reconciled information, unresolved conflicts and resulting historical profile lineage.

## 8. Observable Completion Conditions

Completion requires the duplicate relationship to be confirmed, exactly one EmployeeProfile to be canonical, applicable profile-owned information to be reconciled, duplicate records to receive their upstream-defined resolved treatment, and Person/Identity ownership and historical evidence to remain intact.

## 9. Traceability & Provenance

**Parent:** L3-E-070 — Duplicate Employee Profiles to Canonical Profile Merge  
**Primary verification source:** L3-E-070 §§3, 5, 9, 11–17 and authoritative E07/A02 architecture.

## 10. Open Issues / Deferred Design

The exact duplicate-profile terminal treatment and merge event vocabulary remain upstream open issues recorded by L3-E-070. L4 does not invent them. Technical deduplication algorithms, persistence, APIs and rollback mechanics are deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
