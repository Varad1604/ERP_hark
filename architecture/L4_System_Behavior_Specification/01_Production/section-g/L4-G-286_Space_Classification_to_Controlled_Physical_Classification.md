# Space Classification to Controlled Physical Classification

## 1. Specification Metadata

**Specification ID:** L4-G-286  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Infrastructure Management  
**Realizes Scenario:** L3-G-286  
**Primary Business Owner:** Infrastructure Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-G-286_Space_Classification_to_Controlled_Physical_Classification.md`; L2-G10 — Infrastructure Management Workflows; L2-A01 — Platform & Tenant Foundation; `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md`  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP shall govern application or change of a G10-owned primary Space Type using valid active configuration while preserving Space identity, hierarchy, lifecycle and G10 semantic ownership. It shall not resolve the parent's ambiguity between initial classification and reclassification, nor select between conflicting classification-event names.

## 3. Required Business Context

- A canonical Space exists under G10.
- A permitted Space Type is available from authoritative G10-owned configuration through an Active resolved configuration context.
- Infrastructure Administrator is authorized to perform the classification action.
- Space Type is mandatory during Space registration, so existence of a valid unclassified Space is not assumed.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-286-BH-001 | The system shall resolve the canonical Space and valid Active configuration context before evaluating a classification request. | L3-G-286 §§7,9,12 |
| L4-G-286-BH-002 | The system shall reject a requested Space Type that is absent from authoritative G10 configuration, invalid for the Space, or outside the actor's authorized scope. | L3-G-286 §§7,9,11–12 |
| L4-G-286-BH-003 | When a source-supported classification change on an existing Space is valid, the system shall change the primary Space Type while preserving exactly one primary classification, Space identity, physical hierarchy and lifecycle. | L3-G-286 §§2–5,9,12–13 |
| L4-G-286-BH-004 | The system shall expose the resulting Space Type through the authoritative G10 Space fact without transferring classification semantics or mutation authority to consuming domains or Platform Foundation. | L3-G-286 §§5,9,12,14,16 |
| L4-G-286-BH-005 | The system shall not create an unclassified-Space lifecycle or initial-classification path to reconcile the conflict between mandatory registration Space Type and this parent's trigger. | L3-G-286 §§7,17 |

## 5. Cross-Domain & Event Behavior

The exact classification event remains contradictory: `InfrastructureClassificationChanged` versus `SpaceClassificationChanged`. The later G10 completion amendment does not resolve this pair, so no event is selected or aliased.

## 6. Failure / Rejection / Continuation

Invalid/inactive configuration, unsupported Space Type, failed authorization or unresolved initial-classification semantics shall leave the existing authoritative Space fact unchanged. Where classification succeeds but the canonical event identity is unresolved, the full frozen completion cannot be certified.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-286-BH-006 | The system shall preserve the Space identity, prior/resulting primary Space Type where a valid change occurs, configuration context, actor/authorization evidence and the unresolved classification-event identity without altering Space lifecycle history. | L3-G-286 §§15–17 |

## 8. Observable Completion Conditions

The controlled classification fact is observable when one authorized primary Space Type is present on the canonical Space with identity/hierarchy/lifecycle unchanged. Full scenario completion remains blocked until upstream clarifies initial classification versus reclassification and resolves the exact canonical classification event.

## 9. Traceability & Provenance

**Parent:** L3-G-286 — Space Classification to Controlled Physical Classification  
**Primary verification sources:** L2-G10 — Infrastructure Management Workflows; L2-A01 — Platform & Tenant Foundation  
**Current-source check:** G10 Governance Completion Amendment v1.2.0 does not normalize the classification-event conflict.

## 10. Open Issues / Deferred Design

The parent’s sequencing ambiguity (Space Type mandatory at registration versus an existing Space requiring classification) and `InfrastructureClassificationChanged`/`SpaceClassificationChanged` event conflict remain unresolved. No new state, event or configuration ownership is introduced.
