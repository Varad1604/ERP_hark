# Approved Supplier to Qualified Institutional Eligibility

## 1. Specification Metadata

**Specification ID:** L4-J-022  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Enterprise Resource Management — Vendor & Supplier Management  
**Realizes Scenario:** L3-J-022  
**Primary Business Owner:** Vendor & Supplier Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-022_Approved_Supplier_to_Qualified_Institutional_Eligibility.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP shall realize the approved supplier-qualification scenario by making scope-specific institutional eligibility observable and enforceable for an already `Active` canonical supplier.

The behavior begins when an `Active` supplier enters an authorised qualification process for a defined scope and ends when the qualification disposition, exact eligibility scope, validity, limitations, evidence basis and authority decision are available for governed downstream use.

This specification does not create supplier identity, activate a supplier, award sourcing, create a Purchase Order or contract, or transfer qualification authority to the system.

## 3. Required Business Context

- canonical supplier identity and current `Active` state;
- authorised qualification scheme and requested scope;
- pre-established criteria, evidence requirements, pass/scoring rules where applicable, validity rules and decision authority;
- current supplier approval conditions, restrictions and material risk facts;
- attributable legal, regulatory, technical, financial, quality, safety, security, sustainability, capacity and performance evidence where applicable;
- authoritative Qualification Authority decision.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-022-BH-001 | When qualification is initiated, the system shall require a canonical supplier in `Active` state and an identifiable authorised qualification scope before allowing qualification progression. | L3-J-022 §§4.1, 7, 8, 12 |
| L4-J-022-BH-002 | The system shall associate the qualification case with the applicable pre-established criteria, evidence requirements, decision method, validity rules and Qualification Authority without allowing the case to replace those governing definitions. | L3-J-022 §§7, 9 steps 3–4, 12 |
| L4-J-022-BH-003 | The system shall preserve attributable evidence and verification findings for the requested qualification scope, including validity, limitations and material authenticity results. | L3-J-022 §§9 steps 4–9, 15.2 |
| L4-J-022-BH-004 | The system shall prevent a mandatory non-waivable failure from being offset by unrelated strengths unless an upstream-authorised rule expressly permits it. | L3-J-022 §§10.2, 11.2, 12 |
| L4-J-022-BH-005 | The system shall represent qualification separately for each approved scope and shall not infer universal eligibility from supplier `Active` status or from eligibility in another scope. | L3-J-022 §§2–3, 10.1, 12 |
| L4-J-022-BH-006 | When the Qualification Authority records a decision, the system shall make the resulting qualified, conditional, returned, rejected or not-applicable disposition attributable to its exact scope, conditions, effective date and validity. | L3-J-022 §§3, 9 steps 13–16, 16 |
| L4-J-022-BH-007 | The system shall prevent downstream reliance on qualification outside its approved scope, validity, capacity or conditions. | L3-J-022 §§9 step 16, 12, 16.2 |
| L4-J-022-BH-008 | When an approved fact materially affects supplier qualification, the system shall expose the need for reassessment without deleting or overwriting the prior qualification history. | L3-J-022 §§11.7, 12, 15.3 |
| L4-J-022-BH-009 | The system shall preserve the complete qualification decision lineage so the institution can reconstruct criteria, evidence, evaluations, authority, conditions, scope and outcome. | L3-J-022 §§9 step 17, 15, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-022-BH-010 | On completion of a granted qualification, the system shall publish or otherwise make available the canonical `SupplierQualificationGranted` business fact on behalf of Vendor & Supplier Management without changing its canonical publisher. | L3-J-022 §14.1 |
| L4-J-022-BH-011 | The system shall expose current qualification scope to authorised sourcing and commitment contexts while preserving those domains' responsibility for their own event-specific evaluation and transaction lifecycles. | L3-J-022 §§4.2, 14.2, 16.2 |

## 6. Failure / Rejection / Continuation

- If supplier status is no longer `Active`, the system shall prevent qualification approval or use and retain the qualification evidence for governed follow-up.
- If required evidence is false, expired or unverifiable, the system shall prevent successful qualification for the affected scope and preserve the returned/rejected outcome.
- If evaluation conflict or inconsistent scoring remains unresolved, the system shall prevent final qualification progression until the governed review is completed.
- If capacity is double-counted across scopes, the system shall prevent approval until the capacity conflict is resolved.
- On interruption, the system shall retain the last valid business position and require time-sensitive status/evidence to be current before continuation.

**Provenance:** L3-J-022 §11.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of:

- supplier identity and `Active` status considered;
- qualification scheme, requested scope and criteria basis;
- evidence sources, validity and verification results;
- evaluator findings and conflict declarations where applicable;
- capacity and limitation findings;
- Qualification Authority decision, reason, conditions, effective/expiry dates and scope;
- publication/availability of the resulting qualification fact;
- later reassessment references without overwriting prior history.

**Provenance:** L3-J-022 §§15–16.

## 8. Observable Completion Conditions

The L4 behavior is complete when:

- the supplier has an attributable governed qualification disposition for the requested scope;
- approved eligibility, if any, has explicit scope, validity, conditions and limitations;
- rejected/returned scope remains non-eligible;
- `SupplierQualificationGranted` is available when qualification is granted;
- downstream use cannot treat qualification as universal entitlement or as an award/commitment;
- the complete qualification evidence and decision history is auditable.

## 9. Traceability & Provenance

**Parent:** L3-J-022 — Approved Supplier to Qualified Institutional Eligibility  
**Primary verification source:** L2-J02 — Vendor & Supplier Management Workflows  

Material anchors: L3-J-022 §§2–16.

## 10. Open Issues / Deferred Design

L3-J-022 retains institution-specific open issues concerning qualification schemes, category/value-band/capacity measures, external-certification equivalence, consortium capacity and the boundary between standing and event-specific qualification. L4 does not resolve those business-policy questions.

API/interface design, persistence, component design, event transport, technical IAM/RBAC, retry/transaction mechanics, infrastructure and UI/UX remain deferred to later architecture levels.
