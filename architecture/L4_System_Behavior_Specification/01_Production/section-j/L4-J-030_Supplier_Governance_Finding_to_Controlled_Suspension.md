# Supplier Governance Finding to Controlled Suspension

## 1. Specification Metadata

**Specification ID:** L4-J-030  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Enterprise Resource Management — Vendor & Supplier Management  
**Realizes Scenario:** L3-J-030  
**Primary Business Owner:** Vendor & Supplier Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-030_Supplier_Governance_Finding_to_Controlled_Suspension.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP shall enforce a controlled supplier suspension only when an authorised governance finding establishes that narrower restriction is insufficient or an express urgent basis authorises interim suspension.

Suspension governs future supplier use prospectively. It does not delete history, impose blacklisting, automatically terminate commitments, or reverse financial records.

## 3. Required Business Context

- canonical supplier and attributable related scope;
- governed finding or urgent authorised basis;
- current supplier state and restriction history;
- source-defined suspension authority, urgency, duration/review, notice/response and reinstatement rules;
- current approval, qualification, sourcing, award, commitment, contract and relevant financial positions;
- specialist mandatory findings where applicable.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-030-BH-001 | The system shall require an authorised governance finding or express urgent basis, canonical supplier linkage, grounds and competent authority before suspension becomes effective. | L3-J-030 §§7–9, 12 |
| L4-J-030-BH-002 | For urgent interim suspension, the system shall preserve the authorised exact scope, effective time, maximum duration, permitted emergency activity and mandatory review deadline. | L3-J-030 §§9 steps 3–4, 10.1, 12 |
| L4-J-030-BH-003 | The system shall prevent an interim suspension from continuing indefinitely solely because final review was not completed. | L3-J-030 §§10.1, 11.5, 12 |
| L4-J-030-BH-004 | The system shall distinguish related entities/scopes and shall not automatically suspend a branch, consortium member, subcontractor, predecessor/successor or affiliate without supported attribution. | L3-J-030 §§9 step 6, 10.3, 12 |
| L4-J-030-BH-005 | The system shall preserve the authority's determination that narrower restriction is insufficient before general suspension is applied. | L3-J-030 §§2, 8, 9 step 8, 12 |
| L4-J-030-BH-006 | When suspension is approved, the system shall represent the canonical supplier `Suspended` position and the exact approved future-use controls, effective time, duration/review and permitted exceptions. | L3-J-030 §§3, 9 steps 12–15, 13 |
| L4-J-030-BH-007 | The system shall expose existing sourcing, awards, commitments, deliveries, warranties, disputes, payables and payments to their authoritative owners for review without cancelling or reversing them automatically. | L3-J-030 §§9 steps 5, 11, 16, 12 |
| L4-J-030-BH-008 | The system shall prevent remediation submission alone from establishing reinstatement; the suspension remains controlling until an authoritative reinstatement/lifting decision becomes effective. | L3-J-030 §§10.4, 11.8, 12 |
| L4-J-030-BH-009 | When the authority lifts or reinstates the supplier according to upstream governance, the system shall preserve the prior suspension as historical evidence and establish the new controlling prospective position. | L3-J-030 §§4.2, 9 steps 14–17, 12 |
| L4-J-030-BH-010 | When evidence requires terminal disposition consideration, the system shall expose the separate escalation without treating suspension as blacklisting. | L3-J-030 §§4.3–4.4, 10.7, 12 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-030-BH-011 | On approved suspension, the system shall publish or make available the canonical `SupplierSuspended` fact on behalf of Vendor & Supplier Management. | L3-J-030 §14.1 |
| L4-J-030-BH-012 | When the canonical reactivation/reinstatement decision becomes effective, the system shall publish or make available the canonical `SupplierReactivated` fact without changing its publisher. | L3-J-030 §14.1 |
| L4-J-030-BH-013 | The system shall make current suspension/reinstatement facts available to qualification, sourcing, J04, Finance and specialist owners without assuming their downstream decisions. | L3-J-030 §14.2 |

## 6. Failure / Rejection / Continuation

- Ambiguous canonical identity/scope shall prevent unsupported broad suspension.
- Insufficient evidence for final suspension shall not convert allegation into a final fact; any interim control must remain within upstream authority.
- Publication failure shall not erase the authoritative business-effective suspension.
- Unauthorised use/override shall not restore general eligibility.
- Incorrect suspension shall be corrected through an attributable lifting/correction decision with impact history preserved.
- System interruption shall not backdate, delete or silently alter business-effective suspension.

**Provenance:** L3-J-030 §11.

## 7. Audit & Evidence Behavior

The system shall preserve:

- underlying governance finding and prior restrictions/breaches;
- urgent/interim authority and final authority where applicable;
- supplier/related-scope attribution;
- reason narrower restriction was insufficient;
- notice, response, due-process limitations and conflicts;
- scope, effective time, duration, review schedule, exceptions and permitted activity;
- affected existing positions and referrals;
- continuation, variation, lifting, reinstatement/escalation history;
- actors, reasons, timestamps and versions.

**Provenance:** L3-J-030 §15.

## 8. Observable Completion Conditions

The behavior is complete when the suspension proposal has a governed disposition; any approved suspension is authoritative and observable with exact scope/effective conditions; `SupplierSuspended` is available when applicable; downstream owners can identify affected positions; and any later `SupplierReactivated` status is prospective and preserves suspension history.

## 9. Traceability & Provenance

**Parent:** L3-J-030 — Supplier Governance Finding to Controlled Suspension  
**Primary verification source:** L2-J02 — Vendor & Supplier Management Workflows  

Material anchors: L3-J-030 §§2–16.

## 10. Open Issues / Deferred Design

Competent suspension authorities, urgent duration, notice/review timing, exact continuation exceptions and reinstatement operating rules remain upstream governance matters.

Implementation design remains deferred to L5/L6.
