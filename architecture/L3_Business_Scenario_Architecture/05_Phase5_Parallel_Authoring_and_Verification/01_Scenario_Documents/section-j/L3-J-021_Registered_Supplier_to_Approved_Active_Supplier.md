# Registered Supplier to Approved Active Supplier

# 1. Scenario Metadata

**Scenario ID:** L3-J-021  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Vendor & Supplier Management  
**Participating Domains:** J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** supplier qualification, sourcing, award, commitment, receipt, payable, performance, evidence-renewal, restriction, suspension, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs institutional due diligence, risk review, approval, and activation of a canonical Registered Supplier. It determines whether the institution may maintain an active commercial relationship with the supplier, independently of whether the supplier is qualified for a particular category, tender, geography, value band, funding source, or engagement.

It prevents a mere registration record, sourcing response, historic transaction, requester preference, or externally known supplier from being treated as institutionally approved.

---

# 3. Business Outcome

A Supplier in `Registered` reaches the canonical `Approved` and then `Active` states, or receives a returned, rejected, or deferred approval decision without a new Supplier lifecycle state. The outcome records due diligence, risk classification, declarations, restrictions, approval authority, activation date, review date, conditions, and evidence lineage.

Outcome labels in this section are business dispositions or supporting-record results, not additional canonical lifecycle states. Section 13 states the only canonical lifecycle impact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a canonical Registered Supplier is nominated for institutional approval and activation for a legitimate prospective relationship.

## 4.2 End Boundary

The scenario ends when approval is granted, conditionally granted, returned, deferred, or rejected; the supplier's institutional activity state is explicit; and later qualification or sourcing processes can consume only valid status.

## 4.3 In Scope

- validating canonical identity, current evidence, ownership, related parties, declarations, and registration completeness;
- performing proportionate legal, compliance, integrity, sanctions, debarment, conflict, financial, operational, data, safety, and reputational due diligence;
- assessing supplier capability at enterprise level without granting category-specific qualification;
- assigning institutional risk tier, review frequency, conditions, restrictions, and monitoring obligations;
- approving or rejecting the supplier relationship and controlling effective activation;
- notifying responsible roles and preserving evidence, approvals, exceptions, and versions.

## 4.4 Out of Scope

- creating or correcting the canonical supplier identity governed by L3-J-020;
- granting category, technical, geographical, funding, project, or tender-specific eligibility governed by L3-J-022;
- approving supplier bank-payment instructions;
- awarding sourcing, creating a Purchase Order or contract, confirming receipt, payable, accounting entry, or payment;
- suspension, blacklisting, evidence renewal, qualification restriction, or closure after activation;
- technical screening services or workflow design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-020 | Supplies the canonical Registered Supplier identity. |
| Followed by | L3-J-022 | Establishes scope-specific institutional qualification. |
| Followed by | L3-J-023 and L3-J-024 | Govern evidence renewal and expiry restriction. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Vendor & Supplier Management | Owns supplier due diligence coordination, risk classification, approval recommendation, activation, conditions, review scheduling, and evidence. |

Legal, compliance, finance, tax, information security, safety, and other specialists retain their authoritative findings. Supplier approval does not replace their continuing controls.

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

- One canonical Registered Supplier exists and is not an unresolved duplicate.
- Legal identity and authorised representative facts are sufficiently verified and current.
- A legitimate institutional reason and expected relationship profile are recorded.
- Due-diligence checks, risk tiers, declarations, approval authorities, review periods, conditions, and rejection rules are effective.
- Required specialist sources and supplier responses can be obtained.
- The supplier is not already Active under the same canonical identity unless this is an authorised reapproval or status correction.

---

# 8. Trigger

An authorised procurement or supplier-governance role submits a Registered Supplier for institutional approval and activation.

---

# 9. Main Business Flow

