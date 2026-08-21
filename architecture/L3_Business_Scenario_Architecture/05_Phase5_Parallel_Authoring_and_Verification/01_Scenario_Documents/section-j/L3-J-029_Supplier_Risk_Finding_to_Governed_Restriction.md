# Supplier Risk Finding to Governed Restriction

# 1. Scenario Metadata

**Scenario ID:** L3-J-029  
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
**Referenced By:** supplier qualification, sourcing eligibility, evaluation, award, commitment, contract review, suspension, reinstatement, remediation, monitoring, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs conversion of an authorised supplier risk finding into a proportionate, scoped, time-bound or condition-bound restriction on future supplier use. It ensures that risk is controlled without automatically suspending the supplier universally or rewriting existing sourcing, contract, receipt, payable, accounting, or payment records.

It preserves the distinction between evidence, assessment, restriction, suspension, blacklisting, contract action, and financial action. Each later consequence remains subject to its own authority and scenario.

---

# 3. Business Outcome

A substantiated supplier risk finding reaches a Governed Restriction with defined supplier, grounds, affected scope, effective time, prohibited and permitted activity, duration or exit conditions, downstream controls, supplier notice, review rights, monitoring, approval, and immutable audit lineage.

Outcome labels in this section are business dispositions or supporting-record results, not additional canonical lifecycle states. Section 13 states the only canonical lifecycle impact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised assessment, compliance, qualification, performance, incident, legal, safety, security, financial, ethical, regulatory, continuity, or other control owner submits a governed risk finding requiring restriction consideration.

## 4.2 End Boundary

The scenario ends when the restriction is approved, rejected, returned, or referred for suspension; its exact scope and effective conditions are published to authorised consumers; affected open positions are referred to their owners; the supplier is notified where permitted; and review, remediation, expiry, or escalation paths are established.

## 4.3 In Scope

- validating the authoritative finding, canonical supplier linkage, evidence lineage, severity, urgency, recurrence, and current relevance;
- determining whether no action, monitoring, conditional use, category restriction, location restriction, value restriction, participation restriction, award restriction, new-commitment restriction, or another approved scoped control is proportionate;
- defining affected categories, sites, campuses, units, funding contexts, transaction stages, value bands, risk classes, and effective period;
- identifying existing sourcing, recommendation, award, and commitment positions for governed review by their authoritative owners;
- recording supplier response, remediation obligations, monitoring, expiry, review, appeal, extension, variation, and release conditions;
- escalating broader or urgent risk for controlled suspension under L3-J-030.

## 4.4 Out of Scope

- creating the underlying assessment record, governed by L3-J-028 or another authoritative process;
- imposing universal supplier suspension, blacklisting, debarment, or terminal disposition;
- cancelling or amending an executed Purchase Order or contract;
- reversing a receipt, payable, accounting entry, payment, or bank transaction;
- deciding legal liability, damages, recovery, employee discipline, or criminal consequence;
- technical enforcement, scoring, notification, or case-management implementation.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-028 | Governed assessment may supply the risk finding. |
| Related | L3-J-022 to L3-J-025 | Qualification and expiry controls remain distinct restriction grounds. |
| Followed by | L3-J-030 | A material or expanding risk may require supplier suspension. |
| Related | J04 commitment scenarios | Existing and proposed commitments retain their own decisions. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Vendor & Supplier Management | Owns restriction consideration, proportional scope, approval coordination, publication, supplier communication, monitoring, review, and history. |

Legal, compliance, finance, safety, security, audit, qualification, contract, and operational owners retain their specialist findings and authoritative downstream decisions.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Supplier Representative | Receives notice and supplies permitted response or remediation evidence; cannot remove institutional restrictions. | Responds. |
| Institutional Verifier | Independently verifies evidence material to the governance decision. | Verifies evidence. |
| Supplier Governance Authority | Proposes or approves restriction, suspension, blacklisting, reactivation, inactivation, closure or merge treatment. | Decides governed supplier treatment. |
---

# 7. Preconditions and Entry Criteria

- The affected supplier, site, branch, consortium member, or controlled related entity is linked to a canonical identity.
- A governed finding exists with source, facts, severity, affected scope, evidence lineage, and authorised owner.
- Restriction types, approval thresholds, urgent authority, conflict rules, notice, review, duration, monitoring, release, and retention rules are effective.
- Current approval, qualification, sourcing, award, commitment, and contract positions can be identified.
- Mandatory legal or regulatory consequences can be distinguished from discretionary risk treatment.
- Sensitive, whistleblower, investigative, personal, security, or legally privileged material can be protected.

---

# 8. Trigger

An authorised risk finding indicates that continued unrestricted supplier eligibility or future use may expose the institution to material legal, regulatory, financial, operational, quality, safety, security, ethical, reputational, eligibility, or continuity risk.

