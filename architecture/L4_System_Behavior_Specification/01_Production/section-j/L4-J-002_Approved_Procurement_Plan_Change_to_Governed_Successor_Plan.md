# Approved Procurement Plan Change to Governed Successor Plan

## 1. Specification Metadata

**Specification ID:** L4-J-002  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Procurement Planning & Sourcing  
**Realizes Scenario:** L3-J-002  
**Primary Business Owner:** Procurement Planning & Sourcing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-002_Approved_Procurement_Plan_Change_to_Governed_Successor_Plan.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern a material change to an approved or active Procurement Plan through a controlled successor version while preserving the prior approved version as immutable historical truth.

The behavior begins when authoritative business evidence establishes that the effective plan requires material revision. It ends when the exact successor version is approved and becomes effective, with predecessor-successor lineage and every changed planned-requirement disposition preserved.

This behavior does not rewrite downstream requisitions, sourcing records, Purchase Orders, financial records, or the prior approved plan.

---

## 3. Required Business Context

Required context includes:

- unique approved or active Procurement Plan;
- exact effective predecessor version;
- immutable predecessor approval evidence;
- material revision reason and attributable evidence;
- affected planned requirements;
- applicable revision, review, approval, and effective-date rules;
- identifiable downstream references for impact assessment.

J01 owns the successor plan and version relationship. External organisational, item, specialist, downstream procurement, and Finance facts retain their canonical owners.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-002-BH-001 | When revision begins, the system shall identify the exact effective predecessor plan version. | L3-J-002 §§7-9 |
| L4-J-002-BH-002 | The system shall record the revision reason, initiating actor, supporting evidence, requested effective timing, and affected planned requirements. | L3-J-002 §§3, 9, 15 |
| L4-J-002-BH-003 | The system shall prevent a planning revision from directly rewriting downstream authoritative records. | L3-J-002 §§4.4, 9, 12 |
| L4-J-002-BH-004 | The system shall create a controlled successor linked explicitly to its direct predecessor while leaving the predecessor unchanged. | L3-J-002 §§3, 9, 12 |
| L4-J-002-BH-005 | For each material planned-requirement change, the system shall preserve prior value or disposition, successor value or disposition, reason, source evidence, and requirement identity. | L3-J-002 §§9, 12, 15 |
| L4-J-002-BH-006 | Consolidation, separation, reduction, deferral, removal, or addition shall preserve originating requirement and unit traceability. | L3-J-002 §§9-10, 12 |
| L4-J-002-BH-007 | On submission, the system shall preserve the exact successor version as the controlled review baseline. | L3-J-002 §§9, 11 |
| L4-J-002-BH-008 | A returned or unapproved successor shall not replace the currently effective plan. | L3-J-002 §§10-12 |
| L4-J-002-BH-009 | The system shall require approval against the exact successor version before it can become effective. | L3-J-002 §§6, 9, 12, 15 |
| L4-J-002-BH-010 | A future-dated approved successor shall not become current before its governed effective date. | L3-J-002 §§10, 12 |
| L4-J-002-BH-011 | When the successor becomes effective, the predecessor shall remain immutable and historically valid for its former effective period. | L3-J-002 §§3, 9, 12-13 |
| L4-J-002-BH-012 | If a later material error is found in an effective successor, the system shall require another governed successor rather than rewrite history. | L3-J-002 §11.7 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-002-BH-013 | When a controlled successor is created, the system shall publish `ProcurementPlanRevisionCreated`. | L3-J-002 §14 |
| L4-J-002-BH-014 | When the successor enters governed review, the system shall publish `ProcurementPlanSubmitted`. | L3-J-002 §14 |
| L4-J-002-BH-015 | When the successor is returned for correction, the system shall publish `ProcurementPlanReturnedForRevision`. | L3-J-002 §14 |
| L4-J-002-BH-016 | When the exact successor version is approved, the system shall publish `ProcurementPlanRevisionApproved`. | L3-J-002 §14 |
| L4-J-002-BH-017 | When the approved successor becomes effective, the system shall publish `ProcurementPlanActivated`. | L3-J-002 §14 |

No separate successor-effective or predecessor-superseded event is invented.

Successor preparation/review positions described in L3-J-002 are business progression markers and shall not be promoted into new canonical Procurement Plan lifecycle states.

---

## 6. Failure / Rejection / Continuation

A stale predecessor baseline, incomplete change traceability, invalid approval authority, competing successor proposals, or unresolved downstream-reference conflict shall prevent the successor from becoming effective.

The current effective plan remains authoritative until one valid successor completes approval and reaches its governed effective date.

No technical concurrency, retry, or rollback behavior is defined.

---

## 7. Audit & Evidence Behavior

The system shall preserve:

- predecessor and successor identities;
- complete change set;
- revision reason and evidence;
- changed planned-requirement dispositions;
- review and return history;
- approving actor, authority, decision, timestamp, and conditions;
- proposed and actual effective date;
- downstream reference impacts;
- complete predecessor-successor chain.

---

## 8. Observable Completion Conditions

The behavior is complete when:

1. the successor is linked to the exact predecessor;
2. each material change is reconstructable;
3. the exact successor version is validly approved;
4. the successor becomes effective only at its governed effective date;
5. the predecessor remains immutable;
6. downstream authoritative records remain under their owners.

---

## 9. Traceability & Provenance

**Parent:** L3-J-002 — Approved Procurement Plan Change to Governed Successor Plan  
**Primary verification source:** L2-J01 — Procurement Planning & Sourcing Workflows  
**Governance source:** L2-J00 — Enterprise Resource Management Architecture & Governance  

Parent readiness confirmed against the active 1,084-parent Locked L4 baseline.

---

## 10. Open Issues / Deferred Design

L2-J01 does not define a canonical distinction between non-material administrative correction and controlled material revision. This L4 does not invent that distinction.

API/interface design, persistence/versioning implementation, technical concurrency, document storage, event transport, infrastructure, and UI/UX remain deferred.

---

## Canonical Validation Note

This file is a Draft. It requires self-review and independent review before Architecture Approval.
