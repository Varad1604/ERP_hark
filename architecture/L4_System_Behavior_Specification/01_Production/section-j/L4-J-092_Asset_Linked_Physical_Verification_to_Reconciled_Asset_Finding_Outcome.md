# Asset-Linked Physical Verification to Reconciled Asset Finding Outcome

## 1. Specification Metadata

**Specification ID:** L4-J-092  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Enterprise Resource Management — Physical Verification, Audit & Reconciliation  
**Realizes Scenario:** L3-J-092  
**Primary Business Owner:** Physical Verification, Audit & Reconciliation  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-092_Asset_Linked_Physical_Verification_to_Reconciled_Asset_Finding_Outcome.md`; `L2-J08_Asset_Lifecycle_Management.md`; `L2-J09_Physical_Verification_Audit_and_Reconciliation.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall reconcile physical verification evidence for inventory linked to an operational asset and produce an authorised J09 asset finding outcome.

J09 owns the inventory-side verification, reconciliation, finding and referral. J08 retains authority for asset identity, custody, location, condition and lifecycle corrections. J06 retains inventory-state authority. The behavior shall not directly alter asset, stock or Finance records.

## 3. Required Business Context

- verified physical item and candidate canonical inventory/asset references;
- asset tag, serial, make/model, item reference, ownership, location, custodian and condition;
- acquisition, receipt, recognition, assignment, transfer, maintenance, retirement, disposal and prior-verification evidence;
- authoritative J08 and J06 current records;
- custody and movement evidence;
- component/bundle/tag/serial rules;
- Audit Approver finding and downstream referral owners.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-092-BH-001 | When an asset-linked item is verified, the system shall preserve attributable physical evidence for item, location, custodian, tag, serial, make/model, condition, use and verification time. | L3-J-092 §§4.1, 9 step 1 |
| L4-J-092-BH-002 | The system shall make candidate canonical inventory and asset references, acquisition/recognition/assignment/transfer/maintenance/retirement/disposal history available for comparison without rewriting either owner record. | L3-J-092 §§9 step 2, 14.2 |
| L4-J-092-BH-003 | Asset and inventory identity shall be determined from governed evidence; tag or serial similarity alone shall not create or merge asset identity. | L3-J-092 §§9 steps 3, 6, 12 |
| L4-J-092-BH-004 | The system shall preserve independent verification of physical existence, accessibility, identity, quantity, location, custodian, condition, use and lifecycle position. | L3-J-092 §9 step 4 |
| L4-J-092-BH-005 | In-transit, repair, loan, third-party custody, cannibalised, component, spare, returned, retired, disposed, lost and damaged positions shall be tested against authoritative evidence before discrepancy classification. | L3-J-092 §§9 step 5, 10 |
| L4-J-092-BH-006 | Custody handovers, transfers, access, acknowledgements, maintenance, incidents, movements and disposal evidence shall remain reconstructable where a discrepancy is investigated. | L3-J-092 §9 step 7 |
| L4-J-092-BH-007 | Physical, inventory and asset positions shall be compared at the verification cut-off while subsequent movements remain separately visible. | L3-J-092 §9 step 8 |
| L4-J-092-BH-008 | The system shall preserve attributable custodian/owner explanations where applicable, while non-response shall not be treated automatically as admission. | L3-J-092 §9 step 9 |
| L4-J-092-BH-009 | The system shall represent the source-supported finding classification, including matched, location/custody/condition mismatch, tag/serial error, unrecorded/unlinked, missing, duplicate, retired/disposed mismatch, component issue or unresolved as applicable. | L3-J-092 §§9 step 10, 12 |
| L4-J-092-BH-010 | Root cause, materiality and safety/security/financial/grant/maintenance/loss/damage implications shall remain separate findings/referrals rather than direct asset or finance outcomes. | L3-J-092 §§9 steps 11–12, 12 |
| L4-J-092-BH-011 | The system shall preserve the Audit Approver determination and shall require each material discrepancy to have an approved explanation, correction owner or governed unresolved outcome before J09 reconciliation is complete. | L3-J-092 §§9 steps 13–16 |
| L4-J-092-BH-012 | The system shall prevent J09 from directly creating, assigning, transferring, repairing, retiring, disposing or correcting a J08 asset record, or posting inventory/accounting adjustments. | L3-J-092 §§4.4, 12, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-092-BH-013 | The system shall preserve J09 publication of the canonical `PhysicalVerificationCompleted`, `InventoryVarianceDetected`, `VarianceReconciliationDetermined` and `InventoryVarianceReconciled` facts at their source-supported points. | L3-J-092 §14.1 |
| L4-J-092-BH-014 | J08, J06, Finance, Grant, Maintenance, Security, J10 and other authoritative owners shall receive governed referrals/facts without allowing J09 to execute their corrections or lifecycle decisions. | L3-J-092 §§5, 14.2 |

## 6. Failure / Rejection / Continuation

- Unreadable tag/serial shall require alternate authoritative evidence; identity remains unresolved if evidence is insufficient.
- Wrong asset link shall be corrected through an attributable successor correction preserving both records and affected actions.
- Disagreement between record owners shall remain governed unresolved until a competent authority decides.
- Unacknowledged asset correction/referral shall be reissued/monitored; J09 finding shall not be mistaken for completed J08 correction.
- Compromised evidence shall be qualified and reverified where possible.
- Business interruption shall preserve physical evidence and authoritative source extracts before continuation.

**Provenance:** L3-J-092 §11.

## 7. Audit & Evidence Behavior

The system shall preserve:

- physical item evidence and verification time;
- tag/serial/make/model/item/asset identifiers;
- ownership/location/custody/condition/use;
- acquisition/receipt/recognition/assignment/transfer/maintenance/retirement/disposal history;
- custody handovers and movements;
- responses and unresolved issues;
- discrepancy classification and root-cause implications;
- Audit Approver determination;
- downstream referrals and acknowledgements;
- actors, timestamps and versions.

**Provenance:** L3-J-092 §15.

## 8. Observable Completion Conditions

Completion is observable when the asset-linked verification has an approved J09 finding; each material discrepancy has an explicit owner/referral or governed unresolved treatment; canonical J09 verification/reconciliation facts are available; J09 may reconcile once referral requirements are met; and no J08 asset-state, J06 stock or Finance record has been changed by this behavior.

## 9. Traceability & Provenance

**Parent:** L3-J-092 — Asset-Linked Physical Verification to Reconciled Asset Finding Outcome  
**Primary verification sources:** L2-J09 — Physical Verification, Audit & Reconciliation; L2-J08 — Asset Lifecycle Management

Material anchors: L3-J-092 §§2–17.

## 10. Open Issues / Deferred Design

Asset-link eligibility, tag/serial authority, component/bundle rules, materiality, treatment of leased/loaned/grant-funded/under-repair/cannibalised/retired/disposed assets and correction sequencing remain upstream J08/J09 governance issues.

Technical tag scanning, matching, depreciation, register and integration mechanics remain deferred.
