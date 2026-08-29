# Asset-Linked Physical Verification to Reconciled Asset Finding Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-J-092  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Physical Verification, Audit & Reconciliation  
**Participating Domains:** J  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J08_Asset_Lifecycle_Management.md`; `L2-J09_Physical_Verification_Audit_and_Reconciliation.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** asset lifecycle, custody, location, condition, transfer, missing-asset, maintenance, retirement, inventory adjustment, finance reconciliation, audit, and supplemental determination scenarios  

---
# 2. Scenario Purpose

This scenario governs reconciliation of physical-verification evidence for inventory linked to an operational asset. It establishes whether the physical item, inventory record, asset identity, tag, serial, location, custodian, condition, and lifecycle state agree, and produces an authorised Asset Finding Outcome without directly rewriting the asset register or stock ledger.

---

# 3. Business Outcome

Asset-linked verification reaches a Reconciled Asset Finding Outcome with canonical inventory and asset linkage, verified physical existence, identity, location, custody, condition, discrepancy classification, evidence, responses, referrals, approval, and immutable lineage.

Outcome labels in this section are business dispositions or supporting-record results, not additional canonical lifecycle states. Section 13 states the only canonical lifecycle impact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an inventory verification includes stock recognised, capitalised, tagged, serialised, assigned, controlled, or otherwise linked to an operational asset record.

## 4.2 End Boundary

The scenario ends when the inventory-asset relationship is confirmed or a discrepancy finding is approved; relevant asset, inventory, custody, finance, loss, damage, maintenance, transfer, retirement, or correction owners receive governed referrals; and audit evidence is reconciled.

## 4.3 In Scope

- verifying physical item, asset tag, serial, make/model, inventory reference, asset identity, ownership, location, custodian, condition, status, and supporting documents;
- comparing inventory, asset register, custody, transfer, maintenance, repair, verification, retirement, and disposal records;
- classifying missing, untagged, duplicate-tagged, wrong-serial, wrong-location, wrong-custodian, unrecorded, component, bundled, damaged, idle, retired, disposed, or status-mismatch findings;
- obtaining custodial and specialist responses;
- approving asset findings and routing authoritative corrections or lifecycle actions;
- preserving both inventory and asset lineage.

## 4.4 Out of Scope

- creating, assigning, transferring, returning, repairing, retiring, disposing, or correcting an asset record directly;
- posting inventory or accounting adjustments;
- determining theft, loss, liability, recovery, insurance, or disciplinary outcome;
- treating every serialised item as an asset without policy authority;
- technical tag scanning, asset matching, depreciation, or register implementation.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-087 / L3-J-089 | Verification and variance evidence create the asset-linked finding. |
| Related | L3-J-090 | Inventory correction remains separately authorised. |
| Related | L3-J-091 | Confirmed loss or damage may require J10 referral. |
| Related | Asset lifecycle scenarios J074-J086 | Retain authoritative asset actions and correction. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Physical Verification, Audit & Reconciliation | Owns inventory-side verification, linkage reconciliation, finding coordination, referral, and evidence. |

Asset, custody, maintenance, finance, security, legal, insurance, disposal, and operational owners retain their authoritative lifecycle decisions.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Variance Investigator | Investigates and records attributable findings where required. | Investigates. |
| Reconciliation Officer | Determines the J09 reconciliation position, issues governed referrals and records returned outcomes. | Reconciles; does not execute J06, J08 or J10 state changes. |
| Audit Approver | Approves the J09 reconciliation or supplemental determination within established authority. | Decides the J09 outcome. |
| Audit Reviewer | Reviews evidence, separation and lineage without modifying the outcome. | Reviews evidence. |
| Inventory Custodian | Supplies custody and operational evidence without controlling the finding. | Supports and responds. |
---

# 7. Preconditions and Entry Criteria

- A verified physical item and candidate inventory or asset reference exist.
- Canonical item, asset, tag, serial, location, custody, condition, ownership, and movement records are available.
- Inventory-asset linkage, component, bundle, tag, serial, custody, verification, correction, loss, damage, retirement, disposal, finance, and retention rules are effective.
- Verification roles are independent where required.
- Asset and inventory records cannot be silently merged or overwritten.

---

# 8. Trigger

Physical verification encounters an item expected to correspond to an asset, or identifies a discrepancy between physical, inventory, custody, and asset records.

---

# 9. Main Business Flow

