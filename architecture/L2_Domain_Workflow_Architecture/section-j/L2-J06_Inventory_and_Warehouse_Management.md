# L2-J06 — Inventory & Warehouse Management

**Repository:** SutraERP
**Section:** J — Enterprise Resource Management
**Domain:** J06 — Inventory & Warehouse Management
**Document ID:** L2-J06
**Version:** 1.1 — INT-018 structural maintenance
**Status:** Locked
**Classification:** Level 2 Domain Workflow Architecture
**Parent Architecture:** L2-J00 — Enterprise Resource Management Architecture & Governance
**Canonical Owner:** Inventory & Warehouse Management
**Document Owner:** Enterprise Resource Management — Inventory & Warehouse

---

## Canonical Section Crosswalk

> Added 2026-08-04 under integrity finding INT-018. `governance/constitution/Volume_3_Architecture_Framework.md` section 18 requires every L2 document to define eleven sections. This document uses its own numbered-section convention; the table below maps the required sections to where they actually live. No content was moved.

| Required section (Volume 3 §18) | Location in this document |
|---|---|
| Purpose | §1 Purpose |
| Scope | §14 Scope |
| Actors | §15 Canonical Actors |
| Entities | §16 Canonical Entities |
| Business Rules | Rules / Invariants |
| Workflow | §12 Event and Workflow Integration; Workflows |
| State Machine | States |
| Contracts | §17 Domain Contracts |
| Events | §6 Domain Events; §12 Event and Workflow Integration |
| Dependencies | §7 Dependencies |
| Locked Decisions | §18 Locked Decisions |

> **INT-018 maintenance:** The five former structural gaps were closed on 2026-08-13 by consolidating semantics already present in §§1–13. No canonical ownership, lifecycle, event, or rule was added.

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

| Capability | Canonical Owner |
|---|---|
| Purchase Requisition | J03 — Purchase Requisition & Approval |
| Purchase Order | J04 — Purchase Order Management |
| Goods Receipt Decision and Inspection | J05 — Goods Receipt, Inspection & Acceptance |
| Stock Movement Execution and Distribution | J07 — Stock Operations & Distribution |
| Operational Asset Lifecycle | J08 — Asset Lifecycle Management |
| Physical Verification / Audit Conclusion | J09 — Physical Verification, Audit & Reconciliation |
| Disposal / Write-Off Case | J10 — Disposal, Scrap & Write-Off |
| Vendor Invoice, Payment, Accounting Entries | Section F — Finance |
| Authentication | L2-A03 — Authentication |
| Authorization Policy and SoD | L2-A04 — Authorization and Policy |
| Workflow Routing | L2-A05 — Workflow Engine |
| Event Transport | L2-A06 — Event Infrastructure |
| Enterprise Audit Persistence | L2-A07 — Audit Infrastructure |

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
- Maintain authoritative stock balances and availability, updated from the movement
  facts published by J07.
- Govern stock adjustment, which corrects the ledger itself.

**Not owned here.** Operational stock movement --- reservation, issue, return and
transfer --- is owned and executed by `L2-J07 --- Stock Operations & Distribution`,
which J00's canonical publisher matrix names as the sole publisher of `StockIssued`,
`StockReturned`, `StockTransferDispatched` and `StockTransferReceived`. J06 consumes
those facts and applies their ledger consequences; it never performs the movement.

---

# 6. Domain Events

J06 publishes:

- ItemCreated
- StockReceived
- StockAdjusted
- ReorderThresholdReached

J06 consumes, and does not publish, the operational movement events owned by J07:

- StockReserved
- StockIssued
- StockReturned
- StockTransferDispatched
- StockTransferReceived

---

# 7. Dependencies

J06 depends on:

- J05 for receipt completion facts.
- Institution and organisation context for warehouse scope.
- Finance for valuation references where policy requires it.

---

# 8. Completion Criteria

J06 is complete when the enterprise item master, warehouse model, stock ledger, stock availability, inventory controls, authorization semantics, audit evidence, tenant isolation, integration contracts, and domain events are canonical and no other domain maintains a competing inventory truth.

---

# 9. Authorization & Segregation of Duties

J06 defines inventory and warehouse authorization semantics while L2-A04 owns policy evaluation.

Authorization may consider:

- Tenant and institution.
- Campus, warehouse, store, location, and bin.
- Item category, batch, serial, expiry, and controlled-resource classification.
- Inventory transaction type.
- Quantity and value band.
- Adjustment reason and variance tolerance.
- Actor role and delegated authority.
- Inventory lifecycle state.

Representative SoD rules include:

```text
Stock Adjuster != Stock Adjustment Approver
Warehouse Configurator != Physical Verifier where configured
Inventory Balance Viewer for controlled stock requires explicit policy authorization
```

---

# 10. Audit & Evidence

J06 produces domain evidence for:

