# Expiring Supplier Evidence or Qualification to Renewed Eligibility

# 1. Scenario Metadata

**Scenario ID:** L3-J-023  
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
**Referenced By:** sourcing, award, commitment, evidence-expiry restriction, supplier performance, classification, profile change, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs renewal of supplier evidence or qualification before current eligibility expires. It verifies that the supplier, evidence, capability, approval status, performance, capacity, risks, and qualification scope remain current and support a new attributable eligibility period.

It prevents silent extension, replacement of historical evidence, renewal based only on prior approval, and continued sourcing against expired or materially changed eligibility.

---

# 3. Business Outcome

An expiring supplier evidence item or qualification reaches Renewed Eligibility, Conditionally Renewed Eligibility, Partially Renewed Eligibility, Returned for Renewal Evidence, Renewal Rejected, or Expired Pending Restriction. The result identifies renewed scope, evidence, conditions, effective date, new expiry date, review triggers, authority, and predecessor lineage.

Outcome labels in this section are business dispositions or supporting-record results, not additional canonical lifecycle states. Section 13 states the only canonical lifecycle impact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a governed alert, supplier submission, or authorised review identifies that evidence supporting Active status or Qualified Institutional Eligibility is approaching expiry or scheduled review.

## 4.2 End Boundary

The scenario ends when eligibility is renewed, partially or conditionally renewed, returned, rejected, or allowed to expire; downstream users receive the new validity fact; and any expired position is handed to L3-J-024.

## 4.3 In Scope

- identifying expiring evidence, qualification scope, dependencies, deadlines, and affected downstream use;
- requesting and validating replacement or renewed licences, certificates, insurance, registrations, financial, technical, safety, security, quality, sustainability, and capacity evidence;
- refreshing supplier Active status, ownership, conflicts, restrictions, performance, incidents, and material-change facts;
- reassessing qualification criteria, capacity, category, location, value band, risk, and conditions;
- approving full, partial, conditional, or rejected renewal;
- creating a successor eligibility period while preserving predecessor history;
- notifying sourcing and commitment roles of renewed, reduced, rejected, or expired scope.

## 4.4 Out of Scope

- original registration, activation, or qualification governed by L3-J-020 to L3-J-022;
- allowing expired eligibility to remain usable, governed by L3-J-024;
- ordinary profile changes governed by L3-J-026 and sensitive changes governed by L3-J-027;
- sourcing award, Purchase Order, contract, receipt, payable, accounting entry, or payment;
- technical expiry scheduler, workflow, or document-verification implementation.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-021 and L3-J-022 | Supply Active status and current qualification. |
| Followed by | L3-J-024 | Restricts eligibility when renewal is not effective before expiry. |
| Related | L3-J-026 and L3-J-027 | Material profile changes may require renewal reassessment. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Vendor & Supplier Management | Owns renewal scheduling, evidence coordination, reassessment, eligibility decision, validity, conditions, downstream communication, and audit evidence. |

Specialist domains retain authority for legal, tax, financial, technical, safety, security, privacy, quality, and regulatory evidence.

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

- A canonical supplier and identifiable current evidence or qualification position exist.
- The current position has an expiry date, review date, or renewal trigger.
- Supplier status, qualification scope, predecessor evidence, criteria, conditions, and downstream dependencies are available.
- Renewal lead times, grace rules if any, review requirements, authorities, and restriction rules are effective.
- The supplier can receive renewal notice through an authorised channel.
- Existing transactions can be distinguished from new sourcing or commitment reliance.

---

# 8. Trigger

An evidence item, approval condition, licence, certificate, insurance policy, qualification, or scheduled review enters its governed renewal window.

---

# 9. Main Business Flow

1. The institution identifies the expiring item, supplier, supporting qualification positions, expiry date, renewal deadline, affected scope, and downstream dependencies.
2. The Institutional Verifier confirms current Active status, qualification scope, conditions, previous decision, evidence lineage, and outstanding remediation.
3. The supplier receives a controlled request specifying required evidence, declarations, changes, formats, sources, response date, and consequence of non-renewal.
4. New evidence is captured with issuer, source, authenticity, issue date, effective date, expiry date, scope, version, and limitations.
5. The supplier declares changes in identity, ownership, control, representatives, addresses, licences, capacity, personnel, facilities, subcontractors, conflicts, litigation, incidents, sanctions, and performance.
6. Authoritative supplier status and restriction facts are refreshed; renewal does not override a suspension, prohibition, or unresolved material change.
7. Technical, financial, regulatory, quality, safety, security, sustainability, insurance, and other applicable criteria are reassessed proportionately.
8. Performance, complaints, defects, delays, claims, audit findings, corrective actions, capacity utilisation, and new risks since the previous decision are considered.
9. Requested renewal scope is compared with current evidence and demonstrable capacity; unsupported scope is reduced, conditioned, returned, or rejected.
10. Conflicts of supplier, sponsor, evaluator, and approver are declared and governed.
11. Gaps and inconsistencies are returned for clarification without overwriting previous evidence or extending validity automatically.
12. The recommendation identifies full, partial, conditional, returned, rejected, or non-renewed positions with scope, reasons, conditions, and new validity.
13. Required specialist and independent reviews are completed before the current expiry where practicable.
14. The Qualification Authority decides with attributable reason, effective time, and new expiry or review date.
15. Approved renewal creates a successor eligibility period linked to the prior period; it does not rewrite the original expiry or evidence.
16. Reduced, rejected, or incomplete scope is handed to L3-J-024 when current validity ends, and new reliance is blocked accordingly.
17. Supplier and downstream roles receive the outcome, while notices, evidence, reviews, decisions, actors, timestamps, and versions remain auditable.

---

# 10. Alternate Business Flows

## 10.1 Early Renewal

