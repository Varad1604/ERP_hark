# Active Budget to Authorized Revision or Virement

## 1. Specification Metadata

**Specification ID:** L4-F-034
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Budget & Cost Center Management
**Realizes Scenario:** L3-F-034
**Primary Business Owner:** Budget & Cost Center Management
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-02
**Depends On:** `L3-F-034_Active_Budget_to_Authorized_Revision_or_Virement.md`; `L2-F06_Budget_and_Cost_Center_Management.md`; `L2-B02_Organisation_Workflows.md`
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

SutraERP shall apply an authorized revision or virement to an Active Budget while preserving Budget bounds, grant ring-fencing, independent maker-checker governance, prior versions, and Finance ownership. The behavior begins with a governed change request against an Active Budget and ends when the authorized successor version is effective and the applicable canonical event is published, or when the request is rejected with the Active Budget unchanged.

This specification does not define named maker/checker roles, initial Budget approval, ordinary allocation creation, accounting correction, procurement execution, Budget closure, or implementation design.

---

## 3. Required Business Context

The behavior requires:

- an `Active` Budget;
- the affected Budget Period, allocations and Cost Centers;
- a business justification;
- source-defined Budget bounds, ring-fencing and virement constraints;
- independent maker-checker authorization;

No source-owned fact or business authority is transferred to L4 by its use in this specification.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-034-BH-001 | The system shall accept a revision or virement decision only for a Budget that is in the source-defined `Active` position and whose affected references and business justification are available. | L3-F-034 §§7-9 step 1 |
| L4-F-034-BH-002 | Before authorizing the change, the system shall evaluate the source-defined Budget bounds, grant ring-fencing, and virement constraints. | L3-F-034 §9 step 2; §12 |
| L4-F-034-BH-003 | For an Active-Budget structural change, the system shall require evidence that the initiating and approving responsibilities are independent before applying the change. | L3-F-034 §§4.3, 9 step 3, 12, 15 |
| L4-F-034-BH-004 | For a virement, the system shall prevent completion when the change would alter the parent Budget `TotalAmount`. | L3-F-034 §§10, 12 |
| L4-F-034-BH-005 | The system shall prevent a grant Budget virement that would violate the source-defined ring-fencing restriction. | L3-F-034 §§11-12 |
| L4-F-034-BH-006 | When the change is authorized, the system shall apply it as a preserved successor Budget version while retaining the Budget in `Active`. | L3-F-034 §§3, 9 step 4, 13, 16 |
| L4-F-034-BH-007 | When the authorized outcome is a revision, the system shall publish `BudgetRevised` with Budget & Cost Center Management as canonical publisher. | L3-F-034 §14.1 |
| L4-F-034-BH-008 | When the authorized outcome is a virement, the system shall publish `BudgetVirementApproved` with Budget & Cost Center Management as canonical publisher. | L3-F-034 §14.1 |
| L4-F-034-BH-009 | When the request is rejected, the system shall leave the Active Budget unchanged and preserve the rejection evidence without inventing a new lifecycle state. | L3-F-034 §11 |
| L4-F-034-BH-010 | The system shall preserve the change request, justification, prior and successor versions, independent decision evidence, and applicable canonical event as attributable business evidence. | L3-F-034 §15.3 |

---

## 5. Cross-Domain & Event Behavior

Section B supplies authoritative organisational context only. Budget & Cost Center Management retains canonical ownership of the Budget and Cost Center context. Organisational references are consumed without duplication or ownership transfer. `BudgetRevised` and `BudgetVirementApproved` remain distinct canonical Budget & Cost Center Management publications.

No event transport, payload, queue, delivery, retry, or technical integration behavior is defined at L4.

---

## 6. Failure / Rejection / Continuation

A rejected request leaves the Active Budget unchanged. Any change that violates Budget bounds or grant ring-fencing shall be prevented until the governed request is corrected or rejected. No new lifecycle position is created for rejection.

Technical retry, rollback, transaction compensation, exception-handler, and infrastructure-recovery mechanics are outside L4.

---

## 7. Audit & Evidence Behavior

Preserve request identity, business justification, affected references, applicable constraints, independence evidence for maker-checker governance, prior and successor versions, decision outcome, and the applicable canonical event.

Section A retains ownership of Authorization and Audit infrastructure. The business-domain owners identified by the parent retain ownership of the underlying business decisions and evidence.

---

## 8. Observable Completion Conditions

Completion requires the authorized structural change to be reflected in the Active Budget, prior versions to remain preserved, and the applicable canonical event to be published. Rejection is observable as no Budget change plus preserved rejection evidence.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-034` — Active Budget to Authorized Revision or Virement
**Primary verification source:** L2-F06 — Budget & Cost Center Management
**Additional verification:** L2-B02 — Organisation Workflows; locked L1 Finance and organisation ownership rules.

Every BH above is anchored to the approved Locked L3 parent. Higher-level/L2 sources are used only to verify exact ownership, event, rule, lifecycle, or dependency semantics; no material L3 omission is silently repaired.

---

## 10. Open Issues / Deferred Design

L2/L3 do not name the institutional maker/checker roles. L4 preserves the required independence control without assigning roles. Implementation details remain deferred.

---

