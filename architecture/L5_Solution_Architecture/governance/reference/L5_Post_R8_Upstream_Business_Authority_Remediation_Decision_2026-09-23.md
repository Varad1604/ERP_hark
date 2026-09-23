# SutraERP — Post-R8 Upstream Business-Authority Remediation Decision — 2026-09-23

**Version:** 1.0.0  
**Status:** Architecture Approved  
**Owner:** Lead Architect  
**Scope:** Targeted L2/L3/L4 remediation required by post-R8 lifecycle closure  
**Final L5 Corpus Lock:** NOT AUTHORIZED

---

## 1. Authorized Decisions

The Lead Architect selected Option A and explicitly approved four canonical upstream business decisions.

### A016 — Workflow recovery

- `Workflow Recovery Authority` is the canonical abstract authority for protected Workflow Instance recovery/resumption/cancellation decisions.
- Workflow Engine / L2-A05 owns Workflow Instance lifecycle execution and recovery transitions.
- Workflow-Owning Domain retains business-outcome and repeatability authority.
- Authorization & Policy enforces protected action authorization without taking lifecycle ownership.
- Existing lifecycle `Failed → Recovering → Running/Cancelled` is preserved.
- Existing canonical recovery event family used here: `WorkflowFailed`, `WorkflowRecovered`, `WorkflowCancelled`.
- No new recovery state or event family is introduced.

### A057 — Controlled Tenant change

- `Tenant Change Authority` is the canonical abstract authority for governed Maintenance/Upgrade entry and return to Active.
- An Authorized Platform Actor may request/execute an approved controlled change.
- Platform & Tenant / L2-A01 remains sole owner of Tenant lifecycle.
- Return to Active requires successful governed validation and authority acceptance.
- No dedicated Maintenance/Upgrade event family is introduced; Tenant lifecycle state/context remains authoritative.

### F001 — Student Charge publication

- Standard event-driven Student Charge generation is policy-driven and does not require universal human approval.
- Source domains retain ownership of originating business facts/events.
- L2-F01 owns Charge Source resolution, Charge Policy evaluation, validation, Student Charge creation and publication.
- `ChargePublicationCompleted` remains the canonical L2-F01 publication fact.
- Manual/exceptional charge authority remains separately governed where required.

### F035 — Budget allocation

- `Authorized Budget Administrator` is the canonical abstract preparation actor.
- `Budget Governance Authority` is the canonical abstract allocation-decision authority.
- L2-F06 remains owner of Budget and Budget Allocation business truth.
- Governed allocation requires existing L2-F06 validity/balance rules plus Budget Governance Authority approval.
- `BudgetAllocationCreated` remains an L2-F06 canonical event.
- No universal Finance Committee/job title or universal SoD rule is invented.

---

## 2. Locked L2 Amendments

Created:

- `L2-A05_Workflow_Recovery_Authority_Amendment_2026-09-23.md`
- `L2-A01_Tenant_Controlled_Change_Authority_Amendment_2026-09-23.md`
- `L2-F01_Student_Charge_Publication_Authority_Amendment_2026-09-23.md`
- `L2-F06_Budget_Allocation_Authority_Amendment_2026-09-23.md`

All four are Locked and authoritative only for their defined subject.

---

## 3. L3 Reconciliation

Reconciled:

- `L3-A-016` → v1.1 Locked
- `L3-A-057` → v1.1 Locked
- `L3-F-001` → v1.2 Locked
- `L3-F-035` → v1.2 Locked

Mechanical audit:

```text
stale current BLOCKED classifications       0
undefined authority markers                 0
unresolved A016/A057 blocker markers        0
new business ownership transfers            0
```

---

## 4. L4 Approval

Promoted after L3 reconciliation:

- `L4-A-016` → v1.0.1 Architecture Approved
- `L4-A-057` → v1.0.1 Architecture Approved
- `L4-F-001` → v1.0.2 Architecture Approved
- `L4-F-035` → v1.0.1 Architecture Approved

These promotions do not authorize automatic L5 lifecycle promotion.

---

## 5. Repository Boundary

This remediation does not:

- merge or supersede PR #87;
- promote the remaining post-R8 L5 Review population;
- invent L4→L5 relations;
- allocate a new ADR;
- modify L6 architecture;
- authorize Final L5 Corpus Lock.

The next step after canonical integration is targeted L4→L5 relation validation and lifecycle revalidation of the remaining Review population.

---

## 6. Decision

**Decision:** `PASS — FOUR POST-R8 UPSTREAM BUSINESS-AUTHORITY GAPS RESOLVED AT THEIR CANONICAL OWNERS; L3/L4 RECONCILED; READY FOR TARGETED L5 CLOSURE AFTER MERGE.`

**Final L5 Corpus Lock:** **NOT AUTHORIZED**