1. The Institutional Verifier records the supplier, sponsor, intended relationship, expected categories, geography, value or exposure profile, data or site access, and submission time.
2. Canonical identity, ownership, related entities, authorised representatives, tax and registration evidence, contacts, and duplicate disposition are confirmed from L3-J-020.
3. Required checks are determined according to supplier type, jurisdiction, category interests, public or private status, risk, value, data, safety, funding, and expected relationship.
4. Sanctions, debarment, watchlist, prohibition, conflict, related-party, beneficial ownership, politically exposed or other applicable integrity checks are performed from authorised sources.
5. Legal existence, licences, regulatory standing, litigation or insolvency indicators, adverse findings, and authority to operate are assessed proportionately.
6. Financial capacity, tax standing, insurance, credit or solvency evidence, and concentration or dependency risk are assessed where required.
7. Operational capacity, business continuity, quality, safety, information security, privacy, sustainability, labour, environmental, and ethical declarations are assessed where applicable.
8. Historic institutional experience, performance, disputes, incidents, complaints, and predecessor or related-entity history are linked without assuming guilt from relationship alone.
9. Conflicts involving supplier owners, representatives, sponsors, reviewers, approvers, or institutional personnel are declared and mitigated or escalated.
10. Gaps, inconsistencies, adverse findings, expired evidence, or remediation needs are communicated to the supplier through controlled channels.
11. Supplier explanations and additional evidence are assessed independently, with original findings and chronology preserved.
12. The supplier receives an institutional risk tier and proposed review frequency based on evidence and expected exposure.
13. The recommendation specifies approval, conditional approval, deferral, return, or rejection; permitted activity state; conditions; restrictions; monitoring; effective date; and next review date.
14. Required specialist concurrence and exception approvals are obtained without allowing a sponsor to override mandatory restrictions.
15. The Supplier Approver decides with attributable reason and effective time.
16. Approved suppliers become Active; conditionally approved suppliers become Active with explicit enforceable limits; rejected or deferred suppliers remain unavailable for ordinary institutional sourcing.
17. The outcome, conditions, review dates, evidence, specialist findings, declarations, decisions, actors, timestamps, and versions remain linked and auditable.

---

# 10. Alternate Business Flows

## 10.1 Conditional Approval

The supplier may be activated with defined value, category, geography, duration, access, payment, supervision, insurance, remediation, or monitoring conditions. Conditions cannot waive mandatory prohibitions.

## 10.2 Low-Risk or Public Entity

A proportionate reduced check set may apply where policy permits, but canonical identity, authority, conflicts, prohibitions, approval, and review timing remain mandatory.

## 10.3 Foreign Supplier

Equivalent foreign evidence, sanctions, ownership, tax, legal, currency, jurisdiction, serviceability, and enforcement risks are assessed without inventing domestic documents.

## 10.4 Previously Used or Externally Approved Supplier

Prior transactions, marketplace presence, government registration, certification, or another institution's approval are evidence inputs only. Institutional approval remains explicit.

## 10.5 Adverse Finding with Permitted Remediation

Activation is deferred or conditional until defined remediation is verified. Serious mandatory prohibitions produce rejection or referral.

## 10.6 Reapproval After Material Identity Change

Legal form, ownership, control, merger, demerger, insolvency, or successor change triggers refreshed approval after canonical identity governance.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Mandatory Check Unavailable

Approval is held unless a valid risk-based rule permits conditional action with explicit compensating controls and higher authority.

## 11.2 Conflicting Evidence

The Supplier remains in its current canonical state; any interim restriction is recorded separately until authoritative sources resolve the conflict.

## 11.3 Sanction, Debarment, Fraud, or Prohibition Concern

Activation is blocked and the matter is referred to the appropriate authoritative process. Registration history remains preserved.

## 11.4 Supplier Fails to Respond

The case may expire, be returned, or be rejected after the defined response period. Silence is not approval.

## 11.5 Approval Interrupted

The Supplier remains `Registered` and has not reached `Approved` or `Active`. On resumption, time-sensitive checks and evidence are refreshed.

## 11.6 Unauthorised Activity Attempt

Sourcing, award, or commitment against a non-active supplier is blocked and escalated; the attempt does not activate the supplier.

## 11.7 Adverse Fact Discovered After Activation

The fact is recorded and referred immediately for restriction, suspension, investigation, or other governed action without deleting the original approval.

---

# 12. Business Rules and Decision Constraints

