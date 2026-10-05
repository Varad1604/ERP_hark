# L2-J00 — Proposed J05 Variance Event Vocabulary Clarification

| Field | Value |
|---|---|
| Target | Locked L2-J00 §27.7 J05 event catalog row only |
| Proposed patch version | 1.0.1 |
| Status | PROPOSED — not part of canonical Locked baseline until Section J/A06 review and recorded Lead decision |
| Date | 2026-10-03 |
| Decision record | governance/adr/ADR-B18-W4-J05-EVENT-001_Receipt_Variance_Event_Naming_2026-10-03.md |

**Proposed replacement of one catalog row, only upon approval:**

| Event | Meaning |
|---|---|
| `ReceiptVarianceRecorded` | J05-owned governed receipt quantity/specification variance, including an actually authorized non-destructive supplemental correction when L3-J-050 prerequisites are independently satisfied |

The original L2-J00 §27.7 currently reads `GoodsReceiptVarianceRecorded`; specialized Locked L2-J05 §6, Locked L3-J-050 and downstream documents read `ReceiptVarianceRecorded`. No separate J05 business publisher, new event or automatic event-name alias is created. **This patch is not active until governed review and approval.** Actual publishability additionally requires A06 verification of the active registered definition/version/schema and affected consumer compatibility. Do not represent this text as a released A06 registry artifact.
