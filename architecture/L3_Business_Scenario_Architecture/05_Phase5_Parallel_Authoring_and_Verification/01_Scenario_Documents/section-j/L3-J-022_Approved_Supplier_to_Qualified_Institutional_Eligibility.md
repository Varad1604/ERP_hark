# Approved Supplier to Qualified Institutional Eligibility

# 1. Scenario Metadata

**Scenario ID:** L3-J-022  
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
**Referenced By:** sourcing, prequalification, bid eligibility, award, commitment, evidence renewal, qualification restriction, performance, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs establishment of scope-specific institutional eligibility for a Supplier that has reached canonical `Approved` and then `Active` states. It determines the categories, goods, works, services, locations, campuses, value bands, risk classes, funding contexts, licences, capacities, and time periods for which the supplier is qualified.

It prevents Active status from being interpreted as universal competence or eligibility and preserves the distinction between institutional approval, standing qualification, event-specific evaluation, award, and commitment.

---

# 3. Business Outcome

A Supplier in canonical `Active` reaches one or more Qualified Institutional Eligibility positions, Conditional Qualification, Returned Qualification, Qualification Rejected, or Not Applicable. Each position identifies exact scope, evidence, criteria, capacity, limitations, effective and expiry dates, review triggers, conditions, authority, and lineage.

Outcome labels in this section are business dispositions or supporting-record results, not additional canonical lifecycle states. Section 13 states the only canonical lifecycle impact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a Supplier in canonical `Active` applies or is nominated for qualification against a defined institutional category, capability, location, value band, risk profile, funding context, or other controlled scope.

## 4.2 End Boundary

The scenario ends when the qualification position is approved, conditionally approved, returned, rejected, or deemed not applicable; the exact eligibility scope and validity are visible to sourcing; and renewal or restriction obligations are scheduled.

## 4.3 In Scope

- defining the qualification scheme, criteria, scope, evidence, scoring or pass requirements, authority, validity, and monitoring conditions;
- validating Active supplier status and applicable registration or approval conditions;
- assessing technical, professional, regulatory, financial, operational, quality, safety, security, sustainability, geographical, capacity, experience, and resource criteria as applicable;
- evaluating personnel, facilities, equipment, licences, certifications, references, past performance, insurance, turnover, capacity, and subcontracting facts;
- assigning exact category, subcategory, item, service, works class, campus, geography, value band, risk tier, and effective period;
- approving, conditioning, returning, rejecting, limiting, renewing-hand-off, or restricting-hand-off for qualification;
- preserving decisions, evidence, conflicts, evaluator independence, and qualification history.

## 4.4 Out of Scope

- canonical supplier registration governed by L3-J-020;
- institutional supplier approval and activation governed by L3-J-021;
- renewal of expiring evidence or qualification governed by L3-J-023;
- restriction caused by expired qualification governed by L3-J-024;
- event-specific bid responsiveness, technical scoring, award, Purchase Order, contract, receipt, payable, accounting entry, or payment;
- technical catalogue, scoring engine, or workflow design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-020 and L3-J-021 | Supply canonical identity and the sequential `Approved` then `Active` states. |
| Followed by | L3-J-023 | Renews expiring qualification or evidence. |
| Followed by | L3-J-024 | Restricts eligibility when qualification expires. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Vendor & Supplier Management | Owns qualification scheme governance, evidence coordination, evaluation, eligibility scope, approval, validity, conditions, and audit evidence. |

Technical, quality, safety, finance, legal, security, sustainability, and other specialists retain their authoritative criteria and findings. Sourcing events retain event-specific evaluation.

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

- The supplier has one canonical identity and is in the canonical `Active` state; any conditions are recorded separately from lifecycle state.
- The requested qualification scheme and scope are identifiable and currently authorised.
- Criteria, evidence, evaluation method, pass rules, validity, conditions, authorities, conflicts, and review triggers were established before evaluation.
- Applicable licences, certificates, references, financial evidence, personnel, facilities, equipment, and performance sources can be verified.
- Supplier-approval conditions do not prohibit the requested qualification.
- No existing qualification already covers the same scope and period unless this is an authorised expansion, correction, or reassessment.

---

# 8. Trigger

A Supplier in canonical `Active` submits or is invited to a qualification process for a defined institutional procurement scope.

---

# 9. Main Business Flow

