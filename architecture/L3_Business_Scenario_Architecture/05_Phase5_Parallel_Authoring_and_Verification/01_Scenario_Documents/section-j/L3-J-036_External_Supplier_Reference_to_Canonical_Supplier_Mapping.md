# External Supplier Reference to Canonical Supplier Mapping

# 1. Scenario Metadata

**Scenario ID:** L3-J-036  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Vendor & Supplier Management  
**Participating Domains:** J  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** supplier onboarding, integrations, e-procurement, government portals, marketplaces, banking, tax, contract, finance, reporting, duplicate detection, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs mapping of supplier identifiers received from external authorities, portals, institutions, banks, tax systems, marketplaces, or partner platforms to the correct canonical supplier identity. It enables traceable interoperability without allowing an external reference to create, approve, merge, rename, or financially authorise a supplier automatically.

---

# 3. Business Outcome

An external supplier reference reaches an approved, rejected, unresolved, expired, or superseded mapping to a canonical supplier, with source, namespace, validity, evidence, scope, confidence, authority, privacy classification, effective period, and immutable mapping history.

Outcome labels in this section are business dispositions or supporting-record results, not additional canonical lifecycle states. Section 13 states the only canonical lifecycle impact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised business process receives or needs to register an external supplier identifier or reference.

## 4.2 End Boundary

The scenario ends when the reference is mapped, rejected, held unresolved, corrected, expired, or superseded; authorised consumers receive the current mapping; and original source and historical mappings remain traceable.

## 4.3 In Scope

- identifying source authority, namespace, identifier, issuer, supplier attributes, validity, purpose, and evidence;
- resolving the reference against canonical supplier and alias identities;
- detecting ambiguous, reused, conflicting, expired, duplicate, branch, successor, and related-entity references;
- approving one-to-one, one-to-many, many-to-one, scoped, or time-bound mapping only where semantics permit;
- governing corrections, supersession, expiry, revocation, source changes, and downstream notification;
- protecting tax, banking, personal, confidential, sanctions, and security-sensitive references.

## 4.4 Out of Scope

- creating or approving a supplier solely from an external identifier;
- reconciling duplicate internal identities, governed by L3-J-035;
- changing supplier legal identity, tax, banking, qualification, or payment authority;
- trusting an external source beyond its authorised purpose;
- rewriting historical transactions after mapping correction;
- technical API, message, matching, synchronisation, or master-data implementation.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-035 | Canonical identity and aliases must be reconciled first. |
| Related | L3-J-020 to L3-J-027 | Registration and profile evidence may support mapping. |
| Related | Contract and Finance scenarios | Consume references but retain transaction authority. |
| Related | Audit and Records scenarios | Preserve source, validity, mapping, and correction lineage. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Vendor & Supplier Management | Owns mapping intake, canonical resolution, approval, publication, correction, and history. |

Source-system, legal, tax, finance, banking, privacy, security, integration, audit, and data-governance owners retain source meaning and specialist authority.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Supplier Representative | Receives notice and supplies permitted response or remediation evidence; cannot remove institutional restrictions. | Responds. |
| Institutional Verifier | Independently verifies evidence material to the governance decision. | Verifies evidence. |
| Supplier Governance Authority | Proposes or approves restriction, suspension, blacklisting, reactivation, inactivation, closure or merge treatment. | Decides governed supplier treatment. |
---

# 7. Preconditions and Entry Criteria

- A canonical supplier identity or governed unresolved-candidate process exists.
- External source, namespace, identifier, issuer, purpose, validity, and evidence can be identified.
- Source trust, identity resolution, ambiguity, approval, privacy, confidentiality, correction, expiry, revocation, retention, and publication rules are effective.
- Duplicate and alias relationships from L3-J-035 are available.
- Sensitive reference access is restricted appropriately.

---

# 8. Trigger

An authorised external process supplies a supplier reference, or an internal process needs to associate an existing external identifier with a canonical supplier.

---

# 9. Main Business Flow

1. The originator records source, namespace, identifier, issuer, issue and validity dates, purpose, supplier attributes, evidence, and confidentiality.
2. The Institutional Verifier validates source authenticity, authority, identifier format, namespace uniqueness, effective period, and permitted use.
3. Canonical supplier names, aliases, registrations, tax identifiers, addresses, branches, ownership, representatives, and external references are searched.
4. Exact and candidate matches are distinguished; similarity or source assertion alone does not establish canonical identity.
5. Duplicate, branch, site, consortium, predecessor, successor, related-entity, recycled-identifier, and identifier-reuse risks are checked.
6. Sensitive tax, banking, sanctions, personal, or security references receive specialist validation and restricted disclosure.
7. Conflicts between external source and internal authoritative facts are preserved and referred; neither is silently overwritten.
8. The mapping recommendation defines canonical identity, source identity, namespace, scope, confidence limitations, validity, permitted consumers, and unresolved conditions.
9. One-to-many or many-to-one mapping is permitted only where source semantics and internal legal structure expressly justify it.
10. The Supplier Governance Authority reviews source authority, evidence, uniqueness, ambiguity, identity lineage, scope, privacy, and operational risk.
11. The Supplier Governance Authority rejects, returns, holds unresolved, or approves the mapping.
12. Approved mapping becomes effective prospectively and is linked immutably to the canonical identity and original external reference.
13. Authorised consumers receive the minimum mapping fact required for their process; supplier approval, qualification, banking, contract, and payment authorities remain separate.
14. Changes from the source are assessed as corrections, supersession, expiry, revocation, or new references rather than silent replacement.
15. Historical transactions retain the reference and mapping version effective at their time plus current canonical lineage.
16. Ambiguous or conflicting mappings may block automated reliance and require manual governed resolution.
17. Source evidence, mappings, corrections, consumers, actors, timestamps, and versions remain auditable.