---

# 9. Main Business Flow

1. The Institutional Verifier submits the canonical supplier, finding reference, facts, source, affected scope, severity, urgency, recurrence, exposure, and recommended treatment.
2. The Supplier Governance Authority validates authority, evidence lineage, canonical linkage, current relevance, duplicate or related cases, and whether the finding is final, interim, disputed, or under investigation.
3. Immediate mandatory, safety, security, fraud, sanctions, data, legal, or continuity risk is escalated under urgent authority without concealing the need for subsequent review.
4. The institution identifies current supplier approval, qualifications, categories, sites, sourcing events, recommendations, awards, proposed commitments, executed commitments, and active remediation.
5. Responsibility and scope are distinguished across supplier, site, branch, subcontractor, consortium member, predecessor, successor, parent, subsidiary, and related entities; risk is not attributed automatically.
6. Conflicts involving the Institutional Verifier, sourcing team, contract owner, supplier, approver, or related party are declared and mitigated.
7. Available treatments are assessed from no action or monitoring through conditional use, category, location, value, stage, funding, invitation, participation, evaluation, award, or new-commitment restriction.
8. The proposed restriction is no broader or longer than necessary, subject to mandatory rules and credible risk containment.
9. The supplier receives sufficient particulars and an opportunity to respond where lawful and compatible with urgency, confidentiality, privilege, or investigation.
10. Supplier response, remediation, corrective action, mitigating and aggravating factors, recurrence, cooperation, and residual risk are evaluated.
11. The recommendation defines grounds, affected and unaffected scope, prohibited and permitted activity, effective time, duration or exit conditions, monitoring, review date, notice, and downstream referrals.
12. Existing sourcing, recommendation, award, and commitment positions are listed for separate review; the restriction does not silently rewrite their authoritative records.
13. The Supplier Governance Authority reviews evidence sufficiency, authority, conflicts, consistency, fairness, proportionality, mandatory consequence, supplier response, and proposed exceptions.
14. The approver rejects, returns, approves, varies, or refers the case for suspension.
15. An approved restriction becomes an immutable governed status linked to its finding, reasons, scope, conditions, approver, effective time, and review path.
16. Authorised supplier, qualification, sourcing, award, commitment, contract, finance, legal, safety, security, and audit consumers receive only the restriction facts needed for their decisions.
17. Monitoring, remediation, exceptions, breaches, reviews, extensions, variations, release, escalation, actors, timestamps, and versions remain auditable.

---

# 10. Alternate Business Flows

## 10.1 Monitoring Without Restriction

If residual risk is acceptable, the authority records no restriction, defined monitoring, review date, and reasons. The finding remains visible.

## 10.2 Narrow Category or Site Restriction

Only the affected category, service, works, site, branch, campus, location, or risk class is restricted. Other independently valid eligibility remains available.

## 10.3 Urgent Interim Restriction

An authorised interim restriction may take immediate effect for a defined scope and maximum review period. Full evidence and approval review follow within the governed deadline.

## 10.4 Supplier Remediation

The supplier submits corrective evidence. Restriction remains until an independent authorised review confirms exit conditions; submission alone does not restore eligibility.

## 10.5 Finding Under Dispute

The dispute is recorded and reviewed through the approved path. Restriction may remain, narrow, or pause only according to risk and authority, not automatically because a dispute exists.

## 10.6 Risk Requires Suspension

Where scoped restriction cannot contain the risk, the case is referred to L3-J-030. Restriction may remain as interim protection pending the separate decision.

## 10.7 Existing Commitment Exposure

Contract and commitment owners assess legality, continuity, performance, termination, remediation, or other action. No automatic cancellation or financial reversal occurs here.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Supplier Identity or Scope Ambiguous

The case is held or a narrow interim control is applied where authorised until canonical identity and exposure are resolved.

## 11.2 Evidence Is Incomplete or Unverifiable

The proposal is returned, narrowed, monitored, or handled under authorised interim rules. Allegation is not converted into a final finding.

## 11.3 Restriction Fails to Propagate

The failure is recorded, publication is reissued, affected decisions are reconciled, and any unauthorised activity is reviewed. Authoritative effective time remains unchanged.

## 11.4 Over-Restriction

Unaffected scope is restored by attributable correction. Error, impact, approvals, transactions, and corrective action remain visible.

## 11.5 Unauthorised Override or Exception

The attempt is blocked or revoked and escalated. Any resulting sourcing, award, or commitment activity is preserved for authoritative review.

## 11.6 Review Deadline Missed

The restriction follows approved expiry or continuation rules; it does not disappear silently. Overdue review is escalated.

## 11.7 System Interruption

Business-effective restriction remains controlling. On recovery, authoritative status and affected activity are reconciled without backdating.

---

# 12. Business Rules and Decision Constraints

