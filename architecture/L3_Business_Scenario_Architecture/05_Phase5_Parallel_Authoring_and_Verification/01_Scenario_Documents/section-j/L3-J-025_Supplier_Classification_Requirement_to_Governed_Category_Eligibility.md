# Supplier Classification Requirement to Governed Category Eligibility

# 1. Scenario Metadata

**Scenario ID:** L3-J-025  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Vendor & Supplier Management  
**Participating Domains:** J  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** supplier search, sourcing eligibility, category planning, qualification, award, spend analysis, performance, renewal, restriction, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs assignment of suppliers to controlled procurement categories and determination of the exact category eligibility that may be used institutionally. It connects canonical supplier identity and qualification facts to a governed category taxonomy without allowing self-selected labels or administrative tagging to create sourcing entitlement.

---

# 3. Business Outcome

A supplier classification requirement reaches Approved Category Eligibility, Conditional Category Eligibility, Classification Only, Returned, or Rejected. The result identifies taxonomy version, category and subcategory, supplier role, goods, works or services scope, geography, value or risk limitations, qualification dependency, effective and expiry dates, conditions, and authority.

Outcome labels in this section are business dispositions or supporting-record results, not additional canonical lifecycle states. Section 13 states the only canonical lifecycle impact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a Registered or Approved Supplier requires classification or eligibility determination against an institutional procurement category taxonomy.

## 4.2 End Boundary

The scenario ends when the category mapping and eligibility consequence are approved, returned, or rejected; sourcing can distinguish descriptive classification from usable eligibility; and taxonomy or supplier changes have a governed reassessment path.

## 4.3 In Scope

- identifying the supplier, requested categories, goods, works, services, manufacturer, distributor, dealer, contractor, consultant, or other role;
- validating the current institutional category taxonomy and applicable version;
- mapping supplier capability, authorisations, licences, products, services, geography, value, risk, and qualification scope to categories;
- distinguishing descriptive classification, qualified eligibility, conditional eligibility, and prohibited or unsupported scope;
- checking overlaps, exclusions, parent-child categories, multiple roles, and aggregate capacity;
- approving effective category eligibility with conditions and validity;
- preserving taxonomy version, evidence, decision, and change lineage.

## 4.4 Out of Scope

- creating supplier identity or Active status;
- performing the underlying qualification governed by L3-J-022;
- renewing or restricting expired qualification governed by L3-J-023 and L3-J-024;
- general or sensitive supplier profile changes governed by L3-J-026 and L3-J-027;
- event-specific technical evaluation, award, Purchase Order, contract, receipt, payable, accounting entry, or payment;
- category-taxonomy technical design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-020 and L3-J-021 | Supply canonical identity and Active status where eligibility is needed. |
| Related | L3-J-022 | Supplies qualification scope supporting eligibility. |
| Related | L3-J-023 and L3-J-024 | Renewal or expiry changes category eligibility. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Vendor & Supplier Management | Owns taxonomy governance interface, supplier classification review, category eligibility decision, conditions, validity, and evidence. |

Category and technical owners define controlled category semantics. Supplier governance retains identity, approval, and qualification truth.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Supplier Representative | Supplies permitted qualification or classification evidence; cannot approve its own eligibility. | Submits and responds. |
| Institutional Verifier | Verifies identity and supporting evidence independently. | Verifies evidence. |
| Qualification Authority | Grants, expires or changes category- and scope-specific qualification. | Decides qualification and eligibility. |
| Supplier Governance Authority | Applies supplier restriction or other governance treatment where the outcome requires it. | Decides governed supplier treatment. |
---

# 7. Preconditions and Entry Criteria

- A canonical supplier identity exists.
- An approved institutional category taxonomy and version are available.
- The requested supplier role, goods, works, services, geography, and category interests are identifiable.
- Rules distinguish classification from Active and Qualified eligibility.
- Required supplier status, qualification, licences, authorisations, and evidence can be verified.
- Category ownership, approval authority, validity, and reassessment rules are effective.

---

# 8. Trigger

