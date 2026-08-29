# Material Inventory Variance to Investigated and Reconciled Closure

# 1. Scenario Metadata

**Scenario ID:** L3-J-089  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Physical Verification, Audit & Reconciliation  
**Participating Domains:** J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J06_Inventory_and_Warehouse_Management.md`; `L2-J09_Physical_Verification_Audit_and_Reconciliation.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** inventory adjustment, loss or damage referral, asset finding, finance reconciliation, disciplinary or legal referral, supplemental audit determination, and oversight scenarios  

---
# 2. Scenario Purpose

This scenario governs investigation of a material inventory variance from verified difference through cause analysis, evidence-based findings, reconciliation, and closure. It prevents unexplained differences from being cleared by unsupported adjustment and separates physical fact, record error, timing difference, loss, damage, misuse, fraud indicator, accountability, and financial consequence.

---

# 3. Business Outcome

A material inventory variance reaches Investigated and Reconciled Closure with verified population and cut-off, evidence, root cause, affected stock and records, responsible control owners, approved corrective and referral outcomes, residual exposure, authority, and immutable lineage.

Outcome labels in this section are business dispositions or supporting-record results, not additional canonical lifecycle states. Section 13 states the only canonical lifecycle impact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised audit or control confirms a variance exceeding approved materiality, recurrence, risk, or suspicion thresholds.

## 4.2 End Boundary

The scenario ends when the variance is substantiated, disproved, resolved, partially resolved, or remains governed unresolved; causes and impacts are approved; adjustment, loss, asset, finance, legal, security, disciplinary, or control referrals are created; and the investigation is formally closed.

## 4.3 In Scope

- validating count, recount, item, unit, location, lot, serial, condition, custody, ownership, book position, and cut-off;
- reconstructing receipts, issues, transfers, returns, adjustments, reservations, distributions, consumption, disposals, and interfaces;
- distinguishing timing, documentation, unit, location, lot, serial, custody, valuation-reference, physical shortage, excess, damage, deterioration, and unauthorised movement;
- gathering statements, records, access, surveillance, correspondence, and specialist evidence lawfully;
- determining substantiated, partially substantiated, unsubstantiated, or unresolved findings;
- identifying root cause, control failure, corrective action, recovery opportunity, and downstream referral.

## 4.4 Out of Scope

- posting stock or accounting adjustments, governed by L3-J-090 and finance authority;
- deciding final loss, damage, recovery, disciplinary, fraud, criminal, insurance, or legal outcome;
- altering asset records, governed by L3-J-092 where applicable;
- disposing of or writing off inventory;
- technical investigation, analytics, surveillance, or case-management implementation.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-087 / L3-J-088 | Audit evidence identifies the material variance. |
| Followed by | L3-J-090 | Approved audit determination may require stock adjustment. |
| Followed by | L3-J-091 / L3-J-092 | Loss, damage, or asset-linked findings require separate outcomes. |
| Related | L3-J-093 | New evidence may supplement a closed determination. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Physical Verification, Audit & Reconciliation | Owns variance case, evidence coordination, root-cause finding, reconciliation, referrals, closure, and history. |

Inventory, warehouse, custody, asset, finance, security, legal, HR or disciplinary, audit, grant, insurance, safety, and operational owners retain their authoritative decisions.

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

- The variance is linked to an authorised count or control and exceeds an approved threshold or risk criterion.
- Physical, book, cut-off, item, location, unit, lot, serial, custody, and movement evidence is identifiable.
- Investigation authority, independence, confidentiality, response, evidence custody, materiality, referral, closure, retention, and legal-hold rules are effective.
- Immediate preservation or protective controls can be applied where needed.
- Adjustment cannot occur solely to eliminate the variance before investigation requirements are satisfied.

---

# 8. Trigger

A verified difference is material by value, quantity, controlled-resource status, recurrence, risk, suspicion, grant condition, asset linkage, safety, or other approved criterion.

---

# 9. Main Business Flow

