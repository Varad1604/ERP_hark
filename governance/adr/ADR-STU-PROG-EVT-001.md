# ADR-STU-PROG-EVT-001 — Canonical Academic Progression Outcome Events

| Attribute | Value |
|---|---|
| Status | Approved |
| Date | 2026-08-22 |
| Architecture Level | L2 — Business Workflow Architecture |
| Domain | Section C — Student Lifecycle / Academic Progression |
| Decision Authority | Lead Architect authorization recorded in the architecture working session |
| Trigger | L3 Phase 5 architecture review of progression scenarios |

## 1. Context

Locked `L2-C04_Student_Academic_Progression_Workflows.md` contains competing event names for the same progression outcomes.

An earlier event definition/ownership set uses:

- `StudentConditionallyProgressed`
- `StudentPlacedOnProbation`
- `StudentDetained`

Later workflow/event-publication tables, including the explicit Event ID / Canonical Publisher table, use:

- `ConditionalPromotionGranted`
- `AcademicProbationAssigned`
- `AcademicDetentionApplied`

The latter names are also used consistently in the workflow-to-event map and business-event classification. Level 3 requires exactly one canonical event identity for each business fact.

## 2. Decision

The canonical published event names are:

| Business Outcome | Canonical Event | Canonical Publisher |
|---|---|---|
| Conditional promotion approved | `ConditionalPromotionGranted` | Academic Progression |
| Academic probation assigned | `AcademicProbationAssigned` | Academic Progression |
| Academic detention finalized | `AcademicDetentionApplied` | Academic Progression |

The earlier names are superseded as canonical publication names:

- `StudentConditionallyProgressed` → `ConditionalPromotionGranted`
- `StudentPlacedOnProbation` → `AcademicProbationAssigned`
- `StudentDetained` → `AcademicDetentionApplied`

They may remain visible in the historical original L2 text, but downstream architecture shall not treat them as separate canonical events.

## 3. Ownership

Academic Progression remains the single canonical publisher for all three events.

The Academic Probation and Academic Detention aggregates may own their respective internal business lifecycles, but this ADR does not create a second publisher. The canonical cross-domain publication boundary remains Academic Progression as established by L2-C04's canonical publisher table.

## 4. Unaffected Events

This decision does not rename or alter other L2-C04 events, including:

- `StudentProgressed`
- `AcademicStandingUpdated`
- `CreditLedgerUpdated`
- `CreditTransferRecognised`
- `CreditTransferRejected`
- `ProgrammeMigrationApproved`
- `AcademicBreakApproved`
- `StudentRejoined`
- `AcademicCreditsSynchronized`

## 5. Downstream Rule

L3 and later architecture shall use only the canonical names defined in Section 2 for those three outcomes. Any downstream use of the superseded names shall be reconciled during controlled remediation.

## 6. Implementation in Architecture Repository

This ADR is incorporated through:

`architecture/L2_Domain_Workflow_Architecture/section-c/L2-C04_Student_Academic_Progression_Workflows_Amendment_2026-08-22.md`

The amendment has precedence only for the event-vocabulary conflict addressed by this ADR. All unaffected L2-C04 content remains authoritative.