1. The Institutional Verifier records the supplier, scheme, requested categories, locations, value bands, risk classes, funding contexts, intended use, and submission time.
2. Canonical identity, Active status, approval conditions, ownership, relationships, conflicts, restrictions, and current evidence are confirmed.
3. The applicable qualification criteria, evaluators, scoring or pass method, evidence requirements, validity, and approval authority are locked for the case.
4. Supplier evidence is captured with issuer, source, issue date, expiry date, scope, authenticity result, version, and limitations.
5. Mandatory legal, regulatory, professional, trade, product, works, safety, quality, security, environmental, labour, or other licences and certifications are verified.
6. Technical experience, past contracts, references, completion evidence, personnel competence, equipment, facilities, methodology, service coverage, and delivery capacity are assessed.
7. Financial capacity, turnover, insurance, working capital, solvency, exposure, value-band suitability, and concentration risk are assessed where required.
8. Quality, safety, information security, privacy, continuity, sustainability, ethical sourcing, and subcontracting controls are evaluated proportionately.
9. Past institutional and external performance, defects, delays, disputes, claims, incidents, sanctions, and corrective actions are considered under approved relevance rules.
10. Supplier capacity is allocated across requested scopes to prevent qualification beyond demonstrable personnel, equipment, facilities, geography, or financial capacity.
11. Evaluators declare conflicts and independently record findings against pre-established criteria; undocumented intuition or requester preference cannot replace evidence.
12. Clarifications may resolve ambiguity but cannot create materially new capability after the deadline where a competitive qualification process would be prejudiced.
13. The evaluation produces a scope-level recommendation identifying qualified, conditionally qualified, returned, rejected, or not-applicable positions and reasons.
14. The recommendation defines exact categories, subcategories, locations, campuses, value bands, risk classes, capacity, conditions, effective date, expiry date, and monitoring triggers.
15. The Qualification Authority reviews evaluation completeness, independence, mandatory failures, exceptions, aggregate capacity, and supplier approval conditions, then decides.
16. Approved positions become Qualified Institutional Eligibility; downstream sourcing may use them only within their scope and validity and must still perform event-specific checks.
17. Evidence, evaluations, clarifications, scores, conditions, decisions, actors, timestamps, versions, and renewal or restriction dates remain linked and auditable.

---

# 10. Alternate Business Flows

## 10.1 Partial Qualification

Only categories, locations, value bands, goods, works, services, personnel, facilities, or risk classes meeting criteria are approved. Failed scope remains explicitly unqualified.

## 10.2 Conditional Qualification

Qualification may require supervision, limited value, restricted geography, approved subcontractors, additional insurance, capacity limit, trial period, remediation, or enhanced monitoring. Mandatory legal failures cannot be conditionally waived.

## 10.3 Qualification by Recognised External Certification

External certification or prequalification may satisfy defined criteria only after authenticity, scope, issuer, validity, equivalence, and institutional acceptance are verified.

## 10.4 Consortium or Joint Qualification

Member responsibilities, combined capability, lead authority, joint liability, resource sharing, substitutions, validity, and whether qualification survives member change are explicit.

## 10.5 Manufacturer, Dealer, Distributor, or Service Partner

Authorisation chain, territory, product or service scope, validity, warranty role, support capacity, and dependence on another entity are verified.

## 10.6 Qualification Expansion

New scope is evaluated as an attributable extension. Existing valid positions are not overwritten, and aggregate capacity is reassessed.

## 10.7 Event-Specific Prequalification

Where qualification applies only to one event, the position is linked to that event and cannot silently become standing institution-wide eligibility.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Supplier Status Becomes Non-Active

Qualification approval or use is blocked and referred to supplier-status governance. Qualification evidence remains preserved but cannot override status.

## 11.2 Mandatory Criterion Fails

Affected scope is rejected unless an explicit lawful exception process exists. Scores cannot compensate for non-waivable failure.

## 11.3 Evidence Is False, Expired, or Unverifiable

The case is returned, held, or rejected and may be referred for integrity review. Existing eligibility is reassessed where relevant.

## 11.4 Evaluator Conflict or Inconsistent Scoring

The affected evaluation is held and independently reviewed or repeated under the same criteria.

## 11.5 Capacity Double Counted

Overlapping personnel, equipment, facilities, financial capacity, consortium membership, or geography is reconciled before approval.

## 11.6 Process Interruption

The case resumes from the last valid checkpoint; time-sensitive supplier status and evidence are refreshed.

## 11.7 Adverse Fact After Qualification

The fact is appended and referred for restriction, suspension, reassessment, renewal denial, or other governed action without deleting the original qualification.

