# Supplier Profile Change to Governed Current Profile

# 1. Scenario Metadata

**Scenario ID:** L3-J-026  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Vendor & Supplier Management  
**Participating Domains:** J  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** supplier approval, qualification, classification, sourcing, commitment, communication, evidence renewal, sensitive-profile change, performance, restriction, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs ordinary changes to an existing supplier profile so current operational information remains accurate, verified, attributable, and effective-dated without rewriting canonical identity or bypassing sensitive-change approval.

It covers controlled updates such as ordinary contacts, communication preferences, non-legal operating addresses, catalogue descriptions, service coverage, and other permitted non-sensitive attributes while preserving prior values and downstream impact.

---

# 3. Business Outcome

A submitted supplier profile change reaches Governed Current Profile, Returned for Correction, Rejected Change, or Referred as Sensitive Change. An approved change creates a new effective profile version with verified attributes, source, reason, authority, validity, and history.

Outcome labels in this section are business dispositions or supporting-record results, not additional canonical lifecycle states. Section 13 states the only canonical lifecycle impact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a supplier or authorised institutional role proposes a change to a current non-sensitive supplier-profile attribute.

## 4.2 End Boundary

The scenario ends when the change is approved, returned, rejected, or routed to L3-J-027; the current profile is explicit; affected roles receive the fact; and prior values remain auditable.

## 4.3 In Scope

- validating supplier identity, requester authority, change reason, source, and evidence;
- classifying the requested attribute as ordinary, sensitive, legal-identity, qualification, classification, or status-related;
- updating permitted operational contacts, communication channels, operating locations, catalogue descriptions, service coverage, and similar non-sensitive fields;
- checking consistency with legal identity, Active status, qualification, category eligibility, and existing commitments;
- approving, returning, rejecting, or referring the change;
- creating an effective-dated profile version and publishing appropriate downstream facts.

## 4.4 Out of Scope

- legal name, constitution, ownership, tax identity, bank instructions, sanctions, or other sensitive changes governed by L3-J-027;
- qualification, category eligibility, renewal, restriction, suspension, blacklisting, or closure;
- altering Purchase Orders, contracts, receipts, payables, accounting entries, or payments;
- technical master-data schema, portal, workflow, or synchronisation implementation.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-020 and L3-J-021 | Supply canonical identity and supplier status. |
| Related | L3-J-027 | Receives sensitive or authoritative-profile changes. |
| Related | L3-J-022 to L3-J-025 | May require reassessment when changed facts affect eligibility. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Vendor & Supplier Management | Owns change intake, classification, verification, approval, profile versioning, downstream notification, and evidence. |

J04 retains commitment truth and Finance retains bank, tax, payable, accounting, and payment truth.

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

- A canonical supplier identity and current profile exist.
- The requester and authority to propose the change can be verified.
- Attribute ownership and ordinary-versus-sensitive classification rules are effective.
- Required evidence, approval, effective-date, notification, and version rules are available.
- Current supplier status, qualifications, category eligibility, and affected references can be identified.
- The proposed change is not used to create a second supplier identity silently.

---

# 8. Trigger

A supplier or authorised internal role requests an update to a current supplier-profile attribute, or an authoritative ordinary-profile discrepancy is detected.

---

# 9. Main Business Flow

1. The request records supplier identity, requester, relationship, attribute, old and proposed values, reason, evidence, requested effective date, and submission time.
2. The requester and supplier representative authority are verified.
3. The Institutional Verifier confirms the current canonical record, supplier state, existing value, source, validity, and pending changes.
4. The attribute is classified as ordinary, sensitive, legal-identity, qualification, category, status, or transaction-owned.
5. Sensitive or authoritative changes are referred to L3-J-027 without changing the current value here.
6. Ordinary evidence is checked for completeness, plausibility, consistency, issuer or source, and effective date.
7. Duplicate, branch, site, related-entity, and legal-identity implications are assessed where addresses, contacts, names, or coverage could indicate a different entity.
8. The change is compared with current approval conditions, qualifications, category eligibility, locations, communications, and open sourcing dependencies.
9. Any impact on existing commitments is sent to J04; this scenario does not amend those records.
10. Conflicts between supplier statements and institutional or authoritative sources are preserved and returned or escalated.
11. Required category, qualification, data, legal, finance, or other contributors provide findings.
12. The recommendation identifies approval, partial approval, return, rejection, or sensitive referral and exact effective scope.
13. The Institutional Verifier decides the ordinary profile change with reason and effective time; sensitive changes are referred to L3-J-027.
14. An approved change creates a new current profile version; the previous version remains historical.
15. Dependent qualification or classification reassessment is initiated where the approved fact changes eligibility.
16. The supplier and authorised downstream users receive appropriate change facts without exposing restricted information.
17. Request, evidence, verification, decision, versions, downstream referrals, actors, and timestamps remain auditable.

---

# 10. Alternate Business Flows

## 10.1 Contact Change

