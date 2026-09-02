# Expired Supplier Qualification to Governed Eligibility Restriction

## 1. Specification Metadata

**Specification ID:** L4-J-024  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Enterprise Resource Management — Vendor & Supplier Management  
**Realizes Scenario:** L3-J-024  
**Primary Business Owner:** Vendor & Supplier Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-024_Expired_Supplier_Qualification_to_Governed_Eligibility_Restriction.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP shall make supplier qualification expiry operationally effective for the exact affected eligibility scope when no approved successor is effective.

The system shall prevent new reliance on expired eligibility while preserving canonical supplier identity, unaffected valid qualifications and existing commitments for disposition by their own authoritative owners. Supplier-level `Restricted` is applied only where the source-defined J02 governance transition requires it.

## 3. Required Business Context

- canonical supplier and current supplier state;
- qualification/evidence dependency and authoritative expiry;
- any successor, overlap or authorised interim eligibility;
- exact qualification scope and affected sourcing/commitment references;
- specialist consequences where legal, regulatory, safety, insurance, security or other authoritative requirements apply;
- supplier-governance authority where supplier-level restriction is required.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-024-BH-001 | At authoritative qualification/evidence expiry, the system shall determine whether an effective approved successor or authorised interim eligibility covers the same scope before applying restriction. | L3-J-024 §§4.1, 7–9 |
| L4-J-024-BH-002 | When no effective successor exists, the system shall mark the affected qualification as expired without erasing or altering the historical qualification evidence. | L3-J-024 §§9 step 7, 12, 13 |
| L4-J-024-BH-003 | The system shall restrict only the eligibility scope that depends on the expired qualification/evidence and shall preserve unrelated valid qualifications. | L3-J-024 §§3, 9 steps 4–5, 10.1, 12 |
| L4-J-024-BH-004 | The system shall prevent new invitation, participation advancement, award approval or commitment creation when those actions require the expired eligibility scope. | L3-J-024 §§9 step 8, 12, 16 |
| L4-J-024-BH-005 | The system shall identify existing sourcing events, recommendations and commitments materially reliant on the expired scope and make the expiry fact available to their authoritative owners for governed disposition. | L3-J-024 §§9 steps 9–10, 10.4–10.6, 12 |
| L4-J-024-BH-006 | The system shall not automatically cancel existing Purchase Orders, contracts, deliveries, warranties, payables or payments solely because eligibility expires. | L3-J-024 §§4.4, 9 step 10, 12 |
| L4-J-024-BH-007 | Where J02 governance requires supplier-level restriction, the system shall apply the source-defined transition to canonical `Restricted` at the authoritative effective time; otherwise supplier state shall remain governed independently of scope-level expiry. | L3-J-024 §§2, 9 step 7, 13 |
| L4-J-024-BH-008 | The system shall prevent a pending renewal submission, uploaded document or review activity from being treated as restored eligibility. | L3-J-024 §§10.3, 12 |
| L4-J-024-BH-009 | When a later approved renewal becomes effective, the system shall recognize the successor eligibility prospectively and preserve the historical restricted gap. | L3-J-024 §§9 step 15, 11.6, 12 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-024-BH-010 | On qualification expiry, the system shall publish or make available the canonical `SupplierQualificationExpired` fact on behalf of Vendor & Supplier Management. | L3-J-024 §14.1 |
| L4-J-024-BH-011 | When the canonical supplier transition to `Restricted` is required, the system shall publish or make available the canonical `SupplierRestricted` fact without changing its publisher. | L3-J-024 §§13–14.1 |
| L4-J-024-BH-012 | The system shall make material expiry/restriction facts available to sourcing, J04 commitment ownership and relevant specialist owners without rewriting their owned records. | L3-J-024 §§14.2, 16 |

## 6. Failure / Rejection / Continuation

- Where expiry timing is disputed, the system shall retain the governed restricted position required by upstream rules until authoritative expiry facts are resolved.
- Incorrect over-restriction shall be corrected through an attributable successor correction while preserving the error and impact evidence.
- An unauthorised override shall not establish valid eligibility.
- Business expiry remains effective through system interruption; on continuation, affected activity shall be reconciled against the authoritative expiry.
- Late renewal shall not eliminate the historical restriction gap.

**Provenance:** L3-J-024 §11.

## 7. Audit & Evidence Behavior

The system shall preserve:

- expired qualification/evidence and dependency;
- authoritative expiry, issuer/version and scope;
- absence/timing of successor eligibility;
- resulting scope restriction and any supplier-level `Restricted` transition;
- affected sourcing/commitment references and their hand-off;
- supplier and institutional notices where required;
- exceptions, corrections, unauthorised reliance and later restoration;
- actors, reasons, timestamps and versions.

**Provenance:** L3-J-024 §15.

## 8. Observable Completion Conditions

The behavior is complete when expired scope is no longer usable for new governed reliance, affected existing activities have been exposed to their owners for disposition, unaffected scope remains distinguishable, canonical expiry/restriction facts are available as applicable, and the complete history remains auditable.

## 9. Traceability & Provenance

**Parent:** L3-J-024 — Expired Supplier Qualification to Governed Eligibility Restriction  
**Primary verification source:** L2-J02 — Vendor & Supplier Management Workflows  

Material anchors: L3-J-024 §§2–16.

## 10. Open Issues / Deferred Design

Institution-specific rules for dependency, grace/interim allowance, urgent escalation, eligibility checkpoints and treatment of existing commitments remain upstream issues. L4 does not resolve them.

Implementation mechanisms remain deferred to L5/L6.