---

# 10. Alternate Business Flows

## 10.1 No Canonical Supplier Exists

The reference remains unmapped and may support a separate registration process. It does not create a supplier automatically.

## 10.2 Multiple Candidate Suppliers

Automated reliance is held while identity evidence is reviewed. No arbitrary first-match mapping is permitted.

## 10.3 External Reference Covers a Branch

The branch relationship and legal-entity mapping are recorded according to source semantics without flattening distinct entities.

## 10.4 Source Corrects or Revokes Reference

The new fact is validated and a prospective correction, expiry, revocation, or successor mapping is approved. Original history remains.

## 10.5 Identifier Reused by External Source

Effective periods and source versions distinguish entities. Historical transactions retain the mapping valid at their event time.

## 10.6 Sensitive Banking Reference

Mapping may support identity corroboration but cannot alter beneficiary or payment authority without the separate finance-controlled process.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Incorrect Mapping

An attributable correction is approved, consumers are notified, and affected transactions are reviewed by their owners without rewriting history.

## 11.2 Source Unavailable or Unverifiable

The mapping is held, expired, or restricted to previously verified scope according to policy.

## 11.3 Mapping Not Propagated

Publication is reissued and downstream inconsistencies reconciled using authoritative versions.

## 11.4 Unauthorised Consumer Use

Access is blocked and the event is referred to data governance, privacy, security, and audit.

## 11.5 Namespace Collision

References are quarantined until source, namespace, and effective identity are resolved.

## 11.6 System Interruption

The last authoritative mapping version remains controlling and is reconciled on recovery.

---

# 12. Business Rules and Decision Constraints

- Every external reference is qualified by source, namespace, identifier, validity, scope, and evidence.
- Mapping does not create, approve, qualify, merge, rename, or financially authorise a supplier.
- External source confidence is limited to its authorised purpose and reliability.
- Ambiguous mapping cannot be converted into certainty by automation alone.
- One-to-many and many-to-one mappings require explicit source semantics and authority.
- Duplicate and alias lineage must be resolved through L3-J-035.
- Sensitive tax, banking, personal, sanctions, and security references follow restricted access.
- Correction, expiry, revocation, and supersession preserve earlier mappings and transaction-time lineage.
- External mapping cannot erase adverse status, qualification gaps, or closed history.
- Downstream transactions retain their own decision and correction authority.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Supplier | Existing canonical J02 state | Updates or records the scenario-specific governed fact without silently changing Supplier lifecycle state. | Existing canonical J02 state remains controlling | Vendor & Supplier Management (J02) | `L2-J02` Section 6 |

No separate lifecycle is defined upstream for the scenario-specific record; this scenario does not create one.
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `ExternalSupplierMapped` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| L3-J-035 | Supplies canonical and alias identity. | Mapping cannot merge identities. |
| External Source / Integration Owner | Supplies reference meaning and validity. | Does not grant internal supplier authority. |
| Legal / Tax / Banking / Privacy / Security | Validates sensitive semantics and use. | Retains specialist authority. |
| Contract / Finance / Reporting | Consumes authorised mapping. | Retains transaction truth and corrections. |
| Audit / Data Governance | Reviews lineage, versions, and access. | Retains oversight authority. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J02 roles are separated as policy requires: Supplier Representative; Institutional Verifier; and Supplier Governance Authority. The Supplier Representative cannot verify, approve, or remove an institutional restriction on itself, and a conflicted verifier cannot decide the governed outcome.

## 15.2 Required Evidence

- source, namespace, identifier, issuer, purpose, issue and validity dates, attributes, source evidence, confidentiality, and permitted use;
- canonical candidates, aliases, legal and tax identifiers, branches, related entities, duplicates, historical references, ambiguity, conflicts, and specialist findings;
- recommendation, approval, rejection, effective period, consumer scope, corrections, supersession, revocation, affected records, actors, timestamps, and versions.

## 15.3 Audit Obligations

The institution must reconstruct where the reference originated, how its meaning and validity were verified, why it mapped to the canonical supplier, what ambiguity or sensitive facts existed, who approved it, which consumers relied on it, and how later changes affected lineage.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- an external supplier reference from an authoritative external system, government registry, or banking source has been reviewed and assessed for mapping to an institutional canonical supplier identity;
- the mapping decision has been made by an authorised Supplier Governance Authority with evidence from both the external reference and the institutional record;
- the mapping relationship, its basis, confidence level, effective date, and limitations are documented and attributable;
- unresolvable references are preserved with their mapping status explicitly recorded rather than silently discarded;
- the mapping decision, evidence, authority, and audit trail are preserved and immutable;
- the canonical supplier identity remains the authoritative institutional record and the external reference is a cross-reference only.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- each resolved external supplier reference is mapped to one canonical institutional supplier identifier with its basis, confidence, and limitations permanently on record;
- the mapping is a cross-reference only and does not transfer ownership of the canonical institutional identity to the external system;
- unresolved references retain their provisional status and are not silently treated as matched;
- the complete mapping history, evidence, and Supplier Governance Authority decisions are permanently auditable;
- downstream sourcing, commitment, and payment scenarios may use the canonical identity without depending on the external reference's internal structure.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Authoritative external sources, namespaces, trust levels, mapping authorities, confidence rules, validity, and consumer scopes require confirmation.
2. Recycled identifiers, branches, government portals, tax systems, banking references, and cross-institution identifiers require source-specific mapping.
3. Historical transaction correction after a mapping error requires upstream owner-specific correction contracts; this L3 scenario cannot re-own them.

## 17.2 Assumptions

- External references are evidence and interoperability keys, not supplier authority.
- Canonical identity remains internally governed.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-036**
