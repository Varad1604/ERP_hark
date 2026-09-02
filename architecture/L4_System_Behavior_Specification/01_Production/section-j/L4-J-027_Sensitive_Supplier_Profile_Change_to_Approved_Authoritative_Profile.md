# Sensitive Supplier Profile Change to Approved Authoritative Profile

## 1. Specification Metadata

**Specification ID:** L4-J-027  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Enterprise Resource Management — Vendor & Supplier Management  
**Realizes Scenario:** L3-J-027  
**Primary Business Owner:** Vendor & Supplier Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-027_Sensitive_Supplier_Profile_Change_to_Approved_Authoritative_Profile.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP shall govern sensitive supplier-profile changes through enhanced verification, independent authority and attributable effective-dated profile history.

The behavior begins when a legal, ownership, tax, authority, succession or banking-related supplier-profile change is proposed or detected and ends when the change is approved, restricted, returned, rejected or referred for new canonical supplier identity/investigation.

This specification does not transfer Finance payment authority, does not novate commitments, and does not rewrite historical transactions.

## 3. Required Business Context

- canonical supplier identity and current authoritative profile;
- requester identity and current trusted contact/authority context;
- affected sensitive attributes, prior values and proposed values;
- authoritative legal, ownership, tax, signatory, succession or banking-related evidence as applicable;
- current approval, qualification, category, sourcing, commitment and relevant financial-control context;
- specialist findings from Finance, Tax, Legal, Compliance or other authoritative owners where applicable;
- independent Sensitive-Change approval decision.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-027-BH-001 | When a sensitive change is initiated, the system shall require the canonical supplier, requester, affected attributes, old and proposed values, reason/legal basis, evidence and requested effective time to be attributable. | L3-J-027 §§7–9 step 1 |
| L4-J-027-BH-002 | The system shall prevent proposed new contact or banking information from being the sole basis used to authenticate or authorise its own change. | L3-J-027 §§7, 9 step 2, 10.5, 12 |
| L4-J-027-BH-003 | Where the source-defined risk requires protective control, the system shall retain the affected profile or dependent activity in the governed protective position until authoritative review permits continuation. | L3-J-027 §§9 step 3, 11.1–11.2 |
| L4-J-027-BH-004 | The system shall require authoritative evidence sufficient to establish legal-identity continuity before updating the existing canonical supplier with a sensitive legal or succession change. | L3-J-027 §§9 steps 4–6, 10.1–10.2, 12 |
| L4-J-027-BH-005 | When evidence establishes a distinct legal entity rather than identity continuity, the system shall prevent silent replacement of the current supplier identity and shall expose the case for new canonical supplier registration with predecessor-successor linkage. | L3-J-027 §§9 step 5, 10.2, 11.3, 12 |
| L4-J-027-BH-006 | The system shall preserve sanctions, conflict, fraud, related-party and identity-continuity findings as attributable inputs to the sensitive-change decision without making their specialist determinations itself. | L3-J-027 §§9 steps 6–9, 14.2 |
| L4-J-027-BH-007 | Banking-related changes shall not become authoritative for payment use until the required Finance-controlled verification outcome exists; procurement approval alone shall not authorise payment execution. | L3-J-027 §§4.3–4.4, 9 step 8, 10.5, 12 |
| L4-J-027-BH-008 | When the independent authority approves a sensitive change, the system shall create a new effective authoritative profile version while preserving predecessor values and transaction-time identity. | L3-J-027 §§9 steps 12–15, 12, 16 |
| L4-J-027-BH-009 | The system shall make material approved change facts available for reassessment by qualification, category, sourcing, commitment, tax, payable and payment owners without changing those owners' records locally. | L3-J-027 §§9 steps 10, 16, 14.2 |
| L4-J-027-BH-010 | The system shall prevent backdating of a sensitive profile change where backdating would rewrite historical commitments or financial records. | L3-J-027 §12 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-027-BH-011 | When a governed sensitive-change request is established, the system shall publish or make available the canonical `SupplierSensitiveChangeRequested` fact on behalf of Vendor & Supplier Management. | L3-J-027 §14.1 |
| L4-J-027-BH-012 | On approved sensitive change, the system shall publish or make available the canonical `SupplierSensitiveChangeApproved` fact without changing its publisher. | L3-J-027 §14.1 |
| L4-J-027-BH-013 | Where the approved change is the governed supplier bank-profile fact defined upstream, the system shall publish or make available the canonical `SupplierBankProfileApproved` fact while preserving Finance payment authority. | L3-J-027 §§12, 14.1–14.2 |

## 6. Failure / Rejection / Continuation

- Failed requester authentication shall prevent the sensitive change from becoming effective.
- Conflicting authoritative evidence shall preserve the current profile or applicable protective restriction until resolved.
- Detection of a new legal entity shall stop ordinary profile replacement and route to new canonical supplier governance.
- Unauthorised payment-destination change shall not become valid through supplier-profile approval.
- Downstream record conflict shall be referred to the authoritative record owner rather than reconciled by rewriting owned transactions.
- On interruption, active protective holds remain governed by their source-defined business authority.
- Post-approval fraud/error shall be corrected by an attributable successor version; history shall remain intact.

**Provenance:** L3-J-027 §11.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of:

- requester and trusted-channel verification;
- current and proposed sensitive profile values;
- authoritative registrations, ownership/control, tax, signatory, succession and banking-related evidence;
- identity-continuity and duplicate analysis;
- sanctions/conflict/fraud findings and specialist contributions where applicable;
- protective holds and affected downstream positions;
- approval/return/rejection/referral decision, authority, conditions and effective time;
- predecessor/successor profile lineage;
- published canonical events and downstream reassessment referrals.

**Provenance:** L3-J-027 §15.

## 8. Observable Completion Conditions

The behavior is complete when the sensitive change has a governed disposition; any approved profile version is effective-dated and independently approved; predecessor history remains preserved; distinct legal entities have not been silently merged; Finance payment authority has not been transferred; and all material reassessment/referral facts are available to their authoritative owners.

## 9. Traceability & Provenance

**Parent:** L3-J-027 — Sensitive Supplier Profile Change to Approved Authoritative Profile  
**Primary verification source:** L2-J02 — Vendor & Supplier Management Workflows  

Material anchors: L3-J-027 §§2–16.

## 10. Open Issues / Deferred Design

Sensitive-attribute classification, authentication strength, bank-verification controls, cooling periods, competent approval levels, entity-continuity rules and exact protective-hold policies remain upstream governance issues recorded by L3-J-027.

Implementation mechanisms remain deferred to L5/L6.