- Item creation, classification, amendment, and archival.
- Warehouse, store, location, and bin configuration.
- Inventory receipt consequences from J05.
- Stock reservation, issue, return, transfer consequence, and adjustment.
- Batch, serial, expiry, and controlled-resource state.
- Reorder threshold and stock availability changes.

L2-A07 owns enterprise audit persistence, retention governance, access governance, and evidence integrity.

---

# 11. Tenant and Institution Isolation

Every inventory command, aggregate, event, read model, and audit reference shall preserve tenant and institution context.

An Item Identifier, Warehouse Identifier, or Stock Ledger Identifier alone shall not authorize access.

Cross-institution inventory visibility or transfer consequence processing shall require explicit governed authorization.

---

# 12. Event and Workflow Integration

J06 owns the business meaning and timing of inventory events.

L2-A06 owns event transport, routing, retry, replay, and event registry governance.

L2-A05 owns workflow routing where inventory configuration, adjustment, or controlled-stock approvals require human workflow. J06 owns resulting inventory business state.

---

# 13. Privacy, Retention, and Recovery

J06 shall minimize copied supplier, employee, student, department, finance, and asset information by referencing canonical identifiers from owning domains.

Inventory and stock-ledger history shall remain reconstructable according to enterprise and institutional retention policy.

Retried commands shall be idempotent for item creation, warehouse configuration, stock receipt consequence, reservation, issue consequence, return consequence, transfer consequence, and adjustment.

---

# 14. Scope

J06 owns the enterprise `Item` master, `Warehouse` and storage-location structure, and authoritative `InventoryPosition` stock ledger and availability. It applies accepted-receipt and J07 movement consequences, governs ledger adjustments, and maintains batch, lot, serial, expiry, controlled-resource and replenishment controls without executing operational stock movement or owning Finance truth.

# 15. Canonical Actors

| Actor | Existing responsibility |
|---|---|
| Item Master Administrator | Creates, classifies, amends and archives enterprise Item records under policy. |
| Warehouse Configurator | Maintains warehouse, store, location and bin structure. |
| Inventory Controller / Stock Adjuster | Requests and records governed ledger corrections and adjustment reasons. |
| Stock Adjustment Approver | Independently authorizes adjustments where policy requires it. |
| Controlled Stock Viewer | Reads controlled inventory only under explicit authorization. |

# 16. Canonical Entities

| Entity / Aggregate | Existing responsibility |
|---|---|
| `Item` | Enterprise item identity and classification. |
| `Warehouse` | Institution-scoped warehouse and storage structure. |
| `InventoryPosition` | Authoritative quantity, stock-ledger and availability state, including applicable batch/lot/serial/expiry traceability. |

# 17. Domain Contracts

| Contract direction | Existing contract |
|---|---|
| Consumes | J05 accepted-receipt facts, J07 movement facts, institution/organisation warehouse scope and Finance valuation references where required. |
| Consumes platform outcomes | A03 authentication context, A04 authorization decisions, A05 workflow outcomes, A06 event transport and A07 audit persistence. |
| Publishes | Item and inventory-position references, applied ledger consequences, stock-adjustment facts and availability/reorder facts. |
| Publishes events | `ItemCreated`, `StockReceived`, `StockAdjusted`, and `ReorderThresholdReached`; J07 remains publisher of operational movement facts. |

# 18. Locked Decisions

1. J06 is the exclusive owner of enterprise physical inventory truth, stock ledger, stock balance and availability.
2. J07 exclusively owns operational reservation, issue, return, transfer, distribution and movement facts; J06 only applies their ledger consequences.
3. J05 owns receipt/inspection decisions; J06 recognizes only their accepted inventory consequence.
4. Finance owns authoritative valuation and accounting consequences.
5. No other domain may maintain a competing Item, Warehouse or InventoryPosition truth.

---

## Lifecycle

The inventory lifecycle is described as a flow in Section 1 (Purpose): Goods Accepted -> Inventory Receipt -> Stock Ledger Update -> Available Stock, branching into Reservation, Issue, Transfer, or Adjustment.

## Workflows

Inventory and warehouse workflow steps are captured in Section 5 (Core Responsibilities): maintaining the enterprise item master, maintaining warehouses and storage locations, recording inventory receipt consequences from J05, governing stock adjustment, and maintaining authoritative stock balances and availability from the movement facts published by J07.

## States

This document does not define a discrete state machine separate from the lifecycle flow in Section 1; stock position progression is expressed through Received, Reserved, Issued, Returned, Transferred, and Adjusted, as referenced in the Domain Events (Section 6) and Audit & Evidence (Section 10).

## Rules / Invariants

Inventory rules are stated across the document rather than consolidated: SoD rules (Section 9), tenant/institution isolation rules (Section 11), and idempotency guarantees for item creation, warehouse configuration, stock receipt consequence, reservation, issue, return, transfer, and adjustment (Section 13).
