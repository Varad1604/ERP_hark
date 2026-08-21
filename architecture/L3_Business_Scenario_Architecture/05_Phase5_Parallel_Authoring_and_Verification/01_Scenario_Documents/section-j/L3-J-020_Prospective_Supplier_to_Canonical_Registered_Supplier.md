# Prospective Supplier to Canonical Registered Supplier

# 1. Scenario Metadata

**Scenario ID:** L3-J-020  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Vendor & Supplier Management  
**Participating Domains:** J  
**Scenario Type:** Simple  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** supplier approval, qualification, sourcing, commitment, receipt, payable, tax, compliance, performance, restriction, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs conversion of a prospective supplier into one canonical Registered Supplier identity. It establishes a verified, deduplicated, attributable supplier record with controlled legal identity, ownership, tax, address, contact, banking-reference, classification, and evidence facts.

Registration is an identity and record-governance outcome only. It does not approve the supplier, qualify it for a category, create sourcing entitlement, authorise an award or commitment, validate bank-payment instructions, or establish financial liability.

---

# 3. Business Outcome

A prospective supplier reaches Canonical Registered Supplier, Returned for Correction, Rejected Registration, or Matched to Existing Supplier. The registered outcome provides a unique institutional supplier identifier, verified identity attributes, provenance, evidence validity, duplicate and relationship links, declared ownership and conflicts, controlled contacts, classifications, and lifecycle status.

Outcome labels in this section are business dispositions or supporting-record results, not additional canonical lifecycle states. Section 13 states the only canonical lifecycle impact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a prospective supplier submits registration information or an authorised institutional role initiates supplier registration for a legitimate procurement purpose.

## 4.2 End Boundary

The scenario ends when the candidate is registered under one canonical identity, linked to an existing supplier, returned, or rejected; sensitive evidence is controlled; and the resulting fact is available to the separate approval process.

## 4.3 In Scope

- capturing legal name, constitution, registration identifiers, tax identifiers, addresses, contacts, ownership, beneficial ownership, authorised representatives, classifications, and evidence;
- verifying identity against authoritative or acceptable sources;
- normalising names, addresses, identifiers, and classifications without changing legal truth;
- searching for duplicates, aliases, branches, related entities, shared ownership, shared banking references, and predecessor or successor relationships;
- identifying supplier type, geography, category interests, MSME or other declared status, and institutional relationship;
- obtaining declarations, consent, confidentiality acknowledgement, and data-use notice where required;
- creating the unique canonical supplier record and preserving provenance, versions, validity, and correction history.

## 4.4 Out of Scope

- approving or activating the supplier, governed by L3-J-021;
- establishing category, technical, financial, regulatory, geographical, or institutional qualification, governed by L3-J-022;
- validating a payment instruction or changing a bank account for payment;
- awarding a sourcing event, creating a Purchase Order or contract, recording receipt, payable, accounting entry, or payment;
- supplier performance assessment, suspension, blacklisting, or closure;
- technical master-data model, integration, user interface, or document-storage design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Followed by | L3-J-021 | Separately approves and activates a Registered Supplier. |
| Followed by | L3-J-022 | Separately establishes scope-specific institutional eligibility. |
| Related | Sourcing and J04 commitment scenarios | Consume supplier facts but cannot infer approval from registration. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Vendor & Supplier Management | Owns supplier identity intake, verification, deduplication, canonical registration, classification, data stewardship, and audit evidence. |

Finance and tax roles retain financial and statutory interpretation. Legal and compliance roles retain ownership and regulatory interpretation. Registration does not replace their later validations.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Supplier Representative | Supplies permitted supplier information and evidence; cannot verify or approve its own governed state. | Submits and responds. |
| Supplier Creator / Onboarding Requester | Registers and submits the prospective supplier. | Initiates onboarding. |
| Institutional Verifier | Performs supplier identity and evidence verification independently. | Verifies and records evidence decisions. |
| Supplier Approver | Approves, rejects or returns onboarding and activation within established authority. | Decides. |
| Supplier Governance Authority | Governs duplicate, merge or other supplier-governance treatment where material. | Decides governed supplier treatment. |
---

# 7. Preconditions and Entry Criteria

- A legitimate supplier-registration purpose and initiating party are identifiable.
- The supplier or authorised representative can be authenticated to the degree required.
- Mandatory identity fields, document types, declarations, privacy notices, verification sources, duplicate rules, and registration authorities are effective.
- The candidate's legal constitution and jurisdiction can be identified.
- Existing supplier records are available for duplicate and relationship checks.
- Sensitive tax, ownership, identity, contact, and banking-reference evidence can be handled under appropriate access and retention rules.

---

# 8. Trigger

