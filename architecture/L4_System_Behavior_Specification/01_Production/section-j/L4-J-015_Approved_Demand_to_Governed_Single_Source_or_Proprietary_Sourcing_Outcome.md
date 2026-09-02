# Approved Demand to Governed Single-Source or Proprietary Sourcing Outcome

## 1. Specification Metadata

**Specification ID:** L4-J-015  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Procurement Planning & Sourcing  
**Realizes Scenario:** L3-J-015  
**Primary Business Owner:** Procurement Planning & Sourcing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-015_Approved_Demand_to_Governed_Single_Source_or_Proprietary_Sourcing_Outcome.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern exceptional single-source or proprietary sourcing for approved procurement demand by requiring an authorised exceptional basis, independent scrutiny, supplier eligibility, alternatives assessment, commercial reasonableness, conflict evidence, and attributable approval before producing an approved sourcing outcome.

The behavior begins when approved, available demand is proposed for single-source or proprietary sourcing with an identifiable supplier and documented exceptional basis. It ends when the proposal is approved, returned, or rejected and the governed result is available to the appropriate downstream commitment or alternate sourcing path.

This specification does not approve or alter J03-owned demand, change J02 supplier lifecycle, create a Purchase Order or contract, execute emergency procurement, or create receipt, payable, accounting, or payment outcomes.

---

## 3. Required Business Context

- J03-approved, identifiable, current, uncommitted procurement demand with available quantity/scope.
- Defined requirement scope, specification, quantity, location, timing, estimated value, and funding context.
- An identifiable proposed supplier and documented claimed exceptional basis.
- Institution-governed single-source/proprietary grounds, competition thresholds, aggregation, tenure, disclosure, delegation, and review rules.
- Authoritative J02 supplier identity, eligibility, beneficial-ownership/related-party, restriction, capacity, and performance facts where applicable.
- Evidence sufficient to assess feasible alternatives and commercial reasonableness.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-015-BH-001 | When approved demand is proposed for single-source or proprietary sourcing, the system shall preserve the demand reference, proposed supplier, claimed exceptional ground, initiating actor, submission time, and supporting evidence. | L3-J-015 §§4.1, 8, 9.1 |
| L4-J-015-BH-002 | Before exceptional sourcing can progress, the system shall require confirmation that the demand remains approved, current, available for sourcing, and not already consumed by another sourcing outcome except for an authorised replacement or residual quantity. | L3-J-015 §§7, 9.2, 11.4, 12 |
| L4-J-015-BH-003 | The system shall permit the exceptional sourcing recommendation only when the claimed basis corresponds to an institutionally permitted single-source or proprietary ground supported by evidence. | L3-J-015 §§2, 7, 9.3, 12 |
| L4-J-015-BH-004 | The system shall preserve the requester certification of factual need separately from the independent review of supplier bias, substitutes, interoperability, migration, alternative standards, and other feasible alternatives required by the parent. | L3-J-015 §§9.4–9.5, 12 |
| L4-J-015-BH-005 | The system shall preserve the proportionate market-check and alternatives evidence used to determine why other identified sources or solutions cannot satisfy the governed requirement. | L3-J-015 §§9.6, 10.3, 15.2 |
| L4-J-015-BH-006 | The system shall make related/cumulative demand, prior awards, renewals/extensions, and supplier-concentration evidence available to the exceptional-sourcing assessment so that fragmented demand or recurring exceptions remain visible. | L3-J-015 §§9.7, 11.4, 12 |
| L4-J-015-BH-007 | Before approval, the system shall require authoritative J02 supplier identity and eligibility facts and shall prevent approval while a source-supported supplier restriction or ineligibility blocks participation. | L3-J-015 §§7, 9.8, 11.2, 12 |
| L4-J-015-BH-008 | The system shall preserve conflict, beneficial-ownership, related-party, and prior-relationship disclosures required by the parent and shall prevent the affected decision from progressing until the source-supported governance disposition exists. | L3-J-015 §§9.9, 11.3, 12 |
| L4-J-015-BH-009 | The system shall require commercial-reasonableness evidence independent of the technical exclusivity claim and shall keep the recommendation unapproved when the parent-required price or commercial basis is not defensible. | L3-J-015 §§9.10, 10.5, 11.5, 12 |
| L4-J-015-BH-010 | The system shall preserve the specialist technical, legal, compliance, finance, and independent-review facts required by the parent without treating those reviews as substitutes for the Procurement Approver's decision. | L3-J-015 §§9.11, 9.13–9.14, 15.1 |
| L4-J-015-BH-011 | The recommendation shall identify the exact supplier, demand scope, quantity, value basis, exceptional ground, evidence, conditions, validity, limitations, demand consumption, and downstream path supported by the parent. | L3-J-015 §§3, 9.12, 12 |
| L4-J-015-BH-012 | When the Procurement Approver authorises the exact recommendation, the system shall recognise an approved single-source or proprietary sourcing outcome only within the verified supplier, demand, scope, quantity, value basis, validity period, and conditions. | L3-J-015 §§3, 9.14–9.15, 12, 16 |
| L4-J-015-BH-013 | When the proposal is returned or rejected, the system shall preserve the demand reference and permit only a parent-supported correction or alternate competitive, simplified, consolidated, or split sourcing route. | L3-J-015 §§3, 4.2, 9.16, 10.6, 12 |
| L4-J-015-BH-014 | An approved exceptional sourcing outcome shall not itself create a Purchase Order, contract, receipt, payable, accounting entry, or payment. | L3-J-015 §§3, 4.4, 12, 16 |
| L4-J-015-BH-015 | When a material post-approval change affects supplier, scope, price, quantity, exceptional ground, rights, specification, or risk, the system shall prevent silent continuation under the original outcome and require the parent-supported reassessment path. | L3-J-015 §11.7 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-015-BH-016 | When the exceptional single-source/proprietary sourcing basis is formally declared, the system shall publish `SingleSourceProcurementDeclared` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-015 §14.1; L2-J01 §35.11 |
| L4-J-015-BH-017 | When the authorised exceptional sourcing decision is approved, the system shall publish `SingleSourceProcurementApproved` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-015 §14.1; L2-J01 §35.11 |
| L4-J-015-BH-018 | When the approved sourcing outcome is finalised for downstream use, the system shall publish `ApprovedSourcingOutcomeFinalised` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-015 §14.1; L2-J01 §35 |

