# Sensitive Supplier Profile Change to Approved Authoritative Profile

# 1. Scenario Metadata

**Scenario ID:** L3-J-027  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Vendor & Supplier Management  
**Participating Domains:** F, J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** supplier approval, qualification, sourcing, commitment, tax, banking, payable, ownership, risk, restriction, suspension, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs high-risk or authoritative supplier-profile changes such as legal name, constitution, ownership, control, tax identity, registered address, authorised signatory, merger, demerger, successor relationship, and banking-related master references.

It prevents account takeover, payment diversion, duplicate identity, sanctions evasion, unauthorised ownership substitution, and silent reassignment of commitments or financial history.

---

# 3. Business Outcome

A sensitive change reaches Approved Authoritative Profile, Approved with Restrictions, Returned for Enhanced Evidence, Rejected, or Referred for New Canonical Supplier or Investigation. Approval creates a separately verified, effective-dated authoritative profile version and triggers necessary reassessment without rewriting historical transactions.

Outcome labels in this section are business dispositions or supporting-record results, not additional canonical lifecycle states. Section 13 states the only canonical lifecycle impact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a supplier, authoritative source, or institutional control identifies a proposed sensitive change to the supplier's legal, ownership, tax, authority, or financial-reference profile.

## 4.2 End Boundary

The scenario ends when the change is approved, limited, returned, rejected, or referred; dependent approval and eligibility are reassessed or restricted; and all historical identity and transaction links remain preserved.

## 4.3 In Scope

- legal name, constitution, registration, tax identity, registered address, ownership, beneficial ownership, control, parent or subsidiary, merger, demerger, succession, authorised signatories, and similar sensitive facts;
- banking-reference or payment-instruction master changes requiring independent financial verification, while Finance retains payment authority;
- enhanced requester authentication and independent authoritative verification;
- duplicate, sanctions, related-party, conflict, fraud, and identity-continuity analysis;
- deciding update of existing canonical identity versus new supplier identity and successor relationship;
- reassessing Active status, qualification, category eligibility, sourcing, commitments, tax, and financial controls;
- approving effective profile and preserving predecessor lineage.

## 4.4 Out of Scope

- ordinary operational profile changes governed by L3-J-026;
- payment execution or validation of a specific invoice or payable;
- supplier qualification, category, restriction, suspension, blacklisting, or closure decisions themselves;
- novating or amending Purchase Orders and contracts;
- rewriting receipts, payables, accounting entries, or payments;
- technical authentication, bank-validation, or screening implementation.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-020, L3-J-021, and L3-J-026 | Supply canonical identity, status, and change referral. |
| Related | L3-J-022 to L3-J-025 | Sensitive changes may invalidate eligibility. |
| Followed by | L3-J-029 and L3-J-030 | Risk findings may require restriction or suspension. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Vendor & Supplier Management | Owns sensitive-change intake, canonical-identity assessment, governance coordination, profile decision, reassessment hand-offs, and evidence. |

Finance retains bank, tax, payable, accounting, and payment controls. Legal and compliance retain authoritative interpretation of legal succession, ownership, sanctions, and regulatory consequences.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Supplier Representative | Supplies permitted profile information and evidence; cannot approve its own governed change. | Requests or responds. |
| Institutional Verifier | Independently verifies identity and supporting evidence. | Verifies evidence. |
| Sensitive-Change Requester / Approver | Requests and independently approves governed legal, tax, banking, contact or other sensitive changes. | Requests or decides subject to separation of duties. |
| Supplier Governance Authority | Governs supplier identity, merge, restriction or closure treatment where material. | Decides governed supplier treatment. |
---

# 7. Preconditions and Entry Criteria

- A canonical supplier identity and current authoritative profile exist.
- The requested change and affected sensitive attributes are identifiable.
- The requester can be authenticated through a channel independent of unverified changed contact details.
- Authoritative documents, registries, issuers, bank controls, tax sources, and specialist reviewers are available as required.
- Sensitive-change, identity-continuity, reassessment, hold, notification, and approval rules are effective.
- Affected qualifications, sourcing events, commitments, payables, and payment controls can be identified.

---

# 8. Trigger

