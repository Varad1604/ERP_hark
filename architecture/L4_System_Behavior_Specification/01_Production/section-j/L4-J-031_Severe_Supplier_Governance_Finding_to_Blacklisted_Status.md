# Severe Supplier Governance Finding to Blacklisted Status

## 1. Specification Metadata

**Specification ID:** L4-J-031  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Enterprise Resource Management — Vendor & Supplier Management  
**Realizes Scenario:** L3-J-031  
**Primary Business Owner:** Vendor & Supplier Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-031_Severe_Supplier_Governance_Finding_to_Blacklisted_Status.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP shall make an authorised Blacklisted Status effective only after a severe substantiated supplier governance finding has passed the competent, source-defined blacklisting process.

Blacklisting remains distinct from assessment, restriction, suspension, inactivity, closure, contract termination and financial action. The system shall enforce the approved scope and effective period without inventing grounds, authority, duration or appeal rules.

## 3. Required Business Context

- canonical supplier identity and any proposed related-entity scope;
- severe governed finding and reliable evidence lineage;
- competent authority, jurisdiction and applicable blacklisting rules;
- notice, representation/hearing/review and appeal/stay context;
- prior restriction/suspension and current supplier/qualification/sourcing positions;
- approved scope, duration/effective time and disclosure classification;
- legal/compliance/specialist findings where required.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-031-BH-001 | The system shall require severe substantiated grounds, canonical supplier linkage, competent authority and applicable lawful procedure before Blacklisted Status becomes effective. | L3-J-031 §§2, 7–9, 12 |
| L4-J-031-BH-002 | The system shall preserve the distinction between allegation, interim finding, substantiated fact, regulatory/conviction fact, contractual dispute and final governance finding. | L3-J-031 §§9 steps 2–3, 12 |
| L4-J-031-BH-003 | The system shall not extend proposed blacklisting to branches, related entities, directors, consortium members, successors or affiliates without supported attribution and applicable authority/due process. | L3-J-031 §§9 step 4, 10.6, 12 |
| L4-J-031-BH-004 | Where notice/representation/review rights apply, the system shall preserve the required notice, response/hearing evidence and governing deadlines before final effective status, subject to source-defined stay/urgent rules. | L3-J-031 §§9 steps 6–8, 10.4–10.5, 12 |
| L4-J-031-BH-005 | The system shall preserve the competent authority's reasoned decision, approved scope, duration/effective period, prohibited future activity and publication/disclosure limits. | L3-J-031 §§9 steps 9–14, 12 |
| L4-J-031-BH-006 | Blacklisted Status shall become effective only according to the approved decision and any controlling stay/appeal rule; the system shall not silently backdate it. | L3-J-031 §§9 step 13, 10.4, 12 |
| L4-J-031-BH-007 | The system shall prevent new registration reliance, qualification, invitation, participation, evaluation advancement, award or commitment within the approved blacklisting scope while the status is controlling. | L3-J-031 §§4.3, 9 steps 10, 14–15, 12 |
| L4-J-031-BH-008 | The system shall preserve historical supplier identity and evidence and shall not delete or rewrite prior supplier, sourcing, contract or financial records because of blacklisting. | L3-J-031 §§2, 4.4, 9 step 16, 12 |
| L4-J-031-BH-009 | The system shall expose existing awards, contracts, deliveries, payables, payments, claims and warranties to their authoritative owners for separate disposition rather than cancelling/reversing them automatically. | L3-J-031 §§9 steps 15–16, 10.7, 12 |
| L4-J-031-BH-010 | Appeal, stay, variation, correction, expiry or restoration eligibility shall be represented as attributable successor facts while preserving the original decision and history. | L3-J-031 §§10.2–10.4, 11.4–11.6, 12 |
| L4-J-031-BH-011 | Where restoration is permitted, the system shall not restore ordinary eligibility from remediation alone; it shall require the separately governed L3-J-032 authority decision. | L3-J-031 §§4.5, 10.2–10.3, 12 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-031-BH-012 | On approved effective blacklisting, the system shall publish or make available the canonical `SupplierBlacklisted` fact on behalf of Vendor & Supplier Management. | L3-J-031 §14.1 |
| L4-J-031-BH-013 | The system shall expose only the lawful, accurate, current and necessary blacklisting status to authorised registration, qualification, sourcing, commitment, Finance, Legal, Compliance and audit consumers. | L3-J-031 §§9 step 15, 12, 14.2 |

## 6. Failure / Rejection / Continuation

- Defective authority or notice shall prevent unsupported final effect; any interim suspension remains separately governed.
- Lost/altered/legally unusable evidence shall preserve the limitation and shall not be silently treated as sufficient.
- Publication failure shall not change the business-effective decision time.
- Incorrect entity/overbroad scope shall be corrected through attributable variation/stay/withdrawal with history preserved.
- Unauthorised override shall not create valid future eligibility.
- Appeal/stay outcomes shall become the controlling position when effective without deleting the prior blacklisting decision.
- System interruption shall not delete or backdate the legal business-effective status.

**Provenance:** L3-J-031 §11.

## 7. Audit & Evidence Behavior

The system shall preserve:

- canonical supplier and related-scope attribution;
- severe findings, evidence lineage, legal/policy grounds and jurisdiction;
- notice, delivery, disclosed particulars, response/hearing/review;
- conflicts, specialist advice and alternatives considered;
- reasoned authority decision, scope, duration/effective time and prohibited activity;
- publication classification and downstream notifications;
- appeals, stays, corrections, variations, expiry/restoration eligibility;
- actors, reasons, timestamps and versions.

**Provenance:** L3-J-031 §15.

## 8. Observable Completion Conditions

The behavior is complete when the blacklisting proposal has a governed decision; any effective Blacklisted Status has explicit competent authority, grounds, scope and effective period; `SupplierBlacklisted` is available when applicable; future use is prevented within the controlling scope; existing owned obligations remain with their owners; and appeal/stay/restoration history remains attributable.

## 9. Traceability & Provenance

**Parent:** L3-J-031 — Severe Supplier Governance Finding to Blacklisted Status  
**Primary verification source:** L2-J02 — Vendor & Supplier Management Workflows  

Material anchors: L3-J-031 §§2–16.

## 10. Open Issues / Deferred Design

Competent authorities, permitted grounds, duration categories, appeal/stay rules, disclosure, restoration, related-entity treatment and external debarment-list treatment remain upstream legal/governance matters.

Implementation design remains deferred to L5/L6.
