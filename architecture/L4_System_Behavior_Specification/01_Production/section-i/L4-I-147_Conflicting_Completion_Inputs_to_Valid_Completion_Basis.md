# Conflicting Completion Inputs to Valid Completion Basis

## 1. Specification Metadata

**Specification ID:** L4-I-147  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination  
**Realizes Scenario:** L3-I-147 — Conflicting Completion Inputs to Valid Completion Basis  
**Primary Business Owner:** Academic Completion & Completion Evidence  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-03  
**Depends On:** L3-I-147; applicable Academic Completion & Completion Evidence architecture; authoritative Student Lifecycle, Academic Delivery and Examination source workflows  
**Referenced By:** Downstream completion evaluation/evidence workflows

## 2. Behavioral Objective & Boundary

Govern conflicting completion inputs so that one valid completion basis is assembled only from authoritative current source facts. The completion owner coordinates the outcome but does not arbitrate or transfer ownership of Student Lifecycle, Academic Delivery or Examination facts.

## 3. Required Business Context

- Affected student/programme/completion scope.
- Materially conflicting source facts and their canonical owners.
- Applicable completion-evidence rules.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-147-BH-001 | Identify each conflicting input, its provenance and canonical owner. | L3-I-147 §§3,7–9 |
| L4-I-147-BH-002 | Distinguish historical/derived differences from genuinely conflicting current facts. | L3-I-147 §§9–10,12 |
| L4-I-147-BH-003 | Route source-owned conflicts to their canonical owners rather than applying a local precedence rule. | L3-I-147 §§5,9,11–12 |
| L4-I-147-BH-004 | Reassemble the completion basis only after authoritative source positions are established. | L3-I-147 §§3,9,12,16 |
| L4-I-147-BH-005 | Prevent unresolved conflicting mandatory inputs from producing an authoritative completion position. | L3-I-147 §§11–12,16 |
| L4-I-147-BH-006 | Preserve the conflicting positions, resolution history and resulting completion basis. | L3-I-147 §§3,9,15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-147-BH-007 | Student Lifecycle, Academic Delivery and Examination shall remain publishers of their own authoritative facts. Academic Completion & Completion Evidence shall make the resulting valid completion basis/position available through its authoritative workflow. No `CompletionInputsReconciled` event is introduced because the parent does not establish one. | L3-I-147 §14 |

## 6. Failure / Rejection / Continuation

- If no canonical source can resolve a material conflict, completion remains unresolved/blocked under upstream governance.
- Simultaneously conflicting authoritative facts shall be escalated; no local compromise is invented.
- A later source correction triggers governed re-evaluation with prior decisions preserved.

## 7. Audit & Evidence Behavior

Preserve source provenance, conflict classification, owner resolution, decision basis, prior positions and resulting completion basis.

## 8. Observable Completion Conditions

Completion occurs when every material conflict has an authoritative source resolution, one valid completion basis is assembled, and competing source positions are not treated as current truth.

## 9. Traceability & Provenance

**Parent:** L3-I-147  
**Primary verification sources:** Parent §§3,5,9,11–14,16–17 and applicable source/completion workflows.

## 10. Open Issues / Deferred Design

The parent does not define a local precedence hierarchy or a dedicated completion-input reconciliation event. Exact completion lifecycle labels remain upstream-owned.

## Canonical Validation Note

This specification preserves one canonical owner per source fact and prohibits local conflict arbitration.