A supplier or authoritative source reports a change to legal identity, ownership, control, tax, registered authority, succession, or banking-related profile facts, or an institutional control detects a discrepancy.

---

# 9. Main Business Flow

1. The change request records supplier identity, requester, affected attributes, old and proposed values, legal basis, effective date, evidence, channel, and submission time.
2. The requester and representative authority are authenticated using current trusted channels and independent confirmation; proposed new contacts alone cannot authorise the change.
3. A protective hold is applied to affected profile, sourcing, award, commitment, payable, or payment activity where diversion or identity risk requires it.
4. Legal name, constitution, registration, tax, ownership, control, signatory, succession, or bank-related evidence is verified from authoritative sources.
5. The Institutional Verifier determines whether legal identity continues, a new legal entity exists, a predecessor-successor link is required, or records must remain separate.
6. Duplicate, branch, parent, subsidiary, merger, demerger, acquisition, beneficial ownership, common control, and related-party implications are assessed.
7. Sanctions, debarment, conflicts, prohibited relationships, fraud indicators, account takeover, and unusual timing or communication are reviewed.
8. Banking-related changes receive independent Finance-controlled verification and cooling, dual-control, or payment-hold treatment as applicable; procurement does not approve payment execution.
9. Tax, legal, contractual, regulatory, licensing, insurance, data, security, and operational consequences are assessed.
10. Current Active status, qualification, category eligibility, sourcing participation, awards, open commitments, receipts, payables, and payment instructions are identified for authoritative-owner review.
11. Supplier explanations and additional evidence are assessed while conflicting facts remain preserved.
12. The recommendation defines approved attributes, identity treatment, effective time, restrictions, reassessment, notifications, and downstream actions.
13. Required legal, compliance, finance, tax, data, security, and independent approvals are completed.
14. The independent approver within the canonical Sensitive-Change Requester / Approver role approves, conditionally approves, returns, rejects, or refers the change for new identity or investigation.
15. Approval creates a new authoritative profile version; predecessor values and transaction-time identity remain immutable.
16. Dependent approval, qualification, category, sourcing, contract, tax, payable, and payment owners receive only the facts necessary for their decisions.
17. Authentication, evidence, verification, holds, decisions, effective dates, identity links, notifications, actors, and versions remain auditable.

---

# 10. Alternate Business Flows

## 10.1 Legal Name Change with Identity Continuity

Authoritative registration and unchanged legal identifier or succession evidence support a new profile version while aliases and historical names remain searchable.

## 10.2 Merger, Acquisition, Demerger, or Successor

The institution determines whether a new canonical supplier is required. Rights and obligations do not move automatically; J04 and Legal govern novation or assignment.

## 10.3 Ownership or Control Change

Beneficial owners, related parties, sanctions, conflicts, approval risk, qualification, and contract consequences are reassessed before ordinary use continues.

## 10.4 Tax Identity Change

The legal basis, effective date, registration scope, jurisdiction, invoices, withholding, and entity-continuity implications are reviewed with Finance and Tax.

## 10.5 Bank-Reference Change

Independent verification, segregation, trusted callback or equivalent control, payment hold, and Finance approval apply. Supplier email alone is insufficient.

## 10.6 Authorised Signatory Change

Corporate authority, board or equivalent evidence, scope, effective date, and revocation of predecessor authority are verified.

## 10.7 Institution-Detected Change

An authoritative registry or screening source may initiate review. The supplier is notified where lawful and current profile use is restricted proportionately.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Authentication Fails

The change is rejected or held, affected financial activity remains protected, and potential account takeover is escalated.

## 11.2 Evidence Conflicts

No authoritative change becomes effective until resolved; the current profile or protective restriction remains controlling.

## 11.3 New Legal Entity Detected

The update is stopped and canonical registration begins for the new entity with explicit predecessor-successor relationships.

## 11.4 Unauthorised Payment-Destination Change

The instruction is blocked and referred to Finance, fraud, security, and compliance roles; procurement profile approval cannot validate it.

## 11.5 Downstream Record Conflict

Each authoritative owner reviews its record. Profile change does not silently amend contracts, invoices, payables, or payments.

## 11.6 Process Interruption

Protective holds remain effective. On resumption, time-sensitive evidence and trusted contacts are refreshed.

