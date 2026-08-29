# External Procurement Requirement to Institutionally Recognised Sourcing Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-J-017  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Procurement Planning & Sourcing  
**Participating Domains:** J  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** downstream commitment, supplier, receipt, payable, funding, reporting, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs institutional recognition of a sourcing outcome produced by an authorised external procurement body, such as a government marketplace, central procurement agency, lead institution, parent entity, consortium, funding agency, framework owner, or other competent body.

It enables legitimate external sourcing to be used without pretending that the institution conducted or approved the external process, while requiring sufficient evidence, institutional eligibility, demand mapping, local acceptance, and downstream control.

---

# 3. Business Outcome

An externally conducted procurement requirement reaches an Institutionally Recognised Sourcing Outcome, Conditional Recognition, Returned outcome, or Rejected Recognition. Recognition identifies the external authority, process, reference, award, supplier, scope, terms, dates, institutional entitlement, mapped internal demand, limitations, approval, and evidence lineage without re-performing the external evaluation or creating a Purchase Order or financial record.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when approved institutional demand is proposed to be satisfied using an identifiable external procurement process or award and its evidence is submitted for institutional recognition.

## 4.2 End Boundary

The scenario ends when the institution recognises, conditionally recognises, returns, or rejects the external outcome; internal demand and permitted scope are mapped; and the result is available to a separate commitment or alternate sourcing path.

## 4.3 In Scope

- validating the external body's identity, mandate, authority, jurisdiction, process role, and procurement reference;
- validating method, dates, scope, evaluation and award evidence at the level required for reliance;
- verifying that the institution and its demand are eligible to use the external outcome;
- mapping external lots, suppliers, quantities, prices, terms, ceilings, periods, and locations to approved internal demand;
- verifying supplier eligibility for institutional use and institution-specific compliance conditions;
- identifying conflicts, funding conditions, data, security, tax, legal, commercial, and operational gaps;
- approving, limiting, conditioning, returning, rejecting, expiring, or superseding internal recognition;
- preserving external identifiers and original evidence without claiming ownership of the external decision.

## 4.4 Out of Scope

- re-performing or retroactively approving the external body's sourcing evaluation;
- altering external bids, scoring, award, contract, marketplace, framework, or supplier records;
- approving internal demand that lacks its own authority;
- creating a Purchase Order, contract, receipt, payable, accounting entry, or payment;
- technical integration with external portals or marketplaces.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-J-004 to L3-J-010 | Govern institution-owned sourcing methods and outcomes. |
| Related | L3-J-011 to L3-J-014 | Govern institution-owned rate-contract arrangements. |
| Followed by | J04 commitment scenarios | Separately authorise institutional commitment against a recognised source. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Procurement Planning and Sourcing | Owns institutional reliance assessment, demand mapping, local supplier and condition checks, recognition decision, validity, downstream availability, and internal evidence. |

The external body retains ownership of its procurement process and award. Requesting units retain demand authority, J02 supplier truth, J04 commitment truth, and Finance monetary truth.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Procurement Officer | Administers the governed exception, external-procurement or closure activity. | Performs procurement administration. |
| Requesting Unit Representative | Supplies requirement, urgency or operational context without approving the procurement outcome. | Provides business input. |
| Procurement Recommender | Records the attributable recommendation and supporting basis. | Recommends. |
| Procurement Approver | Approves, rejects or returns the outcome within established authority. | Decides. |
| External Procurement Coordinator | Records externally executed procurement where applicable without changing the external record. | Coordinates external-procurement recognition only. |
| Auditor / Reviewer | Reviews authority, evidence and decision history without modifying the outcome. | Reviews evidence. |
---

# 7. Preconditions and Entry Criteria

- Approved, current, identifiable institutional demand exists and is available for sourcing.
- The external procurement body, process, reference, award, supplier, scope, dates, and evidence source are identifiable.
- A lawful or policy-permitted route exists for institutional reliance on the external procurement.
- The institution's eligibility, participation, membership, funding, mandate, geography, or beneficiary status can be established.
- External terms and the institution's local approval, conflict, supplier, legal, tax, security, and budget rules are available.
- External evidence can be retained or reliably referenced for the required audit period.

---

# 8. Trigger

An authorised role submits approved institutional demand and an external procurement outcome for recognition as a permissible internal sourcing basis.

---

# 9. Main Business Flow

