# Supplier Remediation to Restored Eligibility

## 1. Specification Metadata

**Specification ID:** L4-J-032  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Enterprise Resource Management — Vendor & Supplier Management  
**Realizes Scenario:** L3-J-032  
**Primary Business Owner:** Vendor & Supplier Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-032_Supplier_Remediation_to_Restored_Eligibility.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern restoration of supplier eligibility after a reversible restriction, suspension, qualification deficiency, expiry condition, or legally restorable blacklisting position.

Restoration shall be prospective, scope-specific and independently verified. It shall not erase the original finding, adverse status, expiry gap, breach or transaction history.

## 3. Required Business Context

- canonical supplier and original governance condition;
- source-defined restoration route and exit conditions;
- current supplier profile, ownership, qualification and governance states;
- remediation evidence and independent verification findings;
- mandatory legal, sanctions, regulatory, safety, security, qualification or external-clearance facts;
- competent Supplier Governance Authority decision;
- residual restriction/monitoring requirements where applicable.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-032-BH-001 | When restoration is requested, the system shall require the canonical supplier, original adverse condition, affected scope, permitted restoration route and claimed remediation completion to be identifiable. | L3-J-032 §§4.1, 7–9 |
| L4-J-032-BH-002 | The system shall require independent verification of remediation evidence against each controlling exit condition before restored eligibility may become effective. | L3-J-032 §§9 steps 4–8, 12 |
| L4-J-032-BH-003 | The system shall distinguish remediation planned, implemented, operating, independently tested and proven sustainable rather than treating submission or partial completion as restoration. | L3-J-032 §§9 steps 5–6, 11.2, 12 |
| L4-J-032-BH-004 | The system shall prevent restoration where any incompatible mandatory prohibition, sanction, external debarment, unresolved legal/regulatory condition or non-restorable decision remains effective. | L3-J-032 §§4.4, 9 step 7, 10.4–10.5, 12 |
| L4-J-032-BH-005 | The system shall represent full, partial, conditional, staged, rejected or deferred restoration only to the exact scope supported by verified remediation and current qualification. | L3-J-032 §§3, 9 steps 10–13, 10.1–10.2, 12 |
| L4-J-032-BH-006 | On approved restoration, the system shall establish a prospective authoritative eligibility position linked to all prior findings and decisions without replacing or deleting those records. | L3-J-032 §§9 step 14, 12, 16 |
| L4-J-032-BH-007 | The system shall prevent elapsed time or expiry of a prior adverse period from being treated as automatic proof of restored eligibility unless the governing decision expressly provides that outcome. | L3-J-032 §§2, 8, 10.3, 12 |
| L4-J-032-BH-008 | The system shall preserve continuing restrictions, probation, monitoring, review dates and breach conditions where the authority approves conditional or partial restoration. | L3-J-032 §§9 step 11, 10.1–10.2, 12 |
| L4-J-032-BH-009 | Restored eligibility shall become usable only from the authoritative effective time and shall not reopen earlier sourcing, award, contract, receipt, payable, accounting or payment outcomes. | L3-J-032 §§9 steps 14–16, 12 |
| L4-J-032-BH-010 | Where eligibility is restored but the supplier remains `Inactive`, the system shall preserve inactivity as a separate controlling relationship state until its own reactivation governance completes. | L3-J-032 §§10.7, 12, 14.2 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-032-BH-011 | When restoration results in source-supported reactivation, the system shall publish or make available the canonical `SupplierReactivated` fact on behalf of Vendor & Supplier Management. | L3-J-032 §14.1 |
| L4-J-032-BH-012 | The system shall make current restored scope and residual controls available to authorised qualification, sourcing, J04, Finance and specialist owners without reopening or transferring their owned records. | L3-J-032 §14.2 |

## 6. Failure / Rejection / Continuation

- Fraudulent or unverifiable remediation evidence shall prevent restoration.
- Incomplete verification shall result in return/deferral rather than implied restoration.
- Conflicted verifier or approver participation shall require source-supported replacement or independent review.
- Incorrect early restoration shall be corrected prospectively with impact history preserved.
- Publication failure shall not make eligibility usable before the authoritative restored position is reliably available where required.
- Breach of restoration conditions shall be referred to the appropriate restriction/suspension governance without locally inventing a new status.

**Provenance:** L3-J-032 §11.

## 7. Audit & Evidence Behavior

The system shall preserve:

- original findings, restrictions/suspension/blacklisting or deficiency;
- remediation requirements and supplier evidence;
- independent verification and specialist clearances;
- conflicts and residual risk;
- restoration recommendation and authority decision;
- restored/continuing scope, effective time, conditions and monitoring;
- later breaches/reviews;
- actors, reasons, timestamps and versions.

**Provenance:** L3-J-032 §15.

## 8. Observable Completion Conditions

Completion is observable when remediation has an attributable governed disposition; any restored eligibility has explicit scope/effective time/conditions; prior adverse history remains intact; mandatory prohibitions are not bypassed; and `SupplierReactivated` is available only where source-supported reactivation occurs.

## 9. Traceability & Provenance

**Parent:** L3-J-032 — Supplier Remediation to Restored Eligibility  
**Primary verification source:** L2-J02 — Vendor & Supplier Management Workflows

Material anchors: L3-J-032 §§2–16.

## 10. Open Issues / Deferred Design

Remediation evidence standards, independent-verification requirements, restoration authorities, probation/monitoring thresholds, restoration after different blacklisting forms and qualification revalidation rules remain upstream governance items.

Implementation design remains deferred to L5/L6.
