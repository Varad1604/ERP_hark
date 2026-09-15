# Operational Read Model & Projection Architecture

## 1. Specification Metadata

**Specification ID:** `L5-B-06-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-06 — Reporting, Analytics & Data Projection Architecture  
**Canonical Technical Responsibility:** Operational read models/projections and freshness/ownership boundaries.  
**Technical Architecture Owner:** L5-B-06 — Reporting, Analytics & Data Projection Architecture  
**Assigned Author:** Chetan  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-15`  
**Depends On:** `L5-B-06-001`; `L5-B-02-003`; `L5-D-02-004`; `L5-C-01-*`; `L5-C-02-*`; `L5-A-04-002`; applicable source-domain L4/L2 architecture.  
**Referenced By:** H07 dashboard/query consumers; `L5-B-06-003`; other authorized operational read consumers.  
**Applicable ADRs:** `ADR-L5-0004`; `ADR-L5-0006`; `ADR-L5-0010`; `ADR-L5-0011`.  

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines how SutraERP builds and serves operational read models that are optimized for authorized queries while remaining derived from canonical source-owned facts.

Its primary concern is freshness, safe incremental update/rebuild, access enforcement and preservation of ownership.

### In Scope

- operational read-model identity and source binding;
- event/query-fed projection update;
- denormalized/read-optimized technical representation;
- freshness/currentness context;
- idempotent incremental updates;
- rebuild/reconciliation;
- query-time authorization;
- partial-failure behavior;
- read-model lifecycle as technical materialization state only where necessary.

### Out of Scope

- canonical operational aggregate ownership;
- business KPI/report calculation;
- analytical/statutory dataset semantics — B06-003;
- dashboard composition — H07;
- generic event transport — C02;
- source-domain commands/writes;
- physical database/index/cache technology selection.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Source Matrix

| Source | Status | Exact Reference | B06-002 Effect |
|---|---|---|---|
| Candidate Corpus 105 | Architecture Approved | `L5-B-06-002` | Owns operational read models/projections and freshness/ownership boundaries. |
| L2-H07 | Locked | §§1.2, 1.8, 2.5, 3–4 | H07 consumes Query APIs, CQRS Read Models and Published Projections; widgets are query-only and business data remains source-owned. |
| L4-H-048 | Draft | `BH-001`, `BH-003`, `BH-004` | H07 composition uses authoritative source projections/references and cannot repair invalid data by changing source facts. **Specified**. |
| L4-F-077 | Draft | `BH-002`, `BH-006`–`BH-008` | Source-owned projection may be refreshed and consumed downstream without ownership transfer. **Specified**. |
| L5-B-06-001 | Draft | §§4–8 | Supplies shared source/freshness/authorization/rebuild rules. |
| L5-C-02-001 | Architecture Approved | §§3–6 | At-least-once-compatible event delivery and source publisher preservation. |
| L5-D-02-004 | Architecture Approved | query enforcement semantics | Protected query access is fail-closed and scope-aware. |
| L5-B-02-003 | Draft | §§3–6 | Owner-local concurrency and explicit conflict detection. |

### 3.2 Constraints

1. An operational read model is never a command/write authority for its source.
2. Query optimization does not transfer ownership.
3. A read model must carry enough source/freshness context to identify what it represents.
4. Consumer-facing currentness shall not be inferred from successful query execution alone.
5. Rebuild is from canonical source/event history or governed source snapshot, not from another stale projection unless authoritative architecture explicitly permits it.
6. Authorization is evaluated for the actual consumer/scope/resource context.
7. No universal numeric freshness threshold is invented.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

- Operational Projection Definition;
- Projection Instance;
- Source Progress / Freshness Context;
- Incremental Projection Updater;
- Projection Rebuilder/Reconciler;
- Authorized Operational Query Boundary.

These are technical constructs only.

### Must Not Own

- source aggregate lifecycle;
- business rules;
- source event publisher identity;
- command processing;
- dashboard lifecycle;
- analytical KPI definition.

---

## 5. Solution Architecture

### 5.1 Operational Projection Flow

```text
Source committed fact
   ├─ C02 event
   └─ C01 governed read/snapshot
          ↓
Operational Projection Updater
          ↓
validate scope + projection definition/version
          ↓
idempotent transformation
          ↓
owner-local B06 projection persistence
          ↓
freshness/source-progress update
          ↓
D02-protected query
```

### 5.2 Projection Instance

Conceptually:

```text
OperationalProjectionInstance
{
  projection identity
  projection-definition/version reference
  Tenant / Institution scope
  source-owner reference set
  source progress/snapshot reference
  refresh/rebuild evidence reference
  freshness/currentness context
}
```

Business data fields are projection-specific and shall not be standardized here without source authority.

### 5.3 Incremental Update

For C02-fed projections:

- the consumer recognizes the source event contract/version;
- repeated delivery shall not apply the same logical source change twice;
- ordering assumptions are limited to those explicitly guaranteed by C02/source architecture;
- one projection update failure does not roll back the source business fact;
- unresolved gaps prevent the projection from claiming currentness.

### 5.4 Query/Snapshot Refresh

Where authoritative data is obtained through C01:

- the source owner defines the query contract;
- B06 does not directly read the owner's database;
- one refresh cycle shall use a coherent governed source basis sufficient for the projection's declared consistency requirement;
- failure leaves the prior projection identifiable as older/stale rather than fabricating current data.

### 5.5 Freshness Model

B06-002 does not invent a fixed "fresh/stale after N seconds" rule.

It exposes technical evidence needed to evaluate freshness, such as:

- last successfully applied source position/reference where supplied;
- source snapshot/version reference where supplied;
- successful projection refresh/rebuild reference;
- known gap/failure state;
- consumer-specific freshness requirement reference where upstream defines one.

The consumer or upstream policy decides whether that context is acceptable.

### 5.6 Rebuild & Reconciliation

```text
projection drift/gap detected
        ↓
prevent false-current claim
        ↓
obtain governed canonical source basis
        ↓
rebuild into controlled projection state
        ↓
verify source progress / reconciliation
        ↓
atomically make rebuilt projection eligible
```

No source-domain write is performed.

Rebuild may use generic background execution architecture where separately governed, but B06 owns projection correctness/reconciliation meaning.

### 5.7 Authorization-Aware Query

The query boundary evaluates:

- authenticated/technical principal context from its canonical owner;
- Tenant/Institution scope;
- requested projection/resource;
- applicable D02 policy;
- source-defined sensitivity/disclosure constraints where applicable.

A dashboard/widget assignment is not authorization.

### 5.8 Partial Failure

Operational consumers may continue using other independent projections when one projection is unavailable, provided they can distinguish missing/stale data and upstream behavior permits partial presentation.

B06-002 does not invent a business decision from incomplete data.

---

## 6. Architecture & Technology Decisions

| ID | Decision | Position |
|---|---|---|
| `AD-001` | Read-side authority | Operational read models are derived/query-only and never command authority. |
| `AD-002` | Update patterns | Event-fed and owner-query/snapshot-fed projection updates are both supported. |
| `AD-003` | Repeat delivery | C02-fed projection consumers are idempotent under governed repeated delivery. |
| `AD-004` | Freshness | Freshness is explicit contextual evidence, not a universal hard-coded duration. |
| `AD-005` | Rebuild | Rebuild uses canonical source basis and controlled eligibility switch; no source write-back. |
| `AD-006` | Product neutrality | No database, materialized-view, cache, search or streaming product is selected. |

---

## 7. Security, Privacy, Quality & Operational Impact

- Read-only does not mean unrestricted: D02 is mandatory.
- Projection persistence remains Tenant/Institution isolated.
- Sensitive projections consume B05 controls where applicable.
- Event lag/rebuild failure is observable through E05.
- Projection data may be disposable/rebuildable technically only where source authority and retention/audit obligations permit; no blanket deletion assumption is made.

---

## 8. Architecture Conformance Requirements

- `CR-001`: Every operational projection shall be traceable to canonical source owner(s).
- `CR-002`: A projection shall never accept commands or become write authority for its source.
- `CR-003`: C02-fed updates shall tolerate permitted repeated delivery.
- `CR-004`: Projection currentness shall not be inferred solely from successful storage/query availability.
- `CR-005`: Known source gaps/update failures shall prevent a false-current claim.
- `CR-006`: Rebuild shall use governed canonical source basis and preserve lineage.
- `CR-007`: Protected queries shall enforce D02 authorization and validated scope.
- `CR-008`: H07 shall consume projection results without receiving source-business ownership.
- `CR-009`: No fixed freshness threshold or technology product is canonicalized here.

---

## 9. Open Decisions & Deferred Design

| ID | Item | Blocking? |
|---|---|---|
| `OD-001` | `L4-H-048` and `L4-F-077` remain Draft; mappings are `Specified`. | **Blocks Architecture Approved; not Draft authoring.** |
| `OD-002` | Exact projection-store/cache/index technology remains unresolved. | No. |
| `OD-003` | Per-projection numeric freshness requirements must come from authoritative consumers/operational profiles. | No for architecture; required for concrete runtime SLOs. |

---

## 10. Traceability Summary

- Canonical responsibility: Candidate Corpus `L5-B-06-002`.
- Material L4: `L4-H-048-BH-001/003/004`; `L4-F-077-BH-002/006/007/008`.
- Material L2: H07 query/read-model rules; F13 dashboard-ready projections.
- L5 dependencies: B06-001, B02, D02, C01, C02, A04, B05, E05.
