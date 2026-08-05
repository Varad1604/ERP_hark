# L2-J04 — Purchase Order Management

**Repository:** SutraERP
**Section:** J — Enterprise Resource Management
**Domain:** J04 — Purchase Order Management
**Document ID:** L2-J04
**Version:** 1.0
**Status:** Draft
**Classification:** Level 2 Domain Workflow Architecture
**Parent Architecture:** L2-J00 — Enterprise Resource Management Architecture & Governance
**Canonical Owner:** Purchase Order Management
**Document Owner:** Enterprise Resource Management — Purchase Orders

---

## Canonical Section Crosswalk

> Added 2026-08-04 under integrity finding INT-018. `governance/constitution/Volume_3_Architecture_Framework.md` section 18 requires every L2 document to define eleven sections. This document uses its own numbered-section convention; the table below maps the required sections to where they actually live. No content was moved.

| Required section (Volume 3 §18) | Location in this document |
|---|---|
| Purpose | §1 Purpose |
| Scope | — not present — |
| Actors | — not present — |
| Entities | §4 Primary Aggregate |
| Business Rules | Rules / Invariants |
| Workflow | §12 Event and Workflow Integration; Workflows |
| State Machine | States |
| Contracts | — not present — |
| Events | §6 Domain Events; §12 Event and Workflow Integration |
| Dependencies | §7 Dependencies |
| Locked Decisions | — not present — |

> **Gap:** 4 required section(s) have no counterpart in this document: Scope, Actors, Contracts, Locked Decisions. Recorded, not authored.

---

# 1. Purpose

J04 defines the canonical architecture for converting an approved procurement outcome into an authorised purchase order.

J04 governs purchase order creation, amendment, approval, issue, cancellation, and closure.

The canonical lifecycle is:

```text
Released Purchase Requisition
        |
        v
Sourcing Outcome
        |
        v
Purchase Order Draft
        |
        v
Approval / Validation
        |
        v
Issued Purchase Order
        |
        v
Goods Receipt / Financial Settlement
```

---

# 2. Domain Mission

J04 is the canonical institutional commitment boundary for external suppliers.

It preserves procurement control while allowing downstream receipt, inventory, asset, and finance processes to react independently.

---

# 3. Explicit Non-Ownership

J04 does not own:

| Capability | Canonical Owner |
|---|---|
| Purchase Requisition | J03 — Purchase Requisition & Approval |
| Procurement Planning and Sourcing | J01 — Procurement Planning & Sourcing |
| Supplier Master | J02 — Vendor & Supplier Management |
| Goods Receipt | J05 — Goods Receipt, Inspection & Acceptance |
| Inventory Stock and Warehouse Operations | J06 — Inventory & Warehouse Management |
| Stock Movement | J07 — Stock Operations & Distribution |
| Asset Lifecycle | J08 — Asset Lifecycle Management |
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
| PurchaseOrder | Governs order content, approval state, issue state, and closure |

---

# 5. Core Responsibilities

- Create and amend purchase orders.
- Capture supplier acknowledgement and approval state.
- Publish order issue facts for receiving and finance.
- Track cancellation, closure, and fulfilment references.

---

# 6. Domain Events

J04 publishes:

- PurchaseOrderDrafted
- PurchaseOrderApproved
- PurchaseOrderIssued
- PurchaseOrderAmended
- PurchaseOrderCancelled
- PurchaseOrderClosed

---

# 7. Dependencies

J04 depends on:

- J01 for sourcing recommendations.
- J02 for supplier identity and status.
- J03 for requisition release context.
- Finance for validation references where policy requires it.

---

# 8. Completion Criteria

J04 is complete when purchase orders are governed as the canonical supplier commitment with explicit authorization, audit evidence, tenant isolation, events, integration contracts, and recovery controls, without owning requisition, supplier, receipt, inventory, asset, invoice, or payment lifecycle responsibilities.

---

# 9. Authorization & Segregation of Duties

J04 defines purchase-order authorization semantics while L2-A04 owns policy evaluation.

Authorization may consider:

- Tenant and institution.
- Supplier status and supplier eligibility from J02.
- Sourcing outcome from J01.
- Requisition release context from J03.
- Procurement value band.
- Item category and controlled-resource classification.
- Funding or budget-validation references.
- Amendment type and lifecycle state.
- Delegated authority.

Representative SoD rules include:

```text
Purchase Order Creator != Purchase Order Approver
Supplier Selector != Final PO Approver where configured
PO Amendment Approver requires authority for the amended value/scope
```

---

# 10. Audit & Evidence

J04 produces domain evidence for:

- Draft creation.
- Supplier and sourcing reference validation.
- PO approval, issue, amendment, cancellation, and closure.
- Supplier acknowledgement.
- Budget or finance validation reference.
- Goods-receipt and finance handoff reference.

L2-A07 owns enterprise audit persistence, retention governance, access governance, and evidence integrity.

---

# 11. Tenant and Institution Isolation

Every purchase-order command, aggregate, event, read model, and audit reference shall preserve tenant and institution context.

A Purchase Order Number alone shall not authorize access.

Cross-institution purchase commitments shall require explicit governed authorization.

---

# 12. Event and Workflow Integration

J04 owns the business meaning and timing of purchase-order events.

L2-A06 owns event transport, routing, retry, replay, and event registry governance.

L2-A05 owns workflow routing, approval tasks, escalation, timeout, and orchestration. J04 owns resulting PO business state.

---

# 13. Privacy, Retention, and Recovery

J04 shall preserve supplier, requester, finance, and item references using stable identifiers and transaction-time snapshots only where required for historical interpretation.

Issued purchase orders and amendments shall remain reconstructable according to enterprise and institutional retention policy.

Retried commands shall be idempotent for draft creation, approval, issue, amendment, cancellation, and closure.

---

## Lifecycle

The purchase order lifecycle is described as a flow in Section 1 (Purpose): Released Purchase Requisition -> Sourcing Outcome -> Purchase Order Draft -> Approval/Validation -> Issued Purchase Order -> Goods Receipt/Financial Settlement.

## Workflows

Purchase order workflow steps are captured in Section 5 (Core Responsibilities): order creation and amendment, supplier acknowledgement and approval capture, order issue publication for receiving and finance, and cancellation/closure/fulfilment tracking.

## States

This document does not define a discrete state machine separate from the lifecycle flow in Section 1; order progression is expressed through the stages Drafted, Approved, Issued, Amended, Cancelled, and Closed referenced in the Domain Events (Section 6) and Audit & Evidence (Section 10).

## Rules / Invariants

Purchase order rules are stated across the document rather than consolidated: SoD rules (Section 9), tenant/institution isolation rules (Section 11), and idempotency guarantees for draft creation, approval, issue, amendment, cancellation, and closure (Section 13).
