# L2-C04 — Student Academic Progression Workflows — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-C04 — Student Academic Progression Workflows |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-22 |
| Approved Through | ADR-STU-PROG-EVT-001 |
| Authority | Lead Architect approval recorded 2026-08-22 |

This amendment is part of the canonical L2-C04 architecture. Where this amendment conflicts with the original L2-C04 document, this amendment is authoritative for the amended event-vocabulary subject only. Unaffected L2-C04 content remains unchanged.

## 1. Canonical Progression Outcome Events

The following names are the sole canonical published events for their respective business outcomes:

| Business Outcome | Canonical Event | Canonical Publisher |
|---|---|---|
| Conditional promotion approved | `ConditionalPromotionGranted` | Academic Progression |
| Academic probation assigned | `AcademicProbationAssigned` | Academic Progression |
| Academic detention finalized | `AcademicDetentionApplied` | Academic Progression |

## 2. Superseded Synonyms

The following names appearing in earlier sections of the original L2-C04 text are superseded as canonical publication names:

| Superseded Name | Canonical Name |
|---|---|
| `StudentConditionallyProgressed` | `ConditionalPromotionGranted` |
| `StudentPlacedOnProbation` | `AcademicProbationAssigned` |
| `StudentDetained` | `AcademicDetentionApplied` |

The superseded names do not represent additional business events and must not be published independently by downstream architecture.

## 3. Publisher and Aggregate Boundary

Academic Progression is the canonical publisher of all three events.

The existence of Academic Probation and Academic Detention aggregates does not create duplicate cross-domain publishers. Their lifecycle ownership remains governed by the original L2-C04 architecture; this amendment only resolves the canonical published-event identity and publisher boundary.

## 4. Downstream Rule

L3 scenarios concerning conditional promotion, probation, detention, progression revision, and related cross-domain collaboration shall use the canonical event names in Section 1. They shall not preserve the superseded names as parallel canonical events.

## 5. Unaffected Architecture

No other L2-C04 event, workflow, aggregate, lifecycle state, rule, domain ownership, or dependency is changed by this amendment.
