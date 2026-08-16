# L2-J07 — Stock Operations & Distribution

## Document Metadata

| Attribute | Value |
|---|---|
| Document ID | L2-J07 |
| Document Name | Stock Operations & Distribution |
| Section | J - Enterprise Resource Management |
| Domain | J07 - Stock Operations & Distribution |
| Architecture Level | Level 2 |
| Document Type | Domain Workflow Architecture |
| Parent Architecture | L2-J00 - Enterprise Resource Management Architecture & Governance |
| Primary Aggregate | StockMovement |
| Canonical Owner | Stock Operations & Distribution |
| Document Owner | Enterprise Resource Management — Stock Operations |
| Status | Locked |
| Version | 1.2 — INT-018 structural maintenance |

---

## Canonical Section Crosswalk

> Added 2026-08-04 under integrity finding INT-018. `governance/constitution/Volume_3_Architecture_Framework.md` section 18 requires every L2 document to define eleven sections. This document uses its own numbered-section convention; the table below maps the required sections to where they actually live. No content was moved.

| Required section (Volume 3 §18) | Location in this document |
|---|---|
| Purpose | §1 Purpose |
| Scope | §2 Scope |
| Actors | §14 Canonical Actors |
| Entities | §15 Canonical Entities |
| Business Rules | §6 Business Rules |
| Workflow | Workflows |
| State Machine | States |
| Contracts | §16 Domain Contracts |
| Events | §10 Domain Events |
| Dependencies | §17 Dependencies |
| Locked Decisions | §18 Locked Decisions |

> **INT-018 maintenance:** The five former structural gaps were closed on 2026-08-13 by consolidating semantics already present in §§2–11. No canonical ownership, lifecycle, event, or rule was added.

---

# 1. Purpose

J07 defines the canonical architecture for authorised movement, distribution, issue, return, transfer, and consumption recording of institutional stock after that stock exists within the canonical inventory model owned by J06.

J07 governs operational stock movement facts.

J06 remains the canonical owner of item master, warehouse structure, stock ledger, stock balance, and availability.

---

# 2. Scope

J07 owns:

- Stock Reservation Execution
- Stock Issue
- Stock Return
- Stock Transfer Request
- Stock Transfer Dispatch
- Stock Transfer Receipt
- Stock Distribution
- Consumption Recording
- Movement Exception
- Movement Cancellation
- Movement Completion
- Chain-of-Custody Evidence
- Stock Movement Audit Metadata

---

# 3. Explicit Non-Ownership

| Capability | Canonical Owner |
|---|---|
| Item Master | J06 - Inventory & Warehouse Management |
| Warehouse and Storage Structure | J06 - Inventory & Warehouse Management |
| Stock Ledger and Stock Balance | J06 - Inventory & Warehouse Management |
| Goods Receipt and Inspection | J05 - Goods Receipt, Inspection & Acceptance |
| Purchase Order | J04 - Purchase Order Management |
| Supplier Master | J02 - Vendor & Supplier Management |
| Operational Asset Register | J08 - Asset Lifecycle Management |
| Physical Verification and Inventory Audit | J09 - Physical Verification, Audit & Reconciliation |
| Disposal / Write-off Case | J10 - Disposal, Scrap & Write-Off |
| Accounting and Valuation | Section F - Finance |
| Authentication | L2-A03 - Authentication |
| Authorization Policy | L2-A04 - Authorization and Policy |
| Workflow Routing | L2-A05 - Workflow Engine |
| Event Transport | L2-A06 - Event Infrastructure |
| Enterprise Audit Persistence | L2-A07 - Audit Infrastructure |

J07 shall not directly modify J06 stock balances. It publishes or submits movement facts for J06 to apply according to inventory rules.

---

# 4. Primary Aggregates

| Aggregate | Responsibility |
|---|---|
| StockMovement | Governs stock issue, return, transfer, distribution, custody, and movement status |
| StockReservationExecution | Tracks approved reservation fulfilment without owning reservation policy or stock availability |
| MovementException | Captures loss, damage, shortage, overage, rejection, cancellation, or unresolved custody exceptions |

---

# 5. Canonical Lifecycle

```text
Movement Requirement
        |
        v
Authorization / Policy Validation
        |
        v
Stock Allocation Reference
        |
        v
Issue / Dispatch / Distribution
        |
        v
Receipt / Acknowledgement / Return
        |
        v
Exception Resolution where required
        |
        v
Movement Completion
        |
        v
J06 Stock Ledger Consequence
```

---

# 6. Business Rules

1. Every material movement shall reference tenant, institution, source, destination, item, quantity, and actor context.
2. Completed stock movements shall be corrected through reversal, return, adjustment request, or governed exception rather than destructive editing.
3. A consuming department or domain does not become the owner of inventory truth by receiving stock.
4. Controlled resources shall support stricter visibility, approval, custody, and distribution controls.
5. J07 shall preserve chain-of-custody evidence for stock moving between warehouses, stores, departments, campuses, or custodians.
6. J07 movement completion shall not be interpreted as financial posting.

---

# 7. Authorization & Segregation of Duties

J07 defines stock-movement authorization semantics while L2-A04 owns authorization policy evaluation.

Authorization may consider:

- Tenant and institution.
- Campus, department, warehouse, store, and bin.
- Item category and controlled-resource classification.
- Movement type.
- Quantity and value band.
- Source and destination responsibility.
- Delegated authority.
- Movement lifecycle state.

Representative SoD rules include:

```text
Requester != Approver
Store Custodian != Independent Receiver where configured
Transfer Dispatcher != Transfer Reconciler where configured
Controlled Stock Viewer requires explicit policy authorization
```

