# Evaluated Procurement Demand to Approved Multi-Supplier Award

## 1. Specification Metadata

**Specification ID:** L4-J-010  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Procurement Planning & Sourcing  
**Realizes Scenario:** L3-J-010  
**Primary Business Owner:** Procurement Planning & Sourcing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-010_Evaluated_Procurement_Demand_to_Approved_Multi_Supplier_Award.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall convert a completed sourcing evaluation that validly permits a multi-supplier award into one approved, supplier-specific sourcing outcome whose allocations collectively reconcile to the authoritative procurement demand.

The behavior begins when the governed sourcing evaluation has identified two or more eligible suppliers and an authoritative pre-existing sourcing basis permits or requires multiple awards. It ends when the complete supplier-allocation recommendation is approved and the sourcing outcome is finalised, or when the proposal is returned, rejected, cancelled, or redirected under the parent scenario.

This specification does not create or amend procurement demand, supplier master facts, Purchase Orders, contracts, call-offs, liabilities, receipts, inventory, assets, accounting entries, or payments.

---

## 3. Required Business Context

- Authoritative J03-approved procurement demand with remaining awardable quantity or scope.
- A J01 Sourcing Event that has completed the required evaluation stages and identified at least two eligible suppliers.
- Controlled supplier responses, evaluated terms, qualification results, ranks, capacities, deviations, validity, and sourcing evidence.
- A pre-existing sourcing condition, evaluation plan, framework rule, or other authorised basis that permits multi-supplier award and defines the allocation approach sufficiently.
- Current J02 supplier eligibility, qualification, suspension, blacklisting, and relevant category facts.
- Applicable allocation, ranking, capacity, ceiling, preference, reservation, fallback, exception, and approval rules.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-010-BH-001 | When a completed evaluation identifies two or more eligible suppliers, the system shall permit a multi-supplier award recommendation only when the sourcing conditions or another authoritative pre-existing basis permit or require that award structure. | L3-J-010 §§4.1, 7, 8, 12 |
| L4-J-010-BH-002 | When the award recommendation is prepared, the system shall preserve the authoritative demand, Sourcing Event, supplier-response, and evaluation references used to derive the allocation. | L3-J-010 §§9.1, 15.2 |
| L4-J-010-BH-003 | For every proposed supplier allocation, the system shall record the applicable line, lot, quantity or share, location or phase where used, ceiling, rank or role where used, commercial basis, conditions, and source-demand relationship required by the governed allocation model. | L3-J-010 §§3, 9.7–9.8, 9.15 |
| L4-J-010-BH-004 | Before an allocation can form part of the recommendation, the system shall require that the supplier is eligible and qualified for the exact allocated scope and that the allocation conforms to applicable capacity, delivery, lot, share, ceiling, and deviation constraints. | L3-J-010 §§7, 9.5, 9.11, 12 |
| L4-J-010-BH-005 | Before approval, the system shall verify that the combined supplier allocations do not exceed the valid demand or sourcing authority, do not create unsupported overlap or duplicate allocation, and leave any unawarded balance explicit as residual demand. | L3-J-010 §§3, 9.12–9.14, 11.3–11.4, 12 |
| L4-J-010-BH-006 | When evaluated, matched, negotiated, or common award terms differ from supplier-originated terms under an authorised rule, the system shall preserve the distinction between the original supplier terms and the governed award terms. | L3-J-010 §§9.9–9.10, 11.6, 12 |
| L4-J-010-BH-007 | When a supplier cannot or will not accept a permitted allocation, the system shall allow reallocation only under the authoritative fallback rule; otherwise the affected quantity or scope shall remain explicitly residual. | L3-J-010 §§9.13, 10.8–10.10, 11.8 |
| L4-J-010-BH-008 | When a proposed allocation departs from the issued allocation, rank, share, preference, capacity, common-term, or fallback basis, the system shall prevent approval unless the parent-supported exception evidence and authority exist. | L3-J-010 §§9.16, 11.7, 12 |
| L4-J-010-BH-009 | The system shall keep unawarded, reserve, contingency, optional, and residual positions distinguishable from approved supplier commitment. | L3-J-010 §§9.14, 10.5–10.6, 12 |
| L4-J-010-BH-010 | When the Procurement Approver approves the exact complete allocation schedule, the system shall recognise the recommendation as the approved multi-supplier sourcing outcome and place the applicable Sourcing Event in `Completed`. | L3-J-010 §§4.2, 9.18–9.19, 13 |
| L4-J-010-BH-011 | After approval, the system shall make each approved supplier-specific allocation and its ceiling and demand correlation available to the authorised downstream commitment owner without creating a Purchase Order or supplier commitment in J01. | L3-J-010 §§9.20, 12; §14.2 |
| L4-J-010-BH-012 | The system shall preserve the unallocated balance and the approved allocation limits so that the J01 sourcing outcome does not authorise downstream commitment beyond the approved schedule or source demand. | L3-J-010 §§9.21, 12 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-010-BH-013 | When the governed split-award recommendation business fact is established, the system shall publish `SplitAwardRecommended` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-010 §14.1; L2-J01 §35 |
| L4-J-010-BH-014 | When the award recommendation is formally submitted, the system shall publish `AwardRecommendationSubmitted` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-010 §14.1; L2-J01 §35 |
| L4-J-010-BH-015 | When the authorised award recommendation is approved, the system shall publish `AwardRecommendationApproved` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-010 §14.1; L2-J01 §35 |
| L4-J-010-BH-016 | When the approved sourcing outcome is finalised, the system shall publish `ApprovedSourcingOutcomeFinalised` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-010 §14.1; L2-J01 §35 |

