# L5-D-04-002 Ownership Boundary Correction & Re-Lock Decision — 2026-09-13

## 1. Decision Metadata

**Decision Type:** Controlled locked-specification correction  
**Affected Specification:** `L5-D-04-002 — Audit Event, Provenance & Evidence Model Architecture`  
**Corrected Version:** `1.1.1`  
**Resulting Status:** `Locked`  
**Date:** `2026-09-13`  
**Repository:** `DhruvNaik26/SutraERP_Architecture`  
**Authority Basis:** Explicit correction authorization in the controlled SutraERP architecture session; existing `L5_Varad_Stage2_Final_Lock_Decision_2026-09-12.md`; frozen L5 responsibility allocation; approved L5 persistence/security ADR boundaries.

---

## 2. Reason for Controlled Correction

The locked `L5-D-04-002` v1.1.0 contained stale boundary wording that incorrectly attributed the following responsibilities to `L5-D-04-003`:

- physical database DDL;
- storage-engine selection;
- physical immutable-storage realization;
- cryptographic/Merkle sealing implementation;
- reusable export archive packaging.

Those statements conflict with the later explicit Stage-2 lock boundary and frozen L5 responsibility model, which establish that:

- `L5-B-02` owns physical persistence schema, storage-engine, transaction/concurrency and physical persistence realization;
- `L5-D-03-002/003` own cryptographic protection operations and key custody/lifecycle;
- `L5-D-04-003` owns audit-specific integrity-stream, linkage, checkpoint/anchor, verification and audit evidence/proof semantics;
- `L5-C-03-008` owns reusable export/file packaging, transfer and generic delivery-integrity mechanics.

The correction therefore restores repository consistency. It does not transfer or redefine the canonical provenance-model responsibility owned by `L5-D-04-002`.

---

## 3. Authorized Changes

The controlled correction is limited to:

1. replacing stale out-of-scope ownership wording;
2. correcting the `L5-D-04-003` collaboration description;
3. correcting the D04-002 assumption about physical storage and cryptographic sealing;
4. correcting persistence/integrity wording in Section 5.5;
5. correcting dependency/traceability wording that implied D04-003 owned physical persistence or generic export packaging;
6. incrementing the specification version from `1.1.0` to `1.1.1` and recording this decision as correction authority.

No canonical business concept, entity owner, event publisher, L4 behavior, D04-002 conceptual contract, provenance relationship rule, actor-attribution rule, causality rule, or conformance obligation is changed.

---

## 4. Architectural Effect

After the correction:

```text
L5-D-04-002
    owns provenance/evidence conceptual contracts

L5-D-04-003
    owns audit-specific integrity and proof semantics

L5-B-02
    owns physical persistence realization

L5-D-03
    owns cryptographic protection and key custody

L5-C-03-008
    owns reusable packaging / transfer / delivery mechanics
```

This resolves the identified cross-spec ownership conflict without introducing a new architecture decision or new canonical owner.

---

## 5. Re-Lock Result

`L5-D-04-002` version `1.1.1` is **re-locked** after the boundary-only correction.

Validation result:

```text
Canonical responsibility changed: NO
Business behavior changed: NO
L4 behavioral mapping changed: NO
Canonical entity ownership changed: NO
Canonical event ownership changed: NO
Technology selection introduced: NO
Cross-spec ownership conflict remaining from corrected wording: NO
Status: Locked
```

Any future material change to `L5-D-04-002` remains subject to normal controlled-amendment governance.