---

# 12. Business Rules and Decision Constraints

- Qualification requires a canonical supplier identity and valid institutional Active status.
- Active status does not mean universally Qualified, and qualification does not guarantee invitation, award, or commitment.
- Eligibility must be explicit by scope, validity, capacity, location, value band, risk, funding context, and conditions as applicable.
- Criteria and decision rules must be established before evidence evaluation.
- Mandatory licences, prohibitions, or criteria cannot be offset by unrelated strengths unless an authorised rule expressly permits it.
- Evidence must be attributable, authentic to the required degree, current, and within issuer scope.
- Conditions and capacity limits must be visible and enforceable downstream.
- Event-specific qualification cannot be reused as standing eligibility unless separately approved.
- Qualification cannot validate a bank instruction, funding, tax result, award, receipt, payable, accounting entry, or payment.
- Material changes to supplier status, ownership, licence, certification, personnel, facilities, capacity, performance, or risk require reassessment.
- Expired qualification cannot authorise new sourcing or award activity.
- Evaluation and decision history cannot be overwritten by renewal or expansion.
- This scenario does not create a Purchase Order or contract.

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
| L3-J-020 / L3-J-021 | Supply canonical identity and Active status. | Retain identity and approval lifecycle. |
| Technical / Category Owners | Supply controlled capability criteria and evaluation. | Cannot award or activate supplier alone. |
| Finance / Commercial | Supplies financial capacity and value-band findings. | Retains financial truth. |
| Quality / Safety / Security / Sustainability | Supplies specialist criteria and findings. | Retains specialist authority. |
| Sourcing / J04 | Consumes valid scope but performs event, award, and commitment checks. | Retains transaction lifecycle. |
| L3-J-023 / L3-J-024 | Receive expiry and renewal or restriction facts. | Retain later eligibility lifecycle. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J02 roles are separated as policy requires: Supplier Representative; Institutional Verifier; Qualification Authority; and Supplier Governance Authority. The Supplier Representative cannot verify, qualify, approve, or remove an institutional restriction on itself.

## 15.2 Required Evidence

- canonical supplier identity, Active status, approval conditions, requested qualification scope, and applicable scheme;
- locked criteria, evaluator appointments, conflict declarations, evidence requirements, scoring or pass rules, authority, and validity;
- licences, certifications, experience, references, personnel, equipment, facilities, methodology, geography, capacity, financial, insurance, quality, safety, security, privacy, sustainability, and subcontracting evidence as applicable;
- evaluations, scores, clarifications, authenticity checks, performance history, mandatory failures, capacity allocation, exceptions, conditions, recommendation, decision, actors, timestamps, and versions.

## 15.3 Audit Obligations

The institution must reconstruct which criteria and evidence applied, who evaluated each dimension, how conflicts and capacity were controlled, why each scope was approved or rejected, what conditions and validity governed it, and whether later sourcing used only current eligible scope.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- the approved supplier has been assessed against the specific category, technical, financial, regulatory, geographical, or institutional qualification criteria applicable to the defined scope;
- required evidence, credentials, certifications, financial statements, and declarations have been verified against acceptable sources;
- the qualification decision has been made by an authority appropriate to the scope, category, and institutional risk;
- qualification scope, effective dates, validity period, conditions, and limitations are explicit;
- the qualification decision, evidence, authority, actors, and audit trail are preserved and immutable;
- where the supplier does not qualify, the rejection is documented with reasons and the supplier remains in its prior lifecycle state.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the supplier holds an institutional qualification status for the defined scope and validity period;
- the qualification is scope-specific and does not extend to categories, geographical areas, or institutional purposes outside its defined boundaries;
- qualification does not by itself award a contract, create a commitment, or establish a sourcing preference;
- the complete qualification evidence, decision, and conditions are permanently preserved and auditable;
- expiry, renewal, restriction, and revocation of the qualification require separate governed actions.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Qualification schemes, categories, value bands, capacity measures, criteria, scoring, validity, and approval thresholds require institution-specific confirmation.
2. Equivalence and acceptance rules for external certification, consortium capacity, dealers, manufacturers, and foreign evidence require governance.
3. Boundary between standing qualification, event prequalification, responsiveness, and technical evaluation requires method-specific L4 mapping.

## 17.2 Assumptions

- The canonical `Approved` state followed by `Active` precedes institutional qualification.
- Qualification is scope-specific and never guarantees award or commitment.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-022**
