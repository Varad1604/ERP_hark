# L2-F14 — Procurement Finance Consumption Workflows — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-F14 — Procurement Finance Consumption Workflows v1.1.0 |
| Amendment Version | 1.1.1 |
| Status | Locked |
| Effective Date | 2026-08-21 |
| Approved Through | ADR-FIN-AP-001 |
| Authority | Lead Architect approval recorded 2026-08-21 |

This amendment is part of the canonical L2-F14 architecture. Where it conflicts with L2-F14 v1.1.0 or L2-F08 v1.0, this amendment and the corresponding L2-F08 amendment are authoritative.

## 1. Canonical Boundary

L2-F14 consumes procurement-owned facts from Section J. It does not own supplier/vendor identity, Purchase Requisition, Purchase Order, goods receipt or procurement-side approvals.

L2-F14 owns the Finance interpretation and governance required to turn accepted procurement evidence into a Finance-side vendor invoice approval and recognized vendor liability.

## 2. Canonical Event Contract

### Consumed from Section J

The representative Section J events already listed in L2-F14 remain source-owned. In particular, procurement acceptance facts such as `PurchaseOrderIssued`, `GoodsAccepted` and other approved Section J facts remain published by their canonical Section J owners.

### Published by L2-F14

For the payable hand-off, the following events are canonical L2-F14 publications:

| Event | Meaning |
|---|---|
| `VendorInvoiceMatched` | Finance has completed the required match between vendor invoice evidence and authoritative procurement facts. |
| `VendorInvoiceApproved` | Finance-side invoice governance has completed successfully. This event is not a Section J approval event. |
| `VendorLiabilityRecognized` | Finance has formally recognized the vendor financial obligation and made it eligible for Accounts Payable registration. |

## 3. Canonical Sequence

`Section J procurement fact(s) → VendorInvoiceMatched → VendorInvoiceApproved → VendorLiabilityRecognized → L2-F08 PayableRegistered → L2-F08 PayableValidated`

`VendorLiabilityRecognized` is the single authoritative hand-off into Accounts Payable for procurement-originating vendor liabilities.

## 4. Publisher Rules

1. Section J publishes procurement-owned facts only.
2. L2-F14 publishes `VendorInvoiceMatched`, `VendorInvoiceApproved`, and `VendorLiabilityRecognized`.
3. L2-F08 publishes `PayableRegistered` and `PayableValidated`.
4. No event listed above may be republished by another bounded context under the same canonical event name.
5. No event listed above is an alias or synonym for another; each denotes a distinct completed business outcome.

## 5. Ownership Preservation

This amendment does not transfer vendor, supplier, procurement, payable, treasury or accounting ownership. Cross-domain collaboration remains event-driven and reference-based.

## 6. Locked Decision

These event meanings, publishers and ordering are Locked under ADR-FIN-AP-001. Downstream scenarios must consume this contract without renaming or collapsing its events.
