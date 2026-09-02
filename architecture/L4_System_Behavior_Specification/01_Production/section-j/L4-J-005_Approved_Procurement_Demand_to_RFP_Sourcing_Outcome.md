# Approved Procurement Demand to RFP Sourcing Outcome

## 1. Specification Metadata

**Specification ID:** L4-J-005  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Procurement Planning & Sourcing  
**Realizes Scenario:** L3-J-005  
**Primary Business Owner:** Procurement Planning & Sourcing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-005_Approved_Procurement_Demand_to_RFP_Sourcing_Outcome.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern RFP sourcing for approved demand requiring differentiated solution, capability, methodology, service, implementation, quality, and commercial evaluation.

The behavior begins when approved demand is accepted and RFP is the governed sourcing method. It ends when the exact supplier recommendation is approved and the Sourcing Event reaches `Completed`, or when a supported alternative terminal outcome is established.

The behavior shall not create a Purchase Order, contract, supplier commitment, payable, accounting entry, receipt, inventory movement, asset, or payment.

---

## 3. Required Business Context

Required context includes:

- approved J03 procurement demand;
- approved RFP method;
- pre-approved RFP scope, criteria, weights, thresholds, stages, tie treatment, and approval path;
- J02 supplier eligibility facts;
- nominated evaluators/committee members where required;
- conflict/confidentiality declarations where required.

J01 owns the RFP sourcing process and outcome. J02 owns supplier identity/status. J03 owns procurement demand.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-005-BH-001 | The system shall prevent RFP sourcing unless the referenced procurement demand is valid and approved. | L3-J-005 §§7-9 |
| L4-J-005-BH-002 | The system shall preserve RFP as the governed sourcing method and any approved method override. | L3-J-005 §§7-9, 12 |
| L4-J-005-BH-003 | Before issue, the system shall preserve the approved RFP package, evaluation stages, criteria, weights, thresholds, tie treatment, and decision rules. | L3-J-005 §§7, 9, 12 |
| L4-J-005-BH-004 | The system shall prevent criteria or weight changes after proposal visibility except through a source-supported governed restart or equivalent disposition. | L3-J-005 §§9, 11-12 |
| L4-J-005-BH-005 | Supplier participation shall use J02-owned eligibility facts and preserve the applicable point-in-time basis. | L3-J-005 §§5, 7, 9, 12 |
| L4-J-005-BH-006 | The system shall preserve issued RFP versions, amendments, recipients/publication basis, deadlines, and issue authority. | L3-J-005 §§9-10, 12, 15 |
| L4-J-005-BH-007 | Material clarification affecting proposal preparation shall be communicated consistently to affected participants. | L3-J-005 §§9-10, 12 |
| L4-J-005-BH-008 | Each proposal shall preserve supplier, Sourcing Event, applicable RFP version, actual receipt timestamp, channel, and version history. | L3-J-005 §§9, 12, 15 |
| L4-J-005-BH-009 | The system shall preserve governed dispositions for late, incomplete, withdrawn, duplicate, inaccessible, or otherwise exceptional proposals. | L3-J-005 §§9-11 |
| L4-J-005-BH-010 | Eligibility and mandatory responsiveness shall be evaluated before later stages where the governed method requires it. | L3-J-005 §§9, 12 |
| L4-J-005-BH-011 | Technical/quality evaluation shall use only the approved criteria and shall preserve scores, reasons, clarifications, demonstrations, and attributable evaluator evidence. | L3-J-005 §§9, 12, 15 |
| L4-J-005-BH-012 | Where staged segregation applies, only proposals satisfying the governed technical threshold shall proceed to commercial evaluation. | L3-J-005 §§7, 9, 12 |
| L4-J-005-BH-013 | Commercial evaluation shall preserve original prices and terms alongside any permitted evaluated or normalised view. | L3-J-005 §§9, 12 |
| L4-J-005-BH-014 | Any clarification, negotiation, presentation, demonstration, or best-and-final-offer stage shall occur only where pre-authorised and shall preserve prior proposal history. | L3-J-005 §§9-10, 12 |
| L4-J-005-BH-015 | A competition shortfall shall require an explicit governed disposition rather than silent waiver. | L3-J-005 §§9-10, 12 |
| L4-J-005-BH-016 | The system shall require final approval against the exact recommendation and evaluation evidence before the RFP outcome becomes approved. | L3-J-005 §§6, 9, 15 |
| L4-J-005-BH-017 | The approved RFP outcome shall not itself create downstream supplier commitment. | L3-J-005 §§2-4, 9, 12, 16 |

---

## 5. Cross-Domain & Event Behavior

The system shall use only exact canonical J01 sourcing events supported by L2-J01 and the parent scenario.

J02 supplier facts remain externally owned. J03 demand ownership remains unchanged. J04 or another downstream commitment capability may consume the approved sourcing outcome but does not receive ownership of the J01 sourcing history.

No event shall imply contract or Purchase Order creation.

---

## 6. Failure / Rejection / Continuation

Invalid demand authority, defective RFP/evaluation design, supplier-status change, issue failure, uncertain proposal receipt, confidentiality breach, evaluator conflict, scoring inconsistency, insufficient response, or approval interruption shall preserve the last valid sourcing position and require a governed correction, restart, return, cancellation, or re-sourcing disposition.

No technical retry or scoring-engine design is specified.

---

## 7. Audit & Evidence Behavior

The system shall preserve:

- demand reference;
- RFP method/policy basis;
- controlled RFP versions and amendments;
- approved evaluation design;
- supplier eligibility snapshot;
- proposal versions/timestamps;
- conflict/confidentiality declarations;
- eligibility/technical/commercial evaluation evidence;
- clarifications/presentations/negotiations where applicable;
- recommendation;
- approval;
- terminal outcome.

---

## 8. Observable Completion Conditions

The behavior is complete when either an approved RFP sourcing outcome exists in `Completed`, or a source-supported alternative terminal outcome is established.

The evaluation basis, proposal history, authority, and evidence must be reconstructable, and downstream commitment must remain out of scope.

---

## 9. Traceability & Provenance

**Parent:** L3-J-005 — Approved Procurement Demand to RFP Sourcing Outcome  
**Primary verification source:** L2-J01 — Procurement Planning & Sourcing Workflows  
**Additional verification:** L2-J02; L2-J03; L2-J00.

---

## 10. Open Issues / Deferred Design

No material L4 blocker remains.

Scoring-engine implementation, APIs, persistence, cryptography, portal design, technical access control, event transport, infrastructure, and UI/UX remain deferred.

---

## Canonical Validation Note

This file is a Draft and requires self-review and independent review before Architecture Approval.