A supplier, category owner, procurement officer, sourcing process, or taxonomy change creates a requirement to establish or revise supplier category classification and eligibility.

---

# 9. Main Business Flow

1. The institution records the supplier, requested categories, roles, products or services, geography, intended use, and initiating actor.
2. Canonical identity, Active status where required, conditions, restrictions, and existing category mappings are verified.
3. The current taxonomy version, category definitions, parent-child structure, exclusions, risk level, and qualification dependencies are identified.
4. Supplier catalogues, licences, manufacturer or principal authorisations, dealership or distributorship scope, professional credentials, experience, facilities, personnel, and service coverage are captured.
5. Requested goods, works, and services are mapped to the most specific governed categories without using free-text labels as authoritative classifications.
6. Supplier role is determined for each category, such as manufacturer, authorised dealer, distributor, service provider, contractor, consultant, agent, consortium member, or other controlled role.
7. Required Active status and qualification positions are matched by category, subcategory, geography, value band, risk, funding context, validity, and conditions.
8. Descriptive classifications lacking qualification or approval are marked Classification Only and cannot authorise sourcing where eligibility is required.
9. Overlapping, conflicting, parent-only, obsolete, duplicate, overly broad, or unsupported requests are resolved with the category owner.
10. Aggregate capacity and multiple-category exposure are reviewed where eligibility depends on shared personnel, facilities, equipment, or financial capacity.
11. Conflicts and related-party facts involving supplier, category sponsor, reviewer, or approver are declared and governed.
12. The recommendation identifies approved, conditional, classification-only, returned, or rejected mappings, with evidence and limitations.
13. Effective and expiry dates are derived from supplier status, qualification, authorisation, licence, taxonomy, and approved rule, using the controlling limitation.
14. The Qualification Authority approves, conditions, limits, returns, or rejects each category position.
15. Approved category eligibility becomes visible for supplier search and sourcing only within exact scope, validity, and conditions.
16. Later taxonomy, supplier status, qualification, licence, authorisation, or capacity change triggers reassessment rather than silent remapping.
17. Requests, mappings, evidence, taxonomy versions, decisions, conditions, actors, timestamps, and history remain auditable.

---

# 10. Alternate Business Flows

## 10.1 Multiple Categories

Each category is independently evaluated, while shared capacity, role, qualification, and aggregate exposure are controlled.

## 10.2 Classification Without Eligibility

A Registered supplier may be descriptively classified for discovery or future development, but sourcing entitlement remains blocked until Active and qualified requirements are satisfied.

## 10.3 Manufacturer and Channel Partner

Principal, authorisation chain, territory, product families, warranty, support, validity, and role boundaries are explicit for both entities.

## 10.4 Consortium or Joint Venture

Category eligibility identifies member contributions, lead role, joint scope, capacity, validity, and whether eligibility survives membership change.

## 10.5 Taxonomy Revision

Mappings are translated through an approved crosswalk and reviewed for changed semantics. Old taxonomy positions remain historical and are not overwritten.

## 10.6 Conditional or Limited Eligibility

Eligibility may be limited by location, value, risk, product line, licence, supervision, subcontracting, or qualification condition.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Category Ambiguous

The request is returned to the category owner; a broad or convenient category is not assigned merely to complete registration.

## 11.2 Qualification Missing or Expired

The position remains Classification Only or Restricted. It cannot become eligible through category tagging.

## 11.3 Supplier Non-Active or Restricted

New eligibility is blocked and existing category positions reflect the authoritative status or restriction.

## 11.4 Authorisation or Licence Invalid

Affected role and category eligibility are returned, rejected, or restricted and may trigger integrity review.

## 11.5 Taxonomy Change Interrupted

Old and new versions remain explicit; ambiguous crosswalk positions are restricted until approved.

## 11.6 Unauthorised Manual Classification

The mapping is blocked or corrected, and any downstream reliance is preserved for review.

## 11.7 Post-Approval Capability Change

The affected position is reassessed through qualification or profile-change governance without deleting history.

---

# 12. Business Rules and Decision Constraints

