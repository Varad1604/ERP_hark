# Progression Decision Revision After Re evaluation or Appeal

## 1. Specification Metadata

**Specification ID:** L4-C-038  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Student Lifecycle  
**Realizes Scenario:** L3-C-038 — Progression Decision Revision After Re evaluation or Appeal  
**Primary Business Owner:** Academic Progression  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-C-038; applicable locked C architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall support a fresh Academic Progression evaluation when authoritative changed academic evidence is received and approved policy permits re-evaluation. Prior progression decisions remain immutable. The behavior ends when the subsequent progression outcome is established and applicable canonical outcome events are published.

## 3. Required Business Context

- Existing progression decision.
- Authoritative changed academic evidence.
- Approved progression policy permitting fresh evaluation.
- Applicable academic credits, standing and progression criteria.
- Upstream-authorized review/appeal outcome where such an outcome exists.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-038-BH-001 | The system shall recognise authoritative changed academic evidence such as `RevaluationResultPublished` or `SupplementaryResultPublished` as eligible inputs for fresh evaluation where policy permits. | L3-C-038 §§2, 8, 14 |
| L4-C-038-BH-002 | The system shall verify that the changed evidence is authoritative and that approved policy permits fresh evaluation before establishing a revised decision. | L3-C-038 §§7–9, 12 |
| L4-C-038-BH-003 | The system shall re-evaluate applicable credits, standing and progression criteria using the authoritative changed evidence. | L3-C-038 §9 |
| L4-C-038-BH-004 | The system shall establish the subsequent progression decision without mutating the prior decision. | L3-C-038 §§3, 10–12 |
| L4-C-038-BH-005 | Where fresh evaluation reaches the same conclusion, the system shall preserve the historical decision and retain traceability to the fresh evaluation. | L3-C-038 §10 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-038-BH-006 | The system shall recognise `RevaluationResultPublished` and `SupplementaryResultPublished` as Examination-owned inputs and shall not transfer their publication ownership. | L3-C-038 §14 |
| L4-C-038-BH-007 | On completed fresh evaluation, the system shall publish the applicable canonical Academic Progression outcome, including `ProgressionEvaluated` where the evaluation completes. | L3-C-038 §14 |
| L4-C-038-BH-008 | The system shall use canonical progression event names including `StudentProgressed`, `ConditionalPromotionGranted`, `AcademicProbationAssigned`, `AcademicDetentionApplied`, and `AcademicStandingUpdated` where their corresponding outcomes occur. | L3-C-038 §14; ADR-STU-PROG-EVT-001 |

## 6. Failure / Rejection / Continuation

- If changed evidence is incomplete or not authoritative, the system shall not establish a revised decision.
- If policy does not permit fresh evaluation, the existing decision shall remain authoritative.
- If an appeal/review authority is not established by an upstream process, the system shall not invent an L4 appeal authority or event.
- Historical decisions shall remain preserved when a subsequent decision is established.

## 7. Audit & Evidence Behavior

The system shall preserve the prior decision, changed evidence, policy basis, fresh evaluation basis, subsequent outcome and applicable canonical event evidence so the revision remains attributable without rewriting history.

## 8. Observable Completion Conditions

Completion requires authoritative changed evidence received, policy permission verified, fresh evaluation completed where permitted, historical decision preserved, and the applicable subsequent outcome established and published. No Examination result is changed by this behavior.

## 9. Traceability & Provenance

**Parent:** L3-C-038 — Progression Decision Revision After Re evaluation or Appeal  
**Primary verification source:** Authoritative Student Lifecycle L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
