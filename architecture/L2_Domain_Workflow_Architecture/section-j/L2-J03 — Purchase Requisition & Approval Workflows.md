# L2-J03 — Purchase Requisition & Approval Workflows

**Repository:** SutraERP  
**Section:** J — Enterprise Resource Management  
**Domain:** J03 — Purchase Requisition & Approval  
**Document ID:** L2-J03  
**Version:** 1.0  
**Status:** Architecture Draft  
**Classification:** Level 2 Domain Workflow Architecture  
**Parent Architecture:** L2-J00 — Enterprise Resource Management Architecture & Governance  
**Canonical Owner:** Purchase Requisition & Approval

---

# 1. Purpose

J03 defines the canonical architecture for converting an institutional demand into an approved purchase requisition.

J03 governs requisition creation, amendment, review, validation, approval, rejection, and release for downstream procurement planning.

The canonical lifecycle is:

```text
Requesting Domain
        |
        v
Purchase Requisition
        |
        v
Budget / Policy Validation
        |
        v
Approval Review
        |
        v
Released for Procurement
        |
        v
J01 Procurement Planning & Sourcing
```

---

# 2. Domain Mission

J03 provides the authoritative demand-control boundary for resources that are not fulfilled directly from available stock.

It ensures purchase demand is visible, approvable, auditable, and ready for procurement planning without duplicating supplier, order, inventory, or payment ownership.

---

# 3. Explicit Non-Ownership

J03 does not own:

- Supplier Master
- Purchase Order
- Goods Receipt
- Inventory Stock
- Warehouse Operations
- Asset Lifecycle
- Accounting Entries
- Vendor Payments

---

# 4. Primary Aggregate

| Aggregate | Responsibility |
|-----------|----------------|
| PurchaseRequisition | Governs requisition content, approval state, and release for procurement |

---

# 5. Core Responsibilities

- Create purchase requisitions.
- Manage requisition lines and quantities.
- Coordinate policy and budget validation references.
- Capture approval and rejection decisions.
- Publish requisition release facts for J01 and J04.

---

# 6. Domain Events

J03 publishes:

- PurchaseRequisitionSubmitted
- PurchaseRequisitionApproved
- PurchaseRequisitionRejected
- PurchaseRequisitionReleasedForProcurement

---

# 7. Dependencies

J03 depends on:

- Requesting domains for demand origin.
- Finance for budget-validation references where policy requires it.
- J01 for downstream sourcing.
- J04 for order creation after release.

---

# 8. Completion Criteria

J03 is complete when requisitions, approvals, release conditions, and integration events are defined without overlap with supplier, order, inventory, or asset ownership.



