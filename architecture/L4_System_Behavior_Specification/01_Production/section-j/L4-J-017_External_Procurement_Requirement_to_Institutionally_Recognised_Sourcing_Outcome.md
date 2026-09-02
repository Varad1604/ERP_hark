# External Procurement Requirement to Institutionally Recognised Sourcing Outcome

## 1. Specification Metadata

**Specification ID:** L4-J-017  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Procurement Planning & Sourcing  
**Realizes Scenario:** L3-J-017  
**Primary Business Owner:** Procurement Planning & Sourcing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-017_External_Procurement_Requirement_to_Institutionally_Recognised_Sourcing_Outcome.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`  
**Referenced By:** Downstream commitment, supplier, receipt, payable, funding, reporting, and audit specifications where applicable.

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern the institution's decision to rely on a sourcing outcome produced by an authorised external procurement body while preserving the truth that the external body, not SutraERP or the institution, owns the external procurement process and award.

The behavior begins when approved institutional demand and an identifiable external procurement outcome are submitted for institutional recognition. It ends when the institution recognises, conditionally recognises, returns, or rejects the external outcome; mapped internal demand and permitted scope are explicit; and the result is available to a separately governed commitment or alternate sourcing path.

This specification does not re-perform, recreate, or retroactively approve external evaluation; alter external records; approve J03-owned demand; change J02 Supplier state; or create a Purchase Order, receipt, payable, accounting entry, or payment.

---

## 3. Required Business Context

- Approved, current, identifiable J03-owned institutional demand available for sourcing.
- Identifiable external procurement body, process, award reference, supplier, scope, validity dates, and authoritative evidence source.
- Lawful or policy-permitted institutional reliance basis.
- Institution membership, beneficiary status, mandate, funding, framework participation, geography, or other eligibility evidence where applicable.
- External terms, lots, quantities, prices, ceilings, dates, call-off/use conditions, amendments, and status facts.
- J02-owned institutional supplier identity, eligibility, restriction, ownership, and performance facts.
- Institution-specific legal, tax, security, budget, funding, conflict, and compliance conditions.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-017-BH-001 | When an external procurement outcome is submitted for recognition, the system shall preserve the internal demand reference, external body, external procurement/award identifiers, evidence source, proposed supplier, initiating actor, and submission time. | L3-J-017 §§8, 9.1 |
| L4-J-017-BH-002 | Before recognition, the system shall confirm that the mapped internal demand remains approved, current, available for sourcing, and sufficiently defined without changing J03-owned demand state. | L3-J-017 §§7, 9.2, 12 |
| L4-J-017-BH-003 | The system shall require evidence of the external body's identity, mandate, jurisdiction, delegation, marketplace/framework/consortium/funding role, or other permitted authority before institutional reliance can be approved. | L3-J-017 §§7, 9.3, 11.1, 12 |
| L4-J-017-BH-004 | The system shall require the external process/outcome to be authentic, current, usable, and not authoritatively suspended, cancelled, expired, challenged, or superseded beyond the institution's permitted reliance basis. | L3-J-017 §§9.4–9.5, 11.2, 11.6 |
| L4-J-017-BH-005 | The system shall preserve external method, notice where applicable, scope, evaluation, award approval, supplier, lot, prices, terms, dates, ceilings, amendments, and usage rules only as externally owned facts or authoritative references. | L3-J-017 §§3, 9.5, 12 |
| L4-J-017-BH-006 | Before recognition, the system shall require institution-specific eligibility to rely on the external outcome and shall not infer eligibility merely from supplier willingness or existence of an external document. | L3-J-017 §§7, 9.6, 11.1, 11.3, 12 |
| L4-J-017-BH-007 | The system shall map external lots, items/services, quantities, supplier positions, locations, prices/value bases, ceilings, validity, and call-off/use conditions only to matching approved internal demand. | L3-J-017 §§9.7, 12 |
| L4-J-017-BH-008 | Where external scope and internal demand do not fully match, the system shall limit recognition to the matched portion and preserve residual demand for correction or another governed sourcing route; it shall not silently expand either source. | L3-J-017 §§9.8, 10.6, 11.4, 12 |
| L4-J-017-BH-009 | Before downstream use, the system shall require current authoritative J02 supplier identity and local institutional eligibility/restriction facts even when the supplier was externally awarded. | L3-J-017 §§9.9, 11.5, 12 |
| L4-J-017-BH-010 | The system shall preserve institution-specific commercial, currency, tax, logistics, warranty, service, licensing, data, security, law, dispute, funding, reporting, and total-cost assessments required by the parent without claiming ownership of external procurement facts. | L3-J-017 §§9.10, 15.2 |
| L4-J-017-BH-011 | The system shall distinguish externally accepted facts from locally verified facts and shall explicitly preserve any evidence that could not be independently verified. | L3-J-017 §§2, 9.11, 15.3 |
| L4-J-017-BH-012 | If recognition is conditional, the system shall preserve each mandatory condition and prevent downstream use until the parent-required condition is satisfied. | L3-J-017 §§9.12, 10.5, 12 |
| L4-J-017-BH-013 | The recognition recommendation shall identify exact internal demand, external source, supplier, scope, value basis, validity, limitations, evidence, conditions, and downstream path. | L3-J-017 §§3, 9.13 |
| L4-J-017-BH-014 | When the Procurement Approver authorises reliance, the system shall recognise only the approved mapped demand and external entitlement and shall not itself create a Purchase Order or financial record. | L3-J-017 §§9.15–9.16, 12, 16 |
| L4-J-017-BH-015 | When recognition is returned or rejected, the system shall preserve the reason and keep affected demand available only for a source-supported correction or alternate sourcing path. | L3-J-017 §§3, 4.2, 11.1–11.5 |
| L4-J-017-BH-016 | When an authoritative external amendment, suspension, expiry, challenge, cancellation, supplier change, price change, or ceiling exhaustion affects recognised reliance, the system shall require reassessment before new reliance and shall preserve limitation, withdrawal, or supersession evidence where applicable. | L3-J-017 §§11.6–11.7, 12 |
| L4-J-017-BH-017 | The system shall preserve external provenance and original identifiers/evidence without fabricating internal bid-opening, evaluation, scoring, or award facts for activities that occurred externally. | L3-J-017 §§2, 4.4, 12; L2-J01 §§6.6, 7.8, 30 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-017-BH-018 | When external procurement is registered for institutional tracking, the system shall publish `ExternalProcurementRegistered` with Procurement Planning & Sourcing (J01) as canonical publisher. | L3-J-017 §14.1; L2-J01 §35.12 |
| L4-J-017-BH-019 | When an authoritative external status is recorded, the system shall publish `ExternalProcurementStatusRecorded` with J01 as canonical publisher without claiming ownership of the external status. | L3-J-017 §14.1; L2-J01 §35.12 |
| L4-J-017-BH-020 | When the external evaluation outcome is recorded as relied-on evidence, the system shall publish `ExternalEvaluationOutcomeRecorded` with J01 as canonical publisher. | L3-J-017 §14.1; L2-J01 §35.12 |
| L4-J-017-BH-021 | When the externally selected supplier position is recorded for the recognition case, the system shall publish `ExternalSupplierSelectionRecorded` with J01 as canonical publisher without changing J02 Supplier ownership. | L3-J-017 §14.1; L2-J01 §35.12 |
| L4-J-017-BH-022 | When institutional reliance is approved, the system shall publish `ExternalProcurementOutcomeApproved` with J01 as canonical publisher. | L3-J-017 §14.1; L2-J01 §35.12 |

The external procurement body retains ownership of the external process and award. J03/requesting units retain internal demand authority, J02 retains Supplier lifecycle, J04 retains commitment lifecycle, and Finance/funding authorities retain financial truth and funding eligibility.

---

## 6. Failure / Rejection / Continuation

- If external authority or institutional reliance eligibility cannot be established, recognition shall be returned, limited, or rejected.
- If evidence is incomplete or inauthentic, unsupported assumptions shall not become recognised facts.
- If internal demand and external scope differ, only the matched portion may proceed.
- A locally restricted supplier shall block or constrain institutional downstream use even if externally awarded.
- A material external change shall trigger recognition reassessment before new reliance.
- If continuing validity can no longer be demonstrated because authoritative evidence becomes unavailable, new reliance shall be blocked to the extent required by the parent while retained evidence/reference gaps remain visible.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- approved internal demand, specification, quantity, value/funding context, dates, and remaining availability;
- external body identity, authority/mandate, procurement and award identifiers, method, status, supplier, lot, prices, terms, validity, ceilings, amendments, and authoritative evidence references;
- institution eligibility to rely on the external outcome;
- exact demand-to-external-scope mapping and any excluded/residual portion;
- J02 supplier eligibility/restriction facts and institution-specific conflict/compliance conditions;
- local commercial/legal/security/tax/funding assessments and conditions;
- recommendation, recognition/conditional recognition/return/rejection decision, validity, limitations, source ownership, actors, reasons, and timestamps;
- exact canonical J01 external-procurement events.

---

## 8. Observable Completion Conditions

The scenario is observably complete when:

1. the external owner, process/award reference, authority, status, supplier, scope, and evidence source are attributable;
2. institutional eligibility to rely on the outcome has been established or rejected;
3. approved internal demand is explicitly mapped to permitted external scope without silent expansion;
4. local supplier and mandatory institutional conditions are resolved or explicitly block downstream use;
5. recognition is approved, conditionally approved, returned, or rejected with attributable authority and effective time;
6. original external ownership and provenance remain visible;
7. a recognised outcome is available only as a governed sourcing fact for separately controlled downstream commitment.

---

## 9. Traceability & Provenance

**Parent:** L3-J-017 — External Procurement Requirement to Institutionally Recognised Sourcing Outcome  
**Primary L2 source:** L2-J01 — Procurement Planning & Sourcing Workflows  
**Supporting L2 sources:** L2-J00; L2-J02; L2-J03.  

L2-J01 expressly supports external execution/tracking while prohibiting fabrication of internal procurement activity. L2-J02 retains canonical Supplier truth, including external supplier mapping. L2-J03 retains approved-demand ownership. The exact J01 events are `ExternalProcurementRegistered`, `ExternalProcurementStatusRecorded`, `ExternalEvaluationOutcomeRecorded`, `ExternalSupplierSelectionRecorded`, and `ExternalProcurementOutcomeApproved`.

---

## 10. Open Issues / Deferred Design

- Institution-specific permitted external procurement routes, evidence sufficiency, recognition validity, oversight, and reliance conditions remain upstream-governed.
- The Locked parent defines no separate canonical lifecycle for an internal recognition case; this L4 therefore does not invent recognition states such as "Pending", "Withdrawn", or "Superseded".
- `Section_J_Canonical_Register.md` could not be located by exact filename in the current repository. No substitute semantics are inferred because all material behavior is supported by the Locked parent and Locked J00/J01/J02/J03 sources.
- External portal/API integration, data synchronisation mechanics, persistence, services, technical event transport, retries, infrastructure, and UI/UX remain deferred to L5/L6.

**Review Verdict:** PASS WITH NON-BLOCKING FINDINGS
