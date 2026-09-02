# Material Inventory Variance to Investigated and Reconciled Closure

## 1. Specification Metadata

**Specification ID:** L4-J-089  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Enterprise Resource Management — Physical Verification, Audit & Reconciliation  
**Realizes Scenario:** L3-J-089  
**Primary Business Owner:** Physical Verification, Audit & Reconciliation  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-089_Material_Inventory_Variance_to_Investigated_and_Reconciled_Closure.md`; `L2-J06_Inventory_and_Warehouse_Management.md`; `L2-J09_Physical_Verification_Audit_and_Reconciliation.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern investigation of a material inventory variance from verified difference through evidence-based cause analysis, reconciliation and J09 closure.

The behavior shall separate physical/book discrepancy, cause, loss/damage, misconduct, financial consequence and inventory adjustment. Investigation shall not itself post stock/financial adjustments or decide downstream consequences.

## 3. Required Business Context

- authorised count/control and material variance;
- physical and authoritative book/cut-off evidence;
- canonical item/unit/location/lot/serial/condition/custody/ownership context;
- movement history and current stock references from J06;
- investigation independence/authority and evidence custody;
- affected custodian/user/specialist responses;
- downstream referral ownership;
- Audit Approver determination.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-089-BH-001 | When a variance meets an approved materiality/risk criterion, the system shall require attributable physical count, book position, cut-off, dimensions, variance and source evidence before governed investigation. | L3-J-089 §§7–9 steps 1–2 |
| L4-J-089-BH-002 | The system shall preserve immediate evidence/protective controls where required without treating those controls as proof of cause or liability. | L3-J-089 §§9 step 3, 12 |
| L4-J-089-BH-003 | The system shall make relevant receipts, issues, transfers, returns, reservations, distributions, consumption, adjustments, disposals and other source-supported movement evidence available for reconciliation without rewriting J06 records. | L3-J-089 §§9 step 4, 14.2 |
| L4-J-089-BH-004 | The system shall preserve investigation testing across unit, packaging, lot, serial, location, ownership, consignment, transit, quarantine, timing, duplicate-posting, omission and other approved causes as applicable. | L3-J-089 §§9 step 5 |
| L4-J-089-BH-005 | Offsetting excess/shortage shall remain separately traceable unless upstream identity/ownership/lot/valuation/authority rules expressly permit netting. | L3-J-089 §§9 step 7, 10.2, 12 |
| L4-J-089-BH-006 | Where response rights apply, the system shall preserve attributable responses/evidence from custodians, users and record owners without allowing response or non-response to control the finding. | L3-J-089 §§9 step 8, 12 |
| L4-J-089-BH-007 | Each variance component shall retain an approved cause/findings classification, including explicit unresolved status where evidence is inconclusive. | L3-J-089 §§9 steps 10–11, 10.5, 12 |
| L4-J-089-BH-008 | Root cause, control failure, responsibility, financial exposure, safety/grant/asset/fraud indicators and recovery potential shall remain distinct attributable findings/referrals rather than being inferred from shortage/excess alone. | L3-J-089 §§9 step 11, 12 |
| L4-J-089-BH-009 | The system shall create the applicable governed referrals to L3-J-090, J-091, J-092 or other competent authorities without executing their outcomes. | L3-J-089 §§4.4, 9 step 13, 14.2 |
| L4-J-089-BH-010 | J09 reconciliation closure shall occur only when every material variance component has an approved explanation/correction/referral owner and residual exposure remains governed. | L3-J-089 §§9 steps 14–16, 13 |
| L4-J-089-BH-011 | The system shall prevent inventory adjustment from being treated as proof or erasure of variance cause. | L3-J-089 §§11.2, 12 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-089-BH-012 | The system shall preserve publication of canonical J09 facts `InventoryVarianceEscalated`, `VarianceInvestigationCreated`, `VarianceInvestigationCompleted`, `ReconciliationApproved` and `InventoryVarianceReconciled` at their source-supported points. | L3-J-089 §14.1 |
| L4-J-089-BH-013 | The system shall make approved findings available to J06, J08, J10, Finance, Security, Legal, HR/disciplinary and other owners without taking ownership of their consequence decisions. | L3-J-089 §§5, 14.2 |

## 6. Failure / Rejection / Continuation

- Missing/compromised evidence shall preserve the limitation and narrow or leave findings unresolved.
- Premature adjustment shall remain visible and shall not terminate the investigation or prove cause.
- A conflicted investigator shall require source-supported replacement/independent review.
- Wrong item/unit/lot/location scope shall be corrected by attributable investigation-scope revision.
- Missed investigation deadline shall remain escalated with residual controls and shall not create silent closure.
- On interruption, evidence custody/protective controls remain governed until continuation.

**Provenance:** L3-J-089 §11.

## 7. Audit & Evidence Behavior

The system shall preserve:

- original audit/count/recount/book/cut-off evidence;
- item/unit/location/lot/serial/custody/ownership;
- movement reconstruction;
- access/communications/statements/responses;
- cause testing and root-cause/control findings;
- conflicts;
- corrective recommendations/referrals;
- Audit Approver decision and closure;
- canonical J09 event lineage;
- actors, timestamps and versions.

**Provenance:** L3-J-089 §15.

## 8. Observable Completion Conditions

Completion is observable when each material variance component has an approved governed finding or unresolved treatment; required referrals exist; J09 reaches its authorised reconciliation/closure position; canonical investigation/reconciliation facts are available; and no stock, financial, asset, disciplinary or legal consequence has been executed by the investigation itself.

## 9. Traceability & Provenance

**Parent:** L3-J-089 — Material Inventory Variance to Investigated and Reconciled Closure  
**Primary verification sources:** L2-J09 — Physical Verification, Audit & Reconciliation; L2-J06 — Inventory & Warehouse Management

Material anchors: L3-J-089 §§2–17.

## 10. Open Issues / Deferred Design

Materiality, authority, evidence/deadline criteria, controlled-resource variants and responsibility/recovery/insurance/disciplinary referral rules remain upstream policy matters.

Implementation analytics/surveillance/case-management mechanics remain deferred.