A prospective supplier submits a registration request, or an authorised procurement role requests creation of a supplier identity needed for a governed institutional process.

---

# 9. Main Business Flow

1. The institution records the registration request, submission channel, supplier representative, sponsor where applicable, purpose, consent or notice, and submission time.
2. Mandatory information and evidence are captured according to supplier constitution, jurisdiction, category, risk, and institutional profile.
3. The representative's authority to submit and attest for the supplier is verified.
4. Legal name, trade names, constitution, incorporation or registration number, tax identifiers, registered address, operating addresses, and authoritative dates are verified against acceptable sources.
5. Ownership, beneficial ownership, directors or trustees, controlling persons, parent, subsidiaries, branches, consortium relationships, and related parties are captured where required.
6. Contact persons, official communication channels, authorised roles, and their validity are recorded separately from the legal identity.
7. Supplier names, identifiers, addresses, contacts, ownership, and banking references are normalised for comparison while original submitted values remain preserved.
8. Duplicate and relationship checks compare exact and fuzzy identifiers, aliases, shared contacts, shared addresses, ownership, tax numbers, registrations, and bank-reference indicators.
9. Potential matches are reviewed by the Institutional Verifier and resolved as new identity, existing identity update, branch or site, related entity, predecessor or successor, duplicate, or unresolved relationship.
10. Required conflict, related-party, sanctions, prohibited-entity, debarment, and institutional-affiliation declarations are captured or referred for specialist review without converting registration into approval.
11. Supplier type, size or declared status, geography, goods, works, service interests, category interests, and other controlled classifications are assigned from governed vocabularies.
12. Evidence source, issuer, issue date, expiry date, verification result, version, language or translation, and any limitation are recorded.
13. The Institutional Verifier prepares a recommendation to register, link to an existing record, return for correction, or reject, with reasons and unresolved issues.
14. The Supplier Approver confirms completeness, identity confidence, duplicate disposition, required declarations, and data-governance conditions.
15. On approval, one canonical supplier identifier is created and the Supplier enters the canonical `Registered` state and has not yet reached `Approved` or `Active`.
16. Existing matches are linked or updated through controlled change governance; records are not silently merged or deleted.
17. The supplier and authorised internal roles receive the registration outcome, while all submissions, verifications, decisions, actors, timestamps, and versions remain auditable.

---

# 10. Alternate Business Flows

## 10.1 Existing Canonical Supplier Found

The request is linked to the existing identity. New evidence is assessed as a governed update, and no duplicate supplier identifier is issued.

## 10.2 Branch, Site, or Operating Unit

The institution determines whether the unit is a separate legal supplier, a child organisational record, or an operating location. It receives only the identity structure permitted by authoritative legal facts.

## 10.3 Foreign Supplier

Foreign registration, tax, address, ownership, language, authentication, currency, and equivalent-document rules apply. Lack of a domestic identifier does not justify invented data.

## 10.4 Individual, Professional, or Sole Proprietor

Identity and privacy evidence is collected proportionately, and personal information receives restricted access and retention treatment.

## 10.5 Government, Public, Educational, or Nonprofit Entity

Constitution and authority evidence appropriate to that entity type replaces inapplicable corporate documents.

## 10.6 Consortium or Joint Venture

Members, lead party, joint authority, liability, ownership, tax treatment, validity, and whether a distinct legal identity exists are explicitly recorded.

## 10.7 Supplier-Assisted Internal Registration

Where an internal role captures information on behalf of the supplier, the source, consent or authority, attestation gap, and required supplier confirmation remain visible.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Missing or Invalid Evidence

The request is returned with specific deficiencies and remains non-registered until corrected or formally rejected.

## 11.2 Duplicate Cannot Be Resolved

Canonical creation is held. A temporary case reference may support investigation but cannot be used as an approved supplier identity.

## 11.3 Identity or Ownership Inconsistency

The request is escalated for authoritative clarification, enhanced review, or rejection. Conflicting values are preserved rather than overwritten.

## 11.4 Fraud, Sanction, or Prohibited Relationship Concern

Registration may be held or rejected and the matter referred to the appropriate compliance, legal, security, or investigative process.

## 11.5 Representative Authority Fails

No supplier attestation is accepted. The supplier must provide an authorised representative or acceptable alternative evidence.

## 11.6 Service Interruption

The case resumes from the last verified checkpoint. Repeated submission does not create a second identity, and time-sensitive evidence is refreshed.

## 11.7 Post-Registration Error

Correction follows attributable master-data change governance. Historical values, evidence, approval, and effective dates remain preserved.

---

# 12. Business Rules and Decision Constraints

