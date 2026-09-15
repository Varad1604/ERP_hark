# L5-A-07-001 — Secure Computational Engine Architecture Baseline

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-A-07-001 |
| Specification Title | Secure Computational Engine Architecture Baseline |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-A — Core Platform & Application Architecture |
| Major Area | L5-A-07 — Secure Computational Engine Architecture |
| Canonical Technical Responsibility | Common trust, determinism, rule-package isolation and evidence requirements; engines do not own business rules. |
| Technical Architecture Owner | Secure Computational Engine Platform Architecture |
| Assigned Author | Varad |
| Version | 1.0.1 |
| Status | Locked |
| Last Updated | 2026-09-15 |
| Depends On | L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5-D-03-001; L5-D-03-002; L5-A-08-001; L5-B-02-001; L5-E-04-001; applicable approved L4 System Behavior Specifications |
| Referenced By | L5-A-07-002; L5-A-07-003; L5-A-07-004; L5-A-07-005; L5-A-07-006 |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0002; ADR-L5-0005; ADR-L5-0012; ADR-L5-0013; ADR-L5-0014 |

## 2. Purpose & Scope

Defines the shared technical execution boundary for governed computations. Specialized engines execute authoritative, versioned rule definitions supplied by owning domains; they do not create business rules, approve outcomes, post transactions or publish canonical business state.

In scope: admission validation, context/rule-version binding, deterministic execution where required by parent behavior, bounded execution, output validation, technical lineage/evidence and failure isolation. Out of scope: business rules, domain lifecycle, approval/posting/publication, product selection, database schema, deployment product and numeric SLO selection.

## 3. Canonical Constraints

- Engines never own business rules or business authority.
- Engines do not directly mutate canonical domain aggregates.
- Historical computation attempts/evidence are append-only lineage where upstream behavior requires preservation.
- Runtime isolation and trust boundaries consume D03/E04 controls; this document does not define algorithms, products, key sizes, CPU/memory limits or timeouts.
- Integrity evidence uses the approved D03 cryptographic capability as an opaque service; no digest/signature algorithm is selected locally.
- Asynchronous scheduling/retry is delegated to A08 under ADR-L5-0013.

## 4. Technical Ownership

### Owns
- Computation admission contract and completeness checks.
- Binding of exact rule package/version and authoritative input snapshot/reference.
- Execution isolation contract and deterministic-input controls.
- Structural output validation.
- Calculation-attempt lineage assembly and evidence handoff.
- Standard technical failure classes for computation execution.

### Does Not Own
Business formulas/regulations, domain approval, posting/publication, persistence platform, cryptographic primitives/key custody, background scheduler/retry policy, audit aggregation or runtime product.

## 5. Solution Architecture

| Element | Responsibility | Failure Boundary |
|---|---|---|
| `ComputationAdmissionGate` | Validate tenant/institution context, caller authorization reference, input completeness and requested engine/rule reference. | Reject incomplete or ambiguous requests; never infer missing business inputs. |
| `RulePackageBinder` | Resolve immutable authoritative rule package/version supplied by the owning domain/rule catalog. | No fallback to an unapproved/superseded rule version. |
| `ExecutionBoundary` | Execute the bound computation in an isolated, bounded runtime according to D03/E04 controls. | Terminate safely when governed resource limits are exceeded. |
| `OutputIntegrityValidator` | Validate output structure and caller-supplied domain invariants without inventing new business rules. | Reject malformed/out-of-contract results. |
| `LineageEvidenceAssembler` | Bind input reference, rule version, output reference, prior-attempt link and D03-provided integrity evidence. | Result cannot be represented complete if required evidence handoff fails. |
| `ResultHandoff` | Return technical result/evidence to owning workflow. | Never post or publish domain state directly. |

Primary flow:

```text
Owning workflow -> Admission -> Rule/version bind -> Isolated execution
-> Output validation -> Evidence assembly -> B02/D04 handoff -> Owning workflow
```

### Conceptual Contract Boundaries

A computation request must identify tenant/institution context, correlation identity, specialized engine, authoritative rule package/version and authoritative input snapshot/reference. A computation result returns technical outcome, output payload/reference and evidence/lineage reference. Exact field schemas are downstream contract design and are not frozen here.

### Failure & Recovery

Admission failure, missing rule version, deterministic arithmetic/rule error and output invariant failure are returned to the owning workflow and are not retried automatically. Transient infrastructure execution failures may be retried only through A08-governed work execution. Evidence persistence/handoff failure prevents successful completion from being reported.

## 6. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-A07-001 | Computation engines are stateless with respect to canonical business state and return results to owning workflows. | Repository-derived architecture position |
| AD-A07-002 | Reproducibility binds exact authoritative inputs and exact rule version; historical attempts are preserved when required. | L4/ADR-derived |
| AD-A07-003 | Integrity operations are delegated to D03 and remain algorithm/product neutral here. | ADR-L5-0002 |
| OD-A07-001 | Concrete runtime isolation technology and numeric execution limits remain governed downstream. | Open decision |

## 7. Conformance Requirements

1. No specialized engine may create or override a business rule.
2. No engine may directly approve, post or publish canonical domain outcomes.
3. Every completed computation must bind the exact rule version and authoritative input reference used.
4. Required integrity evidence must be produced through D03-approved capability without locally selecting algorithms.
5. Background retries/work scheduling must be delegated to A08.
6. Numeric resource limits and runtime products must not become normative without approved authority.

## 8. Traceability Summary

This baseline preserves the approved A07 separation: secure computation is reusable technical infrastructure; business meaning and business authority remain in the owning L1-L4 domain/workflow.