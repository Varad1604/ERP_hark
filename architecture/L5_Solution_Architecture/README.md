# L5 Solution Architecture

**Architecture Level:** L5 — Solution Architecture  
**Current State:** Broad authoring authorized for the frozen 105-specification corpus under dependency-gated parallel execution  
**Frozen Authoring Population:** 105 specifications  
**Broad L5 Authoring:** Authorized  
**Final L5 Corpus Lock:** Not authorized  

---

## Purpose

Level 5 defines the engineering solution architecture required to implement approved L4 System Behavior Specifications without redefining upstream business architecture.

L5 may define solution components, technical boundaries, APIs/contracts, event transport, persistence, security mechanisms, integrations, runtime/deployment architecture, observability, resilience, engineering-quality architecture and governed technology decisions.

L5 shall not redefine L1/L2/L3/L4 canonical ownership, business rules, lifecycle states, canonical business events/publishers, institutional authority or approved business/system outcomes.

Technology-neutral L5 remains technically concrete: product/framework selection may be pending while product-independent components, control points, contracts, execution contexts, persistence boundaries, messaging roles, trust boundaries and runtime units are defined.

---

## Canonical Repository Structure

```text
architecture/L5_Solution_Architecture/
├── README.md
├── governance/
│   ├── README.md
│   ├── instruction/       # required team authoring/review working set
│   └── reference/         # supporting governance and provenance
├── specifications/        # canonical 105-specification production authoring area
├── historical/            # preserved non-canonical extracted technical material
├── adr/                   # created when the first canonical ADR is allocated
├── L5-00_Technology_Stack_Baseline.md
└── L5-01_Infrastructure_Persistence_Baseline.md
```

The two `L5-00` / `L5-01` root files are temporary compatibility pointers only. Their substantive historical content resides under `historical/`.

The base placement rules are recorded in:

`governance/reference/L5_Repository_Structure_and_Content_Placement_Decision_2026-09-09.md`

The governance subfolder organization is recorded in:

`governance/reference/L5_Governance_Subfolder_Organization_Decision_2026-09-10.md`

The `adr/` directory shall not be materialized until the first canonical ADR document is actually allocated and authored.

---

## Team Governance Entry Point

All authors and reviewers should start at:

```text
governance/instruction/README.md
```

The `instruction/` directory contains the active working set required for routine L5 execution:

- L5 Governance and Architecture Standard;
- L5 Solution Architecture Specification Template;
- L5 Validation Standard;
- canonical 105-specification register;
- frozen-population decision;
- workload and five-person allocation;
- dependency-gated execution timeline;
- naming and document-granularity rules;
- architecture-family register;
- L4-to-L5 traceability methodology;
- technology-selection and ADR methodology;
- ADR identity register;
- broad-authoring authorization.

Supporting planning decisions, approval provenance, coverage evidence and superseded planning artifacts are retained under:

```text
governance/reference/
```

Placement in `reference/` does not reduce a document's authority or status. It only means the record is not normally required in every authoring session.

---

## Canonical Production Specifications

All newly authored canonical L5 specifications belong under:

```text
architecture/L5_Solution_Architecture/specifications/
```

The production entry point is `specifications/README.md`.

Family folders are materialized only when their first real specification is committed:

```text
specifications/L5-A/
specifications/L5-B/
specifications/L5-C/
specifications/L5-D/
specifications/L5-E/
specifications/L5-F/
specifications/L5-G/
```

Every canonical specification filename shall follow:

```text
L5-<Family>-<Area>-<Document>_<Canonical_Solution_Slug>.md
```

---

## Frozen Authoring Population

| Family | Specifications |
|---|---:|
| L5-A — Core Platform & Application Architecture | 20 |
| L5-B — Data & Information Architecture | 21 |
| L5-C — Contracts, Events & Integration Architecture | 18 |
| L5-D — Identity, Security & Trust Architecture | 19 |
| L5-E — Reliability, Deployment & Operations Architecture | 16 |
| L5-F — Engineering Delivery & Quality Architecture | 7 |
| L5-G — Client Platform Architecture | 4 |
| **Total** | **105** |

The frozen authoring population locks the 105 identifiers and responsibility boundaries for controlled authoring. It is not the Final L5 Corpus Lock.

---

## Locked Workload Allocation

| Author | Specifications |
|---|---:|
| Varad | 31 |
| Pallav | 30 |
| Dhruv | 20 |
| Devang | 14 |
| Chetan | 10 |
| **Total** | **105** |

This is an authoring allocation only. It does not transfer canonical technical ownership or upstream business ownership.

---

## Dependency-Gated Parallel Execution

Broad L5 authoring is authorized under the locked six-stage dependency-gated execution model:

