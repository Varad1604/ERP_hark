# Active Supplier Relationship to Governed Inactivity

## 1. Specification Metadata

**Specification ID:** L4-J-033  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Enterprise Resource Management — Vendor & Supplier Management  
**Realizes Scenario:** L3-J-033  
**Primary Business Owner:** Vendor & Supplier Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-033_Active_Supplier_Relationship_to_Governed_Inactivity.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern a prospective, non-punitive transition from supplier `Active` to canonical `Inactive` when approved inactivity conditions are satisfied.

The behavior shall preserve identity, qualifications, history and open obligations. Inactivity shall not substitute for restriction, suspension, blacklisting, closure or any adverse-governance process.

## 3. Required Business Context

- canonical supplier relationship currently `Active`;
- approved inactivity reason/activity definitions and review rules;
- current supplier profile, qualification and adverse-governance states;
- open sourcing, award, commitment, contract, delivery, warranty, claim, payable/payment and retention context;
- Supplier Governance Authority decision;
- source-defined reactivation and closure prerequisites.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-033-BH-001 | When inactivity is proposed, the system shall require a canonical `Active` supplier, approved non-punitive reason, evidence basis, affected scope and proposed effective time. | L3-J-033 §§4.1, 7–9 |
| L4-J-033-BH-002 | A time threshold or absence of recent activity shall create only a review candidate unless upstream governance expressly authorises automatic transition. | L3-J-033 §§10.2, 12 |
| L4-J-033-BH-003 | The system shall prevent fraud, misconduct, safety, security, legal, sanctions, performance, qualification or compliance concerns from being silently classified as ordinary inactivity. | L3-J-033 §§9 step 5, 10.5, 12 |
| L4-J-033-BH-004 | The system shall identify open sourcing, awards, proposed commitments, Purchase Orders, contracts, deliveries, warranties, claims, disputes, balances, payables, payments and statutory evidence before final inactivity. | L3-J-033 §§9 steps 6–7, 15 |
| L4-J-033-BH-005 | The system shall expose each open position to its authoritative owner and shall not cancel, settle, waive, terminate or reverse the position as a consequence of inactivity. | L3-J-033 §§9 step 7, 12 |
| L4-J-033-BH-006 | On approved transition, the system shall apply the canonical supplier state change from `Active` to `Inactive` prospectively while preserving all historical supplier records. | L3-J-033 §§9 steps 13–14, 13, 16 |
| L4-J-033-BH-007 | While `Inactive` is controlling, the system shall prevent ordinary new invitations, participation, award and commitment reliance except for source-supported existing-obligation activity or after governed reactivation. | L3-J-033 §§9 step 15, 12, 16 |
| L4-J-033-BH-008 | The system shall preserve inactivity separately from qualification, restriction, suspension, blacklisting and closure so those facts remain independently observable. | L3-J-033 §§2, 12 |
| L4-J-033-BH-009 | Reactivation shall require the source-defined refresh of current identity, profile, representatives, qualifications and adverse-governance controls before `Active` is restored. | L3-J-033 §§10.6, 12 |
| L4-J-033-BH-010 | The system shall refer closure-ready inactive relationships to L3-J-034 rather than converting inactivity directly into archive/disposal. | L3-J-033 §§10.7, 12 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-033-BH-011 | On approved inactivity, the system shall publish or make available the canonical `SupplierInactivated` fact on behalf of Vendor & Supplier Management. | L3-J-033 §14.1 |
| L4-J-033-BH-012 | On governed reactivation, the system shall publish or make available the canonical `SupplierReactivated` fact without changing its publisher. | L3-J-033 §14.1 |
| L4-J-033-BH-013 | The system shall make the current relationship state and reactivation conditions available to qualification, sourcing, J04, Finance and other authorised owners without modifying their owned records. | L3-J-033 §14.2 |

## 6. Failure / Rejection / Continuation

- Undetected open obligations remain authoritative and shall trigger reconciliation of inactivity impact.
- Incorrect inactivity shall be corrected prospectively with the original decision/error preserved.
- Punitive reasons disguised as inactivity shall be rejected and referred to proper governance.
- Unauthorised reactivation/use shall not establish valid Active status.
- Notification failure shall not silently alter the approved business-effective state.
- System interruption shall preserve the approved effective relationship state.

**Provenance:** L3-J-033 §11.

## 7. Audit & Evidence Behavior

The system shall preserve:

- inactivity request/reason and activity evidence;
- current supplier/adverse-governance states;
- open-position checks and owner referrals;
- operational-need findings;
- supplier communication where applicable;
- approval/effective time;
- blocked/permitted future use;
- reactivation requirements and later reactivation/closure referrals;
- actors, reasons, timestamps and versions.

**Provenance:** L3-J-033 §15.

## 8. Observable Completion Conditions

Completion is observable when the inactivity request has a governed disposition; approved status is `Inactive` from the effective time; ordinary future use is controlled; open obligations remain with their owners; `SupplierInactivated` is available; and reactivation/closure paths are recorded without erasing history.

## 9. Traceability & Provenance

**Parent:** L3-J-033 — Active Supplier Relationship to Governed Inactivity  
**Primary verification source:** L2-J02 — Vendor & Supplier Management Workflows

Material anchors: L3-J-033 §§2–16.

## 10. Open Issues / Deferred Design

Qualifying activity definitions, dormancy periods, review authorities, active-warranty/claim treatment, critical-supplier handling and reactivation checks remain upstream issues.

Implementation design remains deferred to L5/L6.
