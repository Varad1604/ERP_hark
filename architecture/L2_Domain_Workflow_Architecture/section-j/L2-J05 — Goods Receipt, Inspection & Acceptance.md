# L2-J05 — Goods Receipt, Inspection & Acceptance

**Repository:** SutraERP  
**Section:** J — Enterprise Resource Management  
**Domain:** J05 — Goods Receipt, Inspection & Acceptance  
**Document ID:** L2-J05  
**Version:** 1.0  
**Status:** Architecture Draft  
**Classification:** Level 2 Domain Workflow Architecture  
**Parent Architecture:** L2-J00 — Enterprise Resource Management Architecture & Governance  
**Canonical Owner:** Goods Receipt & Acceptance

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

- Purchase Order
- Supplier Master
- Stock Ledger
- Inventory Balances
- Warehouse Structure
- Asset Lifecycle
- Vendor Invoice
- Vendor Payment

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

J05 is complete when receipt and inspection are canonical, auditable, and clearly separated from stock ledger, asset, and payment ownership.



