# SutraERP L5 — Repository Structure and Content Placement Decision

**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Lead Architect  
**Architectural Level:** L5 — Solution Architecture Governance  
**Domain:** Cross-domain  
**Decision Date:** 2026-09-09  

**Depends On:** L5 Architecture Family Register; L5 Naming and Document Granularity Specialization; L5 Governance and Architecture Standard; L5 Authoring Population Freeze Decision 105; L5 Broad Authoring Authorization 105; Repository Constitution Volume 2 — Repository Standards.

---

## 1. Decision

The canonical L5 repository structure shall separate governance, canonical production specifications, ADRs, and historical/non-canonical technical material.

```text
architecture/L5_Solution_Architecture/
├── README.md
├── governance/
├── specifications/
├── historical/
└── adr/                  # materialize only when the first canonical ADR exists
```

The existing `governance/` directory remains the canonical governance location and shall remain flat unless a later governance decision justifies subdivision.

---

## 2. Governance Placement

The following document classes belong under `governance/`:

- governance and architecture standards;
- validation standards;
- specification templates;
- naming/granularity rules;
- architecture-family and major-area registers;
- candidate/frozen corpus registers and decisions;
- L4-to-L5 coverage and traceability methodology;
- ADR methodology and ADR identity register;
- authoring-population freeze decisions;
- workload/allocation decisions;
- execution-plan and authoring-authorization decisions;
- repository-structure decisions.

Governance records shall not be mixed with production L5 specifications.

---

## 3. Canonical Specification Placement

All newly authored canonical L5 solution-architecture specifications shall be placed under:

```text
architecture/L5_Solution_Architecture/specifications/
```

Specifications shall use the approved canonical filename convention:

```text
L5-<Family>-<Area>-<Document>_<Canonical_Solution_Slug>.md
```

The seven approved L5 families remain the primary organizational model. Family-specific child directories may be materialized when the first real production specification for that family is committed. Empty family directories shall not be created merely as placeholders.

When family directories are materialized, their canonical directory names shall be the stable family identifiers:

```text
specifications/L5-A/
specifications/L5-B/
specifications/L5-C/
specifications/L5-D/
specifications/L5-E/
specifications/L5-F/
specifications/L5-G/
```

A specification's directory does not alter its canonical technical ownership, identifier, or cross-family dependencies.

---

## 4. Historical Placement

Historical extraction documents, superseded technical baselines, and technical material preserved only as provenance shall be placed under:

```text
architecture/L5_Solution_Architecture/historical/
```

Historical material:

- is not part of the frozen 105-specification production corpus unless separately re-authored and approved under a canonical L5 identifier;
- shall not become automatic technology or architecture authority;
- shall retain its provenance and original status/history;
- may inform feasibility only subject to current L5 governance and ADR rules.

The historical extraction artifacts `L5-00_Technology_Stack_Baseline.md` and `L5-01_Infrastructure_Persistence_Baseline.md` belong in this location.

---

## 5. ADR Placement

Canonical ADR documents shall be stored under:

```text
architecture/L5_Solution_Architecture/adr/
```

The directory shall not be materialized while the canonical ADR register contains no allocated ADR document. The existing `L5_ADR_Register.md` remains under `governance/` because it is a governance/identity register rather than an ADR decision document.

---

## 6. No-Placeholder Rule

Repository Constitution Volume 2 prohibits meaningless empty architecture folders.

Therefore:

- `specifications/` is materialized with a substantive routing README because broad L5 authoring is now authorized;
- `historical/` is materialized with real historical content;
- family child folders are created only when their first real specification is committed;
- `adr/` is created only when its first canonical ADR is allocated and authored.

---

## 7. Compatibility and Reference Integrity

Moving historical material shall not silently break references from locked upstream documents or historical repository evidence.

Where old root paths are still referenced by locked or historical records, lightweight compatibility-pointer files may remain temporarily at the former paths. Such pointer files are not canonical L5 specifications and shall be removed only after repository-wide reference migration is safely completed.

Historical changelog and review evidence shall not be rewritten merely to make historical paths appear current.

---

## 8. Root Directory Rule

The L5 root shall contain only:

- the L5 navigation README;
- canonical category directories;
- temporary compatibility pointers required for reference integrity.

Substantive governance, production specification, ADR, or historical architecture content shall not be authored directly at the L5 root.

---

## 9. Architecture Boundary

This repository-structure decision changes document placement only.

It does not:

- alter the frozen 105-specification corpus;
- change any L5 identifier or responsibility boundary;
- change author allocations;
- change canonical business or technical ownership;
- approve technology choices;
- change dependency-gated execution;
- authorize branch or merge operations beyond existing Lead Architect control;
- issue the Final L5 Corpus Lock.

---

**Status:** Locked  
**Canonical Production Location:** `architecture/L5_Solution_Architecture/specifications/`  
**Canonical Governance Location:** `architecture/L5_Solution_Architecture/governance/`  
**Historical Location:** `architecture/L5_Solution_Architecture/historical/`  
**Canonical ADR Location:** `architecture/L5_Solution_Architecture/adr/` when first ADR exists
