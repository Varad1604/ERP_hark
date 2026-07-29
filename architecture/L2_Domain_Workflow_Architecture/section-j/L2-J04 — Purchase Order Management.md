# L2-J04 — Purchase Order Management

**Repository:** SutraERP  
**Section:** J — Enterprise Resource Management  
**Domain:** J04 — Purchase Order Management  
**Document ID:** L2-J04  
**Version:** 1.0  
**Status:** Architecture Draft  
**Classification:** Level 2 Domain Workflow Architecture  
**Parent Architecture:** L2-J00 — Enterprise Resource Management Architecture & Governance  
**Canonical Owner:** Purchase Order Management

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

- Purchase Requisition
- Supplier Master
- Goods Receipt
- Inventory Stock
- Warehouse Operations
- Asset Lifecycle
- Vendor Invoice
- Vendor Payment

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

J04 is complete when purchase orders are governed as the canonical supplier commitment without owning receipt, inventory, asset, or payment lifecycle responsibilities.



