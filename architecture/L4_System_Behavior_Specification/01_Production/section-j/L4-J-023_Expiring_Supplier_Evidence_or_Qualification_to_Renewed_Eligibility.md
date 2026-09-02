# Expiring Supplier Evidence or Qualification to Renewed Eligibility

## 1. Specification Metadata

**Specification ID:** L4-J-023  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Enterprise Resource Management — Vendor & Supplier Management  
**Realizes Scenario:** L3-J-023  
**Primary Business Owner:** Vendor & Supplier Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-023_Expiring_Supplier_Evidence_or_Qualification_to_Renewed_Eligibility.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP shall govern renewal of supplier evidence or qualification before current eligibility expires, creating a successor eligibility period only after current evidence, supplier status, performance, capacity, risk and applicable qualification conditions support the authorised renewal decision.

The behavior ends with full, partial, conditional, returned, rejected or expired-pending-restriction disposition. It does not silently extend the predecessor record or backdate a later renewal.

## 3. Required Business Context

- canonical supplier and current evidence/qualification position;
- current validity/expiry or review trigger;
- current supplier `Active` status and restrictions;
- predecessor qualification scope, evidence and conditions;
- renewal rules, validity rules and Qualification Authority;
- current replacement evidence and declarations of material change;
- performance, capacity, incident and risk facts where applicable.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-023-BH-001 | When a governed renewal window is reached, the system shall identify the expiring evidence or qualification, its affected scope, expiry time and predecessor eligibility. | L3-J-023 §§4.1, 7, 8, 9 step 1 |
| L4-J-023-BH-002 | The system shall require current supplier status, restrictions and material-change facts to be evaluated before a renewed eligibility period can become effective. | L3-J-023 §§9 steps 2, 5–8, 12 |
| L4-J-023-BH-003 | The system shall preserve replacement evidence as a distinct attributable evidence set with its own source, effective date, expiry date, scope and limitations. | L3-J-023 §§9 step 4, 12, 16 |
| L4-J-023-BH-004 | The system shall prevent document submission, pending issuer action or prior satisfactory performance from being treated as approved renewal. | L3-J-023 §§10.4, 11.1–11.3, 12 |
| L4-J-023-BH-005 | The system shall represent full, partial or conditional renewal only for scope currently supported by evidence, capacity and governing conditions. | L3-J-023 §§9 steps 9–14, 10.2–10.3 |
| L4-J-023-BH-006 | When the Qualification Authority approves renewal, the system shall create a successor eligibility period linked to the predecessor rather than changing the predecessor's evidence, expiry or decision history. | L3-J-023 §§9 steps 14–15, 12, 16 |
| L4-J-023-BH-007 | The system shall prevent a renewed eligibility period from becoming effective before the authorised effective time or outside the validity permitted by its supporting evidence. | L3-J-023 §§10.1, 10.5, 12 |
| L4-J-023-BH-008 | If current eligibility expires before an effective successor exists, the system shall make the affected scope unavailable for new reliance and hand the expiry fact to the L3-J-024 restriction behavior. | L3-J-023 §§4.2, 9 step 16, 11.3, 16.2 |
| L4-J-023-BH-009 | The system shall preserve any historical eligibility gap and shall not backdate late approval to conceal that gap. | L3-J-023 §§10.5, 12, 16.2 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-023-BH-010 | When renewal establishes a new qualified period, the system shall publish or make available the canonical `SupplierQualificationGranted` fact on behalf of Vendor & Supplier Management. | L3-J-023 §14.1 |
| L4-J-023-BH-011 | When qualification expires without effective renewal, the system shall publish or make available the canonical `SupplierQualificationExpired` fact and make the affected scope available to governed restriction processing. | L3-J-023 §§14.1–14.2 |
| L4-J-023-BH-012 | The system shall expose renewed, reduced, rejected or expired scope to authorised downstream sourcing/commitment contexts without altering their transaction lifecycles. | L3-J-023 §§4.3, 9 step 17, 14.2 |

## 6. Failure / Rejection / Continuation

- Non-response shall not extend eligibility beyond authoritative expiry.
- Invalid or unverifiable replacement evidence shall prevent successful renewal for the affected scope.
- If review is incomplete at expiry, eligibility shall become restricted unless an upstream-authorised interim rule already applies.
- A current supplier restriction shall prevent renewed scope from becoming effective where the restriction governs that scope.
- On interruption, the system shall preserve the last valid position while continuing to respect authoritative business dates.

**Provenance:** L3-J-023 §11.

## 7. Audit & Evidence Behavior

The system shall preserve:

- predecessor eligibility and evidence;
- renewal trigger and timing;
- replacement evidence and authenticity/validity findings;
- supplier declarations of change;
- refreshed status, risk, performance and capacity findings;
- renewal recommendation and Qualification Authority decision;
- successor period, conditions and scope;
- any expiry gap, interim exception or restriction hand-off;
- actors, reasons, timestamps and versions.

**Provenance:** L3-J-023 §15.

## 8. Observable Completion Conditions

Completion is observable when the expiring position has an attributable renewal disposition; any successor period has distinct evidence and validity; prior history remains unchanged; downstream contexts can distinguish current from expired scope; and any unrenewed scope transitions to restriction behavior at expiry.

## 9. Traceability & Provenance

**Parent:** L3-J-023 — Expiring Supplier Evidence or Qualification to Renewed Eligibility  
**Primary verification source:** L2-J02 — Vendor & Supplier Management Workflows  

Material anchors: L3-J-023 §§2–16.

## 10. Open Issues / Deferred Design

Renewal lead times, grace/interim rules, validity derivation across multiple evidence expiries and treatment of regulator/issuer delay remain upstream business-policy issues recorded by L3-J-023. L4 does not invent them.

Implementation design remains deferred to L5/L6.