1. Procurement records the internal demand, external body, procurement and award identifiers, evidence source, proposed supplier, initiating actor, and submission time.
2. Internal demand approval, remaining quantity, specification, value, funding context, delivery need, and sourcing availability are verified.
3. The external body's legal or institutional identity, mandate, jurisdiction, delegation, marketplace, consortium, framework, funding, or lead-procurement role is verified.
4. Procurement establishes that the external process and award are authentic, current, final or otherwise usable, and not suspended, cancelled, expired, challenged, or superseded.
5. Required external evidence is reviewed, including method, notice where applicable, scope, evaluation, award approval, supplier, lot, prices, terms, dates, ceilings, amendments, and usage rules.
6. The institution's eligibility to rely on the outcome is verified through membership, mandate, beneficiary status, funding agreement, framework participation, geographic coverage, or another permitted basis.
7. External items, lots, quantities, services, locations, supplier positions, value ceilings, validity, and call-off conditions are mapped precisely to approved internal demand.
8. Any gap between internal specification or need and the external scope is identified; recognition cannot silently expand either record.
9. Supplier identity, local eligibility, restrictions, conflicts, tax, ownership, registration, capacity, and any mandatory institutional conditions are verified.
10. Commercial terms, currency, tax, logistics, warranty, service, licensing, data, security, law, dispute, funding, reporting, and total institutional cost are assessed.
11. Procurement distinguishes facts accepted from the external owner from institution-specific checks and explicitly records evidence that could not be independently verified.
12. Conditions for use, such as supplier confirmation, remaining ceiling, external approval, local security review, funding consent, or time limit, are documented.
13. Procurement prepares a recognition recommendation specifying exact demand, external source, supplier, scope, value basis, validity, limitations, evidence, and downstream path.
14. Required specialist and local governance reviews are completed; they do not rewrite or purport to approve the external procurement.
15. The authorised approver recognises, conditionally recognises, limits, returns, or rejects institutional reliance with reason and effective time.
16. Recognition creates an internal controlled sourcing outcome and consumes or reserves only the mapped demand and external entitlement; a separate commitment remains required.
17. External originals or authoritative references, internal checks, mapping, conditions, decisions, updates, actors, and timestamps remain linked and auditable.

---

# 10. Alternate Business Flows

## 10.1 Government Marketplace or Central Procurement

The institution validates marketplace or central-body authority, product or lot status, seller position, price and validity, buyer eligibility, and applicable purchase conditions.

## 10.2 Lead Institution or Consortium Procurement

The participation agreement, delegation, allocation, cost-sharing, award, institution share, supplier acceptance, and member-specific obligations are verified.

## 10.3 Funding-Agency Directed Procurement

Funding conditions, eligible expenditure, method approval, supplier or award status, donor no-objection where applicable, reporting, currency, tax, and institutional authority are mapped.

## 10.4 External Framework or Parent-Entity Arrangement

The institution validates permitted users, scope, call-off rules, ceiling, geography, term, prices, amendments, and whether a local competition or confirmation remains necessary.

## 10.5 Conditional Recognition

Recognition may be issued subject to defined evidence, funding, supplier, security, legal, ceiling, or external-confirmation conditions. Downstream use remains blocked until mandatory conditions are satisfied.

## 10.6 Partial Recognition

Only conforming lots, quantities, periods, suppliers, or locations are recognised. Residual demand remains available for another governed route.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 External Authority Not Established

Recognition is rejected or returned; an external document alone is not sufficient authority for institutional reliance.

## 11.2 Evidence Incomplete or Inauthentic

The case is held or returned for authoritative evidence. Unsupported assumptions are not converted into recognised facts.

## 11.3 Institution Not Eligible

Recognition is rejected or limited. Eligibility cannot be inferred solely from supplier willingness.

## 11.4 Demand or Scope Mismatch

Only the matched portion may be recognised. The remainder is corrected, changed through demand governance, or sourced separately.

## 11.5 Supplier Locally Restricted

Downstream use is blocked or escalated even if the external body awarded the supplier; J02 retains authoritative institutional supplier status.

## 11.6 External Outcome Changes

Suspension, amendment, expiry, challenge, cancellation, supplier change, price change, or ceiling exhaustion triggers reassessment, limitation, withdrawal, or supersession of recognition.

## 11.7 External Evidence Becomes Unavailable

The institution preserves retained copies, hashes or authoritative references where permitted, records the gap, and blocks new reliance if continuing validity cannot be demonstrated.

---

# 12. Business Rules and Decision Constraints

