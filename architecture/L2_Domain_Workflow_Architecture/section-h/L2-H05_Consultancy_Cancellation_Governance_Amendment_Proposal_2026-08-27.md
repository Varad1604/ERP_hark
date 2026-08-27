# L2-H05 — Research Workflows — Consultancy Cancellation Governance Amendment Proposal

| Attribute | Value |
|---|---|
| Target | L2-H05 — Research Workflows v1.0 plus locked 2026-08-26 H05 governance amendment |
| Proposed Amendment Version | 1.2.0 |
| Status | Archived |
| Date | 2026-08-27 |
| Owner | Lead Architect |
| Architectural Level | L2 — Domain Workflow Architecture |
| Domain | H05 — Research Management |
| Triggering Gap | `L3-H-037` frozen Consultancy cancellation outcome |
| Superseded By | `L2-H05_Consultancy_Cancellation_Governance_Amendment_2026-08-27.md` |

---

## 1. Historical Purpose

This proposal resolved the remaining L2-H05 gap identified by `L3-H-037`: the frozen L3 outcome permits a governed Consultancy cancellation outcome, while earlier locked H05 architecture defined approval, execution, delivery acceptance, Finance-confirmed reconciliation and closure but no canonical Consultancy cancellation branch.

The proposal was explicitly approved by repository authority on 2026-08-27 and incorporated into the Locked supplemental amendment referenced above. This file is retained only as historical proposal evidence.

---

## 2. Approved Proposal Content

Ownership remained unchanged: H05 owns the Research Consultancy aggregate and Research lifecycle; Finance owns monetary/accounting/reconciliation truth; E03 owns workforce facts.

Approved cancellation branch:

`Approved / Active / Delivery Submitted → Cancellation Requested → Cancelled`

Approved semantics:

- cancellation is allowed only before `Delivery Accepted`;
- `Cancelled` is terminal for that Consultancy record;
- prior history is immutable;
- Research Director owns institutional cancellation approval where required;
- Finance consequences remain Finance-owned;
- H05 publishes `ResearchConsultancyCancellationRequested` and `ResearchConsultancyCancelled`.

---

## 3. Current Decision Status

**PROPOSAL STATUS:** ARCHIVED  
**CANONICAL SOURCE:** `L2-H05_Consultancy_Cancellation_Governance_Amendment_2026-08-27.md`  
**APPROVAL RECORD:** `governance/adr/H05_H09_H10_UPSTREAM_REMEDIATION_APPROVAL_RECORD_2026-08-27.md`
