# L2-J07 - Stock Operations & Distribution

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
| Status | Trust Certification Ready |
| Version | 1.1 |

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
