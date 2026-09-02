# Registered Supplier to Approved Active Supplier

## 1. Specification Metadata

**Specification ID:** L4-J-021  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Vendor & Supplier Management  
**Realizes Scenario:** L3-J-021  
**Primary Business Owner:** Vendor & Supplier Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-021_Registered_Supplier_to_Approved_Active_Supplier.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`  
**Referenced By:** Supplier qualification, sourcing, award, commitment, receipt, payable, performance, evidence-renewal, restriction, suspension, and audit specifications where applicable.

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern institutional due diligence, approval, and activation of a canonical J02 Supplier that is already `Registered`.

The behavior begins when a canonical Registered Supplier is nominated for institutional approval and activation for a legitimate prospective relationship. Successful completion occurs when the Supplier follows the exact J02 lifecycle to `Approved` and then `Active`, with approval conditions, effective date, review date, risk evidence, and decision lineage preserved.

Returned, rejected, or deferred approval are business dispositions only unless they correspond to exact J02 lifecycle positions defined upstream. This specification does not invent additional Supplier states, grant category-specific qualification, validate payment instructions, or create sourcing, award, commitment, receipt, payable, accounting, or payment outcomes.

---

## 3. Required Business Context

- One canonical Supplier in `Registered` with no unresolved duplicate.
- Current L3-J-020 identity, ownership, relationship, representative, registration, and evidence facts.
- Legitimate institutional relationship purpose and expected exposure profile.
- Institution-governed due-diligence checks, risk tiers, mandatory prohibitions, approval authorities, conditions, review periods, and rejection rules.
- Authoritative specialist findings from legal, compliance, Finance/tax, security, safety, privacy, and other required owners.
- Current supplier responses, remediation evidence, historic institutional experience, incidents, disputes, and related-entity history where applicable.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-021-BH-001 | When approval begins, the system shall require a canonical Supplier in `Registered` and preserve the sponsor, intended relationship, expected categories, geography, exposure profile, applicable access context, and submission time. | L3-J-021 §§7–9.1, 12 |
| L4-J-021-BH-002 | Before due diligence proceeds, the system shall make the canonical identity, ownership, related entities, representatives, tax/registration evidence, contacts, and duplicate disposition from L3-J-020 available without changing that identity governance. | L3-J-021 §§9.2, 14.2 |
| L4-J-021-BH-003 | The system shall apply only the institutionally governed check set applicable to supplier type, jurisdiction, category interests, public/private status, risk, value/exposure, data, safety, funding, and expected relationship. | L3-J-021 §§7, 9.3, 12 |
| L4-J-021-BH-004 | The system shall preserve authoritative sanctions, debarment, watchlist, prohibition, conflict, related-party, beneficial-ownership, politically exposed, and other applicable integrity findings with source/date attribution. | L3-J-021 §§9.4, 12, 15.2 |
| L4-J-021-BH-005 | Where required, the system shall preserve legal/regulatory, financial/tax, operational, continuity, quality, safety, security, privacy, sustainability, labour, environmental, ethical, insurance, credit, solvency, and concentration findings without transferring their specialist ownership to J02. | L3-J-021 §§5, 9.5–9.7, 14.2 |
| L4-J-021-BH-006 | The system shall preserve historic performance, disputes, incidents, complaints, predecessor/related-entity evidence, and supplier explanations without treating relationship alone as proof of adverse conduct. | L3-J-021 §§9.8, 9.11 |
| L4-J-021-BH-007 | The system shall require declared conflicts involving supplier owners, representatives, sponsors, reviewers, approvers, or institutional personnel to remain attributable to their mitigation or escalation outcome. | L3-J-021 §§9.9, 12 |
| L4-J-021-BH-008 | When gaps, expired evidence, inconsistencies, adverse findings, or remediation needs exist, the system shall preserve the deficiency and subsequent supplier response/evidence without overwriting the original finding or chronology. | L3-J-021 §§9.10–9.11, 15.3 |
| L4-J-021-BH-009 | The system shall preserve the institutionally governed supplier risk tier and proposed review timing with the evidence basis used for that decision. | L3-J-021 §§9.12, 12, 15.2 |
| L4-J-021-BH-010 | The approval recommendation shall preserve the parent-supported disposition, permitted activity position, conditions/restrictions, monitoring obligations, effective date, and next review date. | L3-J-021 §§3, 9.13 |
| L4-J-021-BH-011 | The system shall require the source-supported specialist concurrences and exception approvals applicable to the case and shall not allow sponsor preference to override mandatory prohibitions. | L3-J-021 §§9.14, 12 |
| L4-J-021-BH-012 | The system shall recognise the Supplier Approver's attributable decision and shall not permit the supplier representative, requester alone, verifier alone, or conflicted actor to substitute for that decision. | L3-J-021 §§9.15, 12, 15.1 |
| L4-J-021-BH-013 | On successful approval, the system shall move the Supplier through the exact J02 lifecycle to `Approved` and then `Active`. | L3-J-021 §§3, 9.16, 13, 16 |
| L4-J-021-BH-014 | When approval is conditional, the system shall preserve explicit enforceable limits and shall represent the Supplier as `Active` only within the parent-supported approved conditions; it shall not invent a separate "Conditionally Active" lifecycle state. | L3-J-021 §§3, 10.1, 12, 13 |
| L4-J-021-BH-015 | If approval is interrupted, deferred, or unresolved before the J02 approval transition, the system shall retain the last authoritative Supplier lifecycle position and shall not infer `Approved` or `Active`. | L3-J-021 §§3, 11.1–11.5, 13 |
| L4-J-021-BH-016 | If the Supplier is rejected through the authoritative approval path, the system shall preserve the source-defined `Returned / Rejected` outcome and shall not infer suspension or blacklisting. | L3-J-021 §§3, 11.3–11.4, 12, 13 |
| L4-J-021-BH-017 | The system shall prevent ordinary sourcing, award, or commitment from treating a non-Active Supplier as Active merely because it is registered, historically used, externally approved, or requester-sponsored. | L3-J-021 §§2, 10.4, 11.6, 12 |
| L4-J-021-BH-018 | `Active` status shall not be represented as category-, tender-, geography-, funding-, project-, or event-specific qualification; that outcome remains governed by L3-J-022. | L3-J-021 §§2, 4.4, 12, 16.2 |
| L4-J-021-BH-019 | Supplier approval/activation shall not be represented as bank-payment approval or as financial liability. | L3-J-021 §§4.4, 12 |
| L4-J-021-BH-020 | When a material adverse fact is discovered after activation, the system shall preserve the new fact and its referral without deleting or silently reversing the original approval history. | L3-J-021 §§10.6, 11.7, 12 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-021-BH-021 | When a Registered Supplier is submitted into the J02 verification path, the system shall publish `SupplierSubmittedForVerification` with Vendor & Supplier Management (J02) as canonical publisher. | L3-J-021 §14.1; L2-J02 §13 |
| L4-J-021-BH-022 | When the source-supported verification activity is completed, the system shall publish `SupplierVerificationCompleted` with J02 as canonical publisher. | L3-J-021 §14.1; L2-J02 §13 |
| L4-J-021-BH-023 | When the Supplier reaches canonical `Approved`, the system shall publish `SupplierApproved` with J02 as canonical publisher. | L3-J-021 §§13–14.1; L2-J02 §§6, 13 |
| L4-J-021-BH-024 | When the Supplier reaches canonical `Active`, the system shall publish `SupplierActivated` with J02 as canonical publisher. | L3-J-021 §§13–14.1; L2-J02 §§6, 13 |
| L4-J-021-BH-025 | When the authoritative onboarding/approval decision rejects the Supplier, the system shall publish `SupplierRejected` with J02 as canonical publisher. | L3-J-021 §14.1; L2-J02 §13 |

Legal/compliance, Finance/tax, security, safety, privacy, and other specialist owners retain their findings. L3-J-022 consumes valid Active status for separate qualification. J01 and J04 consume supplier status/conditions but retain sourcing and commitment lifecycles.

---

## 6. Failure / Rejection / Continuation

- If a mandatory check is unavailable, approval shall remain blocked unless an explicit upstream risk rule permits the parent-supported conditional path.
- Conflicting evidence shall leave the Supplier in the last authoritative lifecycle position while the conflict remains unresolved.
- A sanction, debarment, fraud, or mandatory prohibition concern shall block activation and remain referred to the appropriate authoritative process.
- Supplier non-response shall not imply approval; only the source-supported return, rejection, expiry, or continuation disposition may apply.
- Interrupted approval shall not advance `Registered` to `Approved` or `Active`.
- An unauthorised sourcing/award/commitment attempt shall not activate the Supplier.
- An adverse fact discovered after activation shall be referred to separately governed restriction, suspension, investigation, or other J02 action without deleting the original approval.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- canonical Supplier identity/version, ownership, relationships, representative authority, registration evidence, and intended relationship;
- applicable check set and each material specialist source finding;
- sanctions, debarment, conflicts, legal/regulatory, financial/tax, insurance, operational, security, safety, privacy, sustainability, ethical, and other required evidence;
- historic performance/incidents, supplier responses, remediation, unresolved risks, and chronology;
- risk tier, review timing, conditions, restrictions, recommendation, specialist concurrence/exception decision, approver authority, effective time, and next review date;
- canonical lifecycle movement through `Approved` and `Active`, or source-supported rejection/no-progression outcome;
- exact canonical J02 events published by their source-supported conditions.

---

## 8. Observable Completion Conditions

Successful completion requires:

1. a canonical Registered Supplier has undergone the required source-supported due diligence;
2. an attributable Supplier Approver decision exists against the relevant registration/evidence version;
3. required conditions and specialist findings are preserved;
4. the Supplier reaches canonical `Approved` and then `Active`;
5. approval/activation effective date and review timing are attributable where required by the parent;
6. `Active` remains distinct from L3-J-022 qualification;
7. no sourcing, commitment, payment instruction, or financial liability has been created by this scenario.

Returned, deferred, rejected, or interrupted cases are complete only as the parent-supported alternative disposition and do not satisfy the successful Active outcome.

---

## 9. Traceability & Provenance

**Parent:** L3-J-021 — Registered Supplier to Approved Active Supplier  
**Primary verification source:** L2-J02 — Vendor & Supplier Management Workflows  
**Additional verification source:** L2-J00 — Enterprise Resource Management Architecture & Governance.

The exact J02 Supplier lifecycle is preserved: `Prospective / Registered → Submitted for Onboarding → Verification Pending → Approved → Active`, with `Returned / Rejected` as the source-defined alternate branch. Exact events are `SupplierSubmittedForVerification`, `SupplierVerificationCompleted`, `SupplierApproved`, `SupplierActivated`, and `SupplierRejected`.

---

## 10. Open Issues / Deferred Design

- Check matrices, risk tiers, mandatory prohibitions, exception authorities, and review periods by supplier profile remain upstream-governed.
- Institution-specific sanctions, debarment, politically exposed person, beneficial-ownership, sustainability, security, safety, and financial thresholds require upstream policy mapping.
- The boundary between conditional activation and L3-J-022 scope-specific qualification remains an upstream J02 governance issue; this L4 does not resolve it.
- `Section_J_Canonical_Register.md`, although listed by the Locked parent, could not be located by exact filename during current repository verification. No material Supplier lifecycle, event, ownership, or authority semantics are inferred from it.
- Due-diligence service implementation, APIs/interfaces, persistence/data model, workflow routing, event transport, technical access control, retries/rollback, infrastructure, and UI/UX are deferred to L5/L6.

**Review Verdict:** PASS WITH NON-BLOCKING FINDINGS