1. The verification team records physical item, location, custodian, tag, serial, make/model, condition, use, attachments, photographs or other evidence, and verification time.
2. Inventory and asset owners identify candidate canonical item and asset references, source acquisition, receipt, recognition, assignment, transfer, maintenance, retirement, disposal, and prior verification history.
3. Tag, serial, manufacturer reference, specifications, ownership, funding source, component or bundle relationship, and historical identifiers are compared.
4. Physical existence, accessibility, identity, quantity, location, custodian, condition, operational use, and lifecycle status are verified independently.
5. In-transit, repair, loan, third-party custody, cannibalised, component, spare, returned, retired, disposed, lost, or damaged positions are checked against authoritative evidence.
6. Duplicate tags, missing tags, replacement tags, mismatched serials, split or combined components, and prior corrections are investigated without creating identity by convenience.
7. Custody handovers, transfers, access, acknowledgements, maintenance, incidents, movements, and disposal evidence are reconstructed where discrepancies exist.
8. Inventory and asset book positions are compared as of the verification cut-off and subsequent movements remain separately visible.
9. The custodian and responsible owners provide explanations and evidence; non-response is not automatic admission.
10. The finding is classified as matched, location mismatch, custody mismatch, condition mismatch, tag or serial error, unrecorded asset, unlinked inventory, missing, duplicate, retired or disposed mismatch, component issue, unresolved, or other approved category.
11. Root cause, materiality, safety, security, financial, grant, maintenance, loss, damage, and control implications are assessed without deciding downstream outcomes.
12. Required inventory adjustment, asset correction, assignment, transfer, return, maintenance, missing-asset, retirement, disposal, finance, or J10 referral is defined.
13. The Audit Approver reviews evidence, identity, cut-off, responses, classification, and referral completeness.
14. The Audit Approver rejects, returns, approves, or partially approves the finding and records unresolved components.
15. Asset and inventory owners acknowledge referrals and retain separate authority for their corrections and lifecycle outcomes.
16. Reconciliation confirms that each physical, inventory, and asset discrepancy has an approved explanation, correction owner, or governed unresolved outcome.
17. Evidence, findings, responses, referrals, acknowledgements, corrections, actors, timestamps, and versions remain auditable.

---

# 10. Alternate Business Flows

## 10.1 Fully Matched Asset

The finding confirms existence, identity, location, custody, condition, and status with no correction required.

## 10.2 Asset Temporarily Elsewhere

Authorised transfer, repair, loan, calibration, or third-party custody evidence resolves location difference while remaining traceable.

## 10.3 Untagged or Unrecorded Physical Item

Ownership and recognition authority are verified before any new asset or inventory record is created.

## 10.4 Missing Asset

The finding is referred to the missing-asset and J10 processes; audit does not determine recovery or disposition.

## 10.5 Retired or Disposed Asset Physically Present

Lifecycle, custody, disposal, safety, finance, and record-integrity owners review the conflict. Prior disposition is not silently reversed.

## 10.6 Component or Bundled Asset

Parent-child identity and material component requirements are applied without duplicating asset value or stock quantity.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Tag or Serial Unreadable

Alternative authoritative evidence is gathered; identity remains unresolved if insufficient.

## 11.2 Wrong Asset Link Applied

An attributable correction removes or supersedes the link while preserving both records and affected actions.

## 11.3 Record Owners Disagree

The discrepancy remains governed unresolved and is escalated to the competent identity or asset authority.

## 11.4 Asset Correction Not Acknowledged

The referral is reissued and monitored; the audit finding is not mistaken for completed correction.

## 11.5 Evidence Compromised

Reliability is qualified, reverification occurs where possible, and integrity concerns are escalated.

## 11.6 System Interruption

Physical evidence and source extracts are preserved and reconciled before resumption.

---

# 12. Business Rules and Decision Constraints

