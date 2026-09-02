# Supplier Risk Finding to Governed Restriction

## 1. Specification Metadata

**Specification ID:** L4-J-029  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Enterprise Resource Management — Vendor & Supplier Management  
**Realizes Scenario:** L3-J-029  
**Primary Business Owner:** Vendor & Supplier Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-029_Supplier_Risk_Finding_to_Governed_Restriction.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP shall realize an authorised, proportionate and attributable supplier restriction when a governed risk finding requires control of future supplier use.

Restriction shall remain scoped to the approved risk treatment and shall not automatically become universal suspension, blacklisting, contract action or financial action.

## 3. Required Business Context

- canonical supplier/related-scope identity;
- governed risk finding, source and evidence lineage;
- current supplier state, qualification, categories and existing restrictions;
- current sourcing, recommendation, award, proposed commitment and executed commitment references;
- applicable mandatory specialist consequences;
- restriction authority, scope, duration/review/exit rules and supplier response context.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-029-BH-001 | The system shall require an attributable governed risk finding, canonical supplier linkage, explicit scope and competent restriction authority before a final restriction can become effective. | L3-J-029 §§7–9, 12 |
| L4-J-029-BH-002 | The system shall distinguish supplier, site, branch, subcontractor, consortium member, predecessor/successor and related-entity scope and shall not extend restriction automatically without supported attribution. | L3-J-029 §§9 step 5, 12 |
| L4-J-029-BH-003 | The system shall support only source-defined restriction treatments and shall preserve the distinction between monitoring, scoped restriction and suspension. | L3-J-029 §§3–4, 9 steps 7–8, 10.1–10.2, 12 |
| L4-J-029-BH-004 | An approved restriction shall be no broader or longer than the authorised scope/duration or exit conditions established by the governance decision, subject to mandatory upstream rules. | L3-J-029 §§9 steps 8, 11, 13–15, 12 |
| L4-J-029-BH-005 | Where supplier response rights apply, the system shall preserve the notice/response evidence and shall not treat a dispute or response as automatic release of the restriction. | L3-J-029 §§9 steps 9–10, 10.5, 12 |
| L4-J-029-BH-006 | When restriction is approved, the system shall make prohibited and permitted future activity explicit for the affected category/site/value/stage/funding or other approved scope. | L3-J-029 §§3, 9 steps 11, 14–15 |
| L4-J-029-BH-007 | The system shall identify existing sourcing, recommendation, award and commitment positions affected by the restriction and expose them to their authoritative owners without rewriting those records. | L3-J-029 §§9 steps 4, 12, 14.2 |
| L4-J-029-BH-008 | The system shall prevent remediation submission alone from releasing an active restriction; release requires the source-supported authorised verification and decision. | L3-J-029 §§10.4, 12, 16.2 |
| L4-J-029-BH-009 | Where risk cannot be contained by the approved restriction model, the system shall expose the case for L3-J-030 suspension consideration without converting the restriction itself into suspension. | L3-J-029 §§4.3, 10.6, 14.2 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-029-BH-010 | On an approved restriction, the system shall publish or make available the canonical `SupplierRestricted` fact on behalf of Vendor & Supplier Management. | L3-J-029 §14.1 |
| L4-J-029-BH-011 | The system shall make the current restriction facts available to qualification, sourcing, J04, Finance and specialist owners only to the extent required for their separate decisions. | L3-J-029 §§14.2, 16 |

## 6. Failure / Rejection / Continuation

- Ambiguous supplier identity/scope shall prevent overbroad final restriction; any interim control must remain source-authorised.
- Incomplete/unverifiable evidence shall not be converted silently into a final substantiated finding.
- Publication failure shall not alter the authoritative business-effective time; affected actions shall be reconciled to the current governed status.
- Over-restriction shall be corrected prospectively with the error/impact history preserved.
- Unauthorised overrides/exceptions shall not establish valid unrestricted eligibility.
- Missed review deadlines shall follow upstream continuation/expiry rules rather than silently removing restriction.

**Provenance:** L3-J-029 §11.

## 7. Audit & Evidence Behavior

The system shall preserve:

- underlying risk finding and evidence lineage;
- supplier and related-scope attribution;
- affected/unaffected scope;
- alternatives considered and proportionality basis;
- supplier notice/response and conflicts;
- competent approval, reasons, effective time, duration/exit conditions;
- monitoring, exceptions, breaches, reviews, extensions/variations/release;
- affected downstream references and referrals;
- actors, timestamps and versions.

**Provenance:** L3-J-029 §15.

## 8. Observable Completion Conditions

Completion is observable when an approved/returned/rejected/referred restriction decision exists; any active restriction has exact scope, authority and effective conditions; `SupplierRestricted` is available when applicable; future use is governed within that scope; existing owned records remain with their owners; and remediation/release remains separately governed.

## 9. Traceability & Provenance

**Parent:** L3-J-029 — Supplier Risk Finding to Governed Restriction  
**Primary verification source:** L2-J02 — Vendor & Supplier Management Workflows  

Material anchors: L3-J-029 §§2–16.

## 10. Open Issues / Deferred Design

Restriction types, severity mapping, urgent duration, review/release authorities, related-entity attribution and exact treatment of active sourcing/award/commitment positions remain upstream governance issues.

Implementation design remains deferred to L5/L6.
