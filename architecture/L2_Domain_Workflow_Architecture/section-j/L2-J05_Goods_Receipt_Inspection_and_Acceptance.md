# L2-J05 — Goods Receipt, Inspection & Acceptance

**Repository:** SutraERP
**Section:** J — Enterprise Resource Management
**Domain:** J05 — Goods Receipt, Inspection & Acceptance
**Document ID:** L2-J05
**Version:** 1.1 — INT-018 structural maintenance
**Status:** Locked
**Classification:** Level 2 Domain Workflow Architecture
**Parent Architecture:** L2-J00 — Enterprise Resource Management Architecture & Governance
**Canonical Owner:** Goods Receipt & Acceptance
**Document Owner:** Enterprise Resource Management — Goods Receipt

---

## Canonical Section Crosswalk

> Added 2026-08-04 under integrity finding INT-018. `governance/constitution/Volume_3_Architecture_Framework.md` section 18 requires every L2 document to define eleven sections. This document uses its own numbered-section convention; the table below maps the required sections to where they actually live. No content was moved.

| Required section (Volume 3 §18) | Location in this document |
|---|---|
| Purpose | §1 Purpose |
| Scope | §14 Scope |
| Actors | §15 Canonical Actors |
| Entities | §4 Primary Aggregate |
| Business Rules | Rules / Invariants |
| Workflow | §12 Event and Workflow Integration; Workflows |
| State Machine | States |
| Contracts | §16 Domain Contracts |
| Events | §6 Domain Events; §12 Event and Workflow Integration |
| Dependencies | §7 Dependencies |
| Locked Decisions | §17 Locked Decisions |

> **INT-018 maintenance:** The four former structural gaps were closed on 2026-08-13 by consolidating semantics already present in §§1–13. No canonical ownership, lifecycle, event, or rule was added.

---

# 1. Purpose

J05 defines the canonical architecture for recording the physical receipt and inspection of procured resources.

J05 governs delivery receipt, inspection, acceptance, rejection, variance recording, and supplier return handling.

The canonical lifecycle is:

```text
Issued Purchase Order
        |
        v
Delivery Arrival
        |
        v
Goods Receipt
        |
        v
Inspection
        |
        +----> Acceptance
        |
        +----> Rejection / Variance / Return
```

---

# 2. Domain Mission

J05 is the canonical receiving boundary for institutional procurement.

It records what physically arrived, whether it was acceptable, and what downstream domains need to know without taking ownership of inventory, asset, or accounting state.

---

# 3. Explicit Non-Ownership

J05 does not own:

| Capability | Canonical Owner |
|---|---|
| Purchase Order | J04 — Purchase Order Management |
| Supplier Master | J02 — Vendor & Supplier Management |
| Stock Ledger, Inventory Balances, Warehouse Structure | J06 — Inventory & Warehouse Management |
| Stock Movement and Distribution | J07 — Stock Operations & Distribution |
| Operational Asset Lifecycle | J08 — Asset Lifecycle Management |
| Physical Verification / Audit Conclusion | J09 — Physical Verification, Audit & Reconciliation |
| Disposal / Write-Off Case | J10 — Disposal, Scrap & Write-Off |
| Vendor Invoice, Accounting, Payment | Section F — Finance |
| Authentication | L2-A03 — Authentication |
| Authorization Policy and SoD | L2-A04 — Authorization and Policy |
| Workflow Routing | L2-A05 — Workflow Engine |
| Event Transport | L2-A06 — Event Infrastructure |
| Enterprise Audit Persistence | L2-A07 — Audit Infrastructure |

---

# 4. Primary Aggregate

| Aggregate | Responsibility |
|-----------|----------------|
| GoodsReceipt | Governs receipt header, receipt lines, inspection state, acceptance state, and return references |

---

# 5. Core Responsibilities

- Record receipt against an issued purchase order.
- Record inspection and acceptance outcomes.
- Capture short delivery, excess delivery, damage, and variance references.
- Publish completion facts for inventory, asset, and finance consumers.

---

# 6. Domain Events

J05 publishes:

- GoodsReceived
- GoodsAccepted
- GoodsRejected
- ReceiptVarianceRecorded
- GoodsReceiptClosed

---

# 7. Dependencies

J05 depends on:

- J04 for purchase order authority.
- J02 for supplier identity and status.
- J06 for the resulting stock consequence where applicable.
- J08 for asset registration where received items qualify as assets.
- Finance for accounting consequences where policy requires it.

---

# 8. Completion Criteria

J05 is complete when receipt and inspection are canonical, auditable, tenant-scoped, event-driven, recoverable, and clearly separated from purchase order, stock ledger, asset, invoice, and payment ownership.

---

# 9. Authorization & Segregation of Duties

J05 defines receipt and inspection authorization semantics while L2-A04 owns policy evaluation.

Authorization may consider:

- Tenant and institution.
- Receiving campus, warehouse, store, or department.
- Purchase Order reference and lifecycle state.
- Supplier status.
- Item category and controlled-resource classification.
- Inspection requirement and quality policy.
- Quantity and variance tolerance.
- Actor role and delegated authority.

Representative SoD rules include:

```text
Goods Receiver != Goods Inspector where configured
PO Creator != Receipt Approver where configured
Receipt Variance Approver requires explicit variance authority
```

---

# 10. Audit & Evidence

J05 produces domain evidence for:

- Delivery arrival.
- Receipt header and lines.
- Short, excess, damaged, rejected, or substituted delivery.
- Inspection outcome.
- Acceptance, rejection, supplier return, and closure.
- Downstream inventory, asset, and finance handoff references.

L2-A07 owns enterprise audit persistence, retention governance, access governance, and evidence integrity.

---

# 11. Tenant and Institution Isolation

Every goods-receipt command, aggregate, event, read model, and audit reference shall preserve tenant and institution context.

A Goods Receipt Number alone shall not authorize access.

Cross-institution receiving shall require explicit governed authorization.

---

# 12. Event and Workflow Integration

J05 owns the business meaning and timing of receipt and inspection events.

L2-A06 owns event transport, routing, retry, replay, and event registry governance.

L2-A05 owns workflow routing, inspection tasks, approval tasks, escalation, timeout, and orchestration. J05 owns the resulting receipt and inspection business state.

---

# 13. Privacy, Retention, and Recovery

J05 shall minimize copied supplier, employee, finance, and warehouse data by preserving canonical identifiers and transaction-time snapshots only where required.

Finalized receipt and inspection evidence shall remain reconstructable according to enterprise and institutional retention policy.

Retried commands shall be idempotent for receipt creation, inspection recording, acceptance, rejection, variance recording, supplier return, and closure.

---

# 14. Scope

J05 owns physical delivery arrival, receipt header and lines, inspection, acceptance/rejection, short/excess/damaged/substituted-delivery variance, supplier-return references and receipt closure within the `GoodsReceipt` aggregate. It publishes receipt dispositions without owning the resulting stock ledger, asset register, invoice, payment, or accounting state identified in §3.

# 15. Canonical Actors

| Actor | Existing responsibility |
|---|---|
| Goods Receiver | Records delivery arrival and received quantities against valid purchase-order authority. |
| Goods Inspector | Records condition, quality and inspection evidence independently where policy requires separation. |
| Receipt Approver | Authorizes the governed acceptance or rejection outcome where approval is required. |
| Receipt Variance Approver | Applies explicit variance authority to short, excess, damaged, substituted or otherwise exceptional receipts. |

# 16. Domain Contracts

| Contract direction | Existing contract |
|---|---|
| Consumes | J04 issued Purchase Order authority and J02 supplier identity/status. |
| Consumes platform outcomes | A03 authentication context, A04 authorization decisions, A05 inspection/approval workflow outcomes, A06 event transport and A07 audit persistence. |
| Publishes | Receipt, inspection, acceptance, rejection, variance, return and closure facts for J06 inventory consequences, J08 asset registration and Finance processing. |
| Publishes events | The receipt facts enumerated in §6; J05 owns meaning and timing while A06 owns transport. |

# 17. Locked Decisions

1. J05 is the canonical physical receiving and inspection boundary for institutional procurement.
2. The `GoodsReceipt` aggregate records one governed receipt/inspection/disposition lifecycle, including partial, cumulative, replacement and variance paths.
3. Acceptance does not directly create inventory, asset, invoice, payment or accounting state; downstream owners consume J05 facts.
4. A05 owns task routing; J05 owns the resulting receipt and inspection business state.
5. Finalized receipt and inspection evidence is reconstructable and is never destructively rewritten.

---

## Lifecycle

The goods receipt lifecycle is described as a flow in Section 1 (Purpose): Issued Purchase Order -> Delivery Arrival -> Goods Receipt -> Inspection -> Acceptance, or Rejection/Variance/Return.

## Workflows

Receipt and inspection workflow steps are captured in Section 5 (Core Responsibilities): recording receipt against an issued purchase order, recording inspection and acceptance outcomes, capturing short/excess delivery, damage and variance references, and publishing completion facts to inventory, asset, and finance consumers.

## States

This document does not define a discrete state machine separate from the lifecycle flow in Section 1; receipt progression is expressed through the stages Received, Accepted, Rejected, Variance Recorded, and Closed referenced in the Domain Events (Section 6) and Audit & Evidence (Section 10).

## Rules / Invariants

Receipt and inspection rules are stated across the document rather than consolidated: SoD rules (Section 9), tenant/institution isolation rules (Section 11), and idempotency guarantees for receipt creation, inspection recording, acceptance, rejection, variance recording, supplier return, and closure (Section 13).