- Recognition is an internal decision to rely on an external outcome; it is not re-performance or retroactive approval of the external procurement.
- The external owner, mandate, process, award, supplier, terms, and validity must be identifiable and sufficiently evidenced.
- Institutional eligibility and approved internal demand are mandatory.
- External scope cannot be silently expanded and internal demand cannot be rewritten through recognition.
- Supplier eligibility for institutional use remains subject to J02 and mandatory local law or policy.
- External award approval does not establish local budget, commitment, tax, security, receipt, payable, or payment authority.
- Conditional recognition cannot be used downstream until mandatory conditions are satisfied.
- Recognition is limited by the shorter of internal demand validity, external validity, supplier status, available ceiling, funding, or approved recognition period.
- External changes must be assessed before new reliance.
- Original identifiers, documents, versions, facts, limitations, and source ownership must remain preserved.
- Duplicate demand, entitlement, ceiling, or commitment use is prohibited.
- This scenario does not create a Purchase Order, contract, receipt, payable, accounting entry, or payment.

---

# 13. State and Lifecycle Impact

J01 defines exact external-procurement events but no separate canonical lifecycle for an internal recognition case. This scenario records the governed recognition outcome and its source references without creating local review, conditional-recognition, suspension, withdrawal, supersession or expiry states. Any affected Sourcing Event remains in its existing J01 lifecycle position.
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `ExternalProcurementRegistered` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `ExternalProcurementStatusRecorded` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `ExternalEvaluationOutcomeRecorded` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `ExternalSupplierSelectionRecorded` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `ExternalProcurementOutcomeApproved` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| External Procurement Body | Supplies authoritative process, award, term, usage, and change facts. | Retains external process ownership. |
| Requesting Unit / J03 | Supplies approved demand and accepts only mapped scope. | Retains demand authority. |
| J02 | Supplies institutional supplier identity, eligibility, ownership, restriction, and performance facts. | Retains supplier lifecycle. |
| J04 | Consumes a valid recognised outcome for separate commitment control. | Retains commitment lifecycle. |
| Finance / Funding Owner | Supplies funding eligibility, budget, tax, currency, ceiling, and reporting facts. | Retains financial truth. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J01 roles are separated as institutional policy requires: Procurement Officer; Procurement Manager; Requesting Unit Representative; Technical Evaluator; Commercial Evaluator; Committee Member and Committee Chair / Convenor where used; Procurement Recommender; Procurement Approver; External Procurement Coordinator where applicable; and Auditor / Reviewer. A requester, evaluator, recommender, or conflicted participant cannot replace the Procurement Approver decision.

## 15.2 Required Evidence

- approved internal demand, specification, quantity, value, funding context, dates, and remaining availability;
- external body identity, mandate, delegation, process and award identifiers, method, approvals, supplier, lot, prices, terms, validity, ceiling, amendments, and status;
- institutional membership, beneficiary, funding, framework, consortium, marketplace, or other reliance eligibility;
- scope and quantity mapping, gaps, supplier eligibility, conflicts, commercial and specialist review, and conditions;
- recommendation, approval or rejection, validity, limitations, updates, actors, reasons, source references, versions, and timestamps.

## 15.3 Audit Obligations

The institution must reconstruct which external body decided what, why the institution was entitled to rely on it, which internal demand and external scope matched, which facts were independently checked, what limitations remained, who approved recognition, and how later external changes affected use.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- an externally originated procurement requirement has been reviewed and determined to satisfy or not satisfy the criteria for institutional recognition;
- where recognised, the external outcome is mapped to an institutional demand reference, supplier identity, and governed commitment basis;
- the external originating authority, jurisdiction, and recognition basis are documented and attributable;
- any institutional conditions, deviations, limitations, or required confirmations applied to recognition are explicit;
- recognised and rejected external outcomes are distinguished and governed under separate dispositions;
- the recognition decision, authority, and audit evidence are preserved and immutable.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the external procurement outcome is either institutionally recognised with a traceable demand association or rejected with a documented reason;
- recognised outcomes are available as governed facts for downstream commitment, delivery, and payable scenarios;
- the external originating authority and institutional recognition basis are permanently preserved and auditable;
- no duplicate internal sourcing event is required for a fully recognised external outcome;
- the institution retains its own compliance obligations regardless of the external originating authority.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Approved external-body types, minimum evidence, reliance agreements, and recognition thresholds require institution-specific confirmation.
2. Required local revalidation by external method, funding source, category, and risk requires operating-profile governance.
3. Evidence-retention and change-notification obligations with each external body require agreement mapping.

## 17.2 Assumptions

- The institution has not conducted or owned the external sourcing process.
- Recognition remains distinct from internal demand approval, supplier status, commitment, receipt, and financial truth.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-017**
