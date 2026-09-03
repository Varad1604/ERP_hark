# Budget Proposal to Approved Active Budget

## 1. Specification Metadata

**Specification ID:** L4-F-033
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Budget & Cost Center Management
**Realizes Scenario:** L3-F-033
**Primary Business Owner:** Budget & Cost Center Management
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-02
**Depends On:** `L3-F-033_Budget_Proposal_to_Approved_Active_Budget.md`; `L2-F06_Budget_and_Cost_Center_Management.md`; `L2-B02_Organisation_Workflows.md`
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern the system behavior by which a prepared Budget Proposal is admitted to approval, receives an authoritative governed decision, and becomes an Active Budget only when that decision supports activation. The behavior begins when a prepared proposal is submitted and ends with either an Active Budget with `BudgetApproved` published or a governed rejection returned to the source-defined Draft position.

This specification does not own institutional approval authority, organisational structure, controlled allocation, Budget revision/virement, actual accounting, expenditure, or any implementation mechanism.

---

## 3. Required Business Context

The behavior requires:

- a valid Budget Period;
- authoritative Section B organisational references;
- the planned allocations required for approval review;
- the source-defined Budget lifecycle positions `Draft`, `PendingApproval`, and `Active`;
- an authoritative governed approval or rejection decision;

No source-owned fact or business authority is transferred to L4 by its use in this specification.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-033-BH-001 | The system shall prevent Budget approval review from progressing when the Budget Period, required organisational references, or required planned allocations are invalid or incomplete. | L3-F-033 §§7, 9 step 1, 11 |
| L4-F-033-BH-002 | When a valid prepared Budget Proposal is submitted for governed approval, the system shall place the Budget in the source-defined `PendingApproval` lifecycle position. | L3-F-033 §§8-9, 13 |
| L4-F-033-BH-003 | The system shall require an authoritative governed approval outcome before permitting `PendingApproval → Active`. | L3-F-033 §§9 step 3, 13, 15 |
| L4-F-033-BH-004 | When the authoritative outcome is approval, the system shall transition the Budget to `Active` without changing ownership of the Budget or referenced organisational data. | L3-F-033 §§5, 9 step 3, 12-13 |
| L4-F-033-BH-005 | When the authoritative outcome is rejection, the system shall return the Budget from `PendingApproval` to `Draft`, preserve the rejection outcome, and permit only governed revision and resubmission. | L3-F-033 §§11, 13 |
| L4-F-033-BH-006 | On successful activation, the system shall publish `BudgetApproved` with Budget & Cost Center Management retained as canonical publisher. | L3-F-033 §§3, 9 step 4, 14, 16 |
| L4-F-033-BH-007 | The system shall preserve the Budget Proposal, Budget Period, organisational scope, approval or rejection outcome, preserved Budget version, and resulting lifecycle position as attributable business evidence. | L3-F-033 §15.3 |
| L4-F-033-BH-008 | Completion of this scenario shall not create an actual accounting transaction or perform controlled Budget allocation. | L3-F-033 §§4.4, 12, 16.2 |

---

## 5. Cross-Domain & Event Behavior

Section B organisational data shall be consumed as authoritative reference context and shall not be duplicated or re-owned by Finance. `BudgetApproved` is published only by Budget & Cost Center Management after the approved activation outcome.

No event transport, payload, queue, delivery, retry, or technical integration behavior is defined at L4.

---

## 6. Failure / Rejection / Continuation

If required period or organisational context is invalid, the system shall prevent governed review from completing until the authoritative dependency is corrected. If the proposal is rejected, the Budget shall not become Active; the rejection shall be preserved and the source-defined return to Draft shall be maintained.

Technical retry, rollback, transaction compensation, exception-handler, and infrastructure-recovery mechanics are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve the proposal identity, Budget Period, authoritative organisational references, decision basis, approval or rejection outcome, prior/resulting Budget position, preserved Budget version, and `BudgetApproved` evidence where activation succeeds.

Section A retains ownership of Authorization and Audit infrastructure. The business-domain owners identified by the parent retain ownership of the underlying business decisions and evidence.

---

## 8. Observable Completion Conditions

Completion requires the Budget to be `Active`, approval/planning history to be preserved, and `BudgetApproved` to be published. A governed rejection is a valid non-completion outcome for activation and leaves the Budget in `Draft` for permitted correction/resubmission. No actual accounting transaction is created.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-033` — Budget Proposal to Approved Active Budget
**Primary verification source:** L2-F06 — Budget & Cost Center Management
**Additional verification:** L2-B02 — Organisation Workflows; locked L1 Finance/Section B ownership rules.

Every BH above is anchored to the approved Locked L3 parent. Higher-level/L2 sources are used only to verify exact ownership, event, rule, lifecycle, or dependency semantics; no material L3 omission is silently repaired.

---

## 10. Open Issues / Deferred Design

The parent does not establish a named institutional approver. L4 therefore requires an authoritative governed approval outcome without inventing an approving role. API/interface, persistence, workflow implementation, technical authorization, infrastructure, and UI/UX remain deferred.

---