```text
Foundation
    ↓
Control Architecture
    ↓
Shared Execution & Operations
    ↓
Specialized Platforms
    ↓
External Integration & Recovery
    ↓
Technology + Verification Consolidation
```

The stages are not strict serial waves. A specification may begin when its required upstream L5 dependency is architecturally stable enough that the author does not need to invent that upstream responsibility.

```text
Do not wait for a whole stage.
Wait only for the specific dependency needed.
```

---

## Historical Technical Material

Substantive historical extracted technical material is stored under:

```text
architecture/L5_Solution_Architecture/historical/
```

Current preserved artifacts are:

- `historical/L5-00_Technology_Stack_Baseline.md` — extracted from earlier L0 technology content under INT-002;
- `historical/L5-01_Infrastructure_Persistence_Baseline.md` — extracted from earlier L2 persistence content under INT-008.

These documents are historical technical inputs only, are not part of the frozen 105-specification production corpus, and are not automatic authority for technology or architecture selection.

---

## ADR Boundary

The canonical ADR register now contains **33 allocated ADRs**. `ADR-L5-0019` through `ADR-L5-0033` are Locked by the 2026-09-22 Technology Decision Wave. New allocation continues from `ADR-L5-0034`. Canonical ADR documents live under `adr/`.

---

## Current Execution Boundary

The representative pilot, methodology remediation, 105-specification authoring-population freeze, ADR-candidate disposition, workload allocation, dependency-gated sequencing, repository navigation synchronization, repository-structure governance and broad-authoring authorization are complete.

The L5 program may proceed with controlled mass authoring in `specifications/` according to the locked dependency gates.

Branch selection, manual task assignment, push permissions and merge timing remain under Lead Architect control.

Individual specifications still require normal validation and remediation where applicable. The immediate technology/product decisions are now governed by Locked ADR-L5-0019 through ADR-L5-0033. Solver selection, universal numeric capacity/SLO values, RPO/RTO/retention values, mobile/cache choices where still open, exact deployment-provider details and Final L5 Corpus Lock remain unresolved. The Final L5 Corpus Lock has not been issued.


## Backend Readiness Decision Group 2 — Data, Caching & Persistence

Locked on 2026-09-22:

- `ADR-L5-0036` — persistence implementation conventions and technical identity standard.
- `ADR-L5-0037` — Valkey derived-cache and failure-behavior standard.

The canonical ADR register now allocates 37 ADRs; next allocation is `ADR-L5-0038`.


## Backend Readiness Decision Group 3 — IAM, Security & Service Trust

Locked on 2026-09-22:

- `ADR-L5-0038` — custom credential API, server session and MFA profile.
- `ADR-L5-0039` — non-human service authentication and authorization binding profile.
- `ADR-L5-0040` — internal transport, cryptography, secrets and key-custody profile.

The default human login surface is a SutraERP custom username/password API backed by Keycloak credential authority; OAuth Direct Access Grant is explicitly not the canonical realization. The ADR register now allocates 40 ADRs; next allocation is `ADR-L5-0041`.


## Backend Readiness Decision Group 4 — Workflow, Jobs & Asynchronous Execution

Locked on 2026-09-22:

- `ADR-L5-0041` — Rust worker, JetStream background execution and retry profile.
- `ADR-L5-0042` — Flowable workflow runtime, task, timer, version and compensation profile.

Two backend-readiness decision groups remain after this group: Group 5 Runtime/Deployment/Operations and Group 6 Capacity/DR/Optimization Targets. Final corpus synchronization/validation remains a separate closure activity. The ADR register now allocates 42 ADRs; next allocation is `ADR-L5-0043`.


## Backend Readiness Decision Group 5 — Runtime, Deployment & Operations

Locked on 2026-09-22:

- `ADR-L5-0043` — deployment toolchain, environment and stateful-platform profile.
- `ADR-L5-0044` — runtime health, configuration-change and graceful-lifecycle profile.

Only Backend Readiness Group 6 remains before the repository-wide L5 consistency/closure audit. The ADR register now allocates 44 ADRs; next allocation is `ADR-L5-0045`.


## Backend Readiness Decision Groups — COMPLETE

All six backend-readiness decision groups are complete and Locked as of 2026-09-22.

- Group 1: `ADR-L5-0034`–`0035`
- Group 2: `ADR-L5-0036`–`0037`
- Group 3: `ADR-L5-0038`–`0040`
- Group 4: `ADR-L5-0041`–`0042`
- Group 5: `ADR-L5-0043`–`0044`
- Group 6: `ADR-L5-0045`–`0047`

The backend architecture is decision-ready for future implementation planning, but **Final L5 Corpus Lock remains not authorized**. The next required activity is a full 105-specification post-decision synchronization and integrity audit. The ADR register now allocates 47 ADRs; next allocation is `ADR-L5-0048`.
