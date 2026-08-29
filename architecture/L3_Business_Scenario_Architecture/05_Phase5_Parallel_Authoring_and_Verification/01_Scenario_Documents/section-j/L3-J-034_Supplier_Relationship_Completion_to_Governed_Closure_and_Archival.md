# Supplier Relationship Completion to Governed Closure and Archival

# 1. Scenario Metadata

**Scenario ID:** L3-J-034  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Vendor & Supplier Management  
**Participating Domains:** J  
**Scenario Type:** Critical  
**Priority:** P2  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** supplier retrieval, audit, legal hold, duplicate reconciliation, external reference mapping, historical sourcing, contract, finance, and records-disposal scenarios  

---
# 2. Scenario Purpose

This scenario governs final closure and archival of a completed supplier relationship after current operational use has ended and all closure conditions are addressed. It prevents premature deletion or concealment of obligations while converting an operational relationship into protected, retrievable historical evidence.

Closure and archival are non-punitive lifecycle outcomes. They do not equal inactivity, restriction, suspension, blacklisting, contract termination, financial settlement, or statutory disposal.

---

# 3. Business Outcome

A completed supplier relationship reaches Governed Closure and Archival with confirmed identity, closure reason, open-position disposition, retention classification, legal-hold treatment, access rules, retrieval path, authority, and immutable lineage, while authoritative contractual and financial history remains intact.

Outcome labels in this section are business dispositions or supporting-record results, not additional canonical lifecycle states. Section 13 states the only canonical lifecycle impact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an inactive or otherwise completed supplier relationship is referred for closure after foreseeable operational use has ceased.

## 4.2 End Boundary

The scenario ends when closure is approved, rejected, returned, or deferred; the relationship becomes closed and archived where approved; retention, access, legal hold, retrieval, and future re-engagement requirements are recorded; and all open positions remain with their owners.

## 4.3 In Scope

- validating canonical identity, relationship state, completion reason, authority, and closure eligibility;
- identifying sourcing, award, commitment, contract, delivery, warranty, claim, dispute, tax, payable, payment, recovery, audit, investigation, and legal-hold positions;
- confirming disposition or continuing ownership of each open obligation;
- applying retention, confidentiality, privacy, access, evidentiary, and legal-hold classifications;
- closing operational use while retaining protected historical identity, profile, qualification, performance, governance, transaction, and communication evidence;
- establishing controlled retrieval, correction, re-engagement, and eventual disposal referrals.

## 4.4 Out of Scope

- terminating or settling contracts, claims, payables, payments, or recoveries;
- blacklisting, suspending, or restricting a supplier;
- merging duplicate supplier records, governed by L3-J-035;
- deleting records or performing statutory disposal;
- creating a new supplier relationship for later re-engagement;
- technical archival storage, indexing, encryption, backup, or disposal implementation.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-033 | Inactivity may mature into relationship completion. |
| Related | L3-J-031 | Blacklisted history may later archive but remains punitive history. |
| Related | L3-J-035 and L3-J-036 | Identity reconciliation and reference mapping must preserve archived lineage. |
| Related | Contract, Finance and Records scenarios | Retain authoritative obligations and eventual disposal authority. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Vendor & Supplier Management | Owns closure review, cross-domain confirmation, archival classification, approval, access referral, and supplier relationship history. |

Contract, finance, legal, tax, audit, records, privacy, security, and operational owners retain their records, holds, obligations, and disposal authority.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Supplier Representative | Receives notice and supplies permitted response or remediation evidence; cannot remove institutional restrictions. | Responds. |
| Institutional Verifier | Independently verifies evidence material to the governance decision. | Verifies evidence. |
| Supplier Governance Authority | Proposes or approves restriction, suspension, blacklisting, reactivation, inactivation, closure or merge treatment. | Decides governed supplier treatment. |
---

# 7. Preconditions and Entry Criteria

- A canonical supplier relationship exists and is inactive or otherwise eligible for closure.
- Closure reasons, open-position checks, retention schedules, legal holds, access, retrieval, correction, re-engagement, and disposal rules are effective.
- Current supplier governance, qualification, sourcing, commitment, contract, finance, tax, audit, and investigation positions are available.
- Record custodians and authoritative owners can confirm continuing obligations.
- Closure cannot bypass an active legal, regulatory, contractual, financial, or evidence requirement.