- A governed restriction requires an attributable risk finding, canonical supplier linkage, explicit scope, authority, reasons, effective time, and review or exit conditions.
- Restriction is proportionate and scoped; universal supplier suspension requires L3-J-030.
- Mandatory legal, regulatory, sanctions, safety, security, or eligibility rules cannot be overridden by ordinary discretion.
- Related-entity risk is not attributed without evidence and an applicable relationship rule.
- Supplier response and review rights apply unless lawful urgent, confidential, privileged, or investigative limitations exist.
- Restriction of future use does not automatically cancel or amend an existing sourcing decision, award, Purchase Order, contract, receipt, payable, accounting entry, or payment.
- An exception requires express authority, reasons, narrow scope, duration, compensating controls, and no conflict with mandatory prohibition.
- Remediation evidence does not itself release restriction; authorised verification and decision are required.
- Expired, released, varied, superseded, and breached restrictions remain historically visible.
- Restriction cannot be disguised as blacklisting or terminal disposition without the separate governed process.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Supplier | Existing canonical J02 state | Applies only an exact J02 supplier-governance transition. | `Restricted`, `Suspended`, `Blacklisted`, `Active`, `Inactive` or `Closed / Archived` as material to the scenario | Vendor & Supplier Management (J02) | `L2-J02` Section 6 |
| SupplierGovernanceCase | No canonical lifecycle defined | Preserves evidence and the attributable governance decision without creating local case states. | Governed case record linked to the exact Supplier state | Vendor & Supplier Management (J02) | `L2-J02` Section 5 |
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `SupplierRestricted` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| Institutional Verifier / L3-J-028 assessment source | Supplies governed evidence and findings. | Does not impose restriction automatically. |
| Qualification / Sourcing Owners | Consume scope controls and review affected positions. | Retain qualification and sourcing decisions. |
| J04 / Contract Owner | Reviews proposed and existing commitments. | Retains commitment and contract lifecycle. |
| Finance | Receives material facts where payable or payment control review is required. | Retains monetary and accounting truth. |
| Legal / Compliance / Safety / Security | Supplies mandatory or specialist consequences. | Retains specialist authority. |
| L3-J-030 | Receives broader or uncontrollable risk. | Retains suspension authority. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J02 roles are separated as policy requires: Supplier Representative; Institutional Verifier; and Supplier Governance Authority. The Supplier Representative cannot verify, approve, or remove an institutional restriction on itself, and a conflicted verifier cannot decide the governed outcome.

## 15.2 Required Evidence

- canonical supplier and related entities, finding, raw evidence references, severity, urgency, recurrence, current exposure, affected and unaffected scope;
- supplier approval, qualification, sourcing, recommendation, award, commitment, contract, remediation, and related restriction positions;
- conflict declarations, specialist reviews, supplier notice and response, treatment analysis, proportionality, mandatory rules, residual risk, and recommendations;
- approval, reasons, effective time, duration, exit conditions, monitoring, exceptions, breaches, reviews, variations, extensions, release or escalation, actors, timestamps, and versions.

## 15.3 Audit Obligations

The institution must reconstruct why restriction was considered, which finding supported it, how scope and duration were selected, what supplier response occurred, who approved each decision, how the restriction affected downstream activity, whether exceptions or breaches occurred, and why it was varied, released, or escalated.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- a risk finding has been substantiated by authoritative evidence and reviewed under institutional risk governance;
- a restriction decision has been made by an authority appropriate to the risk severity, value exposure, and institutional consequence;
- the restriction scope, effective date, duration, conditions, and review timeline are explicit and attributable;
- existing commitments, active contracts, and in-progress sourcing events are assessed and dispositioned under their authoritative owners;
- the restriction decision, evidence, authority, actors, conditions, and audit trail are preserved and immutable;
- the supplier retains its identity and non-restricted qualifications during the restriction period.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the supplier is in a Restricted state for the defined scope and duration with an explicit effective date and review timeline;
- the restriction prevents new sourcing selection or award within the restricted scope from the effective date;
- existing governed obligations are managed by their authoritative domain owners under the disposition established at restriction time;
- the restriction decision and evidence are permanently preserved and auditable;
- reinstatement requires a governed remediation outcome under L3-J-032 or a separate governed authority decision.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Restriction types, severity mapping, approval thresholds, urgent duration, review frequency, and release authorities require confirmation.
2. Rules for related entities, consortium members, subcontractors, branches, predecessors, and successors require legal and procurement mapping.
3. Exact treatment of active sourcing, approved award, proposed commitment, and existing contract positions requires integration with their authoritative scenarios.

## 17.2 Assumptions

- Restriction is a scoped control and does not equal suspension, blacklisting, or terminal supplier disposition.
- Broader supplier suspension begins at L3-J-030.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-029**
