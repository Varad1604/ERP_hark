# Supplier Performance or Incident Evidence to Governed Supplier Assessment Record

# 1. Scenario Metadata

**Scenario ID:** L3-J-028  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Vendor & Supplier Management  
**Participating Domains:** G, J  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** supplier approval, qualification, renewal, sourcing, contract management, risk restriction, suspension, blacklisting, remediation, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs conversion of supplier performance observations, incidents, complaints, defects, delays, service failures, commendations, audit findings, and other evidence into a fair, attributable, reviewable Supplier Assessment Record.

It separates evidence and assessment from sanctions. The record may inform later restriction, suspension, blacklisting, qualification, renewal, award, or contract decisions but does not impose those outcomes automatically.

---

# 3. Business Outcome

Verified supplier performance or incident evidence reaches a Governed Supplier Assessment Record with defined supplier, transaction or period, scope, source, facts, severity, materiality, cause, supplier response, findings, rating where applicable, corrective actions, review rights, authority, and audit lineage.

Outcome labels in this section are business dispositions or supporting-record results, not additional canonical lifecycle states. Section 13 states the only canonical lifecycle impact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised source submits performance evidence or reports an incident associated with a canonical supplier or related engagement.

## 4.2 End Boundary

The scenario ends when evidence is validated and an assessment is approved, returned, closed as unsubstantiated, or referred for urgent risk action; the supplier response and review path are recorded; and downstream owners receive governed findings.

## 4.3 In Scope

- receiving positive and negative performance evidence, incidents, complaints, quality defects, delays, nonconformance, service failure, safety, security, ethics, environmental, labour, fraud, or continuity facts;
- linking evidence to canonical supplier, site, subcontractor, contract, Purchase Order, delivery, service period, category, and institution unit where applicable;
- validating source, custody, chronology, authenticity, relevance, and duplicate reports;
- classifying severity, impact, recurrence, responsibility, materiality, and urgency under approved criteria;
- obtaining supplier response and responsible-owner findings where fairness allows;
- recording ratings, findings, corrective action, closure, dispute, review, and downstream referrals;
- preserving raw evidence separately from conclusions.

## 4.4 Out of Scope

- changing supplier identity or profile;
- deciding restriction, suspension, blacklisting, contract termination, payable deduction, claim, or legal liability;
- creating or amending a Purchase Order, receipt, payable, accounting entry, or payment;
- replacing contract acceptance, inspection, warranty, complaint, audit, safety, security, or disciplinary processes;
- technical score calculation, monitoring, or case-management implementation.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-J-021 to L3-J-025 | Assessment may affect approval, qualification, renewal, and category eligibility. |
| Followed by | L3-J-029 and L3-J-030 | Material findings may support restriction or suspension. |
| Related | J04 and receipt scenarios | Supply transaction evidence but retain transaction lifecycle. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Vendor & Supplier Management | Owns assessment intake, evidence governance, supplier linkage, evaluation coordination, assessment approval, supplier response, referrals, and history. |

Contract, receipt, finance, legal, safety, security, audit, and operational owners retain their authoritative records and decisions.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Supplier Representative | Receives notice and supplies permitted response or remediation evidence; cannot remove institutional restrictions. | Responds. |
| Institutional Verifier | Independently verifies evidence material to the governance decision. | Verifies evidence. |
| Supplier Governance Authority | Proposes or approves restriction, suspension, blacklisting, reactivation, inactivation, closure or merge treatment. | Decides governed supplier treatment. |
---

# 7. Preconditions and Entry Criteria

- The supplier or candidate supplier can be linked to a canonical or controlled identity.
- The evidence originator, source, event, period, location, and affected scope are identifiable.
- Assessment criteria, severity, rating, conflict, response, confidentiality, urgent escalation, review, and retention rules are effective.
- Related transaction and specialist records can be referenced without rewriting them.
- The assessment role is sufficiently independent from the reporter and supplier.
- Confidential, personal, safety, security, whistleblower, or investigative evidence can be protected appropriately.

---

# 8. Trigger

Performance evidence, an incident, complaint, audit finding, delivery or service result, corrective-action outcome, commendation, or monitoring threshold requires a governed supplier assessment.

---

# 9. Main Business Flow