- Only a canonical Registered Supplier may enter institutional approval.
- Registration, response to a bid, historic use, external approval, or requester sponsorship does not establish Active status.
- Due diligence is risk-based but mandatory prohibitions cannot be waived by commercial urgency.
- Supplier ownership, related parties, conflicts, sanctions, debarment, and adverse findings must remain attributable to sources and dates.
- The approver cannot be the supplier, requester alone, verifier alone, or a conflicted actor.
- Conditional approval must be explicit, measurable, time-bound where appropriate, and enforceable downstream.
- Active status has an effective date and review date and may be limited by evidence expiry.
- Active status alone does not grant category or event-specific qualification.
- Approval does not validate bank-payment instructions or create financial liability.
- Material identity, ownership, control, legal, sanction, insolvency, or risk changes require reassessment.
- Rejection is distinct from suspension or blacklisting and must not imply a prohibited status not actually decided.
- This scenario does not create a Purchase Order, contract, receipt, payable, accounting entry, or payment.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Supplier | `Registered` / `Submitted for Onboarding` / `Verification Pending` | Applies verification and approval using the exact J02 lifecycle. | `Approved` then `Active`, or `Returned / Rejected` | Vendor & Supplier Management (J02) | `L2-J02` Section 6 |
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `SupplierSubmittedForVerification` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
| `SupplierVerificationCompleted` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
| `SupplierApproved` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
| `SupplierActivated` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
| `SupplierRejected` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| L3-J-020 | Supplies canonical identity, ownership, relationships, and evidence. | Retains identity governance. |
| Legal / Compliance | Supplies sanctions, debarment, regulatory, integrity, ownership, and conflict findings. | Retains specialist authority. |
| Finance / Tax | Supplies capacity, tax, insolvency, insurance, and exposure findings. | Retains financial truth. |
| Security / Safety / Privacy | Supplies applicable control findings. | Retains specialist control authority. |
| L3-J-022 | Consumes Active status for qualification. | Retains scope-specific eligibility. |
| Sourcing / J04 | Consumes status and conditions but performs separate method, award, and commitment controls. | Retains transaction lifecycle. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J02 roles are separated as policy requires: Supplier Representative; Supplier Creator / Onboarding Requester; Institutional Verifier; Supplier Approver; and Supplier Governance Authority. A supplier representative or conflicted requester cannot verify or approve its own governed outcome.

## 15.2 Required Evidence

- canonical supplier identity, ownership, relationships, representative authority, registration evidence, and intended relationship;
- applicable check plan, sanctions, debarment, conflicts, integrity, legal, regulatory, financial, tax, insurance, operational, security, safety, privacy, sustainability, and ethical findings;
- historic performance, incidents, disputes, related-entity links, supplier responses, remediation, and residual risks;
- risk tier, review frequency, conditions, restrictions, recommendation, specialist concurrence, exception approval, decision, actors, reasons, timestamps, and versions.

## 15.3 Audit Obligations

The institution must reconstruct which checks applied, what each source showed, how adverse findings and conflicts were resolved, why the risk tier and conditions were selected, who approved activation, and whether downstream use respected the effective status and conditions.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- the Registered Supplier has been reviewed and meets the institutional eligibility criteria required for approval;
- an authorised approval decision has been made and recorded with the approving actor, authority, date, conditions, and evidence reference;
- the supplier's identity, registration evidence, declarations, and governance basis used for the approval decision are preserved;
- where conditions apply, they are explicit and attributable to a governing authority;
- the approval is against a specific registration version and cannot be retrospectively applied to a later corrected version without re-approval;
- the Active status is limited to the scope and conditions defined in the approval decision.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the supplier is in an Active lifecycle state and is eligible for use in governed sourcing, evaluation, award, and commitment scenarios subject to qualification;
- the approval decision, authority, conditions, and evidence are permanently preserved and auditable;
- the supplier's Active status does not imply qualification for any specific category, tender, or award unless separately established;
- the approval lifecycle is distinct from and does not replace supplier qualification established under L3-J-022;
- suspension, restriction, blacklisting, or reversal of the Active status requires a separate governed action under its own authoritative scenario.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Check matrices, risk tiers, mandatory prohibitions, exception authorities, and review periods by supplier profile require confirmation.
2. Institution-specific sanctions, debarment, PEP, beneficial ownership, sustainability, security, safety, and financial thresholds require policy mapping.
3. Boundary between conditional activation and scope-specific qualification requires final J02 governance.

## 17.2 Assumptions

- Registration precedes approval, and approval precedes qualification.
- Active status is institution-wide relationship approval, not universal sourcing eligibility.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-021**
