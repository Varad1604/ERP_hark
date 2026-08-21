# Duplicate Supplier Records to Canonical Identity Reconciliation

# 1. Scenario Metadata

**Scenario ID:** L3-J-035  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Vendor & Supplier Management  
**Participating Domains:** J  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** supplier registration, profile, qualification, sourcing, award, commitment, contract, receipt, payable, payment, reporting, external mapping, closure, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs identification and reconciliation of records that may represent the same supplier while preventing incorrect merging of distinct legal entities. It establishes one canonical identity, preserves every source identifier and transaction, resolves conflicting attributes through authoritative evidence, and prevents duplicate operational use.

Reconciliation is identity governance, not record deletion, transaction transfer, legal succession, supplier approval, or financial netting.

---

# 3. Business Outcome

Suspected duplicate supplier records reach a governed determination of confirmed duplicate, related but distinct, or not duplicate, with one canonical identity where appropriate, preserved aliases and source lineage, resolved authoritative attributes, controlled operational status, downstream impact review, authority, and immutable evidence.

Outcome labels in this section are business dispositions or supporting-record results, not additional canonical lifecycle states. Section 13 states the only canonical lifecycle impact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when registration, monitoring, search, tax, banking, transaction, external-reference, audit, or user evidence indicates that two or more supplier records may represent the same entity.

## 4.2 End Boundary

The scenario ends when the records are confirmed as duplicates or distinct; a canonical identity and aliases are approved where applicable; conflicting facts and operational status are governed; downstream owners receive reconciliation facts; and all historical lineage remains traceable.

## 4.3 In Scope

- comparing legal name, former names, trade names, constitution, registration, tax identifiers, addresses, contacts, ownership, banking references, digital identifiers, and source provenance;
- distinguishing duplicate, branch, site, establishment, proprietor, partner, consortium, affiliate, successor, predecessor, and unrelated entity;
- selecting or confirming the canonical identity using authoritative evidence;
- converting duplicate records into non-operational aliases or linked source identities without deletion;
- reconciling profile, qualification, restriction, suspension, blacklisting, inactivity, closure, representative, external-reference, and transaction linkage;
- referring transaction, contract, payable, payment, tax, reporting, or fraud impacts to authoritative owners.

## 4.4 Out of Scope

- approving a supplier or qualification;
- changing legal succession, novation, ownership, or liability without separate authority;
- combining or netting payables, payments, contracts, tax positions, or bank accounts;
- deleting duplicate records or rewriting historical transactions;
- treating similar names or shared contacts as proof of identity;
- technical matching algorithms, confidence scoring, survivorship, or data-migration implementation.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-J-020 to L3-J-027 | Registration and profile evidence support identity determination. |
| Related | L3-J-028 to L3-J-034 | Governance and lifecycle states require canonical consolidation without erasure. |
| Followed by | L3-J-036 | External references must map to the reconciled canonical identity. |
| Related | Contract and Finance scenarios | Retain transaction, liability, tax, and payment authority. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Vendor & Supplier Management | Owns duplicate investigation, canonical identity recommendation, alias control, approval coordination, publication, and history. |

Legal, tax, finance, banking, contract, data governance, privacy, security, audit, and source-system owners retain their authoritative facts and correction authority.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Supplier Representative | Receives notice and supplies permitted response or remediation evidence; cannot remove institutional restrictions. | Responds. |
| Institutional Verifier | Independently verifies evidence material to the governance decision. | Verifies evidence. |
| Supplier Governance Authority | Proposes or approves restriction, suspension, blacklisting, reactivation, inactivation, closure or merge treatment. | Decides governed supplier treatment. |
---

# 7. Preconditions and Entry Criteria

- Candidate records and source identifiers are available and protected from uncontrolled editing.
- Canonical identity, matching evidence, authority, conflict, legal-entity, branch, succession, alias, correction, and retention rules are effective.
- Authoritative registration, tax, profile, qualification, banking-reference, governance, transaction, and external-reference evidence can be accessed.
- Operational use can be held where duplicate risk is material.
- Distinct entities cannot be merged solely for convenience or reporting.