1. The evidence originator records supplier, transaction or period, category, site, event date, discovery date, facts, impact, source, confidentiality, and attachments.
2. The Institutional Verifier confirms reporter authority, evidence custody, completeness, authenticity, chronology, and canonical supplier linkage.
3. Duplicate, related, recurring, predecessor, subsidiary, site, subcontractor, and consortium records are identified without attributing another entity's conduct automatically.
4. Immediate life, safety, security, fraud, legal, data, sanctions, or continuity risks are escalated under urgent authority while assessment continues.
5. The evidence is classified by performance dimension, severity, materiality, recurrence, controllability, affected scope, and potential consequence.
6. Applicable contract, Purchase Order, specification, service level, quality, delivery, acceptance, safety, security, ethical, legal, or policy expectations are identified.
7. Transaction owners and specialists provide corroborating or contrary evidence and distinguish observation, allegation, accepted fact, and professional opinion.
8. Conflicts involving reporter, evaluator, contract owner, supplier, approver, or related parties are declared and mitigated.
9. The supplier receives sufficient particulars and a response opportunity where lawful and compatible with urgent, confidential, or investigative requirements.
10. Supplier response, evidence, root-cause explanation, corrective action, dispute, and mitigating or aggravating factors are assessed.
11. The assessment determines substantiated, partially substantiated, unsubstantiated, unresolved, commendable, or other approved finding status by issue.
12. Any performance score or rating is derived from disclosed criteria and period, not undocumented discretion.
13. The recommendation identifies findings, severity, responsibility, recurrence, corrective actions, monitoring, closure conditions, and referrals, but does not impose sanctions outside this scenario.
14. The Supplier Governance Authority reviews evidence sufficiency, independence, supplier response, rating, consistency, and proposed referrals, then decides within the unresolved upstream assessment-record boundary stated in Section 17.
15. The approved Supplier Assessment Record becomes an immutable governed finding linked to raw evidence and related authoritative records.
16. Relevant approval, qualification, renewal, sourcing, contract, risk, restriction, safety, security, legal, finance, or audit owners receive the permitted findings for separate decisions.
17. Evidence, responses, findings, corrections, reviews, referrals, actors, timestamps, confidentiality, and versions remain auditable.

---

# 10. Alternate Business Flows

## 10.1 Positive Performance or Commendation

Verified delivery, innovation, quality, responsiveness, savings, safety, or service achievement is recorded using the same source and conflict controls and may support later decisions without guaranteeing award.

## 10.2 Minor Correctable Issue

The assessment may close after proportionate correction and verification while retaining the incident and recurrence history.

## 10.3 Serious or Urgent Incident

Protective action is referred immediately to L3-J-029, L3-J-030, contract, safety, security, legal, or other authority without waiting for final rating where valid urgent grounds exist.

## 10.4 Supplier Disputes Finding

The dispute and evidence remain linked. Review follows an independent approved path and does not erase the original assessment.

## 10.5 Shared or Multi-Supplier Responsibility

Findings distinguish supplier, subcontractor, consortium member, institution, design, external event, and shared causes.

## 10.6 Insufficient Evidence

The issue is returned, closed unsubstantiated, or retained unresolved according to rule. Allegation is not converted into fact.

## 10.7 Periodic Performance Assessment

Multiple transaction records are consolidated for a defined period using controlled weighting, exclusions, and source lineage.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Supplier Identity Ambiguous

Assessment is held until canonical linkage is resolved; evidence remains protected.

## 11.2 Evidence Lost, Altered, or Unverifiable

The limitation is recorded, finding strength is reduced or case is closed, and integrity concern is escalated where appropriate.

## 11.3 Conflicted Assessor

The assessor is replaced or independently reviewed. Existing work remains visible.

## 11.4 Supplier Does Not Respond

Assessment may proceed after the governed response period if delivery and fairness requirements were satisfied; non-response is not treated as admission automatically.

## 11.5 Active Investigation or Legal Hold

Ordinary disclosure or closure is limited as authorised, evidence is preserved, and the assessment coordinates with the authoritative process.

## 11.6 Scoring or Source Error

A corrective assessment version is approved. Original score and downstream impact remain auditable.

## 11.7 Unauthorised Downstream Sanction

The attempt is blocked or escalated; an assessment alone cannot create restriction, suspension, blacklisting, deduction, or termination.

---

# 12. Business Rules and Decision Constraints