- Category classification and category eligibility are distinct facts.
- Free-text supplier claims cannot create authoritative category eligibility.
- Eligibility must be supported by current supplier status, qualification, evidence, and controlled taxonomy rules.
- The most specific applicable category should be used; broad parent eligibility is not inferred automatically.
- Manufacturer, dealer, distributor, agent, contractor, consultant, and consortium roles must be explicit.
- Category eligibility is limited by the shortest controlling validity and all relevant conditions.
- Taxonomy changes require versioned mapping and cannot silently rewrite history.
- Expired or restricted qualification blocks dependent category eligibility.
- Category eligibility does not guarantee invitation, award, commitment, or payment.
- This scenario does not create a Purchase Order, contract, receipt, payable, accounting entry, or payment.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Supplier | `Active` | Qualification, renewal or classification evidence is governed without changing the Supplier lifecycle unless an explicit J02 governance transition applies. | `Active` | Vendor & Supplier Management (J02) | `L2-J02` Section 6 |
| SupplierQualification | No canonical lifecycle defined | Creates, expires or changes category- and scope-specific qualification as a governed record. | Governed qualification record without a locally invented state | Vendor & Supplier Management (J02) | `L2-J02` Section 5 and Section 13 |
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `SupplierQualificationGranted` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| L3-J-020 / L3-J-021 | Supply canonical identity and Active status. | Retain identity and approval. |
| L3-J-022 to L3-J-024 | Supply qualification, renewal, and restriction facts. | Retain eligibility foundation. |
| Category / Technical Owners | Supply taxonomy meaning and capability interpretation. | Do not create supplier approval alone. |
| Sourcing | Uses category eligibility with event-specific criteria. | Retains sourcing and evaluation. |
| J04 / Finance | Consume supplier and category facts only through their own controls. | Retain commitment and monetary truth. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J02 roles are separated as policy requires: Supplier Representative; Institutional Verifier; Qualification Authority; and Supplier Governance Authority. The Supplier Representative cannot verify, qualify, approve, or remove an institutional restriction on itself.

## 15.2 Required Evidence

- canonical supplier, status, conditions, requested categories, roles, products, services, geography, and purpose;
- taxonomy version, definitions, hierarchy, exclusions, risk, qualification dependencies, and crosswalks;
- catalogues, licences, authorisations, credentials, qualification, capacity, experience, facilities, personnel, territory, and validity;
- mapping analysis, conflicts, conditions, recommendation, approval or rejection, effective dates, actors, reasons, timestamps, and versions.

## 15.3 Audit Obligations

The institution must reconstruct why each supplier was classified or eligible in each category, which taxonomy and evidence applied, what role and limitations existed, who approved the mapping, and whether sourcing respected scope and validity.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- the supplier has been assessed against the institutional classification criteria for the requested category, type, size, geography, or status designation;
- classification evidence and governing vocabulary references have been verified against acceptable sources;
- the classification decision has been made by an authority with the delegated authority to assign the applicable classification;
- classification scope, effective dates, review cycle, conditions, and limitations are explicit;
- the classification decision, evidence, authority, and audit trail are preserved and immutable;
- where the supplier does not meet the classification criteria, the outcome is documented and the supplier remains in its prior classification state.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the supplier holds an assigned category eligibility, type, size, or status classification for the defined scope and effective period;
- the classification is attributable to specific evidence and an institutional authority decision;
- the classification does not by itself create a sourcing preference, award entitlement, or financial obligation;
- the complete classification evidence, decision, and conditions are permanently auditable;
- reclassification, restriction, or revocation requires a separate governed action with attributed authority.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Institutional taxonomy, hierarchy, roles, risk classes, crosswalks, and ownership require confirmation.
2. Categories requiring standing qualification versus event-only evaluation require method-specific governance.
3. Capacity allocation across categories and consortium or channel relationships requires an upstream J02 qualification business rule.

## 17.2 Assumptions

- Category eligibility depends on valid supplier status and qualification where required.
- General profile-change scenarios begin at L3-J-026 and L3-J-027.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-025**
