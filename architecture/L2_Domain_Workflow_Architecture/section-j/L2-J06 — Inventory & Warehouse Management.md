# L2-J06 — Inventory & Warehouse Management

**Repository:** SutraERP  
**Section:** J — Enterprise Resource Management  
**Domain:** J06 — Inventory & Warehouse Management  
**Document ID:** L2-J06  
**Version:** 1.0  
**Status:** Architecture Draft  
**Classification:** Level 2 Domain Workflow Architecture  
**Parent Architecture:** L2-J00 — Enterprise Resource Management Architecture & Governance  
**Canonical Owner:** Inventory & Warehouse Management

---

# 1. Purpose

J06 defines the canonical architecture for enterprise inventory truth.

J06 governs item master maintenance, warehouse structure, stock ledger, stock availability, batch and serial control, inventory receiving consequences, and stock valuation references.

The canonical lifecycle is:

```text
Goods Accepted
        |
        v
Inventory Receipt
        |
        v
Stock Ledger Update
        |
        v
Available Stock
        |
        +----> Reservation
        +----> Issue
        +----> Transfer
        +----> Adjustment
```

---

# 2. Domain Mission

J06 is the canonical owner of physical inventory truth across the institution.

It records what stock exists, where it exists, how much is available, and how movement affects the authoritative stock ledger.

---

# 3. Explicit Non-Ownership

J06 does not own:

- Purchase Requisition
- Purchase Order
- Goods Receipt Decision
- Asset Lifecycle
- Vendor Invoice
- Vendor Payment
- Accounting Entries

---

# 4. Primary Aggregates

| Aggregate | Responsibility |
|-----------|----------------|
| Item | Enterprise item master and classification |
| Warehouse | Warehouse and storage structure |
| InventoryPosition | Canonical stock ledger and availability |

---

# 5. Core Responsibilities

- Maintain the enterprise item master.
- Maintain warehouses and storage locations.
- Record inventory receipt consequences from J05.
- Govern reservation, issue, return, transfer, and adjustment.
- Maintain authoritative stock balances and availability.

---

# 6. Domain Events

J06 publishes:

- ItemCreated
- StockReceived
- StockReserved
- StockIssued
- StockReturned
- StockTransferred
- StockAdjusted
- ReorderThresholdReached

---

# 7. Dependencies

J06 depends on:

- J05 for receipt completion facts.
- Institution and organisation context for warehouse scope.
- Finance for valuation references where policy requires it.

---

# 8. Completion Criteria

J06 is complete when the enterprise stock ledger, item master, and warehouse model are canonical and no other domain maintains a competing inventory truth.



