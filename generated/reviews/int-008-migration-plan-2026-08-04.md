# INT-008 Migration Plan — B03 Physical Architecture into G10

**Date:** 2026-08-04 | **Status:** PLAN — no content moved yet
**Decision:** migrate (owner, 2026-08-04). Preserve B03's physical architecture by
moving it to its canonical owner rather than deleting it.

## The situation the review understated

INT-008 says "delete or rewrite B03's later physical-ownership passages". In fact:

| | L2-B03 | L2-G10 (canonical owner) |
|---|---:|---:|
| Lines | **9,642** | 3,176 |
| Top-level sections | **320** | 57 |
| Named workflows | 14 `WF-INF-*` plus per-aggregate workflow parts | workflow catalogue only |

B03 Parts 4-14 are a complete physical-infrastructure architecture that is
**more developed than G10**, the document meant to own it. Deleting it would
discard the better artifact.

## Disposition by part

| B03 Part | Lines | Content | Disposition |
|---|---:|---|---|
| 1 §§1-4 | 45-243 | Purpose, Mission, Scope, Out of Scope | **KEEP** — already the corrected institutional boundary |
| 1 §§5-8 | 244-430 | Domain Vision, Guiding Principles, Architecture Position | **REWRITE** — asserts "Campus & Infrastructure owns every physical facility" |
| 2 | 431-928 | Bounded Context, Ubiquitous Language, Responsibilities, Ownership Matrix | **SPLIT** — institutional rows stay, physical rows move |
| 3 §23 | 1026-1084 | InstitutionalCampus Aggregate | **KEEP** — this is B03's real subject |
| 3 §§24-28 | 1085-1479 | Building, Floor, Space, Infrastructure Unit, Physical Hierarchy | **MIGRATE** → G10 §8 Aggregate Design |
| 4 | 1496-2126 | Campus/Building/Floor/Space/Zone/Collection/Portfolio aggregates, value objects | **MIGRATE** → G10 §8, §10 |
| 5 | 2127-2597 | Infrastructure lifecycle & state management | **MIGRATE** → G10 §11, §40, §46 |
| 6 | 2598-3190 | Business rules, validation, constraints | **MIGRATE** → G10 §13, §47 |
| 7 | 3191-3945 | `WF-INF-001` .. `WF-INF-014` | **MIGRATE** → G10 §12 Workflow Catalogue, §16-22 |
| 8 | 3946-4618 | Campus management workflows | **SPLIT** — physical-site workflows → G10 §17; institutional registration/retirement stays |
| 9 | 4619-5432 | Building management workflows | **MIGRATE** → G10 §19 |
| 10 | 5433-6333 | Floor & space management workflows | **MIGRATE** → G10 §§20-23 |
| 11 | 6334-7022 | Zones, collections, portfolios | **MIGRATE** → G10 (new sections; no G10 equivalent today) |
| 12 | 7023-7541 | Domain events, integration contracts, read contracts | **SPLIT** — physical events → G10 §44/§45; InstitutionalCampus events stay |
| 13 | 7542-7999 | Security, authorization, audit, governance | **MIGRATE** → G10 §§48-49 |
| 14 | 8000-8598 | Data model, persistence, repository architecture | **REVIEW** — persistence design is L5 material, not L2 (separate finding) |
| 15 | 8599-9185 | Multi-domain dependencies, consumption matrix | **SPLIT** |
| 16 | 9186-9642 | Principles, quality attributes, future evolution | **SPLIT** |

Roughly **7,500 of 9,642 lines** migrate or split. B03 ends near 1,500-2,000 lines;
G10 grows from 3,176 to roughly 10,000.

## Execution order

Each step is separately reviewable and separately revertible.

1. **Aggregates** — B03 Parts 3(§§24-28) and 4 into G10 §8/§10. Establishes the
   canonical physical model in its owner. Everything else depends on this.
2. **Lifecycle and rules** — Parts 5 and 6 into G10 §11/§13/§40/§46/§47.
3. **Workflows** — Parts 7, 9, 10, 11 into G10 §§12, 16-23, plus new sections for
   zones/collections/portfolios.
4. **Events and contracts** — Part 12 split into G10 §44/§45, keeping
   InstitutionalCampus events in B03.
5. **Governance** — Part 13 into G10 §48/§49.
6. **Reduce B03** — rewrite Parts 1(§§5-8), 2, 8, 15, 16 to institutional campus
   identity only, referencing G10 physical-space IDs.
7. **Reconcile** — dedupe against G10's existing text, verify no orphaned
   cross-references, re-run the crosswalk for both documents.
8. **INT-029** — regenerate the Section B AI context from the corrected B03.

## Constraints

- G10 currently owns Campus (§17), Building (§19), Floor (§20), Space (§21-23) in
  thinner form. Migration **replaces** those sections rather than appending, or the
  result contains two treatments of each aggregate — the exact defect INT-008 names.
- B03 Part 14 (persistence and repository design) should not be in an L2 document at
  all. Flagged, not migrated, pending a separate decision.
- Both documents carry a Canonical Section Crosswalk that must be regenerated after
  the move.
- Every step preserves CRLF line endings.