1. The referrer records audit reference, canonical item, location, lot, serial, custody, physical count, book quantity, variance, unit, value reference, cut-off, materiality, and evidence.
2. The investigation lead validates count independence, recount, population, cut-off, movement controls, book extract, and evidence integrity.
3. Immediate stock, access, document, credential, surveillance, safety, and legal-hold controls are applied proportionately where required.
4. Receipts, inspections, issues, transfers, returns, reservations, distributions, consumption, adjustments, disposals, interfaces, and manual records are reconstructed across the relevant period.
5. Unit conversion, packaging, lot, serial, location, ownership, consignment, in-transit, quarantine, timing, duplicate-posting, omission, and master-data causes are tested.
6. Custody and responsibility handovers, access logs, approvals, count evidence, communications, and physical condition are examined.
7. Related variances, repeated patterns, offsetting excess and shortage, cross-location movement, and possible concealment are analysed without unauthorised netting.
8. Affected custodians, users, record owners, and specialists receive a fair opportunity to provide facts and evidence.
9. Conflicts involving investigators, custodians, approvers, or interested parties are declared and mitigated.
10. Each variance component is classified as resolved timing, record or unit error, location or lot error, physical excess, shortage, damage, deterioration, unauthorised movement, unresolved, or other approved cause.
11. Root cause, control failure, recurrence, responsibility, financial exposure, safety, grant, asset, fraud indicators, and recovery potential are assessed separately.
12. Proposed corrective actions define process, access, custody, training, master data, movement, count, reconciliation, and monitoring improvements.
13. Required stock adjustment is referred to L3-J-090; confirmed loss or damage to L3-J-091; asset-linked finding to L3-J-092; other consequences to their competent authorities.
14. The Audit Approver reviews evidence sufficiency, independence, responses, classification, materiality, root cause, and referral completeness.
15. The Audit Approver rejects, returns, approves, or partially approves findings and records unresolved components explicitly.
16. The case closes as reconciled only when every variance component has an approved explanation, correction or referral owner and residual exposure is governed.
17. Evidence, findings, responses, referrals, corrective actions, monitoring, closure, actors, timestamps, and versions remain auditable.

---

# 10. Alternate Business Flows

## 10.1 Timing or Cut-Off Difference

Authoritative movement evidence resolves the variance without physical adjustment where the ledger position is already correct.

## 10.2 Offsetting Excess and Shortage

Items or locations are not netted unless identity, ownership, lot, valuation, authority, and policy permit. Each component remains traceable.

## 10.3 Physical Excess

Ownership and source are verified before recognition. Unidentified excess is controlled and not treated as free institutional stock automatically.

## 10.4 Suspected Fraud or Evidence Tampering

Evidence is protected and the matter referred urgently; the variance investigation avoids prejudicing the competent inquiry.

## 10.5 Inconclusive Evidence

The finding remains unresolved with risk controls, owner, review date, and no fabricated cause.

## 10.6 Multiple Locations or Custodians

Responsibility and movements are reconstructed by period and handover; current custody does not prove historical responsibility.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Evidence Missing or Compromised

Limitations are recorded, recovery attempted, and findings narrowed or left unresolved.

## 11.2 Premature Adjustment

The posting is preserved and reviewed; investigation continues without treating adjustment as proof of cause.

## 11.3 Conflicted Investigator

The investigator is replaced or independently reviewed and prior work remains visible.

## 11.4 Wrong Item, Unit, Lot, or Location

The variance is corrected through attributable scope revision and related records are reviewed.

## 11.5 Investigation Deadline Missed

The case is escalated with interim controls and residual exposure; it is not silently closed.

## 11.6 System Interruption

Evidence custody and protective controls continue, and records are reconciled before work resumes.

---

# 12. Business Rules and Decision Constraints

