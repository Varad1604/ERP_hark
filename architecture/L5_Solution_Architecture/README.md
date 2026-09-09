# L5 Solution Architecture

**Architecture Level:** L5 — Solution Architecture  
**Current State:** Representative pilot completed; authoring methodology corrected and locked; candidate-corpus finalization pending  
**Active Candidate Corpus:** 105 Architecture Approved candidate specifications  
**Mass L5 Authoring:** Not authorized  
**Final L5 Corpus Lock:** Not authorized  

---

## Purpose

Level 5 defines the engineering solution architecture required to implement approved L4 System Behavior Specifications without redefining upstream business architecture.

L5 may define solution components, technical boundaries, APIs/contracts, event transport, persistence, security mechanisms, integrations, runtime/deployment architecture, observability, resilience and engineering-quality architecture.

L5 shall not redefine L1/L2/L3/L4 canonical ownership, business rules, lifecycle states, canonical business events/publishers, institutional authority or approved business/system outcomes.

Technology-neutral L5 remains technically concrete: product/framework selection may be pending while product-independent components, control points, contracts, execution contexts, persistence boundaries, messaging roles, trust boundaries and runtime units are defined.

---

## Current L5 Governance Baseline

Canonical/current governance records are maintained under:

```text
architecture/L5_Solution_Architecture/governance/
```

Current active planning/governance artifacts include:

- `L5_Major_Architecture_Areas_Baseline.md` — Architecture Approved 30-area baseline;
- `L5_Architecture_Family_Register.md` — Architecture Approved seven-family allocation;
- `L5_Naming_and_Document_Granularity_Specialization.md` — Architecture Approved L5 identifier/granularity specialization;
- `L5_Naming_Specialization_Approval_Decision_2026-09-09.md` — Locked approval of hierarchical L5 naming;
- `L5_L4_to_L5_Coverage_Audit_Decision_105_2026-09-09.md` — Architecture Approved coverage decision;
- `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` — Architecture Approved revised 105-specification candidate corpus;
- `L5_Governance_and_Architecture_Standard.md` — Locked v0.3.1 governance baseline with explicit technical-realization-depth rules;
- `L5_Solution_Architecture_Specification_Template_v1.0.md` — Locked v1.0.1 compact technical authoring template;
- `L5_Validation_Standard.md` — Locked v1.0.1 validation baseline with strengthened HF-11 technical-depth enforcement;
- `L5_Technology_Selection_and_ADR_Methodology.md` — Locked v1.0.1 technology/ADR methodology clarifying that technology-neutral does not mean architecture-neutral;
- `L5_L4_to_L5_Traceability_Methodology.md` — Locked many-to-many L4 BH ↔ L5 traceability methodology;
- `L5_ADR_Register.md` — Locked central L5 ADR identity register;
- `L5_Candidate_Document_Corpus_Register_99_2026-09-09.md` — preserved historical Proposed candidate register superseded for active candidate-corpus planning.

The approved L5 family model is:

```text
L5-A — Core Platform & Application Architecture
L5-B — Data & Information Architecture
L5-C — Contracts, Events & Integration Architecture
L5-D — Identity, Security & Trust Architecture
L5-E — Reliability, Deployment & Operations Architecture
L5-F — Engineering Delivery & Quality Architecture
L5-G — Client Platform Architecture
```

The family letters are technical L5 identities and do not map to L1 business-domain section letters.

---

## Active Candidate Corpus

The current revised candidate corpus contains **105 specifications**:

| Family | Candidate Specifications |
|---|---:|
| L5-A | 20 |
| L5-B | 21 |
| L5-C | 18 |
| L5-D | 19 |
| L5-E | 16 |
| L5-F | 7 |
| L5-G | 4 |
| **Total** | **105** |

This corpus supersedes the earlier 99-specification register for active L5 candidate planning. The 99-register remains historical evidence and shall not be deleted.

The 105-register does not authorize mass authoring or final L5 lock.

---

## Representative Pilot Result

A nine-specification representative pilot was completed across three waves to validate L5 technical depth and cross-family applicability.

Pilot coverage exercised:

- Tenant/Institution isolation;
- context propagation and data isolation;
- policy decision/enforcement;
- transaction/consistency/concurrency;
- durable event delivery;
- runtime/deployment;
- secure computational engines;
- client security/session/state/contract consumption;
- verification/testing architecture.

The pilot established the preferred L5 authoring pattern:

```text
Metadata / upstream applicability
        ↓
Owns / Must Not Own
        ↓
Concrete technical architecture
        ↓
Contracts / state / context where applicable
        ↓
Primary flow + material failure flow
        ↓
AD-* architecture decisions
        ↓
CR-* conformance requirements
        ↓
Open decisions / ADR triggers
```

Pilot methodology validation passed after targeted governance remediation. Individual pilot specifications remain Draft/planning artifacts unless and until their material L4 dependencies and required ADRs satisfy normal approval gates.

Material ADR candidate areas identified by the pilot include reusable isolation enforcement, policy decision/enforcement realization, persistence/concurrency strategy, durable messaging/publication strategy, runtime/packaging strategy and secure-engine isolation. These are candidate decision areas, not automatically one ADR each.

---

## Historical Extracted L5 Documents

Two older L5 documents remain at the L5 root:

| File | Historical Origin |
|---|---|
| `L5-00_Technology_Stack_Baseline.md` | Extracted from earlier L0 technology-stack content under integrity finding INT-002 |
| `L5-01_Infrastructure_Persistence_Baseline.md` | Extracted from earlier L2 persistence content under integrity finding INT-008 |

These documents predate the current L4 production corpus and current L5 governance methodology. They are historical technical inputs, not automatic authority for greenfield technology selection, and must be revalidated under the current L5 governance/ADR process before any decision is adopted from them.

---

## Current Execution Boundary

The representative pilot and methodology-remediation step are complete.

The next controlled L5 work is:

1. finalize the canonical disposition of the 105-specification candidate register;
2. consolidate and scope pilot-discovered ADR candidates without ADR inflation;
3. score specification workload/complexity;
4. approve balanced author/reviewer allocation;
5. confirm repository navigation required for the finalized corpus;
6. issue explicit controlled broad-authoring authorization.

Technology choices, implementation code, detailed L6 UI/UX design and broad L5 authoring remain outside the authority of the current 105 candidate-corpus approval until the applicable gates are explicitly satisfied.