---

# 8. Trigger

Two or more supplier records exhibit identity overlap, or an authorised source reports duplicate registration, external mapping, tax identity, banking reference, or transaction use.

---

# 9. Main Business Flow

1. The reporter submits candidate supplier records, source systems, matching attributes, differences, discovery context, and affected activity.
2. The Institutional Verifier protects the records from uncontrolled merge, deletion, or new reliance while investigation proceeds where required.
3. Legal name, former name, trade name, constitution, registration number, tax identifiers, addresses, ownership, contacts, websites, representatives, and banking references are compared with provenance.
4. Similarity evidence is distinguished from authoritative identity evidence; missing values and data-entry errors do not prove sameness or difference.
5. Branches, sites, establishments, proprietors, partnerships, group companies, consortium members, successors, predecessors, and merged entities are analysed under applicable legal rules.
6. Supplier approval, qualifications, categories, restrictions, suspension, blacklisting, inactivity, closure, performance, incidents, representatives, and external references are compared.
7. Sourcing, awards, commitments, contracts, receipts, invoices, payables, payments, credits, recoveries, tax reports, and audit records linked to each candidate are inventoried without movement.
8. Conflicts involving supplier, data owner, sourcing, finance, or approver are declared and controlled.
9. The supplier or authorised external source provides clarification where needed; sensitive tax and banking facts follow restricted access.
10. The recommendation determines confirmed duplicate, related but distinct, legally succeeded, unresolved, or not duplicate and records reasons and confidence limitations.
11. For confirmed duplicates, the canonical identity is selected using authoritative continuity, completeness, legal identity, and transaction lineage, not record age alone.
12. Non-canonical records become retained aliases or source identities linked permanently to the canonical identity and blocked from independent operational use.
13. Conflicting profile facts are resolved through authoritative evidence and versioned correction; they are not combined indiscriminately.
14. Governance states are applied according to evidence and scope; adverse status is not lost through canonical selection.
15. Downstream owners receive mapping and impact facts to correct references under their authority; historical transactions retain original source identifiers plus canonical linkage.
16. The approver confirms the determination, canonical identity, aliases, effective time, unresolved items, and downstream referrals.
17. Evidence, decisions, mappings, corrections, affected records, actors, timestamps, and versions remain auditable.

---

# 10. Alternate Business Flows

## 10.1 Records Are Distinct Entities

The non-duplicate decision is recorded with distinguishing attributes. Any temporary hold is released.

## 10.2 Branch or Site of Same Legal Entity

The canonical legal entity and governed branch or site relationship are recorded without flattening operational distinctions.

## 10.3 Legal Succession or Merger

The case is referred to sensitive profile, legal succession, contract novation, tax, and finance authority. Duplicate reconciliation alone cannot transfer liability.

## 10.4 Duplicate Has Conflicting Adverse Status

The most relevant authorised status is preserved and reviewed; canonical selection cannot cleanse restriction, suspension, blacklisting, or incident history.

## 10.5 Duplicate Used for Payment Diversion

The case is urgently referred to finance, banking, fraud, security, legal, and audit. Payment authority remains separate.

## 10.6 Evidence Inconclusive

Records remain separately controlled and marked unresolved; stricter new-use review may apply until determination.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Incorrect Merge

The relationship is reversed through attributable correction, distinct identities and links are restored, and affected transactions are reviewed.

## 11.2 Transaction Lineage Lost

Reconciliation is held or corrected; original identifiers, source references, and audit evidence are recovered before completion.

## 11.3 Canonical Record Selected on Weak Evidence

The decision is remanded and operational reliance restricted until independent review.

## 11.4 Unauthorised Deletion or Attribute Overwrite

The event is contained and referred to data governance, security, privacy, finance, and audit.

## 11.5 Mapping Not Propagated

The mapping is reissued and downstream inconsistencies reconciled without rewriting original transactions.