---

# 8. Audit & Evidence

J07 produces domain evidence for:

- Reservation fulfilment.
- Stock issue.
- Dispatch.
- Receipt acknowledgement.
- Return.
- Transfer discrepancy.
- Custody handover.
- Cancellation.
- Completion.
- Reversal or correction.

L2-A07 remains the canonical owner of enterprise audit persistence, retention, access governance, and evidence integrity.

---

# 9. Tenant and Institution Isolation

Every command, aggregate, event, read model, and audit reference shall carry tenant and institution context.

Cross-institution stock movement shall require explicit governed authorization and shall not be inferred from shared deployment or shared platform administration.

---

# 10. Domain Events

J07 publishes business facts such as:

- StockIssueRequested
- StockIssueApproved
- StockIssued
- StockReturned
- StockTransferRequested
- StockTransferApproved
- StockTransferDispatched
- StockTransferReceived
- StockTransferDiscrepancyRecorded
- StockMovementCompleted
- StockMovementCancelled

Event transport, retry, replay, routing, subscription, and registry governance remain owned by L2-A06.

---

# 11. Integration

J07 integrates with:

| Domain | Interaction | Ownership Boundary |
|---|---|---|
| J06 - Inventory & Warehouse Management | Availability, allocation references, stock ledger consequences | J06 owns inventory truth |
| J05 - Goods Receipt, Inspection & Acceptance | Stock may become movable after acceptance | J05 owns supplier receipt facts |
| J08 - Asset Lifecycle Management | Asset-linked stock movement may inform custody transfer | J08 owns asset lifecycle |
| J09 - Physical Verification, Audit & Reconciliation | Audit consumes movement evidence | J09 owns verification/audit conclusion |
| J10 - Disposal, Scrap & Write-Off | Movement exceptions may support disposal/write-off cases | J10 owns disposal case |
| Finance | Consumes movement consequences where financially relevant | Finance owns financial truth |

---

# 12. Completion Criteria

J07 is complete when:

- Stock issue, return, distribution, transfer, custody movement, and movement exception workflows are canonical.
- J07 does not own item master, warehouse, stock ledger, stock balance, asset lifecycle, goods receipt, supplier, purchase order, or finance.
- Authorization, SoD, tenant isolation, audit evidence, events, and integration contracts are explicit.
- All stock-transfer references in Section J point to J07.

---

# 13. Final Architecture Statement

J07 is the canonical owner of stock movement execution and distribution evidence.

J06 owns inventory truth; J08 owns operational asset lifecycle; Finance owns financial truth.

---

# 14. Canonical Actors

| Actor | Existing responsibility |
|---|---|
| Stock Requester | Initiates an issue, reservation, transfer, distribution, return, or other authorised movement requirement. |
| Stock Movement Approver | Applies movement-, quantity-, value-, source- and destination-sensitive authority. |
| Store Custodian / Dispatcher | Issues or dispatches authorised stock and preserves chain-of-custody evidence. |
| Recipient / Independent Receiver | Acknowledges receipt, return, distribution, custody handover, or refusal. |
| Transfer Reconciler | Independently reconciles dispatched, received and discrepant quantities where configured. |
| Controlled Stock Viewer | Accesses controlled-resource movement evidence only under explicit policy authorization. |

# 15. Canonical Entities

The canonical entity set is the aggregate set already defined in §4: `StockMovement`, `StockReservationExecution`, and `MovementException`. These entities reference, but never own, J06 Item, Warehouse, InventoryPosition or stock-balance state.

# 16. Domain Contracts

| Contract direction | Existing contract |
|---|---|
| Consumes | J06 availability/allocation and inventory references, J05 acceptance facts where stock first becomes movable, and authorised requesting-domain context. |
| Consumes platform outcomes | A03 authentication context, A04 authorization decisions, A05 workflow outcomes, A06 event transport and A07 audit persistence. |
| Publishes to J06 | Reservation, issue, return, transfer and completion facts for canonical ledger consequences. |
| Publishes to J08/J09/J10/Finance | Asset-linked custody facts, movement evidence, exceptions, and financially relevant consequences without transferring their ownership. |
| Publishes events | The stock-movement facts enumerated in §10; J07 owns meaning and timing while A06 owns transport. |

# 17. Dependencies

J07 requires J06 inventory identity, availability and allocation references before executing movement. It may consume J05 acceptance facts and the A03–A07 platform trust services. J08, J09, J10 and Finance consume J07 evidence or consequences and remain downstream canonical owners of their respective states.

# 18. Locked Decisions

1. J07 is the exclusive owner and publisher of operational stock movement and distribution facts.
2. J07 never directly changes J06 stock balances; J06 applies movement consequences to inventory truth.
3. Completed movements are corrected by reversal, return, adjustment request, or governed exception rather than destructive editing.
4. Receipt of stock does not transfer inventory ownership to the consuming department or domain.
5. Material movement preserves tenant, institution, source, destination, item, quantity, actor and chain-of-custody context.

---

## Workflows

Stock operations workflow steps are captured in Section 2 (Scope): stock reservation execution, stock issue, stock return, stock transfer request/dispatch/receipt, stock distribution, consumption recording, movement exception, movement cancellation, and movement completion, sequenced through the Canonical Lifecycle in Section 5.

## States

This document does not define a discrete state machine separate from the lifecycle flow in Section 5; movement progression is expressed through the stages Requested, Approved, Issued, Dispatched, Received, Discrepancy Recorded, Completed, and Cancelled referenced in the Domain Events (Section 10).
