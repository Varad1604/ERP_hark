# Approved Procurement Demand to Direct or Simplified Sourcing Outcome

## 1. Specification Metadata

**Specification ID:** L4-J-007  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Procurement Planning & Sourcing  
**Realizes Scenario:** L3-J-007  
**Primary Business Owner:** Procurement Planning & Sourcing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-007_Approved_Procurement_Demand_to_Direct_or_Simplified_Sourcing_Outcome.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern direct or simplified sourcing only when valid approved procurement demand qualifies under an applicable policy ground, threshold, category, market condition, or authorised exception.

The behavior begins when J01 receives valid approved demand proposed for direct or simplified sourcing. It ends when the exact supplier, scope, evaluated commercial reference, method justification, conditions, and authority are approved, or when the request is rejected, cancelled, returned, or redirected to another governed sourcing method.

This behavior shall not create a Purchase Order, contract, liability, receipt, inventory movement, asset, accounting entry, or payment.

---

## 3. Required Business Context

Required context includes:

- valid approved J03 procurement demand;
- applicable direct/simplified policy basis;
- aggregate value and related-demand context where required;
- J02-owned supplier identity and eligibility facts;
- required conflict, related-party, exclusivity, compatibility, standardisation, continuity, urgency, or market-availability evidence;
- appropriate commercial and price-reasonableness evidence;
- configured exception and final approval authority.

J01 owns sourcing-method qualification and the sourcing outcome. J02 retains Supplier ownership. J03 retains demand ownership.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-007-BH-001 | The system shall prevent direct/simplified sourcing unless the referenced J03 demand is valid and approved. | L3-J-007 §§7-9 |
| L4-J-007-BH-002 | The system shall preserve the exact policy ground, threshold, exception basis, supporting facts, anticipated value, and required authority for the proposed method. | L3-J-007 §§8-9, 12 |
| L4-J-007-BH-003 | Where policy requires aggregation, the system shall consider related demand, cumulative value, recent similar purchases, and split indicators before method approval. | L3-J-007 §§9, 12 |
| L4-J-007-BH-004 | If the method basis is invalid, the system shall prevent direct/simplified approval and preserve the governed redirection or rejection outcome. | L3-J-007 §§9-11 |
| L4-J-007-BH-005 | The system shall create a controlled Sourcing Event linked to the approved demand, selected method, justification, policy, and proposed supplier context. | L3-J-007 §§9, 13 |
| L4-J-007-BH-006 | Supplier eligibility shall use J02-owned point-in-time supplier facts without transferring Supplier ownership to J01. | L3-J-007 §§5, 7, 9, 12 |
| L4-J-007-BH-007 | Where applicable, the system shall preserve conflict-of-interest, related-party, exclusivity, compatibility, standardisation, continuity, urgency, or market-availability evidence. | L3-J-007 §§7, 9, 12, 15 |
| L4-J-007-BH-008 | The system shall preserve the original quotation, catalogue, rate, estimate, proposal, or other commercial evidence and its version history. | L3-J-007 §§9, 12 |
| L4-J-007-BH-009 | The system shall assess price and commercial reasonableness using source-supported evidence appropriate to the governed method. | L3-J-007 §§9, 12 |
| L4-J-007-BH-010 | Any evaluated, normalised, or negotiated view shall remain distinguishable from supplier-originated commercial facts. | L3-J-007 §§9, 12 |
| L4-J-007-BH-011 | If negotiation is permitted, the system shall preserve pre-negotiation offer, authorised subjects, participants, supplier response, final offer, and decision basis. | L3-J-007 §§9-10, 12, 15 |
| L4-J-007-BH-012 | The system shall prevent approval when price reasonableness or required commercial evidence is not sufficiently established. | L3-J-007 §11 |
| L4-J-007-BH-013 | The system shall require any method-exception authority before the direct/simplified route may be used where policy requires it. | L3-J-007 §§9, 12, 15 |
| L4-J-007-BH-014 | The system shall require final approval against the exact recommendation before the sourcing outcome becomes approved. | L3-J-007 §§6, 9, 15 |
| L4-J-007-BH-015 | The approved sourcing outcome shall remain separate from downstream supplier commitment. | L3-J-007 §§2-4, 9, 12, 16 |

---

## 5. Cross-Domain & Event Behavior

The system shall publish only exact J01 sourcing events supported by the canonical event catalogue and the parent scenario.

J02 remains the canonical owner of Supplier identity, lifecycle, qualification, restriction, suspension, blacklisting, and closure. J03 remains the canonical owner of approved procurement demand. J04 alone owns Purchase Order supplier commitment.

No sourcing event shall imply a Purchase Order or financial obligation exists.

---

## 6. Failure / Rejection / Continuation

Invalid demand authority, invalid method basis, suspected fragmentation, supplier ineligibility/conflict, unreliable commercial evidence, unestablished price reasonableness, supplier withdrawal, or interrupted approval shall preserve the last valid sourcing position and require the applicable governed correction, redirection, rejection, cancellation, or re-sourcing path.

No technical retry, calculation-engine behavior, or rollback is specified.

---

## 7. Audit & Evidence Behavior

The system shall preserve:

- approved-demand reference;
- method classification and policy basis;
- aggregation/anti-fragmentation evidence;
- supplier eligibility snapshot;
- conflict and exception evidence;
- original commercial evidence and versions;
- price-reasonableness basis;
- negotiation evidence where applicable;
- recommendation;
- exception approval;
- final decision and authority;
- terminal sourcing outcome.

---

## 8. Observable Completion Conditions

The behavior is complete when an approved direct/simplified sourcing outcome exists, or an explicit rejected, cancelled, returned, or redirected disposition is established.

The outcome must be fully traceable and must not itself create downstream supplier commitment.

---

## 9. Traceability & Provenance

**Parent:** L3-J-007 — Approved Procurement Demand to Direct or Simplified Sourcing Outcome  
**Primary verification source:** L2-J01 — Procurement Planning & Sourcing Workflows  
**Additional verification:** L2-J02 — Vendor & Supplier Management; L2-J03 — Purchase Requisition & Approval; L2-J00 governance.

---

## 10. Open Issues / Deferred Design

No material L4 blocker remains.

API/interface design, persistence, aggregation detection algorithms, calculation implementation, event transport, infrastructure, and UI/UX remain deferred.

---

## Canonical Validation Note

This file is a Draft and requires self-review and independent review before Architecture Approval.
