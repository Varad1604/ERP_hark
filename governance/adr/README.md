# Architecture Decision Records

`governance/constitution/Volume_5_Change_Management.md` §8 requires every significant architectural decision to be documented as an Architecture Decision Record, but states ADR folders or files "shall not be referenced as existing repository paths until they are created." This folder is that creation. Nothing above this folder needs to change: existing documents that anticipate `governance/adr/` now resolve correctly.

## Purpose

An ADR is the durable, reviewable artifact for a proposed amendment to a **Locked** L1/L2 document. It exists to turn a "Need" (a gap, contradiction, or blocking dependency discovered while authoring L3 or below) into a "Decision" that the named repository roles can actually approve or reject, per the Architectural Change Process in Volume 5 §6:

```text
Need → Proposal → Impact Analysis → Architecture Review → Decision → Repository Update → Validation
```

An ADR is not itself authoritative. It carries no decision effect until the named approval roles (Volume 5 §5, §9) approve it and the corresponding Locked document is amended and re-locked. Until then it is `Status: Proposed — Non-Authoritative`, exactly like the document it may amend remains `Locked` and unchanged.

## Format

One ADR per decision package. File name: `ADR-<PACKAGE>-<NNN>.md`, e.g. `ADR-FIN-ADJ-001.md`. Each ADR contains:

- **Document metadata** — ADR ID, target document(s) amended, status, version, domain owner, date.
- **Need** — the specific gap or contradiction, with exact section references in the target document(s), and which downstream L3 (or lower) scenarios are blocked by it.
- **Proposal** — the actual drafted answer: the content that would fill the gap if approved. Every proposed answer must cite which existing architectural principle or section it extends, not merely assert a new one.
- **Impact Analysis** — cross-check against L1, `DEPENDENCY_MAP.md`, and any other Locked document that consumes or depends on the target document.
- **Decision** — left blank until the named approval roles record a decision. An ADR with no Decision entry has not been approved and authorizes no repository change.
- **Acceptance checklist** — a explicit checklist against whatever standard governs completeness for that decision (e.g., an upstream proposal document's own acceptance criteria, or `governance/policies/Architecture_Level_Validation_Standards.md`).

## Rules

- ADR history is immutable (Volume 5 §8). Once an ADR is merged, it is not edited to reverse a decision — a superseding ADR is written instead and the two are cross-referenced.
- An ADR does not itself amend the Locked document it targets. The Repository Update step is a separate, subsequent change once Decision is recorded as approved.
- Drafting an ADR is not architecture authorship by the drafter. The proposal section is a candidate for review, not a fait accompli; it must be clearly distinguishable from confirmed canonical content in the target document.

## Current ADR Index

This index satisfies the continuous ADR-indexing obligation in Repository Constitution Volume 5 §13. The ADR file itself remains the authority for status and decision detail; this table is navigational and must not be used to infer approval where the ADR does not record it.

| ADR / Record | Package / Subject |
|---|---|
| `ADR-FIN-ADJ-001.md` | Finance — Financial Adjustment Governance |
| `ADR-FIN-ADJ-002.md` | Finance — Financial Adjustment follow-on decision |
| `ADR-FIN-AP-001.md` | Finance — Accounts Payable |
| `ADR-FIN-AR-001.md` | Finance — Accounts Receivable |
| `ADR-FIN-GL-001.md` | Finance — General Ledger |
| `ADR-FIN-PERIOD-001.md` | Finance — Financial Period |
| `ADR-FIN-TREAS-001.md` | Finance — Treasury |
| `ADR-STU-ADM-EXP-001.md` | Student Lifecycle — Admission Expiry Lifecycle |
| `ADR-STU-PROG-EVT-001.md` | Student Lifecycle — Academic Progression Event Canonicalization |
| `ADR-STU-MEME-EXIT-001.md` | Student Lifecycle — NEP/MEME Intermediate Qualification Exit |
| `FIN_UPSTREAM_L2_APPROVAL_RECORD_2026-08-21.md` | Finance upstream L2 approval/incorporation record |
| `STUDENT_LIFECYCLE_UPSTREAM_L2_APPROVAL_RECORD_2026-08-22.md` | Student Lifecycle upstream L2 approval/incorporation record |

When new ADR files or approval records are added, this index must be updated as part of repository maintenance.
