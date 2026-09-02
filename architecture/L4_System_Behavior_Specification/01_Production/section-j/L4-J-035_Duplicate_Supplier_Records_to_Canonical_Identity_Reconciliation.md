# Duplicate Supplier Records to Canonical Identity Reconciliation

## 1. Specification Metadata

**Specification ID:** L4-J-035  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Enterprise Resource Management — Vendor & Supplier Management  
**Realizes Scenario:** L3-J-035  
**Primary Business Owner:** Vendor & Supplier Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-035_Duplicate_Supplier_Records_to_Canonical_Identity_Reconciliation.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern suspected duplicate supplier identities so that one canonical supplier identity is established only where authoritative evidence proves duplication, while all source identifiers, aliases, adverse history and transaction lineage remain preserved.

Reconciliation is identity governance. It shall not delete records, transfer legal liability, novate contracts, merge financial balances or rewrite historical transactions.

## 3. Required Business Context

- candidate supplier records and source identifiers;
- authoritative legal, registration, tax, ownership, address, representative and banking-reference evidence;
- existing supplier profile, qualification and governance states;
- transaction/contract/Finance references linked to each candidate;
- duplicate/branch/succession/related-entity governance rules;
- Supplier Governance Authority decision;
- external-reference lineage where applicable.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-035-BH-001 | When duplicate risk is raised, the system shall preserve all candidate records and source identifiers and prevent uncontrolled merge, deletion or operational reuse where the governed investigation requires a hold. | L3-J-035 §§7–9 step 2 |
| L4-J-035-BH-002 | The system shall distinguish similarity evidence from authoritative identity evidence and shall not infer duplication from similar names, contacts or missing data alone. | L3-J-035 §§9 steps 3–5, 12 |
| L4-J-035-BH-003 | The system shall preserve distinctions between duplicate identity, branch/site, proprietor/partnership, consortium, affiliate, successor/predecessor and unrelated entities. | L3-J-035 §§4.3, 9 step 5, 10.2–10.3, 12 |
| L4-J-035-BH-004 | Before canonical reconciliation, the system shall inventory affected qualification, governance, sourcing, award, commitment, contract, receipt, invoice, payable, payment, tax, reporting and external-reference lineage without moving those records. | L3-J-035 §§9 steps 6–7, 15 |
| L4-J-035-BH-005 | A canonical identity shall be selected only from the competent authority's evidence-based determination and shall not be chosen solely because one record is older. | L3-J-035 §§9 steps 10–11, 12 |
| L4-J-035-BH-006 | For confirmed duplicates, the system shall retain non-canonical records as permanent aliases/source identities linked to the canonical identity and prevent their independent operational use. | L3-J-035 §§9 step 12, 12 |
| L4-J-035-BH-007 | Conflicting supplier-profile facts shall be resolved through authoritative evidence and attributable versioned correction rather than indiscriminate value combination. | L3-J-035 §§9 step 13, 12 |
| L4-J-035-BH-008 | Canonical reconciliation shall preserve adverse qualification/restriction/suspension/blacklisting/inactivity/closure/performance history and shall not cleanse those facts through canonical selection. | L3-J-035 §§9 step 14, 10.4, 12 |
| L4-J-035-BH-009 | Downstream owners shall receive canonical mapping facts for their own corrections while historical transactions retain original source identifiers plus canonical lineage. | L3-J-035 §§9 step 15, 12, 16 |
| L4-J-035-BH-010 | The system shall prevent automatic combination or transfer of contracts, payables, payments, tax positions, balances, bank accounts or legal liabilities as a consequence of identity reconciliation. | L3-J-035 §§4.4, 10.3–10.5, 12 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-035-BH-011 | On approved confirmed-duplicate reconciliation, the system shall publish or make available the canonical `SupplierMergeCompleted` fact on behalf of Vendor & Supplier Management. | L3-J-035 §14.1 |
| L4-J-035-BH-012 | The system shall make canonical/alias mapping available to Contract, Finance, Tax, Audit, Privacy, Security and L3-J-036 consumers without transferring correction authority. | L3-J-035 §14.2 |

## 6. Failure / Rejection / Continuation

- Incorrect merge shall be corrected by restoring distinct identity/linkage with full audit history.
- Loss of transaction lineage shall prevent final reconciliation.
- Weak-evidence canonical selection shall be remanded and may restrict new reliance.
- Unauthorised deletion/overwrite shall be preserved and referred to specialist governance.
- Mapping-propagation failure shall be reconciled without rewriting original transactions.
- System interruption shall preserve approved canonical/alias controls.

**Provenance:** L3-J-035 §11.

## 7. Audit & Evidence Behavior

The system shall preserve:

- all candidate identities/source identifiers;
- authoritative identity evidence and comparison basis;
- branch/succession/related-entity analysis;
- existing profile/qualification/adverse-governance histories;
- transaction/contract/Finance lineage inventories;
- canonical determination and aliases;
- downstream referrals/corrections;
- actors, reasons, timestamps and versions.

**Provenance:** L3-J-035 §15.

## 8. Observable Completion Conditions

Completion is observable when the suspected duplicates have a governed determination; confirmed duplicates resolve to one canonical identity with non-operational aliases; distinct entities remain separate; `SupplierMergeCompleted` is available where applicable; source/transaction history is intact; and no financial/legal obligations were automatically combined.

## 9. Traceability & Provenance

**Parent:** L3-J-035 — Duplicate Supplier Records to Canonical Identity Reconciliation  
**Primary verification source:** L2-J02 — Vendor & Supplier Management Workflows

Material anchors: L3-J-035 §§2–16.

## 10. Open Issues / Deferred Design

Matching attributes/confidence standards, branch/succession/consortium/legal rules and downstream owner-specific correction contracts remain upstream issues. L4 does not invent matching algorithms or survivorship mechanics.

Implementation design remains deferred to L5/L6.
