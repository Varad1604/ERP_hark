# Approved Demand to Rate-Contract Sourcing Outcome

## 1. Specification Metadata

**Specification ID:** L4-J-012  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Procurement Planning & Sourcing  
**Realizes Scenario:** L3-J-012  
**Primary Business Owner:** Procurement Planning & Sourcing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-012_Approved_Demand_to_Rate_Contract_Sourcing_Outcome.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`  
**Referenced By:** Future downstream L5 design  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern use of an Active Rate Contract as the sourcing route for approved procurement demand by validating exact demand fit, supplier eligibility, contract version, rate, validity, ceiling, call-off rule, and residual position before publishing an approved supplier-specific sourcing outcome.

The behavior begins when valid approved demand is submitted to J01 and an Active Rate Contract is identified as a possible authorised sourcing route. It ends when the exact call-off sourcing recommendation is approved and finalised, or when the demand is returned, deferred, rejected, or redirected because the contract cannot validly cover it.

This specification does not establish or amend a Rate Contract, alter J03-owned demand, change supplier-master state, create a Purchase Order, or create receipt, payable, accounting, or payment outcomes.

---

## 3. Required Business Context

- J03-approved procurement demand with remaining unsourced quantity or scope.
- The exact J01 Rate Contract version in `Active` at the proposed sourcing date.
- Controlled contract scope/catalogue, supplier positions, rates, validity, applicability, ceilings, balances, and call-off rules.
- Current J02 supplier eligibility and contract-relevant supplier status.
- Demand specification, unit, location, required date, quantity, and conditions sufficient for contract matching.
- Applicable budget reference from Finance without transfer of financial ownership.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-012-BH-001 | When approved demand is submitted for rate-contract sourcing, the system shall recognise that sourcing route only for J03-approved demand with remaining unsourced quantity or scope. | L3-J-012 §§4.1, 7, 8, 9.1, 12 |
| L4-J-012-BH-002 | The system shall identify the exact `Active` Rate Contract version and catalogue/scope line proposed to cover the demand. | L3-J-012 §§7, 9.2–9.3, 12 |
| L4-J-012-BH-003 | Before recommendation, the system shall require the approved demand to match the contract's covered scope/specification, unit, location, timing, quantity conditions, service level, delivery, warranty, and other applicable conditions. | L3-J-012 §§9.4, 11.2, 12 |
| L4-J-012-BH-004 | Before recommendation, the system shall require current J02 supplier eligibility and a contract position that remains valid for the proposed sourcing time. | L3-J-012 §§7, 9.5, 11.1, 12 |
| L4-J-012-BH-005 | The system shall determine the applicable approved rate, taxes/charges, discount, indexation or adjustment from the correct controlled contract version and effective date while preserving the historical contract terms. | L3-J-012 §§9.6, 10.4, 12 |
| L4-J-012-BH-006 | Before approval, the system shall verify that the proposed sourcing quantity or value remains within the applicable supplier, line, period, and aggregate ceilings and the authoritative business balance. | L3-J-012 §§9.7, 10.6, 11.3, 12 |
| L4-J-012-BH-007 | When multiple suppliers or contract positions are applicable, the system shall apply the authoritative call-off rule, such as rank, rotation, allocation, location, primary-secondary, choice, or mini-competition, without substituting requester preference for that rule. | L3-J-012 §§9.8–9.9, 10.2, 11.5, 12 |
| L4-J-012-BH-008 | When the Rate Contract requires mini-competition, the system shall limit participation and evaluation to the eligible contract suppliers and controlled mini-competition rules established by the Rate Contract. | L3-J-012 §9.9 |
| L4-J-012-BH-009 | The system shall record the recommended supplier, exact Rate Contract version/line, quantity, location, delivery, rate reference, evaluated value, conditions, utilisation impact, and residual demand required by the parent outcome. | L3-J-012 §§9.10–9.11, 15.2 |
| L4-J-012-BH-010 | When a Rate Contract covers only part of the approved demand, the system shall keep the uncovered quantity or scope explicit and direct it only to a source-supported alternative sourcing disposition. | L3-J-012 §§10.3, 10.6–10.7, 12 |
| L4-J-012-BH-011 | The system shall prevent duplicate sourcing of the same approved demand quantity within the J01 sourcing outcome and shall preserve the J03-owned demand as an authoritative external reference rather than changing its canonical lifecycle. | L3-J-012 §§7, 11.4, 12, 13 |
| L4-J-012-BH-012 | When the Procurement Approver approves the exact recommendation, the system shall recognise the governed supplier-specific rate-contract sourcing outcome without creating a Purchase Order or supplier commitment. | L3-J-012 §§4.2, 9.12–9.14, 12, 16 |
| L4-J-012-BH-013 | On approved sourcing completion, the system shall record the rate-contract sourcing utilisation and resulting available business balance without exceeding the governed contract ceiling. | L3-J-012 §§9.11, 16.1–16.2 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-012-BH-014 | When the applicable J01 sourcing activity is created, the system shall publish `SourcingEventCreated` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-012 §14.1; L2-J01 §35 |
| L4-J-012-BH-015 | When the governed sourcing method for the demand is determined, the system shall publish `ProcurementMethodDetermined` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-012 §14.1; L2-J01 §35 |
| L4-J-012-BH-016 | When the authoritative recommendation is approved, the system shall publish `AwardRecommendationApproved` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-012 §14.1; L2-J01 §35 |
| L4-J-012-BH-017 | When the approved rate-contract sourcing outcome is finalised, the system shall publish `ApprovedSourcingOutcomeFinalised` with Procurement Planning & Sourcing (J01) as the canonical publisher. | L3-J-012 §14.1; L2-J01 §35 |

J03 retains approved-demand ownership, J02 retains supplier truth, Finance retains monetary truth, and J04 alone creates Purchase Order commitment.

---

## 6. Failure / Rejection / Continuation

- Invalid or unapproved demand, an inactive/expired Rate Contract version, a suspended/ineligible supplier, or a closed/inapplicable contract line shall prevent rate-contract sourcing from progressing.
- Specification, unit, scope, location, timing, or condition mismatch shall prevent contract use unless an authoritative upstream correction occurs; the system shall not invent substitution or conversion authority.
- A quantity/value that would exceed a governed ceiling or available business balance shall not be approved beyond that limit; residual demand remains explicit.
- A concurrent utilisation conflict that could duplicate demand or consume the same contract balance shall keep the recommendation unapproved pending governed reconciliation.
- An incorrect rank, rotation, allocation, location, or mini-competition result shall be returned for source-supported correction and re-review.
- An approved outcome shall not be silently edited. A material post-approval change requires the parent-supported successor recommendation or cancellation path before downstream commitment.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- J03 demand identity, approval, remaining quantity/scope, location, required date, and budget reference;
- exact Rate Contract identity/version/line, supplier, applicability, validity, rate, taxes/charges, ceiling, and available business balance;
- current supplier eligibility and the call-off rule applied;
- mini-competition, rate adjustment, exception, partial-coverage, and residual-demand evidence where applicable;
- recommendation, review, approval/return/rejection/redirect decision, actor, authority, reason, and correlation to source demand;
- rate-contract sourcing utilisation and resulting business balance;
- the canonical J01 events published for the sourcing outcome.

---

## 8. Observable Completion Conditions

Successful completion is observable only when:

1. the approved demand is traceable to an `Active`, applicable Rate Contract version and exact contract line;
2. supplier eligibility, demand fit, rate/effective date, validity, ceilings, business balance, and call-off rule all support the recommendation;
3. any partial coverage or residual demand is explicit;
4. the Procurement Approver has approved the exact supplier-specific recommendation;
5. the sourcing utilisation and resulting contract business balance are recorded within governed limits;
6. `AwardRecommendationApproved` and `ApprovedSourcingOutcomeFinalised` are published as applicable J01 facts;
7. the finalised outcome is available to J04 for downstream commitment without this scenario creating a Purchase Order, liability, receipt, or payment.

Returned, deferred, rejected, or redirected demand is a valid terminal alternative where the parent supports it and does not constitute successful rate-contract sourcing.

---

## 9. Traceability & Provenance

**Parent:** L3-J-012 — Approved Demand to Rate-Contract Sourcing Outcome  
**Primary verification source:** L2-J01 — Procurement Planning & Sourcing Workflows  
**Additional verification sources:** L2-J00 — Enterprise Resource Management Architecture & Governance; L2-J02 — Vendor & Supplier Management Workflows; L2-J03 — Purchase Requisition & Approval Workflows; locked L1 Business Domain Architecture.  

Every BH is anchored in the Locked parent. L2 sources verify J01 sourcing ownership, J02 supplier ownership, J03 demand ownership, the applicable Sourcing Event lifecycle, and exact canonical J01 event names.

---

## 10. Open Issues / Deferred Design

- Institution-specific rules for delivery timing versus contract validity, supplier choice, rotation, rank, mini-competition, ceiling reservation, and cancellation remain upstream-governed and are not hardened at L4.
- The parent records an unresolved boundary between J01 sourcing utilisation and J04 committed utilisation. This L4 records only J01 sourcing utilisation and does not create or infer J04 commitment.
- The parent contains both a J03-owned demand boundary with unchanged demand lifecycle and completion language referring to a demand item's sourcing status. This L4 preserves J03 lifecycle ownership and records the J01 sourcing disposition without inventing a J03 state transition.
- The parent lists `Section_J_Canonical_Register.md` as a dependency, but that exact file was not retrievable through the current repository search. No L4 behavior has been derived from an assumed replacement; this is retained as a repository verification finding.
- API/interface design, persistence/data model, component/service design, event transport, technical access control, transaction/retry mechanics, infrastructure, and UI/UX remain deferred to L5/L6.
