# External Supplier Reference to Canonical Supplier Mapping

## 1. Specification Metadata

**Specification ID:** L4-J-036  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Enterprise Resource Management — Vendor & Supplier Management  
**Realizes Scenario:** L3-J-036  
**Primary Business Owner:** Vendor & Supplier Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-036_External_Supplier_Reference_to_Canonical_Supplier_Mapping.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern the mapping of external supplier references to canonical internal supplier identities while preserving source meaning, validity, privacy and mapping lineage.

An external reference shall remain a cross-reference only. It shall not create, approve, qualify, merge, rename or financially authorise a supplier.

## 3. Required Business Context

- canonical supplier identity or governed unresolved candidate process;
- external source, namespace, identifier, issuer, purpose, validity and evidence;
- canonical aliases and duplicate reconciliation from L3-J-035 where applicable;
- source trust/permission and privacy/confidentiality rules;
- branch/successor/related-entity semantics;
- Supplier Governance Authority decision;
- specialist validation for sensitive tax/banking/sanctions/security references where applicable.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-036-BH-001 | When an external supplier reference is introduced, the system shall require source, namespace, identifier, issuer, purpose, validity/effective period and evidence to be attributable. | L3-J-036 §§7–9 step 1, 12 |
| L4-J-036-BH-002 | The system shall verify the source's authorised meaning and identifier context before permitting mapping reliance. | L3-J-036 §§9 step 2, 12 |
| L4-J-036-BH-003 | The system shall resolve the reference against canonical and alias identities while distinguishing exact matches from candidates; similarity or source assertion alone shall not establish identity. | L3-J-036 §§9 steps 3–4, 12 |
| L4-J-036-BH-004 | The system shall identify duplicate, branch/site, consortium, predecessor/successor, related-entity, recycled-identifier and identifier-reuse risks before approving a mapping. | L3-J-036 §§9 step 5, 10.3–10.5 |
| L4-J-036-BH-005 | Sensitive tax, banking, sanctions, personal or security references shall remain subject to their specialist validation and restricted-disclosure rules. | L3-J-036 §§4.3, 9 step 6, 12 |
| L4-J-036-BH-006 | Conflicts between external-source facts and internal authoritative facts shall remain explicit and shall not be resolved through silent overwrite. | L3-J-036 §§9 step 7, 12 |
| L4-J-036-BH-007 | One-to-many or many-to-one mappings shall be permitted only when source semantics and internal legal structure expressly support them. | L3-J-036 §§9 step 9, 12 |
| L4-J-036-BH-008 | On approved mapping, the system shall create a prospective immutable linkage between the external reference and canonical supplier with scope, validity, permitted use and limitations. | L3-J-036 §§9 steps 8–12, 16 |
| L4-J-036-BH-009 | If no canonical supplier exists or multiple unresolved candidates remain, the system shall preserve the reference as unmapped/unresolved and prevent arbitrary automated reliance. | L3-J-036 §§10.1–10.2, 12 |
| L4-J-036-BH-010 | Corrections, revocations, expiry and supersession shall create attributable successor mapping facts while preserving transaction-time mapping lineage. | L3-J-036 §§9 steps 14–15, 10.4–10.5, 12 |
| L4-J-036-BH-011 | External mapping shall not change supplier approval, qualification, banking/payment authority, contract ownership or transaction correction authority. | L3-J-036 §§2, 4.4, 9 step 13, 10.6, 12 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-036-BH-012 | On approved mapping, the system shall publish or make available the canonical `ExternalSupplierMapped` fact on behalf of Vendor & Supplier Management. | L3-J-036 §14.1 |
| L4-J-036-BH-013 | The system shall make only the authorised minimum mapping fact available to Contract, Finance, Reporting, Audit and integration consumers while preserving their own transaction and correction authority. | L3-J-036 §14.2 |

## 6. Failure / Rejection / Continuation

- Incorrect mapping shall be corrected prospectively with consumer impact review and historical mapping retained.
- Unavailable/unverifiable source shall result in held/expired/restricted mapping according to upstream rules.
- Publication failure shall not alter the authoritative mapping version.
- Unauthorised consumer use shall not broaden permitted mapping scope.
- Namespace collisions shall remain unresolved until source/namespace/effective identity is governed.
- System interruption shall preserve the last authoritative mapping version.

**Provenance:** L3-J-036 §11.

## 7. Audit & Evidence Behavior

The system shall preserve:

- source/namespace/identifier/issuer/purpose/validity;
- source evidence and trust context;
- candidate/canonical/alias comparisons;
- ambiguity, branch/succession/reuse findings;
- sensitive-reference specialist findings/access classification;
- mapping authority decision, scope, limitations and permitted consumers;
- corrections/expiry/revocation/supersession;
- affected consumer references;
- actors, reasons, timestamps and versions.

**Provenance:** L3-J-036 §15.

## 8. Observable Completion Conditions

Completion is observable when the external reference has an approved/rejected/unresolved/expired/superseded mapping disposition; any approved mapping is linked to the canonical supplier with explicit scope and validity; `ExternalSupplierMapped` is available; unresolved references cannot be treated as matched; and canonical supplier ownership remains internal.

## 9. Traceability & Provenance

**Parent:** L3-J-036 — External Supplier Reference to Canonical Supplier Mapping  
**Primary verification source:** L2-J02 — Vendor & Supplier Management Workflows

Material anchors: L3-J-036 §§2–16.

## 10. Open Issues / Deferred Design

Authoritative external-source catalogues, namespace trust levels, confidence/mapping rules, recycled-identifier semantics, branch/government/tax/banking reference rules and owner-specific correction contracts remain upstream issues.

Implementation design remains deferred to L5/L6.