## 11.7 Post-Approval Fraud or Error

A corrective version and restriction or investigation are initiated. History is not deleted or backdated.

---

# 12. Business Rules and Decision Constraints

- Sensitive changes require enhanced authentication, independent verification, segregation, and explicit approval.
- Proposed new contact or bank information cannot be the sole channel used to authorise itself.
- Legal identity continuity must be established before updating the existing canonical supplier.
- A new legal entity requires a distinct canonical identity unless authoritative governance proves otherwise.
- Ownership, control, sanctions, conflict, tax, qualification, and contractual impacts must be reassessed.
- Banking-related profile approval does not itself authorise payment or change a transaction-owned payment instruction.
- Historical supplier names, owners, tax identities, signatories, bank references, and effective periods remain preserved with restricted access.
- Rights and obligations do not transfer automatically after merger, acquisition, or succession.
- Sensitive changes cannot be backdated to rewrite historical commitments or financial records.
- This scenario does not create, novate, amend, or cancel a Purchase Order, contract, receipt, payable, accounting entry, or payment.

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
| `SupplierSensitiveChangeRequested` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
| `SupplierSensitiveChangeApproved` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
| `SupplierBankProfileApproved` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| L3-J-020 / L3-J-026 | Supply canonical identity and change referral. | Retain identity and ordinary-profile history. |
| Legal / Compliance | Validate legal succession, ownership, sanctions, conflicts, and authority. | Retain specialist authority. |
| Finance / Tax / Banking | Validate tax and banking controls. | Retain monetary and payment truth. |
| L3-J-021 to L3-J-025 | Reassess approval and eligibility. | Retain status and qualification decisions. |
| J04 | Reviews contract, novation, and commitment effects. | Retains commitment lifecycle. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J02 roles are separated as policy requires: Supplier Representative; Sensitive-Change Requester / Approver; Institutional Verifier; and Supplier Governance Authority. The requester and independent approver for a sensitive change remain distinct where policy requires.

## 15.2 Required Evidence

- current and proposed authoritative profile, requester identity, trusted-channel authentication, reason, legal basis, and effective date;
- registrations, tax, ownership, control, beneficial owners, signatories, succession, bank-related evidence, authoritative-source checks, and conflicts;
- duplicate and identity-continuity analysis, sanctions and fraud review, holds, affected eligibility and transactions, specialist findings, recommendation, approval, notifications, actors, timestamps, and versions.

## 15.3 Audit Obligations

The institution must reconstruct who requested the sensitive change, how authority was independently authenticated, which authoritative evidence established continuity, who verified each risk dimension, what was held or reassessed, and which profile was authoritative at any time.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- the sensitive profile change has been identified as requiring enhanced governance based on the applicable criteria for legal identity, ownership, sanctions, tax registration, or banking reference changes;
- an enhanced review has been performed by the required specialist contributors including legal, compliance, tax, or finance roles;
- the change has been approved by an authority appropriate to the sensitivity, materiality, and institutional risk of the change;
- prior qualified capabilities, active contracts, or existing commitments referencing the changed attribute have been assessed for impact;
- the enhanced review outcome, authority, conditions, and restrictions are documented and attributable;
- the approved change is the canonical authoritative profile version and the prior version is preserved as historical evidence.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the supplier profile reflects the approved and authoritative current values after enhanced governance review;
- the prior sensitive profile version is permanently preserved as historical evidence and is not overwritten;
- the enhanced review decision, specialist contributions, authority, conditions, and audit trail are permanently auditable;
- downstream sourcing, qualification, commitment, and payment scenarios may rely on the updated profile as the authoritative current state;
- any residual impact on existing qualifications, contracts, or obligations has been governed by the appropriate authoritative domain owners.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Sensitive attribute matrix, authentication strength, bank-verification controls, cooling periods, and approval authorities require confirmation.
2. Entity-continuity, merger, demerger, succession, novation, and tax-change rules require legal and finance mapping.
3. Exact protective holds and downstream reassessment by change type require operating-profile governance.

## 17.2 Assumptions

- Ordinary changes are handled under L3-J-026.
- Finance retains final bank and payment authority.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-027**
