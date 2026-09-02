# Split Approved Demand to Traceable Sourcing Outcomes

## 1. Specification Metadata

**Specification ID:** L4-J-009  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Procurement Planning & Sourcing  
**Realizes Scenario:** L3-J-009  
**Primary Business Owner:** Procurement Planning & Sourcing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-009_Split_Approved_Demand_to_Traceable_Sourcing_Outcomes.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall divide one approved procurement demand into multiple controlled sourcing components only for legitimate sourcing reasons while preserving aggregate controls, source-demand authority, quantity conservation, and complete component-to-outcome lineage.

The behavior begins when a valid approved demand requires more than one sourcing package, method, lot, timing window, supplier market, or outcome before commitment. It ends when every approved component has a traceable sourcing outcome or explicit residual/cancelled/returned/deferred disposition and the complete set reconciles to the source demand.

This behavior shall not create new independent demand authority, evade competition or approval thresholds, or create supplier commitment.

---

## 3. Required Business Context

Required context includes:

- one valid approved J03 demand;
- positive remaining unsourced quantity or scope;
- source lines, units, specifications, delivery, locations, category, value, ownership, and funding context;
- legitimate split rationale;
- aggregate and related-demand rules;
- component boundaries capable of reconciling to the source;
- method-specific sourcing governance for each component.

J03 retains authority over the original demand. J01 owns split eligibility, sourcing-component relationships, reconciliation, residual treatment, and outcome set.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-009-BH-001 | The system shall prevent split sourcing unless the source J03 demand is valid, approved, and has remaining unsourced quantity or scope. | L3-J-009 §§7-9 |
| L4-J-009-BH-002 | The system shall preserve the source demand identity, lines, remaining quantity/scope, approval, value, funding, delivery, and existing sourcing/commitment correlations. | L3-J-009 §§7, 9, 15 |
| L4-J-009-BH-003 | The system shall record the legitimate business rationale for splitting before components are approved. | L3-J-009 §§8-9, 12 |
| L4-J-009-BH-004 | Aggregate and related-demand value shall govern method and authority where policy requires; isolated component value shall not be used to evade controls. | L3-J-009 §§9, 12 |
| L4-J-009-BH-005 | A split that appears intended to avoid competition, publication, tender, committee, value, or approval requirements shall be rejected or escalated. | L3-J-009 §§9, 11-12 |
| L4-J-009-BH-006 | Each approved sourcing component shall have a unique identity and direct lineage to source demand lines, quantities, or scope. | L3-J-009 §§9, 12 |
| L4-J-009-BH-007 | Component boundaries shall be checked for overlap, gaps, invalid conversion, and complete reconciliation to the source demand. | L3-J-009 §§9, 11-12 |
| L4-J-009-BH-008 | The system shall require governed approval of the controlled split plan before independent component sourcing begins. | L3-J-009 §§9, 15 |
| L4-J-009-BH-009 | Each component shall use the applicable method-specific sourcing scenario without redefining that method at J-009. | L3-J-009 §§4.4, 9, 12 |
| L4-J-009-BH-010 | Supplier requests, submissions, evaluations, recommendations, and outcomes shall remain attributable to the correct component. | L3-J-009 §§9, 12 |
| L4-J-009-BH-011 | A component change, withdrawal, cancellation, or failure shall trigger governed assessment of source balance, aggregate method, fairness, dependencies, and other components. | L3-J-009 §§9-11 |
| L4-J-009-BH-012 | For each component, the system shall preserve awarded quantity/scope, residual position, conditions, and sourcing-outcome reference. | L3-J-009 §§9, 16 |
| L4-J-009-BH-013 | The system shall prevent overlapping sourcing or commitment of the same source-demand portion. | L3-J-009 §§11-12, 16 |
| L4-J-009-BH-014 | Total awarded and residual positions shall reconcile to the valid source demand and shall never exceed approved remaining demand. | L3-J-009 §§11-12, 16 |
| L4-J-009-BH-015 | Failure or cancellation of one component shall not imply fulfilment or cancellation of other components. | L3-J-009 §§10-12 |
| L4-J-009-BH-016 | The system shall require approval of the complete component outcome set and reconciliation before the overall split sourcing outcome is complete. | L3-J-009 §§9, 15-16 |
| L4-J-009-BH-017 | J01 shall publish correlations to J03 without changing the source demand lifecycle or approval truth. | L3-J-009 §§5, 9, 12, 16 |

---

## 5. Cross-Domain & Event Behavior

When their canonical conditions are satisfied, J01 may publish:

- `SourcingEventCreated`
- `ApprovedSourcingOutcomeFinalised`
- `ReSourcingRequested`

The parent explicitly states that no separately defined canonical split-record lifecycle exists in J01; this L4 does not create one.

J03 remains the canonical owner of the approved source demand and its lifecycle.

---

## 6. Failure / Rejection / Continuation

Invalid source demand, improper fragmentation, component overlap/imbalance, duplicate sourcing/commitment, invalid component method, dependency failure, funding/authority conflict, reconciliation failure, or interrupted approval shall prevent overall completion.

The source demand remains authoritative while the split relationship is corrected, recombined, cancelled, or otherwise governed.

No technical allocation, concurrency, retry, or rollback mechanism is specified.

---

## 7. Audit & Evidence Behavior

The system shall preserve:

- source demand and approval;
- split rationale;
- aggregate-control assessment;
- split-plan versions;
- component identities and mappings;
- method/authority per component;
- component changes and dependencies;
- method-specific outcomes;
- awarded and residual positions;
- reconciliation evidence;
- overall recommendation and approval;
- source-to-component and component-to-outcome traceability.

---

## 8. Observable Completion Conditions

The behavior is complete when every approved component has a governed sourcing outcome or explicit residual/cancelled/returned/deferred position and the full result reconciles exactly to the source demand without duplicate sourcing, over-award, hidden residual, or control evasion.

---

## 9. Traceability & Provenance

**Parent:** L3-J-009 — Split Approved Demand to Traceable Sourcing Outcomes  
**Primary verification source:** L2-J01 — Procurement Planning & Sourcing Workflows  
**Demand source:** L2-J03 — Purchase Requisition & Approval  
**Governance source:** L2-J00.

---

## 10. Open Issues / Deferred Design

The parent retains open issues around institution-specific aggregate-value periods, exact J03 remaining-unsourced-quantity contract, split-vs-lot-vs-multi-supplier boundaries, and completion semantics for deferred/disputed/cancelled components. This L4 does not resolve them.

No separate canonical split-plan/component lifecycle is defined in J01; none is invented here.

Allocation, reconciliation, unit-conversion, rounding, duplicate/fragmentation detection, concurrency, API, persistence, event-payload, and UI implementation remain deferred.

---

## Canonical Validation Note

This file is a Draft and requires self-review and independent review before Architecture Approval.
