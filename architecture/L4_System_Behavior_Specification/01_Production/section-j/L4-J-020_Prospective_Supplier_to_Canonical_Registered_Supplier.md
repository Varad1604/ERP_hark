# Prospective Supplier to Canonical Registered Supplier

## 1. Specification Metadata

**Specification ID:** L4-J-020  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Vendor & Supplier Management  
**Realizes Scenario:** L3-J-020  
**Primary Business Owner:** Vendor & Supplier Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-020_Prospective_Supplier_to_Canonical_Registered_Supplier.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`  
**Referenced By:** Supplier approval, qualification, sourcing, commitment, receipt, payable, tax, compliance, performance, restriction, and audit specifications where applicable.

---

## 2. Behavioral Objective & Boundary

SutraERP shall establish one canonical J02 Supplier identity from a legitimate prospective-supplier registration request while preserving source identity evidence, duplicate/relationship analysis, provenance, and the distinction between registration and later approval or qualification.

The behavior begins when a prospective supplier submits registration information or an authorised institutional role initiates registration for a legitimate procurement purpose. Successful completion occurs only when one canonical Supplier identifier exists in the canonical `Registered` lifecycle position and `SupplierRegistered` is available for the separate approval scenario.

Return for correction, rejection, unresolved duplicate treatment, or linkage to an existing canonical supplier is a governed non-success disposition and shall not be represented as successful registration of a new Supplier.

---

## 3. Required Business Context

- Legitimate supplier-registration purpose and identifiable initiating party.
- Supplier or authorised representative and source-supported representative authority.
- Institution-governed mandatory identity fields, evidence types, declarations, privacy/data-use notices, verification sources, duplicate rules, and registration authority.
- Supplier constitution and jurisdiction.
- Existing J02 Supplier identities for duplicate and relationship comparison.
- Legal identity, registration, tax, address, contact, ownership, beneficial ownership, representative, classification, and banking-reference evidence as applicable.
- Source/evidence issuer, date, validity, verification result, version, and limitation facts.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-020-BH-001 | When supplier registration begins, the system shall preserve the registration request, submission source, supplier representative, sponsor where applicable, legitimate purpose, consent/notice basis, and submission time. | L3-J-020 §§7–9.1 |
| L4-J-020-BH-002 | The system shall require the registration information and evidence mandated for the supplier's constitution, jurisdiction, category, risk, and institutional profile before successful registration. | L3-J-020 §§7, 9.2, 12 |
| L4-J-020-BH-003 | Before supplier-supplied attestations are accepted as governed registration evidence, the system shall require source-supported representative authority or preserve the unresolved authority condition. | L3-J-020 §§9.3, 11.5 |
| L4-J-020-BH-004 | The system shall preserve verified legal name, trade names, constitution, registration identifiers, tax identifiers, registered/operating addresses, and authoritative dates with their source provenance. | L3-J-020 §§9.4, 15.2, 16.1 |
| L4-J-020-BH-005 | Where required, the system shall preserve ownership, beneficial ownership, controlling persons, related entities, branches, consortium relationships, predecessor/successor relationships, and their evidence without converting specialist interpretation into J02 ownership. | L3-J-020 §§9.5, 12, 15.2 |
| L4-J-020-BH-006 | The system shall preserve supplier contacts and authorised communication roles separately from canonical legal identity facts. | L3-J-020 §§9.6, 12 |
| L4-J-020-BH-007 | When registration values are normalised for comparison, the system shall preserve original submitted values and evidence provenance. | L3-J-020 §§9.7, 12, 16.2 |
| L4-J-020-BH-008 | Before creating a new canonical Supplier identifier, the system shall compare the candidate against existing Supplier identities using the institutionally governed duplicate and relationship evidence described by the parent. | L3-J-020 §§4.3, 9.8, 12 |
| L4-J-020-BH-009 | When a potential match exists, the system shall preserve the Institutional Verifier's governed disposition as new identity, existing identity update, branch/site, related entity, predecessor/successor, duplicate, or unresolved relationship without silently merging or deleting records. | L3-J-020 §§9.9, 10.1–10.2, 12 |
| L4-J-020-BH-010 | If an existing canonical Supplier is confirmed, the system shall link the request to that identity and shall not issue a second canonical Supplier identifier as a successful new-registration outcome. | L3-J-020 §§3, 10.1, 12, 16.1 |
| L4-J-020-BH-011 | The system shall preserve required conflict, related-party, sanction, prohibited-entity, debarment, and institutional-affiliation declarations or referrals without treating registration as supplier approval. | L3-J-020 §§9.10, 12 |
| L4-J-020-BH-012 | The system shall assign only source-governed supplier classifications and interests while preserving that classification does not itself establish qualification. | L3-J-020 §§9.11, 12 |
| L4-J-020-BH-013 | For each material registration evidence item, the system shall preserve source/issuer, issue date, expiry date, verification result, version, language/translation where applicable, and limitation. | L3-J-020 §§9.12, 15.2 |
| L4-J-020-BH-014 | The system shall preserve the Institutional Verifier's recommendation to register, link, return, or reject and the supporting reason/unresolved issue. | L3-J-020 §9.13 |
| L4-J-020-BH-015 | Successful registration shall require the Supplier Approver decision on completeness, identity confidence, duplicate disposition, required declarations, and data-governance conditions. | L3-J-020 §§9.14, 15.1 |
| L4-J-020-BH-016 | On successful registration, the system shall establish one canonical institutional Supplier identifier in `Registered` within the J02 `Prospective / Registered` entry position and shall not mark the Supplier `Approved` or `Active`. | L3-J-020 §§3, 4.2, 9.15, 13, 16 |
| L4-J-020-BH-017 | Captured banking details shall remain supplier-profile references only and shall not be represented as approved payment instructions. | L3-J-020 §§2, 4.4, 12 |
| L4-J-020-BH-018 | A registration rejection shall not create `Blacklisted`, `Suspended`, or another prohibited Supplier status unless a separate authoritative J02 governance process establishes it. | L3-J-020 §§11.4, 12 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-020-BH-019 | When a new canonical Supplier successfully reaches `Registered`, the system shall publish `SupplierRegistered` with Vendor & Supplier Management (J02) as canonical publisher. | L3-J-020 §§4.2, 14.1, 16.1; L2-J02 §13 |

The requesting/procurement unit supplies legitimate-purpose context but does not own Supplier identity. Finance/tax, legal, and compliance authorities retain their specialist interpretations. L3-J-021 consumes the Registered Supplier for approval; L3-J-022 separately governs qualification.

---

## 6. Failure / Rejection / Continuation

- Missing or invalid mandatory evidence shall prevent successful registration and permit only source-supported correction or rejection.
- An unresolved duplicate shall prevent creation of a new canonical Supplier identifier.
- Conflicting identity or ownership evidence shall remain visible pending authoritative clarification, enhanced review, or rejection.
- Fraud, sanction, or prohibited-relationship concerns may hold or reject registration and remain referred to the authoritative specialist process.
- Failure of representative authority shall prevent acceptance of supplier attestation.
- An interrupted registration shall not imply `Registered` and shall not create a second Supplier identity.
- A post-registration error shall follow attributable supplier master-data governance while preserving historical values, evidence, approvals, and effective dates.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- registration purpose, initiating party, representative identity/authority, submission source, consent/notice basis, and time;
- legal/trade names, constitution, registrations, tax identifiers, addresses, contacts, ownership/control, related entities, classifications, and banking references;
- original submitted values and any governed normalisation;
- verification sources, issuer, issue/expiry dates, outcome, versions, translations, and limitations;
- duplicate searches, candidate matches, relationship analysis, duplicate/merge disposition, and surviving canonical identity where applicable;
- declarations, conflicts, referrals, verifier recommendation, approver decision, reasons, actors, timestamps, and versions;
- canonical `SupplierRegistered` fact for successful registration.

---

## 8. Observable Completion Conditions

Successful completion requires all of the following:

1. one canonical institutional Supplier identifier has been established;
2. the Supplier is in `Registered` and has not reached `Approved` or `Active`;
3. required identity attributes and evidence have been verified to the parent-defined level;
4. no unresolved duplicate or identity conflict prevents canonical creation;
5. original submitted values and provenance remain attributable;
6. required declarations and duplicate/relationship treatment are preserved;
7. an authorised registration decision is attributable;
8. `SupplierRegistered` represents the completed registration fact.

A return, rejection, unresolved duplicate, or existing-supplier match does not satisfy the successful completion outcome.

---

## 9. Traceability & Provenance

**Parent:** L3-J-020 — Prospective Supplier to Canonical Registered Supplier  
**Primary verification source:** L2-J02 — Vendor & Supplier Management Workflows  
**Additional verification source:** L2-J00 — Enterprise Resource Management Architecture & Governance.

J02 is the exclusive canonical owner of Supplier identity, Supplier lifecycle, duplicate/merge governance, supplier profile evidence, and `SupplierRegistered`. Registration remains distinct from approval, activation, qualification, sourcing entitlement, commitment, and payment authority.

---

## 10. Open Issues / Deferred Design

- Mandatory fields/documents, authoritative verification sources, verification strength, and retention periods by supplier type/jurisdiction remain upstream-governed.
- Exact legal-entity, branch, site, consortium, individual, and foreign-supplier identity semantics remain an upstream J02 entity-model issue.
- Duplicate confidence, merge/split/relationship, and survivorship authorities remain dependent on upstream data-governance rules.
- `Section_J_Canonical_Register.md`, although listed by the Locked parent, could not be located by exact filename during current repository verification. No missing material lifecycle/event/ownership semantics were inferred from it.
- Data model, matching algorithms, APIs/interfaces, persistence, document storage, workflow mechanics, technical access controls, event transport, infrastructure, and UI/UX are deferred to L5/L6.

**Review Verdict:** PASS WITH NON-BLOCKING FINDINGS