---

# 8. Trigger

An authorised owner determines that the supplier relationship has completed, foreseeable ordinary use has ended, and closure and archival should be evaluated.

---

# 9. Main Business Flow

1. The relationship owner submits the canonical supplier, completion reason, last activity, affected scope, and proposed closure date.
2. The Supplier Governance Authority validates identity, inactivity, qualification, restriction, suspension, blacklisting, representative, related-entity, duplicate, and external-reference positions.
3. Active sourcing, recommendations, awards, proposed commitments, Purchase Orders, contracts, deliveries, warranties, claims, disputes, assets, security deposits, payables, credits, payments, recoveries, tax, and audit items are identified.
4. Each open item is confirmed as completed, transferred to a responsible owner, or recorded as a valid reason to defer closure.
5. Legal proceedings, investigations, information requests, audit matters, regulatory duties, legal holds, and evidence-preservation requirements are identified.
6. Records are classified by authoritative source, retention category, retention start, minimum period, confidentiality, privacy, access, legal hold, and eventual disposition authority.
7. Supplier identity, status, qualifications, performance, restrictions, suspension, blacklisting, transactions, communications, representatives, and decisions are preserved as linked history.
8. Closure is distinguished from record disposal; no evidence is destroyed as part of this scenario.
9. Any unresolved duplicate identity is referred to L3-J-035 before archival lineage is finalised.
10. The supplier is notified where policy or contract requires, including continuing obligations and the process for later re-engagement or record correction.
11. The recommendation defines closure reason, effective time, archived scope, retained operational access, open-owner responsibilities, access classification, retention and retrieval conditions.
12. The Supplier Governance Authority reviews completion evidence, open positions, holds, retention, supplier communication, and future need.
13. The approver rejects, returns, defers, or approves closure and archival.
14. The Supplier reaches the canonical `Closed / Archived` state prospectively; the canonical identity remains resolvable and cannot be reused for another entity.
15. Ordinary future procurement use is unavailable unless a governed re-engagement path revalidates identity, profile, qualifications, representatives, and governance status.
16. Authorised custodians receive records with preservation, access, retrieval, correction, and hold obligations.
17. Closure, archive manifests, holds, access, retrievals, corrections, re-engagement referrals, actors, timestamps, and versions remain auditable.

---

# 10. Alternate Business Flows

## 10.1 Open Warranty, Claim, or Financial Balance

The relationship may remain inactive or close with controlled open-obligation ownership only where policy permits. The authoritative item remains active.

## 10.2 Legal Hold or Investigation

Closure may proceed if permitted, but protected evidence is excluded from disposal and access follows the hold authority.

## 10.3 Supplier Legal Entity Ceased

Evidence of cessation is recorded; successor or liability questions are referred to legal and identity governance. History is not transferred automatically.

## 10.4 Future Re-engagement

A new review revalidates current facts and creates a successor operational state. Archived history remains linked and is not overwritten.

## 10.5 Closure Deferred

The authority records unresolved conditions, owner, target date, and controls while the relationship remains inactive or otherwise governed.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Undetected Open Obligation

The obligation remains valid, closure impact is reviewed, and archive access is restored or expanded under authority.

## 11.2 Premature or Incorrect Closure

An attributable correction or reactivation is approved; the error and affected activity remain visible.

## 11.3 Record Missing or Integrity Compromised

Closure is held or limitation recorded, recovery is attempted, and evidence-integrity concerns are escalated.

## 11.4 Unauthorised Deletion or Access

The event is contained and referred to records, privacy, security, legal, and audit authorities.

## 11.5 Archive or Retrieval Failure

Custody and integrity are reconciled from authorised copies without changing business history.

## 11.6 System Interruption

Approved effective state and retention duties remain controlling and are reconciled on recovery.

---

# 12. Business Rules and Decision Constraints