- One legal supplier must have one canonical institutional identity except where authoritative rules require distinct legal records.
- Registration establishes identity, not approval, qualification, sourcing entitlement, award, or payment authority.
- Original submitted values and evidence provenance must remain preserved after normalisation.
- Legal identity changes cannot be treated as ordinary contact edits.
- Duplicate records cannot be silently merged, reused, or deleted.
- Branch, site, trade name, parent, subsidiary, predecessor, successor, and related-party relationships must be explicit.
- Tax, registration, ownership, and representative evidence must be appropriate to constitution and jurisdiction.
- Banking details captured as references are not approved payment instructions and require separate controls.
- Sensitive personal, ownership, tax, identity, and banking information follows least-access, purpose, retention, correction, and disclosure rules.
- Expired evidence remains historical and cannot support current downstream eligibility.
- Supplier self-declaration does not replace independent verification where required.
- Registration rejection does not blacklist the supplier unless a separate authoritative process decides that outcome.
- This scenario does not create a Purchase Order, contract, receipt, payable, accounting entry, or payment.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Supplier | `Prospective / Registered` | Establishes the canonical supplier identity without inventing a registration-review lifecycle. | `Registered` within the canonical `Prospective / Registered` entry position | Vendor & Supplier Management (J02) | `L2-J02` Section 6 |
| SupplierMergeRecord | No canonical lifecycle defined | Records an existing match or duplicate treatment where required. | Governed record without a locally invented state | Vendor & Supplier Management (J02) | `L2-J02` Section 5 |
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `SupplierRegistered` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| Requesting / Procurement Unit | Supplies legitimate purpose and known supplier context. | Cannot establish legal identity alone. |
| Tax / Finance | Supplies tax and financial-reference interpretation. | Retains financial truth and payment controls. |
| Legal / Compliance | Reviews ownership, constitution, sanctions, prohibited relationships, and special entities. | Retains specialist authority. |
| L3-J-021 | Consumes the Registered Supplier for approval. | Retains approval and activation. |
| L3-J-022 | Consumes approved identity and evidence for qualification. | Retains scope-specific eligibility. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J02 roles are separated as policy requires: Supplier Representative; Supplier Creator / Onboarding Requester; Institutional Verifier; Supplier Approver; and Supplier Governance Authority. A supplier representative or conflicted requester cannot verify or approve its own governed outcome.

## 15.2 Required Evidence

- request purpose, supplier and representative identity, authority, consent or notice, and submission source;
- legal and trade names, constitution, registration, tax, address, contacts, ownership, control, related entities, and classifications;
- source documents, issuer, issue and expiry dates, validation method, verification outcome, translations, and limitations;
- duplicate searches, candidate matches, relationship analysis, steward resolution, aliases, predecessor or successor links;
- declarations, conflicts, sanctions or prohibited-entity referrals, recommendation, decision, actors, reasons, timestamps, and versions.

## 15.3 Audit Obligations

The institution must reconstruct who supplied each fact, which source verified it, how duplicates and relationships were assessed, why a new or existing identity was chosen, who approved registration, and every later correction or legal identity change.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- one canonical institutional supplier identifier has been issued for the prospective supplier or the candidate has been matched to an existing supplier, returned, or rejected with documented reasons;
- legal name, constitution, registration identifiers, tax identifiers, and registered address have been verified against acceptable sources;
- ownership, beneficial ownership, and related-party relationships have been captured where required;
- duplicate and relationship checks have been performed, resolved, and documented by an authorised Institutional Verifier;
- required declarations, consents, and conflict disclosures are captured and attributed;
- all evidence sources, issue dates, verification outcomes, and limitations are recorded;
- the registration decision, authority, actors, reasons, and audit trail are preserved and immutable.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- a canonical Supplier exists with a unique institutional identifier, verified identity attributes, and the canonical `Registered` state; it has not yet reached `Approved` or `Active`;
- the registration is distinguished from supplier approval, qualification, and sourcing entitlement;
- duplicate supplier identity creation has been prevented and any matches or relationships are explicitly linked;
- original submitted values and evidence provenance are preserved after any normalisation;
- the complete registration history, evidence, and governance trail is permanently auditable;
- no Purchase Order, commitment, award, sourcing authority, or payment obligation has been created by registration.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Mandatory fields, documents, authoritative sources, verification strength, and retention periods by supplier type and jurisdiction require confirmation.
2. Exact legal-entity, branch, site, consortium, individual, and foreign-supplier identity semantics require an upstream J02 entity-model decision before detailed design.
3. Duplicate confidence, merge, split, relationship, and survivorship authorities require data-governance rules.

## 17.2 Assumptions

- Registration is required before supplier approval.
- A canonical Registered Supplier is not automatically Active or Qualified.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-020**
