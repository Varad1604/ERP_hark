# SutraERP L6 — Stage 3 Batch 1 Canonical Seed Execution Record — 2026-09-21

**Version:** 1.0.0  
**Status:** Architecture Baseline  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Programme:** SutraERP Full Workflow Realization & Mock ERP Programme  
**Stage:** Stage 3 — Coherent Mock ERP Foundation  
**Batch:** Batch 1 — Canonical Fictional Institutional Seed + Deterministic Integrity Validator  
**Depends On:** `L6_Stage3_Coherent_Mock_ERP_Foundation_Reviewed_Gameplan_2026-09-21.md`; locked L1 canonical Person amendment; current frontend Days 12–13 baseline.

---

## 1. Batch 0 Baseline

Batch 0 had previously passed. Batch 1 rechecked the live repositories before implementation and found no drift.

```text
Architecture main:        af2fdf5c79a0f60efb77c9bb1a11c48fc20c52b5
Architecture Stage-3:     af2fdf5c79a0f60efb77c9bb1a11c48fc20c52b5
Frontend main:            e2560877eac4446b925e45f04b7dfa4c24d3625c
Frontend Stage-3 start:   e2560877eac4446b925e45f04b7dfa4c24d3625c
Drift classification:     SAFE / IDENTICAL
```

No work occurred directly on either `main` branch.

---

## 2. Batch 1 Frontend Revision

Validated frontend Stage-3 revision:

```text
bfc1977514cca5736376619a2222d841c15fc589
```

Controlled branch:

```text
stage3-coherent-mock-erp-foundation
```

PR evidence only:

```text
PR #11 — Stage 3 Batch 1 — canonical fictional seed and integrity validator
Not merged.
```

---

## 3. Implemented Scope

Batch 1 added only:

- deterministic canonical seed types;
- one deterministic canonical seed;
- a fresh-copy + deep-freeze seed constructor;
- one fixed fixture reference time;
- a lightweight seed integrity validator;
- the `npm run mock:validate` command;
- the mock validator in the existing frontend PR validation workflow.

Batch 1 did not add mutable demo state, provider/reducer state, reset behavior, presets, persistence, workflow simulation, fake API/service architecture, backend code, database design, or bulk fixture migration.

---

## 4. Seed Population

```text
Tenants                         1
Institutions                    1
Campuses                        1
Organisation units              2
Academic years                  1
Academic periods                1
Programmes                      1
Programme versions              1
Courses                         2
Course offerings                2
Persons                         2
Student contexts                1
Workforce contexts              1
Faculty contexts                1
Academic assignments            2
Minimal cross-domain records    6
```

Minimal cross-domain records comprise one each for timetable, attendance, assessment, examination, finance, and documents/history context.

---

## 5. Shared Subject Coherence

### Aarav Desai

```text
Person person-aarav-desai
  -> Student Context student-context-aarav-sit
  -> Programme Version programme-version-btech-cse-2026
  -> Academic Period academic-period-2026-27-sem1
  -> Attendance / Assessment / Examination / Finance / Document contexts
```

### Dr. Meera Shah

```text
Person person-meera-shah
  -> Workforce Context workforce-context-meera-sit
  -> Faculty Context faculty-context-meera-sit
  -> Academic Assignments
  -> CS201 / CS203 course offerings
  -> Timetable context
```

No separate Person record is created for Student, Finance, Examination, Attendance, Workforce, or Faculty projections.

Mock IDs are fixture-only references and do not propose production identifier formats.

---

## 6. Deterministic Reference Time

```text
2026-09-16T10:30:00+05:30
```

This aligns the Batch-1 seed with the existing fixed review date of Wednesday, 16 September 2026.

No runtime wall clock, random UUID, or random value determines canonical seed content.

---

## 7. Integrity Validation

CI run:

```text
Frontend PR Validation
Run: 35566886007
Head: bfc1977514cca5736376619a2222d841c15fc589
Conclusion: SUCCESS
```

`npm run mock:validate` reported:

```text
duplicateIds                    0
danglingReferences              0
canonicalPersonViolations       0
invalidAcademicReferences       0
crossDomainCoherenceFailures    0
determinismFailures             0
```

A separate repository-content review also found:

```text
missing IDs                             0
duplicate canonical identity keys       0
copied identity fields outside Person   0
cross-domain subject coherence          PASS
```

The canonical seed wrapper constructs a fresh copy and deep-freezes it before exposing it to consumers.

---

## 8. Existing Frontend Gates

All required gates passed on the validated revision:

```text
npm run review:validate   PASS
npm run mock:validate     PASS
npm run typecheck         PASS
npm run build             PASS
```

Review validation retained:

```text
99 workspace families
12 UX archetypes
81 shell-level implementations
18 interactive implementations
99 presentation-only integration maturity
0 integrated maturity
0 production-validated maturity
```

No maturity upgrade was introduced.

---

## 9. Architecture Review

Independent Batch-1 review found:

- no new canonical business entity was invented;
- no ownership transfer to frontend occurred;
- no lifecycle state or transition was invented;
- no business rule, approval authority, permission model, or event publisher was invented;
- canonical Tenant-scoped Person identity is preserved;
- Student Context and Faculty/Workforce contexts reference Person rather than duplicate identity;
- academic references resolve;
- the seed remains deliberately small;
- no generic entity framework or state library was introduced;
- Batch 2 / Batch 3 responsibilities were not started;
- the validator checks fixture structure/coherence rather than business policy;
- ordinary UX was not changed and no architecture IDs were added to end-user surfaces.

**No L0–L5 remediation required.**

---

## 10. Deferred Work / Migration Debt

### Batch 2
Shared read/projection/selectors shall begin consuming the canonical seed and prove the same Student Context across Student, Attendance, Examination, and Finance views.

### Batch 3
Controlled frontend-only mutable demo state and deterministic reset remain deferred.

### Batch 4+
Mock clock generalization, tasks, notifications, exceptions, history/activity, Experience Lab controls, alternate preset(s), and existing fixture convergence remain deferred.

Existing fixture modules may temporarily coexist with the canonical Batch-1 seed. Their convergence is explicit Stage-3 migration debt, not duplicate canonical architecture.

---

## 11. Verdict

```text
STAGE 3 — BATCH 1 PASS
```

Batch 2 is not authorized by this record and was not started.