The institution verifies authorised contact assignment and separates operational, legal-notice, sourcing, service, and financial contacts.

## 10.2 Operating Address or Service-Coverage Change

Legal registered address remains unchanged unless handled by L3-J-027. Qualification, geography, tax, licence, delivery, and support impacts are assessed.

## 10.3 Catalogue or Capability Description

Descriptive updates do not create category eligibility or qualification. Unsupported claims remain non-authoritative.

## 10.4 Institution-Initiated Correction

An authoritative source or verified error may initiate correction. The supplier is notified where required, and provenance remains explicit.

## 10.5 Partial Approval

Supported ordinary attributes are updated while sensitive, conflicting, or unsupported items are returned or referred separately.

## 10.6 Future-Effective Change

The approved value becomes current only at the stated effective time; the existing value remains authoritative until then.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Unauthorised Requester

The request is rejected or held for supplier confirmation; no current value changes.

## 11.2 Insufficient or Conflicting Evidence

The request is returned and the current profile remains authoritative.

## 11.3 Sensitive Change Misclassified

Processing is stopped and routed to L3-J-027. Any premature change is reversed through attributable correction, not deletion.

## 11.4 Duplicate or Identity Concern

The change is held for data-steward review and may trigger canonical-identity governance.

## 11.5 Downstream Synchronisation Failure

The approved effective profile remains authoritative; failed propagation is recorded and reconciled without duplicate changes.

## 11.6 Post-Change Error

A new corrective version is created. The erroneous and corrected versions remain visible.

## 11.7 Fraud or Integrity Concern

The request is held and referred for sensitive review, restriction, or investigation as applicable.

---

# 12. Business Rules and Decision Constraints

- Every profile change requires verified supplier identity, authorised requester, reason, evidence, and effective time.
- Ordinary profile governance cannot change legal identity, ownership, tax identity, payment instructions, supplier status, or qualification silently.
- Current and historical values must be effective-dated and attributable.
- Supplier self-service submission is not approval.
- Descriptive capabilities do not create qualification or category eligibility.
- Existing transaction records are not rewritten by profile change.
- A change suggesting a different legal entity, duplicate, branch, successor, or related party must be escalated.
- Sensitive information follows least-access and controlled notification.
- Future-effective and corrected changes cannot be backdated to conceal history.
- This scenario does not create or amend a Purchase Order, contract, receipt, payable, accounting entry, or payment.

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
| `SupplierProfileChanged` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| L3-J-020 / L3-J-021 | Supply canonical identity and current status. | Retain identity and approval. |
| L3-J-022 to L3-J-025 | Receive capability, geography, or classification-impact facts. | Retain qualification and category eligibility. |
| L3-J-027 | Receives sensitive changes. | Retains authoritative sensitive approval. |
| J04 / Finance | Receive relevant facts without record rewriting. | Retain commitment and monetary truth. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J02 roles are separated as policy requires: Supplier Representative; Sensitive-Change Requester / Approver; Institutional Verifier; and Supplier Governance Authority. The requester and independent approver for a sensitive change remain distinct where policy requires.

## 15.2 Required Evidence

- supplier identity, requester authority, current and proposed values, reason, source, effective date, and supporting evidence;
- attribute classification, duplicate and identity checks, status, qualification, category, location, and downstream impact;
- review, conflicts, recommendation, approval or rejection, profile versions, notifications, referrals, actors, reasons, and timestamps.

## 15.3 Audit Obligations

The institution must reconstruct who requested each change, which evidence supported it, why it was ordinary or sensitive, who approved it, when it became effective, which profile version was current at any time, and what downstream reassessment followed.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- the requested profile change has been reviewed and approved under the authority appropriate to the change type and materiality;
- the changed profile attribute, the prior value, the new value, the reason, the authority, and the effective date are recorded;
- original submitted values and evidence provenance are preserved after any normalisation;
- changes affecting identity, legal status, tax, or ownership have been assessed against the requirement for enhanced review under L3-J-027;
- the updated supplier profile is the canonical current version for institutional use;
- the change decision, evidence, actors, and audit trail are preserved and immutable.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the supplier profile reflects the approved current values for the changed attributes;
- the prior profile version is preserved as historical record and is not overwritten;
- the change is permanently auditable with authority, reason, effective date, and evidence reference;
- downstream sourcing, qualification, and commitment scenarios may rely on the updated profile as the authoritative current state;
- no change to identity, legal basis, or qualification scope has been made without the enhanced governance required under L3-J-027 where applicable.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Ordinary and sensitive attribute matrix, evidence standards, approval levels, and notification rules require confirmation.
2. Attribute ownership across procurement, finance, tax, legal, and contract domains requires an upstream cross-domain ownership decision; it cannot be assigned in L3 or L4.
3. Rules for institution-initiated corrections and supplier disputes require governance.

## 17.2 Assumptions

- Canonical supplier identity already exists.
- Sensitive changes are routed to L3-J-027.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-026**
