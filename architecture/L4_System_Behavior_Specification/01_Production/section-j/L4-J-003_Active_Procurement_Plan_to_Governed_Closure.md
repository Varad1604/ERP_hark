# Active Procurement Plan to Governed Closure

## 1. Specification Metadata

**Specification ID:** L4-J-003  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Procurement Planning & Sourcing  
**Realizes Scenario:** L3-J-003  
**Primary Business Owner:** Procurement Planning & Sourcing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-003_Active_Procurement_Plan_to_Governed_Closure.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern formal closure of an active Procurement Plan while preserving every plan version, planned-requirement disposition, planned-versus-actual reference, approval, and closure fact as historical evidence.

The behavior begins when an `Active` Procurement Plan reaches its institutionally governed completion point or another authorised closure condition. It ends when closure readiness is approved and the plan reaches the upstream-governed `Completed / Closed` outcome.

This behavior does not close or alter downstream requisitions, sourcing events, Purchase Orders, receipts, inventory, assets, financial commitments, payables, or payments.

---

## 3. Required Business Context

Required context includes:

- active Procurement Plan;
- exact effective version and full predecessor-successor chain;
- applicable closure condition and authority;
- planned requirements and final planning dispositions;
- downstream references where authoritative facts exist;
- applicable exception and retention policy.

J01 owns the plan closure and planning interpretation. Downstream domains retain ownership of their own facts.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-003-BH-001 | When closure review begins, the system shall identify the active plan, scope, period, effective version, and applicable closure condition. | L3-J-003 §§7-9 |
| L4-J-003-BH-002 | The system shall preserve the complete predecessor-successor version chain as closure evidence. | L3-J-003 §§3, 9, 12 |
| L4-J-003-BH-003 | The system shall require every planned requirement to have a final planning disposition or an explicitly authorised unresolved exception. | L3-J-003 §§4.2-4.3, 9, 12 |
| L4-J-003-BH-004 | Where downstream facts exist, the system shall preserve references to them without changing their ownership or lifecycle. | L3-J-003 §§5, 9, 12, 14 |
| L4-J-003-BH-005 | Planned-versus-actual evidence shall preserve original planned values and shall not rewrite them to match actual outcomes. | L3-J-003 §§9, 12 |
| L4-J-003-BH-006 | Carry-forward shall remain a closure disposition/reference and shall not automatically create or approve a successor-plan requirement. | L3-J-003 §§9, 10, 12 |
| L4-J-003-BH-007 | The system shall prevent closure when a material plan revision is being bypassed or a required disposition remains ungoverned. | L3-J-003 §§7, 9, 11-12 |
| L4-J-003-BH-008 | On closure submission, the system shall preserve the exact closure package as the controlled review baseline. | L3-J-003 §§9, 15 |
| L4-J-003-BH-009 | A returned or interrupted closure review shall leave the Procurement Plan `Active`. | L3-J-003 §§10-11, 13 |
| L4-J-003-BH-010 | The system shall require valid closure approval against the exact reviewed closure package. | L3-J-003 §§6, 9, 15 |
| L4-J-003-BH-011 | On approved closure, the system shall establish the upstream-governed `Completed / Closed` outcome without silently choosing a different lifecycle term. | L3-J-003 §§4.2, 9, 13, 17 |
| L4-J-003-BH-012 | After closure, the system shall prevent ordinary planning change against the closed plan while preserving historical evidence. | L3-J-003 §§9, 12, 16 |
| L4-J-003-BH-013 | A material error found after closure shall not cause destructive rewrite of the closed plan. | L3-J-003 §11.7 |

---

## 5. Cross-Domain & Event Behavior

When governed closure completes, the system shall publish `ProcurementPlanClosed` with Procurement Planning & Sourcing as canonical publisher.

`ProcurementPlanClosed` shall not imply closure of linked requisitions, sourcing events, Purchase Orders, receipts, inventory, assets, payables, payments, or other downstream records.

Downstream facts are consumed as references only.

---

## 6. Failure / Rejection / Continuation

Incomplete version history, undispositioned planned requirements, conflicting downstream evidence, invalid closure authority, interrupted review, or an unresolved pending revision shall prevent closure.

The plan remains `Active` until the business issue is governed.

No technical archival, retry, rollback, or recovery behavior is defined.

---

## 7. Audit & Evidence Behavior

The system shall preserve:

- plan identity, scope, period, and final effective version;
- complete version/revision chain;
- final disposition of every planned requirement;
- planned-versus-actual references and variances;
- carried-forward, deferred, cancelled, open, or exceptional positions;
- reviewer and approver identity, authority, decision, and timestamp;
- accepted exceptions and conditions;
- closure package and `ProcurementPlanClosed`;
- evidence that no historical version or requirement was destructively rewritten.

---

## 8. Observable Completion Conditions

The behavior is complete when:

1. the active plan and final effective version are identified;
2. the full approved version chain is preserved;
3. every planned requirement has a governed closure disposition or authorised exception;
4. planned-versus-actual references are traceable where available;
5. the exact closure package has valid approval;
6. the plan has reached the upstream `Completed / Closed` outcome;
7. `ProcurementPlanClosed` is published;
8. ordinary planning change is prevented;
9. downstream records remain independently owned and unchanged.

---

## 9. Traceability & Provenance

**Parent:** L3-J-003 — Active Procurement Plan to Governed Closure  
**Primary verification source:** L2-J01 — Procurement Planning & Sourcing Workflows  
**Governance source:** L2-J00 — Enterprise Resource Management Architecture & Governance  

Parent readiness confirmed against the active 1,084-parent Locked L4 baseline.

---

## 10. Open Issues / Deferred Design

L2-J01 uses `Completed / Closed` without defining whether these are alternative labels or distinct sequential states. This ambiguity remains upstream and is intentionally not resolved at L4.

The parent also records that no canonical supplemental post-closure correction scenario currently exists. This L4 does not invent one.

API/interface design, persistence, archival/retention implementation, reporting implementation, event transport, infrastructure, and UI/UX remain deferred.

---

## Canonical Validation Note

This file is a Draft. It requires self-review and independent review before Architecture Approval.
