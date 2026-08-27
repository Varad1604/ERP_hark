# L2-H05 — Research Workflows — Consultancy Cancellation Governance Amendment

| Attribute | Value |
|---|---|
| Target | L2-H05 — Research Workflows v1.0 plus locked 2026-08-26 H05 governance amendment |
| Amendment Version | 1.2.0 |
| Status | Locked |
| Effective Date | 2026-08-27 |
| Approved Through | ADR-H-CONSULT-CANCEL-001 |
| Authority | Explicit Lead Architect / repository-authority approval recorded 2026-08-27 in `H05_H09_H10_UPSTREAM_REMEDIATION_APPROVAL_RECORD_2026-08-27.md` |

This amendment is canonical L2-H05 architecture for the Consultancy cancellation subject. It supplements, and does not replace, the 2026-08-26 H05 governance amendment. Unaffected H05 content remains unchanged.

## 1. Ownership Preservation

- H05 owns the Research Consultancy aggregate, operational lifecycle, delivery evidence and Research-owned lifecycle events.
- Finance remains sole owner of accounting, payment, receivable, settlement, posting and financial-reconciliation truth.
- E03 remains owner of workforce capability, eligibility, availability and capacity facts.
- Cancellation of a Research Consultancy does not create, reverse or certify any Finance-owned monetary fact.

## 2. Consultancy Cancellation Lifecycle

The existing successful Research-side progression remains:

`Approved → Active → Delivery Submitted → Delivery Accepted → Financial Reconciliation Pending → Closed`

Canonical cancellation branch:

`Approved / Active / Delivery Submitted → Cancellation Requested → Cancelled`

### Meanings

- `Cancellation Requested` records a governed request to stop the Research Consultancy before accepted delivery establishes the normal completion path.
- `Cancelled` is the terminal Research-side outcome for an engagement governedly discontinued before `Delivery Accepted`.
- `Cancelled` preserves all prior approval, execution, deliverable and decision evidence.
- Once `Delivery Accepted` has been reached, normal reconciliation/closure governance applies; cancellation shall not erase accepted-delivery or Finance obligations.

## 3. Business Authority

- The Principal Investigator, Consultancy Team, authorized institutional sponsor, or other source-supported participant may originate a cancellation request where policy permits.
- The **Research Director** is the canonical Research business authority for approving or rejecting Consultancy cancellation where institutional approval is required.
- The Research Administrator records/administers the decision but does not replace the Research Director's authority.
- Monetary consequences remain separately governed by Finance and are consumed by H05 only as authoritative reference/status where required.

## 4. Business Rules

1. Cancellation is permitted only before `Delivery Accepted`.
2. Cancellation shall not overwrite prior execution or deliverable history.
3. Cancellation does not imply that Finance-owned balances, receivables, payments, refunds, write-offs or settlement obligations are complete or reversed.
4. Where cancellation creates a financial consequence, H05 records only the applicable Finance-owned confirmation/reference.
5. A `Cancelled` Consultancy is terminal for that Consultancy record; any later engagement requires a new governed Consultancy record unless future locked architecture explicitly defines reopening.
6. Cancellation authority and rationale shall be auditable.

## 5. Canonical H05 Events

H05 is the sole semantic publisher of:

- `ResearchConsultancyCancellationRequested`
- `ResearchConsultancyCancelled`

These events describe Research-owned Consultancy lifecycle facts only and do not republish Finance-owned accounting or settlement facts.

## 6. Downstream Effect

This amendment resolves the upstream gap identified by `L3-H-037` and authorizes scenario-level synchronization to the canonical cancellation branch, authority and exact event vocabulary.

## 7. Locked Decisions

The lifecycle, meanings, authority, rules, Finance boundary and events in §§1–5 are Locked L2-H05 decisions under ADR-H-CONSULT-CANCEL-001.