## 11.6 System Interruption

Approved canonical identity and alias controls remain effective and are reconciled on recovery.

---

# 12. Business Rules and Decision Constraints

- Duplicate candidates require governed evidence and approval; similarity alone cannot merge identities.
- Canonical identity represents the authoritative entity, while every source identifier and historical record remains traceable.
- Non-canonical records are retained as aliases or source identities and are not deleted.
- Legal succession, branch structure, consortium membership, and related ownership are distinct from duplication.
- Canonical selection cannot erase adverse governance, qualification gaps, inactive or closed states, or transaction history.
- Contracts, payables, payments, tax positions, balances, and liabilities are not combined or transferred automatically.
- Sensitive profile and banking-reference changes require their own authority.
- Historical records retain original identity plus canonical mapping to preserve reconstruction.
- Correction creates attributable versions; silent overwrite and identifier reuse are prohibited.
- Unresolved identity risk may block new use but does not prove fraud or duplication.

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
| `SupplierMergeCompleted` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| Registration / Profile / Qualification | Supplies identity and governance evidence. | Retains source history. |
| Legal / Tax / Corporate Registry | Confirms entity, branch, succession, and registration facts. | Retains specialist authority. |
| Contract / Finance | Inventories affected obligations and transactions. | No automatic combination or transfer. |
| L3-J-036 | Maps external references to canonical identity. | Does not determine identity independently. |
| Audit / Privacy / Security | Reviews lineage, access, fraud, and correction. | Retains control authority. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J02 roles are separated as policy requires: Supplier Representative; Institutional Verifier; and Supplier Governance Authority. The Supplier Representative cannot verify, approve, or remove an institutional restriction on itself, and a conflicted verifier cannot decide the governed outcome.

## 15.2 Required Evidence

- every candidate identifier, source, legal and trade names, constitution, registrations, tax identifiers, addresses, ownership, contacts, representatives, banking references, profile and qualification history;
- governance states, performance, incidents, sourcing, awards, commitments, contracts, receipts, invoices, payables, payments, tax, external references, and archive links;
- comparison, source authority, conflicts, supplier clarification, determination, canonical selection, aliases, corrections, downstream referrals, actors, timestamps, and versions.

## 15.3 Audit Obligations

The institution must reconstruct why duplication was suspected, which evidence proved or disproved identity, why the canonical record was selected, what aliases and conflicting facts existed, how every transaction remained traceable, and which downstream corrections occurred.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- duplicate or potentially duplicate supplier records have been identified through authoritative matching criteria;
- an authorised Supplier Governance Authority has reviewed, assessed, and made a governance decision on each match as a merge, link, separation, or retention with explicit reason;
- the canonical surviving identity has been established and is the authoritative institutional supplier reference;
- non-surviving duplicate records are preserved as historical evidence and are not silently deleted or overwritten;
- all downstream references, commitments, awards, purchase orders, and qualifications have been assessed for re-linking under the canonical identity;
- the reconciliation decision, evidence, authority, actors, and audit trail are preserved and immutable.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- one canonical supplier identifier is the authoritative institutional reference for the reconciled entity;
- duplicate identifiers are deprecated, linked, or closed with explicit reasoning permanently on record;
- downstream institutional references, commitments, and qualifications reflect the canonical identity;
- the prior duplicate history, evidence, and Supplier Governance Authority decisions are permanently preserved and auditable;
- no supplier history, transaction, or governance evidence has been destroyed by the reconciliation process.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Matching attributes, authoritative sources, confidence standards, approval levels, and operational hold rules require confirmation.
2. Branch, establishment, proprietor, partnership, consortium, merger, succession, and related-party rules require legal mapping.
3. Transaction correction and reporting treatment require upstream owner-specific correction contracts; this L3 scenario cannot re-own them.

## 17.2 Assumptions

- Reconciliation never deletes source records or rewrites transaction history.
- Financial and legal consequences remain with their authoritative owners.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-035**
