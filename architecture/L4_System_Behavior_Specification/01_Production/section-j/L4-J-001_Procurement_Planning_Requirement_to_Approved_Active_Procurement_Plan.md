# Procurement Planning Requirement to Approved Active Procurement Plan

## 1. Specification Metadata

**Specification ID:** L4-J-001  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Procurement Planning & Sourcing  
**Realizes Scenario:** L3-J-001  
**Primary Business Owner:** Procurement Planning & Sourcing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-001_Procurement_Planning_Requirement_to_Approved_Active_Procurement_Plan.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall establish an approved and active Procurement Plan for a defined institutional scope and planning period while preserving the distinction between planning information and authorised procurement demand or financial commitment.

The behavior begins when an authorised procurement-planning requirement exists for a valid period and organisational scope. It ends when the exact reviewed plan version is validly approved and becomes `Active`.

This behavior does not create a Purchase Requisition, approved procurement demand, sourcing outcome, supplier commitment, Purchase Order, budget commitment, expenditure, payable, or payment.

---

## 3. Required Business Context

Required context includes:

- valid planning period;
- valid organisational scope;
- applicable procurement-planning policy;
- planned requirement information and authoritative external references where used;
- configured review and approval authority;
- Procurement Planner, Procurement Manager, and Procurement Approver responsibilities.

J01 owns the Procurement Plan and planned requirements. Organisational, item, specification, supplier, and Finance references remain externally owned.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-001-BH-001 | When planning begins, the system shall establish a unique Procurement Plan identity, planning period, organisational scope, and initial controlled version. | L3-J-001 §§3, 7-9 |
| L4-J-001-BH-002 | The system shall permit planned requirements to be recorded with traceable identity and available planning context. | L3-J-001 §§3, 9 |
| L4-J-001-BH-003 | The system shall preserve estimated quantity, timing, priority, cost, specification, method, budget, or item references as planning information only. | L3-J-001 §§2, 9, 12 |
| L4-J-001-BH-004 | Planned information shall not itself become authorised procurement demand, financial commitment, expenditure, or supplier commitment. | L3-J-001 §§2-4, 12-13 |
| L4-J-001-BH-005 | Where planned requirements are consolidated, the system shall preserve traceability to each originating requirement and unit. | L3-J-001 §§9, 10, 12 |
| L4-J-001-BH-006 | Before submission, the system shall expose material period, scope, completeness, duplication, conflict, evidence, and policy deficiencies for correction. | L3-J-001 §§9, 11 |
| L4-J-001-BH-007 | On submission, the system shall preserve the exact submitted plan version as the review baseline. | L3-J-001 §§9, 12-13 |
| L4-J-001-BH-008 | A returned plan shall retain the submitted version, return reason, and return authority and shall remain incomplete until corrected and resubmitted. | L3-J-001 §§4.2, 10, 12 |
| L4-J-001-BH-009 | The system shall require valid institutional approval against the exact reviewed version before activation. | L3-J-001 §§6, 9, 12, 15 |
| L4-J-001-BH-010 | The version approved shall be the version activated. | L3-J-001 §§9, 12, 15-16 |
| L4-J-001-BH-011 | On activation, the system shall make the active plan and planned requirements available as planning references without bypassing J03 procurement-demand approval. | L3-J-001 §§9, 12-13, 16 |
| L4-J-001-BH-012 | An approved or active plan shall not be silently overwritten. | L3-J-001 §§11.7, 12 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-001-BH-013 | When a controlled plan identity and draft version are established, the system shall publish `ProcurementPlanCreated` with Procurement Planning & Sourcing as canonical publisher. | L3-J-001 §14 |
| L4-J-001-BH-014 | When an anticipated requirement is added, the system shall publish `PlannedRequirementAdded` where the canonical business condition occurs. | L3-J-001 §14 |
| L4-J-001-BH-015 | When a plan enters governed review, the system shall publish `ProcurementPlanSubmitted`. | L3-J-001 §14 |
| L4-J-001-BH-016 | When a plan is returned for correction, the system shall publish `ProcurementPlanReturnedForRevision`. | L3-J-001 §14 |
| L4-J-001-BH-017 | When the exact reviewed version receives valid approval, the system shall publish `ProcurementPlanApproved`. | L3-J-001 §14 |
| L4-J-001-BH-018 | When the approved plan becomes active, the system shall publish `ProcurementPlanActivated`. | L3-J-001 §14 |

Consumed organisational, specialist, item/category, Finance, and document references shall not transfer ownership to J01.

---

## 6. Failure / Rejection / Continuation

Invalid period/scope, incomplete planned requirements, duplicate/conflicting planning entries, unresolved approval authority, or interrupted review shall prevent activation.

A plan returned for revision shall remain incomplete until a corrected controlled version is resubmitted and approved.

If approval exists but a business condition prevents activation, the system shall retain the approved inactive position until the condition is resolved. No technical retry or rollback behavior is defined.

---

## 7. Audit & Evidence Behavior

The system shall preserve:

- plan identity and version;
- planning period and organisational scope;
- planned requirement origins and consolidation relationships;
- submission baseline;
- review and return evidence;
- approval actor, authority, decision, timestamp, and conditions;
- activation evidence;
- external references used for planning;
- complete lifecycle chronology.

Historical approval and lifecycle evidence shall not be destructively rewritten.

---

## 8. Observable Completion Conditions

The behavior is complete when:

1. a unique Procurement Plan exists for the intended planning context;
2. planning period and scope are valid;
3. the effective version is identifiable;
4. planned requirements are traceable;
5. material review issues are resolved or governed;
6. the exact reviewed version is approved;
7. that same version is `Active`;
8. downstream procurement demand has not been implicitly approved.

---

## 9. Traceability & Provenance

**Parent:** L3-J-001 — Procurement Planning Requirement to Approved Active Procurement Plan  
**Primary verification source:** L2-J01 — Procurement Planning & Sourcing Workflows  
**Governance source:** L2-J00 — Enterprise Resource Management Architecture & Governance  

Parent readiness confirmed against the active 1,084-parent Locked L4 baseline.

---

## 10. Open Issues / Deferred Design

No material L4 blocker remains for this scenario.

API/interface design, persistence, document storage, technical access control, notification delivery, infrastructure, and UI/UX remain deferred.

---

## Canonical Validation Note

This file is a Draft. It requires self-review and independent review before Architecture Approval.