- Asset-linked finding requires physical evidence and canonical inventory, asset, tag, serial, location, custody, and condition reconciliation.
- Physical item, inventory record, asset record, custody record, financial value, and lifecycle state are distinct.
- Tag or serial similarity alone cannot create or merge asset identity.
- Inventory audit cannot directly change asset assignment, location, condition, retirement, disposal, or financial record.
- Current custody does not establish historical responsibility automatically.
- Missing, damaged, retired, disposed, repaired, or third-party positions require their own authoritative outcomes.
- Component and bundle treatment must prevent duplicate identity, quantity, and value.
- Corrections preserve original records, links, and decisions.
- Unresolved findings remain explicit with owner and review path.
- Audit and asset lifecycle authorities remain separated.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Inventory Audit | `Reconciliation` | Issues the governed owned-domain request or referral and waits without taking ownership of the external state change. | `Awaiting External Action` until the authoritative outcome is returned; then `Reconciled` where completion conditions are met | Physical Verification, Audit & Reconciliation (J09) | `L2-J09` Section 12 |
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `PhysicalVerificationCompleted` | Physical Verification, Audit & Reconciliation (J09) | J06, J08, J10, Finance and Audit where material | Records the exact completed business fact material to this scenario. | `L2-J09 Section 31` |
| `InventoryVarianceDetected` | Physical Verification, Audit & Reconciliation (J09) | J06, J08, J10, Finance and Audit where material | Records the exact completed business fact material to this scenario. | `L2-J09 Section 31` |
| `VarianceReconciliationDetermined` | Physical Verification, Audit & Reconciliation (J09) | J06, J08, J10, Finance and Audit where material | Records the exact completed business fact material to this scenario. | `L2-J09 Section 31` |
| `InventoryVarianceReconciled` | Physical Verification, Audit & Reconciliation (J09) | J06, J08, J10, Finance and Audit where material | Records the exact completed business fact material to this scenario. | `L2-J09 Section 31` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| Inventory / Warehouse / Custody | Supplies stock, movement, and custody facts. | Retains inventory records. |
| Asset lifecycle J074-J086 | Supplies and updates asset identity and state. | Retains asset authority. |
| Finance / Grant / Maintenance / Security | Reviews specialist implications. | Retains specialist outcomes. |
| L3-J-090 / L3-J-091 | Receives adjustment or loss referrals. | Finding does not decide consequences. |
| L3-J-093 | Governs later closed-audit evidence. | Requires successor determination. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

Inventory Counter, Inventory Custodian, Variance Investigator, Reconciliation Officer, Audit Approver, and Audit Reviewer are separated where required. J08 asset owners and J06 inventory owners retain separate authority for their own corrections; no custodian can resolve a material discrepancy alone.

## 15.2 Required Evidence

- physical item, tag, serial, make/model, condition, location, custodian, use, ownership, item and asset identifiers, acquisition, receipt, recognition, assignment, transfer, maintenance, retirement, disposal, and prior verification;
- photographs or equivalent evidence, count, cut-off, movements, custody handovers, responses, classification, root cause, specialist findings, referrals, acknowledgements, corrections, actors, timestamps, and versions.

## 15.3 Audit Obligations

The institution must reconstruct what was physically observed, how inventory and asset identity were compared, what custody and lifecycle evidence existed, why the finding was classified, who approved it, and which asset or other authority received and completed the referral.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- the physical verification has been conducted against the authoritative asset register for the defined scope including asset identity, condition, location, custody, and status;
- findings distinguishing present, missing, damaged, unrecorded, obsolete, or misclassified assets have been documented with physical evidence;
- each material finding has been referred to the authoritative asset management, finance, legal, security, or compliance process with an explicit referral record;
- the asset management domain has been notified of findings requiring asset record review, update, disposal, or investigation;
- the verification and finding evidence, authority, actors, referrals, and audit trail are preserved and immutable;
- the J-domain audit is closed with respect to asset-linked findings after confirmed referrals.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the asset-linked physical verification has a reconciled finding outcome with each material asset discrepancy referred to its authoritative governing process;
- no asset record change, write-off, or disposal decision has been made by the J-domain audit itself;
- the asset management domain retains authority over asset lifecycle, valuation, and record changes following the referral;
- the complete verification evidence, findings, referrals, and outcomes are permanently auditable;
- the J-domain audit closure is dependent only on confirmed referrals and not on the final resolution of asset lifecycle decisions by the asset domain.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Asset-link eligibility, tag and serial authority, component and bundle rules, materiality, and finding authority require confirmation.
2. Treatment of leased, loaned, grant-funded, third-party, under-repair, cannibalised, retired, and disposed assets requires upstream J08/J09 business rules and hand-off contracts.
3. Inventory, asset and finance correction sequencing requires cross-domain confirmation.

## 17.2 Assumptions

- Audit findings do not directly change asset lifecycle or finance records.
- Asset and inventory histories remain separately authoritative and linked.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-092**
