# Approved Procurement Demand to Competitive Quotation or RFQ Sourcing Outcome

## 1. Specification Metadata

**Specification ID:** L4-J-004  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Procurement Planning & Sourcing  
**Realizes Scenario:** L3-J-004  
**Primary Business Owner:** Procurement Planning & Sourcing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-004_Approved_Procurement_Demand_to_Competitive_Quotation_or_RFQ_Sourcing_Outcome.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern competitive quotation or RFQ sourcing from valid approved procurement demand through an approved sourcing outcome or governed terminal alternative.

The behavior begins when J01 receives valid approved demand for which quotation or RFQ is the governed method. It ends when the supplier recommendation is approved and the Sourcing Event reaches `Completed`, or when a supported terminal outcome such as `No Response`, `No Suitable Bid`, `Re-Sourcing Required`, or `Cancelled` is established.

The behavior shall not create a Purchase Order, supplier commitment, payable, accounting entry, receipt, inventory movement, asset, or payment.

---

## 3. Required Business Context

Required context includes:

- approved J03 procurement demand;
- sufficiently defined requirements;
- applicable quotation/RFQ method and policy;
- J02-owned supplier identity and eligibility facts;
- configured competition, evaluation, approval, and exception rules;
- applicable sourcing actors and authorities.

J01 owns the Sourcing Event, RFQ/quotation package, invitations, response evidence, evaluation, Comparative Statement, recommendation, approval, and final sourcing outcome.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-004-BH-001 | The system shall prevent ordinary quotation/RFQ sourcing unless the referenced J03 procurement demand is valid and approved. | L3-J-004 §§7-9 |
| L4-J-004-BH-002 | The system shall establish the governed quotation/RFQ method and preserve the applicable policy or approved method override. | L3-J-004 §§7-9, 12 |
| L4-J-004-BH-003 | The system shall create a controlled Sourcing Event linked to the approved demand and applicable sourcing context. | L3-J-004 §§9, 13 |
| L4-J-004-BH-004 | Before issue, the system shall preserve the exact controlled RFQ/quotation request version and required publication/invitation authority. | L3-J-004 §§9, 12 |
| L4-J-004-BH-005 | Supplier invitation eligibility shall use J02-owned supplier facts and preserve the point-in-time eligibility basis. | L3-J-004 §§5, 7, 9, 12 |
| L4-J-004-BH-006 | When the request is issued, the system shall preserve issue version, invited suppliers, issue date, deadline, issuing actor, and approved channel. | L3-J-004 §§9, 15 |
| L4-J-004-BH-007 | Each supplier response shall retain its actual supplier, Sourcing Event, applicable issued version, receipt timestamp, channel, and response version history. | L3-J-004 §§9, 12, 15 |
| L4-J-004-BH-008 | Late, withdrawn, revised, partial, duplicate, incomplete, or otherwise exceptional responses shall receive only a source-supported governed disposition. | L3-J-004 §§9-12 |
| L4-J-004-BH-009 | The system shall preserve original quoted information when any normalised comparison view is produced. | L3-J-004 §§9, 12 |
| L4-J-004-BH-010 | The system shall assess technical compliance where required and commercial comparison according to approved criteria. | L3-J-004 §§9, 12 |
| L4-J-004-BH-011 | Where required, the system shall preserve the exact evaluation inputs supporting the Comparative Statement. | L3-J-004 §§9, 12, 15 |
| L4-J-004-BH-012 | A competition shortfall shall not be silently waived; the system shall expose a governed exception or continuation path. | L3-J-004 §§9-10, 12 |
| L4-J-004-BH-013 | Lowest price shall not be treated as automatic supplier selection. | L3-J-004 §§9, 12 |
| L4-J-004-BH-014 | When policy requires, a non-lowest recommendation shall retain explicit justification. | L3-J-004 §§9, 12, 15 |
| L4-J-004-BH-015 | The system shall require final approval against the exact recommendation before the sourcing outcome becomes approved. | L3-J-004 §§6, 9, 15 |
| L4-J-004-BH-016 | An approved sourcing outcome shall remain separate from downstream supplier commitment and Purchase Order creation. | L3-J-004 §§2-4, 9, 12, 16 |

---

## 5. Cross-Domain & Event Behavior

The system shall use the exact canonical J01 sourcing events defined for this flow, including `SourcingEventCreated`, `SourcingEventPublished`, and `SupplierSubmissionsClosed` where their business conditions are satisfied.

J02 remains the canonical owner of supplier identity/status. J03 remains the owner of approved procurement demand. J04 alone owns Purchase Order creation and supplier commitment.

No sourcing event shall imply that downstream commitment already exists.

---

## 6. Failure / Rejection / Continuation

If demand authority becomes invalid, supplier eligibility changes materially, invitation delivery is uncertain, confidentiality is breached, evaluation evidence is inconsistent, competition is insufficient, or approval is interrupted, the system shall preserve the last valid sourcing position and expose the applicable governed continuation or terminal outcome.

No technical retry, rollback, portal recovery, or message-delivery mechanism is specified.

---

## 7. Audit & Evidence Behavior

The system shall preserve:

- approved-demand reference;
- sourcing method and policy basis;
- Sourcing Event identity;
- issued request versions;
- invitation population and eligibility snapshot;
- supplier response versions and timestamps;
- opening/evaluation evidence;
- Comparative Statement where required;
- recommendation and justification;
- exception decisions;
- final approval;
- terminal sourcing outcome.

---

## 8. Observable Completion Conditions

The behavior is complete when either:

1. an approved sourcing outcome exists and the Sourcing Event is `Completed`; or
2. a source-supported governed terminal alternative is established.

In both cases, supplier commitment must remain downstream and all sourcing evidence must be reconstructable.

---

## 9. Traceability & Provenance

**Parent:** L3-J-004 — Approved Procurement Demand to Competitive Quotation or RFQ Sourcing Outcome  
**Primary verification source:** L2-J01 — Procurement Planning & Sourcing Workflows  
**Additional verification:** L2-J02 Supplier Management; L2-J03 Purchase Requisition and Approval; L2-J00 governance.

---

## 10. Open Issues / Deferred Design

No material L4 blocker remains.

Portal design, APIs, persistence, scoring/comparison implementation, technical confidentiality controls, event transport, infrastructure, and UI/UX remain deferred.

---

## Canonical Validation Note

This file is a Draft and requires self-review and independent review before Architecture Approval.
