# L2-H05 — Research Workflows — Consultancy Cancellation Governance Amendment Proposal

| Attribute | Value |
|---|---|
| Target | L2-H05 — Research Workflows v1.0 plus locked 2026-08-26 H05 governance amendment |
| Proposed Amendment Version | 1.2.0 |
| Status | Review |
| Date | 2026-08-27 |
| Owner | Lead Architect |
| Architectural Level | L2 — Domain Workflow Architecture |
| Domain | H05 — Research Management |
| Triggering Gap | `L3-H-037` frozen Consultancy cancellation outcome |
| Canonical Effect | **None until independently reviewed and approved/locked** |

---

## 1. Purpose

This proposal resolves the remaining L2-H05 gap identified by `L3-H-037`: the frozen L3 outcome permits a governed Consultancy cancellation outcome, while current locked H05 architecture defines approval, execution, delivery acceptance, Finance-confirmed reconciliation and closure but no canonical Consultancy cancellation branch.

This document is **Review-status architecture only**. It does not amend locked H05 until formally approved through repository change control.

---

## 2. Ownership Preservation

Existing ownership remains unchanged:

- H05 owns the Research Consultancy aggregate, its operational lifecycle, delivery evidence and Research-owned lifecycle events.
- Finance remains sole owner of accounting, payment, receivable, settlement, posting and financial-reconciliation truth.
- E03 remains owner of workforce capability, eligibility, availability and capacity facts.
- Cancellation of a Research Consultancy shall not create, reverse or certify any Finance-owned monetary fact.

---

## 3. Proposed Consultancy Cancellation Branch

The existing successful Research-side progression remains:

`Approved → Active → Delivery Submitted → Delivery Accepted → Financial Reconciliation Pending → Closed`

Proposed governed cancellation branch:

`Approved / Active / Delivery Submitted → Cancellation Requested → Cancelled`

### Meaning

- `Cancellation Requested` records a governed request to stop the Research Consultancy before accepted delivery has established the normal completion path.
- `Cancelled` is the terminal Research-side outcome for an engagement that is governedly discontinued before `Delivery Accepted`.
- `Cancelled` preserves all prior approval, execution, deliverable and decision evidence.
- Once `Delivery Accepted` has been reached, normal reconciliation/closure governance applies; this proposal does not permit local cancellation to erase accepted-delivery or Finance obligations.

---

## 4. Proposed Business Authority

- The Principal Investigator, Consultancy Team, authorized institutional sponsor, or other source-supported participant may originate a cancellation request where policy permits.
- The **Research Director** is the canonical Research business authority for approving or rejecting a Consultancy cancellation where institutional approval is required.
- The Research Administrator records/administers the decision but does not replace the Research Director's authority.
- Any monetary consequence is separately governed by Finance and is consumed by H05 only as authoritative reference/status where required.

---

## 5. Proposed Business Rules

1. Cancellation is permitted only before `Delivery Accepted`.
2. Cancellation shall not overwrite prior execution or deliverable history.
3. Cancellation does not imply that Finance-owned balances, receivables, payments, refunds, write-offs or settlement obligations are complete or reversed.
4. Where cancellation creates a financial consequence, H05 records only the applicable Finance-owned confirmation/reference.
5. A `Cancelled` Consultancy is terminal for that Consultancy record; any later engagement requires a new governed Consultancy record unless future locked architecture explicitly defines reopening.
6. Cancellation authority and rationale shall be auditable.

---

## 6. Proposed Canonical H05 Events

If approved, H05 would be the sole semantic publisher of:

- `ResearchConsultancyCancellationRequested`
- `ResearchConsultancyCancelled`

These events describe Research-owned Consultancy lifecycle facts only and do not republish Finance-owned accounting or settlement facts.

---

## 7. Downstream Effect

If this proposal is formally approved and locked:

- `L3-H-037` may be mechanically synchronized to the canonical cancellation branch;
- its frozen cancellation outcome becomes source-supported;
- the scenario may then return to independent Architecture Approval review;
- no other H05 ownership or successful Consultancy progression changes.

---

## 8. Review Questions

Independent architecture review shall confirm:

1. whether cancellation should remain prohibited after `Delivery Accepted`;
2. whether the proposed Research Director authority is consistent with all current H05 governance;
3. whether the exact proposed events are sufficient and non-duplicative; and
4. whether any existing base-H05 completion/administrative-closure wording requires explicit supersession language.

---

## 9. Decision Status

**PROPOSAL STATUS:** REVIEW  
**LOCKED/CANONICAL EFFECT:** NONE  
**L3-H-037:** REMAINS UPSTREAM-BLOCKED UNTIL THIS OR AN ALTERNATIVE RESOLUTION IS APPROVED
