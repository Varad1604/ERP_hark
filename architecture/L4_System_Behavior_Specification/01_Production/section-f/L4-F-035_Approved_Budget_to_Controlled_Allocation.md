# Approved Budget to Controlled Allocation

## 1. Specification Metadata

**Specification ID:** L4-F-035
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Budget & Cost Center Management
**Realizes Scenario:** L3-F-035
**Primary Business Owner:** Budget & Cost Center Management
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-02
**Depends On:** `L3-F-035_Approved_Budget_to_Controlled_Allocation.md`; `L2-F06_Budget_and_Cost_Center_Management.md`; `L2-B02_Organisation_Workflows.md`
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

SutraERP shall establish controlled Finance-owned Budget allocations under an Active Budget only when each target Cost Center is valid, the distribution is fully balanced to the approved Budget total, any reserve is assigned to the Central Reserve Cost Center, and governed authorization exists. The behavior ends with controlled allocations and the canonical allocation events.

This specification does not create organisational units or Cost Centers, approve the parent Budget, perform utilization, procurement, payment, or accounting posting, or prescribe implementation.

---

## 3. Required Business Context

The behavior requires:

- an `Active` Budget;
- the approved Budget total and planning scope;
- authoritative active Cost Center references;
- the Central Reserve Cost Center rule for any unallocated reserve;
- a governed allocation authorization;

No source-owned fact or business authority is transferred to L4 by its use in this specification.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-035-BH-001 | The system shall permit controlled allocation only against a Budget in `Active` and only when every target Cost Center reference is valid and active. | L3-F-035 §§7, 9 step 1, 11 |
| L4-F-035-BH-002 | The system shall prevent allocation completion unless the total authorized allocations exactly equal the approved Budget `TotalAmount`. | L3-F-035 §§9 step 2, 11-12, 16 |
| L4-F-035-BH-003 | When an amount would otherwise remain unallocated, the system shall assign that reserve to the source-defined Central Reserve Cost Center before allocation completion. | L3-F-035 §§9 step 3, 12 |
| L4-F-035-BH-004 | The system shall require the governed allocation authorization before establishing the controlled allocations. | L3-F-035 §§9 step 4, 15 |
| L4-F-035-BH-005 | On authorized completion, the system shall establish the Finance-owned controlled allocations while retaining authoritative organisational references rather than duplicating Section B structure. | L3-F-035 §§3, 5, 9 step 5, 12-13 |
| L4-F-035-BH-006 | On controlled allocation creation, the system shall publish `BudgetAllocationCreated` with Budget & Cost Center Management as canonical publisher. | L3-F-035 §14.1 |
| L4-F-035-BH-007 | For each funded Cost Center outcome supported by the allocation, the system shall publish `CostCenterFunded` with Budget & Cost Center Management as canonical publisher. | L3-F-035 §14.1 |
| L4-F-035-BH-008 | The system shall preserve the approved Budget, Cost Center references, balanced allocation schedule, Central Reserve treatment, and governed allocation decision as attributable evidence. | L3-F-035 §15.3 |
| L4-F-035-BH-009 | Completion of controlled allocation shall not create procurement, payment, utilization, or accounting-posting outcomes. | L3-F-035 §4.4 |

---

## 5. Cross-Domain & Event Behavior

Section B provides authoritative organisational references only. Budget & Cost Center Management owns the allocation outcome and publishes `BudgetAllocationCreated` and `CostCenterFunded`; consuming domains do not acquire Budget ownership.

No event transport, payload, queue, delivery, retry, or technical integration behavior is defined at L4.

---

## 6. Failure / Rejection / Continuation

An invalid or inactive Cost Center shall prevent the affected allocation from completing. An allocation schedule that does not equal the approved Budget total shall remain incomplete until corrected and authorized.

Technical retry, rollback, transaction compensation, exception-handler, and infrastructure-recovery mechanics are outside L4.

---

## 7. Audit & Evidence Behavior

Preserve the parent Budget identity and total, authoritative Cost Center references, allocation amounts, Central Reserve treatment, authorization outcome, and published allocation facts.

Section A retains ownership of Authorization and Audit infrastructure. The business-domain owners identified by the parent retain ownership of the underlying business decisions and evidence.

---

## 8. Observable Completion Conditions

Every allocation references an active Cost Center, the aggregate allocation total equals the approved Budget total, required reserve handling is complete, and the applicable canonical allocation events have been published.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-035` — Approved Budget to Controlled Allocation
**Primary verification source:** L2-F06 — Budget & Cost Center Management
**Additional verification:** L2-B02 — Organisation Workflows; locked L1 ownership constraints.

Every BH above is anchored to the approved Locked L3 parent. Higher-level/L2 sources are used only to verify exact ownership, event, rule, lifecycle, or dependency semantics; no material L3 omission is silently repaired.

---

## 10. Open Issues / Deferred Design

No named institutional allocation approver is established upstream. L4 requires the authoritative governed authorization outcome without naming one. Implementation details remain deferred.

---

