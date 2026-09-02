# Supplier Classification Requirement to Governed Category Eligibility

## 1. Specification Metadata

**Specification ID:** L4-J-025  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Enterprise Resource Management — Vendor & Supplier Management  
**Realizes Scenario:** L3-J-025  
**Primary Business Owner:** Vendor & Supplier Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-025_Supplier_Classification_Requirement_to_Governed_Category_Eligibility.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP shall govern supplier classification against the institutional procurement taxonomy and make a clear distinction between descriptive category classification and category eligibility that may be used for sourcing.

The behavior shall not create supplier identity, supplier activation, underlying qualification, sourcing preference, award entitlement, commitment or financial obligation.

## 3. Required Business Context

- canonical supplier identity and supplier status where eligibility requires it;
- current approved category taxonomy/version and category definitions;
- requested supplier role, goods/works/services scope and geography;
- current qualification, licence, authorisation, capacity and related evidence;
- taxonomy/qualification dependencies, conditions and validity;
- Qualification Authority decision.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-025-BH-001 | When classification is requested, the system shall associate the request with the current governed taxonomy version and canonical supplier identity. | L3-J-025 §§4.1, 7–9 |
| L4-J-025-BH-002 | The system shall map requested goods, works or services to controlled categories and shall not treat free-text supplier claims as authoritative category eligibility. | L3-J-025 §§2, 9 steps 4–6, 12 |
| L4-J-025-BH-003 | The system shall represent supplier role explicitly for each category where role is material to eligibility. | L3-J-025 §§4.3, 9 step 6, 12 |
| L4-J-025-BH-004 | The system shall distinguish `Classification Only` from category eligibility and shall prevent descriptive classification from authorising sourcing where Active/qualified eligibility is required. | L3-J-025 §§3, 9 step 8, 10.2, 12 |
| L4-J-025-BH-005 | The system shall require current supplier status, qualification, evidence and controlled taxonomy dependencies before making category eligibility available. | L3-J-025 §§7, 9 step 7, 12 |
| L4-J-025-BH-006 | The system shall limit category eligibility to the approved category/subcategory, role, geography, value/risk constraints, validity and conditions. | L3-J-025 §§3, 9 steps 12–15, 12 |
| L4-J-025-BH-007 | The system shall use the controlling upstream validity limitation and shall not extend category eligibility beyond an expired or restricted qualification, licence or authorisation. | L3-J-025 §§9 step 13, 11.2–11.4, 12 |
| L4-J-025-BH-008 | When taxonomy changes, the system shall preserve historical taxonomy versions and require governed mapping/reassessment rather than silently rewriting existing category history. | L3-J-025 §§10.5, 11.5, 12 |
| L4-J-025-BH-009 | When supplier capability, qualification, licence, authorisation or capacity changes, the system shall expose affected category positions for reassessment rather than silently maintaining eligibility. | L3-J-025 §§9 step 16, 11.7 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-025-BH-010 | Where an approved category position represents granted supplier qualification, the system shall publish or make available the canonical `SupplierQualificationGranted` fact on behalf of Vendor & Supplier Management. | L3-J-025 §14.1 |
| L4-J-025-BH-011 | The system shall expose current governed category eligibility to authorised sourcing while preserving sourcing's event-specific evaluation and award responsibility. | L3-J-025 §§14.2, 16.2 |

## 6. Failure / Rejection / Continuation

- Ambiguous category requests shall remain unresolved rather than being assigned an expedient broad category.
- Missing/expired qualification shall leave the position classification-only or restricted as applicable.
- Non-Active/restricted supplier status shall block new eligibility where required.
- Invalid licence/authorisation shall prevent dependent eligibility.
- Unauthorised manual classification shall not create authoritative eligibility.
- Post-approval capability change shall require governed reassessment.

**Provenance:** L3-J-025 §11.

## 7. Audit & Evidence Behavior

The system shall preserve the supplier, taxonomy version, requested and approved categories, supplier role, supporting qualification/evidence, mappings, limitations, validity, conditions, authority decision, corrections/reassessments and downstream eligibility history.

**Provenance:** L3-J-025 §15.

## 8. Observable Completion Conditions

Completion is observable when each requested category has an attributable governed disposition; classification-only positions cannot be mistaken for sourcing eligibility; approved eligibility is exact and time-bound; historical taxonomy/mapping evidence remains preserved; and no award, commitment or financial obligation has been created by classification itself.

## 9. Traceability & Provenance

**Parent:** L3-J-025 — Supplier Classification Requirement to Governed Category Eligibility  
**Primary verification source:** L2-J02 — Vendor & Supplier Management Workflows  

Material anchors: L3-J-025 §§2–16.

## 10. Open Issues / Deferred Design

Taxonomy hierarchy/ownership, categories requiring standing versus event-only qualification, and capacity allocation across categories remain upstream issues recorded in L3-J-025.

Implementation design remains deferred to L5/L6.