- Raw evidence, allegation, verified fact, professional opinion, finding, rating, and sanction are distinct.
- Supplier assessment requires canonical linkage, source, period, scope, criteria, independent review, and attributable approval.
- Negative findings require proportionate supplier response opportunity unless lawful urgent or confidential constraints apply.
- Positive and negative evidence follows consistent governance.
- Scores must use approved criteria, period, weighting, and evidence.
- Conduct of related entities cannot be attributed automatically without evidence and applicable relationship rules.
- Urgent protective referrals do not predetermine the final assessment.
- Assessment does not automatically restrict, suspend, blacklist, terminate, deduct, withhold, or create financial liability.
- Corrections and reviews create new versions and do not delete original evidence or decisions.
- Confidential and whistleblower evidence follows controlled access and disclosure.
- This scenario does not create or amend a Purchase Order, receipt, payable, accounting entry, or payment.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Supplier | Existing canonical J02 state | Updates or records the scenario-specific governed fact without silently changing Supplier lifecycle state. | Existing canonical J02 state remains controlling | Vendor & Supplier Management (J02) | `L2-J02` Section 6 |

No separate lifecycle is defined upstream for the scenario-specific record; this scenario does not create one.
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

No exact canonical J02 event is defined for supplier performance/incident assessment completion. This scenario publishes no locally invented substitute; the material event gap is recorded in Section 17.
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| J04 / Receipt / Requesting Unit | Supply commitment, delivery, acceptance, and service evidence. | Retain transaction lifecycle. |
| Quality / Safety / Security / Legal / Finance | Supply specialist facts and urgent referrals. | Retain specialist authority. |
| L3-J-021 to L3-J-025 | Consume governed findings in approval and eligibility reviews. | Retain their decisions. |
| L3-J-029 / L3-J-030 | Consume material findings for separate restriction or suspension. | Retain sanction authority. |
| Supplier | Responds and undertakes corrective action. | Does not control assessment outcome. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J02 roles are separated as policy requires: Supplier Representative; Institutional Verifier; and Supplier Governance Authority. The Supplier Representative cannot verify, approve, or remove an institutional restriction on itself, and a conflicted verifier cannot decide the governed outcome.

## 15.2 Required Evidence

- canonical supplier, site or related entity, transaction or period, reporter, source, chronology, custody, affected requirement, and impact;
- raw documents, communications, inspection, acceptance, service, audit, complaint, safety, security, financial, or performance evidence;
- classification, severity, recurrence, criteria, conflicts, supplier notice and response, root cause, corrective actions, ratings, specialist findings, and limitations;
- recommendation, approval, referral, dispute, review, correction, actors, reasons, timestamps, confidentiality, and versions.

## 15.3 Audit Obligations

The institution must reconstruct each fact and source, how it was linked to the supplier, how severity and rating were determined, what response was offered, who approved the finding, what corrective action occurred, and which later decisions consumed the assessment.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- performance or incident evidence has been captured from an authoritative source with a verifiable basis, originating actor, and effective date;
- the evidence has been reviewed, classified, and assessed against the applicable performance criteria or incident severity scale;
- the assessment record has been approved by an authority appropriate to the evidence type, severity, and institutional consequence;
- the supplier's performance history has been updated with the governed assessment record;
- the assessment record, evidence, authority, actors, and audit trail are preserved and immutable;
- where the assessment indicates a risk or governance concern, it has been referred to the appropriate governing scenario.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- a governed supplier assessment record exists, is attributable, and reflects the current authoritative performance or incident finding;
- the assessment record is available as a governed fact for downstream risk, restriction, suspension, and governance scenarios;
- the prior performance history is preserved and the new record is distinguished as an additive fact, not an overwrite;
- the assessment does not itself impose a restriction or suspension; those require separate governed actions under L3-J-029 or L3-J-030;
- the complete evidence, assessment, authority, and decision trail is permanently auditable.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- The locked J02 event catalogue defines no exact event for completion or correction of the governed Supplier Assessment Record. Event publication remains unresolved and must not be invented in L3.
1. Performance dimensions, severity levels, rating scales, weighting, evidence standards, and review periods require confirmation.
2. Confidential complaint, whistleblower, safety, security, fraud, and legal-hold treatment requires specialist policy mapping.
3. Rules for aggregating transaction, site, subcontractor, consortium, and related-entity performance require an upstream J02 business-rule decision.

## 17.2 Assumptions

- Assessment findings are governed inputs, not sanctions.
- Restriction and suspension begin at L3-J-029 and L3-J-030.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-028**
