# Approved Procurement Demand to Governed Tender Sourcing Outcome

## 1. Specification Metadata

**Specification ID:** L4-J-006  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Procurement Planning & Sourcing  
**Realizes Scenario:** L3-J-006  
**Primary Business Owner:** Procurement Planning & Sourcing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-006_Approved_Procurement_Demand_to_Governed_Tender_Sourcing_Outcome.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern formal tender sourcing from valid approved procurement demand through an authorised, auditable sourcing outcome or governed terminal alternative.

The behavior begins when approved demand requires a formal tender method. It ends when the tender award recommendation is approved and the Sourcing Event reaches `Completed`, or when a supported terminal outcome such as `No Response`, `No Suitable Bid`, `Re-Sourcing Required`, or `Cancelled` is established.

The behavior shall not create a Purchase Order, letter of acceptance, contract, liability, receipt, inventory movement, asset, or payment.

---

## 3. Required Business Context

Required context includes:

- approved J03 demand;
- applicable tender method and policy basis;
- approved tender conditions and publication route;
- defined eligibility, qualification, responsiveness, opening, technical, financial, evaluation, tie, security, and approval rules;
- J02 supplier facts where applicable;
- configured committee and authority requirements.

J01 owns the Tender Sourcing Event, tender package, publication/invitation, bid evidence, opening, evaluation, recommendation, approval, and final sourcing outcome.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-006-BH-001 | The system shall prevent tender initiation unless the referenced demand is valid and approved. | L3-J-006 §§7-9 |
| L4-J-006-BH-002 | The system shall preserve the governed tender form, policy basis, and any authorised method exception. | L3-J-006 §§7-9, 12 |
| L4-J-006-BH-003 | Before publication, the system shall preserve the exact approved tender package and published eligibility, evaluation, opening, tie, security, and award rules. | L3-J-006 §§7, 9, 12 |
| L4-J-006-BH-004 | The system shall prevent material post-visibility rule changes except through an authorised amendment with fair continuation or governed restart. | L3-J-006 §§9-12 |
| L4-J-006-BH-005 | The system shall preserve publication/invitation route, notice period, issue version, issue authority, deadline, and affected supplier population where applicable. | L3-J-006 §§9-10, 12, 15 |
| L4-J-006-BH-006 | Material pre-bid clarification, corrigendum, addendum, or extension shall preserve prior versions, authority, recipients, and deadline impact. | L3-J-006 §§9-10, 12 |
| L4-J-006-BH-007 | Each bid shall preserve bidder, Sourcing Event, lot where applicable, tender version, actual receipt timestamp, channel, and submission lineage. | L3-J-006 §§9, 12, 15 |
| L4-J-006-BH-008 | Bid-security evidence may be associated with the bid without transferring monetary-processing ownership to J01. | L3-J-006 §§4.3-4.4, 9 |
| L4-J-006-BH-009 | Withdrawal, substitution, or modification shall be governed only within published rules and shall preserve prior bid history. | L3-J-006 §§9-10, 12 |
| L4-J-006-BH-010 | Bid opening shall occur only under the applicable authorised timing, stage, quorum, confidentiality, and record requirements. | L3-J-006 §§7, 9, 12 |
| L4-J-006-BH-011 | Every preliminary rejection or disqualification shall retain the governing rule, fact, evidence, evaluator, and authority. | L3-J-006 §§9, 12, 15 |
| L4-J-006-BH-012 | Technical evaluation shall use only the published criteria and preserve attributable scores, deviations, clarifications, and reasons. | L3-J-006 §§9, 12 |
| L4-J-006-BH-013 | Where staged/two-envelope governance applies, only technically qualified bids shall proceed to financial opening. | L3-J-006 §§9, 12 |
| L4-J-006-BH-014 | Financial evaluation shall preserve original bid values while recording only permitted corrections, evaluated views, ranking, and deviations. | L3-J-006 §§9, 12 |
| L4-J-006-BH-015 | Competition shortfall or exceptional commercial result shall require an explicit governed disposition. | L3-J-006 §§9-10, 12 |
| L4-J-006-BH-016 | Negotiation, auction, or best-and-final-offer behavior shall occur only where the tender rules and policy explicitly permit it. | L3-J-006 §§9-10, 12 |
| L4-J-006-BH-017 | The system shall require approval against the exact award recommendation and supporting tender evidence before the sourcing outcome becomes approved. | L3-J-006 §§6, 9, 15 |
| L4-J-006-BH-018 | The approved tender outcome shall remain separate from downstream Purchase Order, letter-of-acceptance, or contract commitment. | L3-J-006 §§2-4, 9, 12, 16 |

---

## 5. Cross-Domain & Event Behavior

Only canonical J01 tender/sourcing events supported upstream shall be published.

J02 remains the owner of supplier identity and supplier lifecycle. J03 remains the owner of procurement demand. Finance/Treasury retain ownership of monetary processing. J04 or another downstream commitment capability may consume the approved sourcing outcome but shall not acquire ownership of tender history.

---

## 6. Failure / Rejection / Continuation

Invalid demand, defective tender conditions, publication/communication failure, late or uncertain bid, confidentiality breach, committee conflict/quorum failure, evaluation inconsistency, bid-security issue, competition shortfall, or interrupted approval shall preserve the last valid sourcing position and require a governed continuation or terminal disposition.

No technical portal recovery, retry, rollback, cryptographic design, or queue behavior is specified.

---

## 7. Audit & Evidence Behavior

The system shall preserve:

- approved-demand reference;
- tender method and policy basis;
- tender package and all amendments;
- publication/invitation evidence;
- bidder/bid versions and timestamps;
- opening records;
- committee/conflict/quorum evidence;
- eligibility, technical, and financial evaluations;
- competition/exception decisions;
- recommendation;
- final approval;
- terminal sourcing outcome.

---

## 8. Observable Completion Conditions

The behavior is complete when either:

1. an approved tender sourcing outcome exists and the Sourcing Event is `Completed`; or
2. a source-supported terminal alternative is established.

All tender evidence, authority, bidder history, and evaluation basis must remain reconstructable, and supplier commitment must remain downstream.

---

## 9. Traceability & Provenance

**Parent:** L3-J-006 — Approved Procurement Demand to Governed Tender Sourcing Outcome  
**Primary verification source:** L2-J01 — Procurement Planning & Sourcing Workflows  
**Additional verification:** L2-J02; L2-J03; L2-J00.

---

## 10. Open Issues / Deferred Design

No material L4 blocker remains.

E-procurement portal design, APIs, digital signature/cryptography, persistence, scoring implementation, event transport, infrastructure, and UI/UX remain deferred.

---

## Canonical Validation Note

This file is a Draft and requires self-review and independent review before Architecture Approval.