The new period begins according to approved overlap or successor rules. Remaining predecessor validity is not silently lost or double-counted.

## 10.2 Partial Renewal

Only supported categories, locations, value bands, capacities, licences, or conditions are renewed. Other scope expires independently.

## 10.3 Conditional Renewal

Time-bound eligibility may require remediation, additional insurance, supervision, reduced value, restricted geography, or enhanced monitoring where policy permits.

## 10.4 Evidence Issuance Pending

Pending applications or receipts do not equal valid evidence. Any permitted interim position requires an explicit rule, higher authority, end date, compensating controls, and no mandatory prohibition.

## 10.5 Renewal Submitted After Expiry

The expired position remains restricted under L3-J-024 until a new approved eligibility period becomes effective. Approval is not backdated.

## 10.6 Material Change Discovered

Renewal is held and routed through applicable profile-change, approval, qualification, restriction, or investigation governance before decision.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Supplier Does Not Respond

The current position expires at its authoritative time and proceeds to restriction. Non-response does not create an extension.

## 11.2 Evidence Invalid or Unverifiable

Affected renewal is returned or rejected and may be referred for integrity review.

## 11.3 Review Not Completed Before Expiry

Eligibility becomes restricted unless an explicit lawful interim rule was approved before expiry. Administrative delay alone cannot preserve eligibility.

## 11.4 Supplier Status Becomes Restricted

Renewal cannot activate the affected scope and is held, rejected, or superseded by the authoritative restriction.

## 11.5 Process Interruption

The case resumes from the last valid checkpoint; dates and time-sensitive checks remain authoritative and are not paused by system outage.

## 11.6 Downstream Reliance After Expiry

The attempt is blocked and investigated. Existing valid commitments remain with their authoritative owner and are not cancelled automatically.

## 11.7 New Adverse Evidence After Renewal

The fact is appended and routed to reassessment or restriction without deleting the renewal decision.

---

# 12. Business Rules and Decision Constraints

- Renewal must be approved before new eligibility becomes effective and cannot be inferred from document submission.
- Previous approval or satisfactory performance does not replace current evidence and reassessment.
- Original evidence, expiry, decision, conditions, and eligibility periods remain immutable.
- Renewal cannot override non-Active supplier status, mandatory prohibition, suspension, or unresolved sensitive change.
- New validity must be within the supporting evidence and authoritative rule period.
- Partial or conditional renewal must be exact and enforceable downstream.
- A grace or interim period exists only where expressly authorised and recorded before expiry.
- Late approval cannot be backdated to conceal an eligibility gap.
- Existing commitments do not automatically terminate when eligibility expires, but new sourcing or commitment reliance is blocked.
- Supplier, evaluator, sponsor, and approver conflicts must be governed.
- This scenario does not create a sourcing award, Purchase Order, contract, receipt, payable, accounting entry, or payment.

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
| `SupplierQualificationExpired` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
| `SupplierQualificationGranted` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| L3-J-020 to L3-J-022 | Supply identity, Active status, and qualification. | Retain predecessor lifecycle. |
| Specialist Reviewers | Validate renewed regulatory, technical, financial, safety, security, quality, and other evidence. | Retain specialist authority. |
| Sourcing / J04 | Consume only current eligibility and receive scope changes. | Retain event and commitment lifecycle. |
| L3-J-024 | Receives non-renewed or expired scope. | Retains expiry restriction. |
| L3-J-026 / L3-J-027 | Receive profile changes discovered during renewal. | Retain profile-change governance. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J02 roles are separated as policy requires: Supplier Representative; Institutional Verifier; Qualification Authority; and Supplier Governance Authority. The Supplier Representative cannot verify, qualify, approve, or remove an institutional restriction on itself.

## 15.2 Required Evidence

- supplier identity, Active status, current qualification, predecessor evidence, conditions, scope, validity, and downstream dependencies;
- renewal notice, supplier submission, declarations of change, issuer and authenticity checks, dates, scope, and limitations;
- refreshed status, ownership, conflicts, restrictions, capability, capacity, financial, regulatory, performance, incident, and specialist findings;
- evaluation, clarification, recommendation, conditions, approval or rejection, successor period, actors, reasons, timestamps, and versions.

## 15.3 Audit Obligations

The institution must reconstruct when renewal became due, what changed, which current evidence was verified, how performance and capacity were considered, why scope was renewed or reduced, who approved it, and whether any eligibility gap or interim exception occurred.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- the expiring evidence or qualification has been identified in advance of its effective expiry with sufficient lead time for governed renewal;
- the supplier has submitted current, valid replacement evidence or renewed qualification information from acceptable sources;
- the renewal has been reviewed and approved by an authority appropriate to the qualification scope and institutional requirements;
- renewed evidence has a new validity period, evidence dates, and version that are distinct from the expiring record;
- the prior expired evidence remains preserved as historical record and is not overwritten;
- the renewal decision, authority, effective dates, and evidence are auditable and immutable.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the supplier's evidence or qualification is in a current, valid, and renewed state for the applicable scope and validity period;
- the prior expired evidence and renewal decision are permanently preserved and auditable;
- the supplier's eligibility for governed sourcing, evaluation, or award continues without interruption where renewal is completed before expiry;
- where renewal is not completed before expiry, the eligibility restriction established under L3-J-024 takes effect;
- no retroactive backdating of evidence validity is permitted.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Renewal lead times, grace rules, interim authorities, review depth, and validity periods require institution-specific confirmation.
2. Evidence dependencies and the rule for deriving qualification validity from multiple expiries require an upstream J02 business rule.
3. Treatment of pending issuer renewal and regulator delay requires policy mapping.

## 17.2 Assumptions

- Renewal creates a successor eligibility period rather than modifying history.
- New sourcing requires current effective eligibility.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-023**