- Closure requires canonical identity, completion evidence, open-position review, authority, reason, and effective time.
- Closure and archival do not settle, terminate, waive, reverse, delete, or dispose of authoritative obligations or records.
- Legal holds and mandatory retention override ordinary disposal schedules.
- A closed supplier identity cannot be reassigned to another entity.
- Blacklisting, suspension, restrictions, disputes, claims, and adverse evidence remain visible after archival.
- Archived records are immutable except through attributable correction or authorised supplemental evidence.
- Access follows least privilege, purpose, confidentiality, privacy, and legal-hold rules.
- Re-engagement requires current revalidation and cannot silently restore old qualifications or representatives.
- Duplicate reconciliation preserves all source and transaction lineage.
- Eventual disposal requires a separate authorised records process after retention expiry and hold clearance.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Supplier | Existing canonical J02 state | Applies only an exact J02 supplier-governance transition. | `Restricted`, `Suspended`, `Blacklisted`, `Active`, `Inactive` or `Closed / Archived` as material to the scenario | Vendor & Supplier Management (J02) | `L2-J02` Section 6 |
| SupplierGovernanceCase | No canonical lifecycle defined | Preserves evidence and the attributable governance decision without creating local case states. | Governed case record linked to the exact Supplier state | Vendor & Supplier Management (J02) | `L2-J02` Section 5 |
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `SupplierClosed` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| L3-J-033 | Supplies inactive relationship and completion evidence. | Does not perform archival. |
| Contract / Operations | Confirms obligations, warranties, claims, and assets. | Retains their lifecycle. |
| Finance / Tax | Confirms monetary and statutory positions. | Retains financial truth. |
| Legal / Records / Privacy / Security | Defines holds, retention, access, and disposal constraints. | Retains specialist authority. |
| L3-J-035 / L3-J-036 | Preserves canonical and external-reference lineage. | Cannot erase archive history. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J02 roles are separated as policy requires: Supplier Representative; Institutional Verifier; and Supplier Governance Authority. The Supplier Representative cannot verify, approve, or remove an institutional restriction on itself, and a conflicted verifier cannot decide the governed outcome.

## 15.2 Required Evidence

- canonical supplier, relationship states, completion reason, last activity, qualifications, governance history, representatives, related entities, duplicates, and external references;
- sourcing, awards, commitments, contracts, deliveries, warranties, claims, disputes, assets, payables, credits, payments, recoveries, tax, audits, investigations, and holds;
- retention classification, access, privacy, confidentiality, archive manifest, approval, effective time, notices, retrievals, corrections, actors, timestamps, and versions.

## 15.3 Audit Obligations

The institution must reconstruct why closure occurred, which open positions were checked, who retained each obligation, which records were archived, what holds and retention applied, who accessed or retrieved evidence, and whether later correction or re-engagement occurred.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- all obligations, commitments, open orders, outstanding payments, and pending deliveries under the relationship have been confirmed as completed or formally dispositioned;
- a closure decision has been made by an authority appropriate to the relationship value, duration, and institutional significance;
- any residual retention, archival, hold, or legal preservation obligations have been identified and attributed to their authoritative owners;
- the closure decision, evidence, authority, actors, effective date, and audit trail are preserved and immutable;
- the relationship record has been archived with its history, transactions, governance decisions, and evidence intact;
- no relationship data, transaction, approval, or evidence has been destroyed by the closure or archival process.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the supplier relationship is in a Closed and Archived state with a complete, immutable history;
- no new sourcing, commitment, or payment activity may be initiated under the closed relationship;
- all residual retention, legal hold, and archival obligations are attributed to their authoritative owners and remain in force after closure;
- the complete relationship history is permanently accessible for audit, reporting, and legal reconstruction purposes;
- the archived relationship is distinguishable from active, inactive, and suspended relationships in institutional records.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Closure eligibility, open-balance tolerances, retention schedules, archive custodians, access levels, and re-engagement authorities require confirmation.
2. Supplier cessation, merger, insolvency, successor liability, and abandoned claims require legal mapping.
3. Eventual disposal and anonymisation rules require records and privacy architecture.

## 17.2 Assumptions

- Closure is non-punitive and does not delete or settle records.
- Disposal is outside this scenario.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-034**