J02 supplier facts and J03 demand facts are consumed as authoritative references only; their ownership does not transfer to J01. J04 remains the owner of Purchase Order commitment.

---

## 6. Failure / Rejection / Continuation

- If no authoritative pre-existing basis permits a multi-supplier award, the system shall withhold the multi-supplier recommendation from approval.
- If an intended supplier becomes ineligible or restricted, the system shall prevent its allocation from progressing and allow only the source-supported fallback, residual, or re-sourcing disposition.
- If combined allocations exceed demand, overlap without authority, contain unresolved gaps, or violate a governing allocation rule, the system shall keep the recommendation unapproved until a source-supported correction or disposition is recorded.
- If capacity, delivery, resilience, dependency, or commercial evidence required by the allocation is missing or inconsistent, the affected allocation shall not progress to approval.
- A returned or rejected recommendation shall remain non-authorising and prior recommendation evidence shall remain reconstructable.
- An approved award shall not be destructively rewritten; later correction or cancellation shall follow the governed parent-supported path and downstream consequences remain with their canonical owners.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable the business evidence necessary to reconstruct:

- authoritative source demand and remaining awardable scope;
- Sourcing Event, responses, evaluations, supplier eligibility, qualification, and commercial evidence;
- the pre-existing multi-supplier allocation rule and allocation objective;
- each supplier-specific allocation, ceiling, rank or role, conditions, and source-demand relationship;
- original versus evaluated, matched, negotiated, or common award terms where applicable;
- residual, reserve, contingency, optional, and unawarded positions;
- exception evidence and authority where the parent permits an exception;
- recommendation, review, approval/return/rejection decision, actor, authority, reason, and resulting outcome;
- the canonical J01 events published for the award outcome.

---

## 8. Observable Completion Conditions

Successful completion is observable only when:

1. the source demand and completed evaluation are traceable;
2. at least two eligible suppliers have governed, non-overlapping supplier-specific allocations under an authorised multi-supplier basis;
3. total approved allocation reconciles to valid demand and any residual remains explicit;
4. the exact allocation schedule has received authoritative Procurement Approver approval;
5. the applicable Sourcing Event is `Completed` and the approved sourcing outcome is finalised;
6. supplier-specific allocation facts are available for authorised downstream commitment without J01 creating a Purchase Order or other downstream financial/physical outcome.

A returned, rejected, cancelled, or re-sourcing outcome is a valid terminal alternative only where the parent scenario supports it and does not constitute a successful multi-supplier award.

---

## 9. Traceability & Provenance

**Parent:** L3-J-010 — Evaluated Procurement Demand to Approved Multi-Supplier Award  
**Primary verification source:** L2-J01 — Procurement Planning & Sourcing Workflows  
**Additional verification sources:** L2-J00 — Enterprise Resource Management Architecture & Governance; L2-J02 — Vendor & Supplier Management Workflows; L2-J03 — Purchase Requisition & Approval Workflows; locked L1 Business Domain Architecture.  

Every BH above is derived from the Locked L3 parent. L2 sources were used only to verify ownership, actors, lifecycle and exact canonical event semantics.

---

## 10. Open Issues / Deferred Design

- The parent records that J01 defines no separate award-allocation lifecycle or dedicated allocation aggregate. This L4 therefore uses only the canonical Sourcing Event lifecycle and does not invent an allocation lifecycle.
- Institution-specific split, minimum/maximum share, panel, primary-secondary, rate-matching, reservation, preference and rotation rules remain upstream-governed configuration and are not hardened at L4.
- The parent lists `Section_J_Canonical_Register.md` as a dependency, but that exact file was not retrievable through the current repository search. No L4 behavior has been derived from an assumed replacement; this is retained as a repository verification finding.
- API/interface design, persistence/data model, component/service design, event transport, technical access control, transaction/retry mechanics, infrastructure, and UI/UX remain deferred to L5/L6.