- Material variance requires governed investigation, evidence, independence, response, findings, and closure authority.
- Physical shortage or excess does not by itself establish cause, liability, fraud, loss, ownership, or financial value.
- Count, book position, cut-off, movement, unit, location, lot, serial, custody, and ownership must be reconciled.
- Offsetting variances cannot be netted merely to reduce reported difference.
- Adjustment corrects stock records but does not prove or erase root cause.
- Loss, damage, asset, finance, legal, disciplinary, insurance, and recovery outcomes require separate authority.
- Unresolved findings remain explicit with controls, owner, and review path.
- Supplier or staff response does not control the finding, and non-response is not automatic admission.
- Closed investigation can be supplemented only through L3-J-093 or an authorised review.
- Evidence follows confidentiality, privacy, legal-hold, and retention rules.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Inventory Audit | `Investigation Required` | Proceeds through `Investigation` and `Reconciliation` using exact J09 positions. | `Reconciled` and then `Completed` / `Closed` when completion rules are satisfied | Physical Verification, Audit & Reconciliation (J09) | `L2-J09` Section 12 |
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `InventoryVarianceEscalated` | Physical Verification, Audit & Reconciliation (J09) | J06, J08, J10, Finance and Audit where material | Records the exact completed business fact material to this scenario. | `L2-J09 Section 31` |
| `VarianceInvestigationCreated` | Physical Verification, Audit & Reconciliation (J09) | J06, J08, J10, Finance and Audit where material | Records the exact completed business fact material to this scenario. | `L2-J09 Section 31` |
| `VarianceInvestigationCompleted` | Physical Verification, Audit & Reconciliation (J09) | J06, J08, J10, Finance and Audit where material | Records the exact completed business fact material to this scenario. | `L2-J09 Section 31` |
| `ReconciliationApproved` | Physical Verification, Audit & Reconciliation (J09) | J06, J08, J10, Finance and Audit where material | Records the exact completed business fact material to this scenario. | `L2-J09 Section 31` |
| `InventoryVarianceReconciled` | Physical Verification, Audit & Reconciliation (J09) | J06, J08, J10, Finance and Audit where material | Records the exact completed business fact material to this scenario. | `L2-J09 Section 31` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| L3-J-087 / L3-J-088 | Supplies audit, count, cut-off, and limitation evidence. | Does not determine cause automatically. |
| Inventory / Warehouse / Custody | Supplies movement and responsibility evidence. | Retains records and custody. |
| Finance / Security / Legal / HR / Grant | Reviews specialist consequences. | Retains decision authority. |
| L3-J-090 to L3-J-092 | Receives approved findings. | Retains adjustment, loss, and asset outcomes. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

Inventory Counter, Inventory Custodian, Variance Investigator, Reconciliation Officer, Audit Approver, and Audit Reviewer are separated where required. Downstream adjustment, loss, asset, Finance or disciplinary decisions remain with their canonical domains.

## 15.2 Required Evidence

- audit, counts, recounts, book position, cut-off, item, unit, location, lot, serial, condition, custody, ownership, materiality, and evidence integrity;
- movement reconstruction, access, communications, statements, responses, related variances, root cause, control failures, responsibility, residual risk, specialist findings, and corrective actions;
- findings, referrals, approval, closure, actors, timestamps, and versions.

## 15.3 Audit Obligations

The institution must reconstruct the original variance, how physical and book positions were verified, what evidence and explanations were tested, how root cause and responsibility were distinguished, who approved findings, and which adjustment or other consequences followed.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- the material variance has been substantiated by authoritative physical count evidence and a confirmed comparison to the authoritative book position;
- an investigation has been conducted with an independent investigator, documented evidence, cause analysis, and finding conclusion;
- the investigation has been reviewed and approved by an authority appropriate to the variance value, materiality, and institutional risk;
- the investigation finding distinguishes between record error, process failure, loss, damage, theft, or other governed cause;
- appropriate referrals to adjustment, loss, disciplinary, legal, or compliance scenarios have been made for findings requiring further action;
- the investigation evidence, cause, authority, actors, decisions, referrals, and audit trail are preserved and immutable.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the material variance has a governed investigation closure with an explicit cause finding and authority decision permanently on record;
- each finding with a material consequence is referred to its authoritative governing scenario with an explicit referral record;
- no stock adjustment, financial posting, or asset record change has been made by the investigation itself;
- the complete investigation evidence, cause analysis, and disposition decisions are permanently accessible and auditable;
- the investigation outcome does not alter the original count evidence or book position records.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Materiality thresholds, investigation authorities, evidence standards, deadlines, and closure criteria require confirmation.
2. Controlled, perishable, grant-funded, consigned, hazardous, high-value, and asset-linked inventory require specialist rules.
3. Responsibility, recovery, insurance, and disciplinary referrals require legal and HR mapping.

## 17.2 Assumptions

- Investigation explains and governs variance but does not post consequences.
- Adjustment and loss outcomes remain separate.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-089**
