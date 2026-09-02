# Supplier Profile Change to Governed Current Profile

## 1. Specification Metadata

**Specification ID:** L4-J-026  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Enterprise Resource Management — Vendor & Supplier Management  
**Realizes Scenario:** L3-J-026  
**Primary Business Owner:** Vendor & Supplier Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-026_Supplier_Profile_Change_to_Governed_Current_Profile.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP shall govern ordinary supplier-profile changes so that the current operational profile is accurate, verified, attributable and effective-dated while preserving historical values.

The system shall distinguish ordinary profile updates from sensitive, legal-identity, qualification, category, status and transaction-owned changes and shall route those changes to their authoritative governance rather than silently changing them here.

## 3. Required Business Context

- canonical supplier identity and current profile;
- authorised requester and supplier-representative relationship where applicable;
- current supplier state and relevant qualification/category facts;
- ordinary-versus-sensitive attribute classification rules;
- supporting evidence, reason and requested effective time;
- current/previous attribute value and source;
- affected downstream references where material.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-026-BH-001 | When a profile change is submitted, the system shall require the canonical supplier, requester, proposed change, prior value, reason, evidence and requested effective time to be attributable. | L3-J-026 §§7–9 step 1, 12 |
| L4-J-026-BH-002 | The system shall verify requester authority before permitting the change to progress toward an approved current profile. | L3-J-026 §§7, 9 steps 2–3, 11.1 |
| L4-J-026-BH-003 | The system shall classify the requested attribute against source-defined ordinary, sensitive, legal-identity, qualification, category, status or transaction ownership before changing the current value. | L3-J-026 §§4.3–4.4, 9 steps 4–5, 12 |
| L4-J-026-BH-004 | When a change is classified as sensitive or otherwise outside ordinary-profile ownership, the system shall prevent modification in this behavior and expose it for the applicable authoritative governance, including L3-J-027 where specified. | L3-J-026 §§4.4, 9 step 5, 11.3, 12 |
| L4-J-026-BH-005 | The system shall preserve conflicting or insufficient evidence and shall retain the current authoritative profile until the governed issue is resolved. | L3-J-026 §§9 step 10, 11.2 |
| L4-J-026-BH-006 | On approval of an ordinary profile change, the system shall create a new effective current profile version while preserving the preceding profile version as historical evidence. | L3-J-026 §§3, 9 steps 12–14, 12, 16 |
| L4-J-026-BH-007 | For future-effective changes, the system shall retain the existing value as current until the approved effective time. | L3-J-026 §10.6 |
| L4-J-026-BH-008 | The system shall not allow descriptive catalogue/capability changes to create qualification or category eligibility. | L3-J-026 §§10.3, 12 |
| L4-J-026-BH-009 | When an approved profile fact materially affects qualification or classification, the system shall make the reassessment need available to the authoritative qualification/classification behavior without changing that eligibility locally. | L3-J-026 §§9 step 15, 14.2 |
| L4-J-026-BH-010 | The system shall not rewrite existing Purchase Orders, contracts, receipts, payables, accounting entries or payments as a consequence of a profile change. | L3-J-026 §§4.4, 9 step 9, 12 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-026-BH-011 | On an approved ordinary profile change, the system shall publish or make available the canonical `SupplierProfileChanged` fact on behalf of Vendor & Supplier Management. | L3-J-026 §14.1 |
| L4-J-026-BH-012 | The system shall make relevant approved profile-change facts available to qualification, classification, J04 and Finance contexts while preserving each participant's canonical ownership and restricting sensitive information to authorised use. | L3-J-026 §§9 step 16, 14.2, 12 |

## 6. Failure / Rejection / Continuation

- An unauthorised requester shall not change the current profile.
- Insufficient/conflicting evidence shall result in no current-value change.
- A misclassified sensitive change shall stop ordinary processing and be referred to the appropriate governed path.
- Duplicate/legal-identity concerns shall prevent ordinary completion until canonical-identity governance resolves them.
- An approved profile remains authoritative even if downstream propagation is incomplete; propagation failure shall not create a duplicate business change.
- A post-change correction shall create a new attributable version rather than delete the erroneous version.

**Provenance:** L3-J-026 §11.

## 7. Audit & Evidence Behavior

The system shall preserve:

- requester identity/authority;
- prior and proposed values;
- evidence, reason and effective time;
- ordinary/sensitive ownership classification;
- duplicate/identity checks and material impacts;
- approval, return, rejection or referral decision;
- profile versions and effective dates;
- qualification/classification or other downstream referrals;
- actors, reasons and timestamps.

**Provenance:** L3-J-026 §15.

## 8. Observable Completion Conditions

The behavior is complete when the request has an attributable governed disposition; an approved ordinary change is represented by a new effective current profile version; the prior version remains historical; sensitive/out-of-scope changes have not been silently applied; `SupplierProfileChanged` is available for approved ordinary changes; and downstream owned records remain unchanged by this scenario.

## 9. Traceability & Provenance

**Parent:** L3-J-026 — Supplier Profile Change to Governed Current Profile  
**Primary verification source:** L2-J02 — Vendor & Supplier Management Workflows  

Material anchors: L3-J-026 §§2–16.

## 10. Open Issues / Deferred Design

The upstream ordinary/sensitive attribute matrix, evidence standards, approval levels, notification rules, and cross-domain attribute ownership remain unresolved business-governance items in L3-J-026. L4 does not assign them.

Implementation design remains deferred to L5/L6.
