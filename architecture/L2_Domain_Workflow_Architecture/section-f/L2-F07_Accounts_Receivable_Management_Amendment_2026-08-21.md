# L2-F07 — Accounts Receivable Management — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-F07 — Accounts Receivable Management v1.0.0 |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-21 |
| Approved Through | ADR-FIN-AR-001 |
| Authority | Lead Architect approval recorded 2026-08-21 |

This amendment is part of the canonical L2-F07 architecture. Where it conflicts with L2-F07 v1.0.0, this amendment is authoritative. Unaffected source content remains unchanged.

## 1. Canonical Actors and Authorities

| Actor / authority | Business responsibility | Boundary |
|---|---|---|
| Receivable Registration Authority | Registers the Receivable representation after a valid source-owned financial obligation event is received. | Cannot originate the source obligation. |
| Receivable Validation Authority | Confirms obligation reference, amount, classification, institution and receivable integrity before recognition. | Cannot alter the source obligation. |
| Receivable Review Authority | Reviews exceptions affecting recognition, ageing, collection status or settlement interpretation. | Review only; corrective financial actions remain with their canonical owners. |
| Collection Review Authority | Governs Accounts Receivable collection classification and escalation status. | Does not execute communication workflows or create penalties. |
| Settlement Reconciliation Authority | Confirms that authoritative settlement facts have been incorporated into the Receivable representation. | Payment/Treasury settlement remains externally owned. |
| Write-Off Referral Authority | Refers an eligible receivable to the applicable Finance adjustment/write-off governance. | Does not self-authorize or execute the write-off. |
| Reopening Review Authority | Confirms that an authoritative Finance fact justifies reopening a previously resolved Receivable. | Reopening must be evidence-driven; no direct source mutation. |

Institutional authorization may map these responsibilities to local roles, but the business responsibilities and separation boundaries are canonical.

## 2. Obligation Recognition Contract

Accounts Receivable never creates the originating obligation. A Receivable is registered only after the canonical owner of the source obligation publishes an established obligation business fact.

### 2.1 Canonical exact event already established

| Obligation source | Consumed event | Publisher | AR effect |
|---|---|---|---|
| Student Charge Management | `ChargeEstablished` | Student Charge Management | Eligible to register the corresponding Receivable representation after validation. |

`ChargeRevised` remains a source-owned change fact affecting an existing Receivable representation; it is not a substitute for the initial `ChargeEstablished` trigger.

### 2.2 Other obligation classes

For Debit Note, Vendor Recovery, Institutional Receivable, Sponsor Receivable or any future obligation source, L2-F07 shall consume the **exact canonical obligation-established event published by that source owner**. If the source L2 architecture has not yet named that event, the contract remains explicitly unresolved for that source and L2-F07 shall not invent a name.

The illustrative event labels in the original Aggregate Creation Sources list are therefore not canonical event contracts unless independently established by their owning source documents.

## 3. Recognition and Ownership Rule

Receivable recognition creates the Finance-owned Receivable representation only. It never transfers, duplicates or supersedes ownership of the originating financial obligation.

Every Receivable shall preserve an immutable reference to exactly one originating obligation. Subsequent settlement, refund and adjustment facts update only the Receivable representation in accordance with L2-F07 invariants.

## 4. Separation of Duties

- Registration and validation must be independently governable responsibilities.
- Write-off referral does not constitute write-off approval.
- Reopening requires an authoritative Finance fact and independent review where institutional policy requires it.
- No AR actor may directly modify Student Charge, Payment, Refund, Financial Adjustment, Accounting or Treasury aggregates.

## 5. Locked Decisions

These actor, authority, event-contract and ownership rules are Locked under ADR-FIN-AR-001. L3-F-039 through L3-F-043 shall consume them and shall continue to mark any source-specific event as unresolved where its canonical publisher has not named the exact event.