J03/requesting units retain approved-demand ownership, J02 retains supplier lifecycle, J04 retains commitment truth, Finance retains financial truth, and specialist legal/compliance authority remains outside J01 ownership.

---

## 6. Failure / Rejection / Continuation

- Unsupported exclusivity or proprietary evidence shall result in return or rejection; no approved exceptional sourcing outcome shall be issued.
- Supplier ineligibility or restriction shall block approval until an authoritative source-supported resolution or alternate sourcing disposition exists.
- A conflict or related-party concern shall prevent the affected decision from progressing until the parent-supported recusal, independent review, escalation, or rejection disposition is established.
- Duplicate or fragmented demand shall remain unapproved until related demand and prior sourcing outcomes are reconciled.
- Commercial unreasonableness shall prevent approval until the parent-supported negotiation, further analysis, scope adjustment, alternate-solution review, or rejection outcome occurs.
- Interrupted approval or evidence review shall leave the sourcing case non-approved with prior valid evidence preserved.
- A material post-approval change shall not silently alter the approved outcome; reassessment, cancellation, amendment, or a new sourcing outcome shall follow the parent-supported path.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- demand approval, specification, quantity, value, dates, funding context, aggregation, and remaining availability;
- exceptional ground, requester certification, rights/proprietary evidence, compatibility/continuity analysis where applicable;
- market search, alternative/substitute assessment, prior sourcing history, cumulative demand/spend, and lock-in evidence;
- supplier identity, eligibility, ownership/related-party facts, restrictions, capacity, performance, and declarations;
- benchmarks, independent estimate, cost/commercial analysis, negotiation evidence, lifecycle-cost evidence, and commercial terms used for reasonableness;
- conflict declarations, specialist reviews, recommendation, approval/return/rejection decision, conditions, actors, reasons, versions, and timestamps;
- `SingleSourceProcurementDeclared`, `SingleSourceProcurementApproved`, and `ApprovedSourcingOutcomeFinalised` facts where applicable.

---

## 8. Observable Completion Conditions

Successful completion is observable only when:

1. approved demand remains traceable and available for the governed exceptional sourcing quantity/scope;
2. the exceptional ground is authorised, documented, independently tested, and supported by alternatives/market evidence;
3. the exact supplier is eligible and conflict/related-party evidence has the required governed disposition;
4. commercial reasonableness is evidenced independently of technical exclusivity;
5. the Procurement Approver has approved the exact supplier/scope/quantity/value basis/conditions/validity outcome;
6. the approved sourcing outcome is finalised and available for separately governed downstream commitment;
7. no Purchase Order, contract, receipt, payable, accounting entry, or payment is created by this scenario alone.

A returned or rejected proposal is a valid parent-supported terminal outcome but does not constitute successful exceptional sourcing.

---

## 9. Traceability & Provenance

**Parent:** L3-J-015 — Approved Demand to Governed Single-Source or Proprietary Sourcing Outcome  
**Primary verification source:** L2-J01 — Procurement Planning & Sourcing Workflows  
**Additional verification sources:** L2-J00 — Enterprise Resource Management Architecture & Governance; L2-J02 — Vendor & Supplier Management Workflows; L2-J03 — Purchase Requisition & Approval Workflows; locked L1 Business Domain Architecture.  

Every BH is derived from the Locked parent. L2 verification confirms J01 exceptional-sourcing ownership, J02 supplier ownership, J03 demand ownership, and the exact J01 single-source and sourcing-outcome events.

---

## 10. Open Issues / Deferred Design

- Institution-specific permitted grounds, thresholds, publication/oversight requirements, review periods, independence rules, and duration/recurrence limits remain upstream-governed and are not universalised at L4.
- The parent lists `Section_J_Canonical_Register.md`; that exact dependency has not been retrieved in the current repository checks, so no behavior is inferred from an assumed substitute.
- API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/transaction mechanics, infrastructure, and UI/UX remain deferred to L5/L6.

**Review Verdict:** PASS WITH NON-BLOCKING FINDINGS
