# L2-F06 — Budget Allocation Authority — Locked Amendment

| Attribute | Value |
|---|---|
| Amendment Version | 1.1.0 |
| Status | Locked |
| Architecture Level | L2 — Business Domain Architecture |
| Canonical Owner | Finance — Budgeting & Cost Centers |
| Approved Through | Explicit Lead Architect decision, 2026-09-23 |
| Authority | Post-R8 Option A upstream business-architecture remediation |

---

## 1. Purpose

This amendment closes the institutional actor/decision-authority gap recorded by L3-F-035 for controlled Budget allocation.

---

## 2. Canonical Business Authorities

### 2.1 Authorized Budget Administrator

`Authorized Budget Administrator` is the canonical abstract actor category permitted to prepare and revise proposed Budget Allocations within institutional policy.

Preparation does not itself authorize the allocation outcome.

### 2.2 Budget Governance Authority

`Budget Governance Authority` is the canonical abstract business authority that makes the governed decision that permits an eligible allocation set to become the approved controlled allocation of an Active Budget.

Institutional policy maps this authority to an eligible office, committee, administrator or governance body.

No universal job title is imposed.

### 2.3 Budget & Cost Center Management

L2-F06 remains canonical owner of:

- Budget;
- Budget Allocation;
- Cost Center references used by the Budget domain;
- allocation validation;
- allocation lifecycle/business truth;
- canonical Budget lifecycle events.

The actor/authority model does not transfer aggregate ownership.

---

## 3. Controlled Allocation Decision

A governed allocation may become effective only when:

- the Budget is `Active`;
- every target Cost Center reference is valid and active;
- the proposed allocations satisfy all L2-F06 business rules;
- total allocations exactly equal the approved Budget `TotalAmount`;
- Budget Governance Authority records the governed allocation decision.

Preparation and approval are distinct responsibilities conceptually.

This amendment does not impose a universal rule that they must be performed by different persons; institutional policy may impose stronger separation of duties.

---

## 4. Canonical Allocation Event

`BudgetAllocationCreated` remains the canonical L2-F06 event for an effective governed Budget Allocation.

Finance — Budgeting & Cost Centers / L2-F06 is the sole semantic publisher.

Event Infrastructure may distribute the fact but does not own its semantics.

---

## 5. Approval Provenance

The authoritative approval provenance for L3-F-035 is this Locked L2 amendment and its explicit Lead Architect approval.

No untraceable ACR reference is required to establish the canonical actor/authority mapping defined here.

---

## 6. Locked Decisions

1. `Authorized Budget Administrator` is the canonical abstract preparation actor for Budget Allocations.
2. `Budget Governance Authority` is the canonical abstract authority for the governed allocation decision.
3. L2-F06 remains sole owner of Budget and Budget Allocation business truth.
4. Allocation effectiveness requires existing L2-F06 validation/business rules and Budget Governance Authority approval.
5. `BudgetAllocationCreated` remains a canonical L2-F06 lifecycle/integration fact and is semantically published by L2-F06.
6. No universal named Finance Committee or job title is introduced.
7. No universal SoD requirement beyond applicable institutional policy is introduced.
8. This amendment is the independently traceable approval provenance for the canonical actor/authority mapping used by downstream architecture.
