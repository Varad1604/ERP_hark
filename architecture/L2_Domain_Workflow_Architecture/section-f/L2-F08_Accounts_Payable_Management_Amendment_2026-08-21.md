# L2-F08 — Accounts Payable Management — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-F08 — Accounts Payable Management v1.0 |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-21 |
| Approved Through | ADR-FIN-AP-001 |
| Authority | Lead Architect approval recorded 2026-08-21 |

This amendment is part of the canonical L2-F08 architecture. Where it conflicts with L2-F08 v1.0, this amendment is authoritative.

## 1. Procurement-to-Payable Boundary

Section J remains canonical owner of procurement, supplier/vendor, purchasing, purchase-order and goods/receipt business outcomes. Finance does not republish those Section J facts under a different owner.

L2-F14 is the Finance procurement-consumption boundary. It consumes authoritative Section J procurement facts and owns the Finance-side invoice/financial-obligation interpretation that follows those facts.

Accounts Payable begins only after an authoritative Finance liability-recognition fact has been established.

## 2. Canonical Event Publishers and Meanings

| Event | Canonical publisher | Business meaning |
|---|---|---|
| `VendorInvoiceApproved` | L2-F14 — Procurement Finance Consumption | Finance-side vendor invoice has completed the Finance approval/matching governance required before liability recognition. It is not a Section J procurement-approval event. |
| `VendorLiabilityRecognized` | L2-F14 — Procurement Finance Consumption | Finance has formally established the vendor financial obligation eligible for Accounts Payable registration. This is the canonical hand-off into L2-F08. |
| `PayableRegistered` | L2-F08 — Accounts Payable | AP has created the canonical Payable representation referencing the recognized liability. |
| `PayableValidated` | L2-F08 — Accounts Payable | The registered Payable has passed AP integrity and lifecycle validation and is eligible for normal AP governance. |

Exactly one publisher exists for each event. None of these events are synonyms.

## 3. Canonical Ordering

For procurement-originating vendor liabilities the authoritative sequence is:

`Section J procurement facts → VendorInvoiceApproved → VendorLiabilityRecognized → PayableRegistered → PayableValidated`

The first two events are Finance procurement-consumption facts owned by L2-F14. The latter two are AP lifecycle facts owned by L2-F08.

`VendorLiabilityRecognized` is the authoritative cross-domain hand-off into Accounts Payable. L2-F08 shall not treat `VendorInvoiceApproved` as a Procurement-published trigger.

## 4. Ownership Rules

- Vendor/supplier identity remains owned by Section J.
- Procurement documents and procurement-side approvals remain owned by Section J.
- Finance-side invoice matching/approval and liability recognition at the procurement-finance boundary remain owned by L2-F14.
- The Payable Aggregate, outstanding liability representation, due schedule, ageing, payment readiness and AP lifecycle remain owned by L2-F08.
- Treasury owns settlement execution; Accounting owns journals/ledger consequences.
- No direct aggregate mutation crosses these boundaries.

## 5. Locked Decisions

The event publishers, meanings, ordering and hand-off rule in this amendment are Locked under ADR-FIN-AP-001 and supersede any L2-F08 wording that describes `VendorInvoiceApproved` as a Procurement-published event.

L3-F-044 through L3-F-050 shall consume this sequence without merging or renaming the events.
