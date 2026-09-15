# L5-A-07-001 — Secure Computational Engine Architecture Baseline

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-A-07-001 |
| Specification Title | Secure Computational Engine Architecture Baseline |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-A — Core Platform & Application Architecture |
| Major Area | L5-A-07 — Secure Computational Engine Architecture |
| Canonical Technical Responsibility | Common trust, determinism, rule-package isolation, execution control and evidence requirements; engines do not own business rules. |
| Technical Architecture Owner | Secure Computational Engine Platform Architecture |
| Assigned Author | Varad |
| Version | 1.1.0 |
| Status | Review |
| Last Updated | 2026-09-15 |
| Depends On | L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5-D-03-001; L5-D-03-002; L5-A-08-001; L5-B-02-001; L5-E-04-001; applicable approved L4 System Behavior Specifications |
| Referenced By | L5-A-07-002; L5-A-07-003; L5-A-07-004; L5-A-07-005; L5-A-07-006 |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0002; ADR-L5-0005; ADR-L5-0012; ADR-L5-0013; ADR-L5-0014 |

## 2. Purpose & Scope

Defines the shared technical execution boundary for governed computations. Specialized engines execute authoritative, versioned rule definitions supplied by owning domains; they do not create business rules, approve outcomes, post transactions or publish canonical business state.

### 2.1 In Scope

- Admission validation and execution-context completeness.
- Binding of authoritative rule/version and authoritative input snapshot/reference.
- Deterministic execution controls where required by parent behavior.
- Runtime isolation and bounded execution policy consumption.
- Output structural validation and caller-supplied invariant evaluation.
- Technical lineage, replay correlation, evidence production and result handoff.
- Failure classification sufficient for safe retry/delegation decisions.

### 2.2 Out of Scope

- Business formulas, regulations, institutional policy or rule precedence.
- Approval, posting, publication or canonical business lifecycle mutation.
- Product/runtime selection, database schema, deployment topology or L6 implementation design.
- Numeric SLOs, CPU/memory/time limits or technology-specific sandbox configuration without approved authority.

## 3. Canonical Architectural Invariants

1. **Business-authority invariant:** an A07 engine may execute a supplied rule but may not originate, approve or reinterpret that rule.
2. **Canonical-state invariant:** an engine never directly commits canonical business aggregate state; results return to the owning workflow.
3. **Version-binding invariant:** every completed result is attributable to the exact authoritative rule version and authoritative input reference used.
4. **No-silent-substitution invariant:** unresolved, missing or superseded rule/input references fail closed; the engine does not substitute a convenient alternative.
5. **Isolation invariant:** executable evaluation is constrained by D03/E04 trust and runtime controls and cannot gain unrestricted persistence, filesystem, process or network authority.
6. **Evidence invariant:** a result that requires lineage/evidence cannot be represented complete until required evidence has been durably handed off.
7. **Retry invariant:** deterministic/business failures are not retried as infrastructure failures; asynchronous retry/work orchestration remains A08-owned.
8. **Algorithm-neutrality invariant:** integrity evidence is consumed from D03 as an approved capability; no local cryptographic primitive is selected.

## 4. Technical Ownership

### 4.1 Owns

- Computation admission contract and completeness checks.
- Binding of exact rule package/version and authoritative input snapshot/reference.
- Technical execution-attempt identity and correlation.
- Execution isolation contract and deterministic-input controls.
- Structural output validation.
- Caller-supplied invariant evaluation mechanics.
- Calculation-attempt lineage assembly and evidence handoff.
- Standard technical failure classes for computation execution.

### 4.2 Consumes / Delegates

| Owner | Responsibility Consumed |
|---|---|
| Owning domain/workflow | Business rule meaning, rule approval, authoritative inputs, business invariants, acceptance and final outcome. |
| `L5-D-03` | Trust-boundary controls, cryptographic integrity capability and security constraints. |
| `L5-E-04` | Governed runtime isolation/operational execution controls where applicable. |
| `L5-B-02` | Persistence mechanics for A07-owned technical records. |
| `L5-D-04` | Canonical audit/evidence aggregation. |
| `L5-A-08` | Background scheduling, work execution, retry and recovery orchestration. |

### 4.3 Must Not Own

Business formulas/regulations, domain approval, posting/publication, business lifecycle, persistence platform, cryptographic primitives/key custody, background scheduler/retry policy, audit aggregation or runtime product selection.

## 5. Solution Architecture

### 5.1 Realization Elements

| Element | Responsibility | Input | Output | Failure Boundary |
|---|---|---|---|---|
| `ComputationAdmissionGate` | Validate tenant/institution context, caller authorization reference, correlation identity, engine type, input completeness and requested rule reference. | Computation request | Admitted technical execution request | Rejects incomplete, ambiguous or unauthorized technical context without inferring missing business input. |
| `RulePackageBinder` | Resolve the immutable authoritative rule package/version supplied by the owning domain/rule catalog. | Rule reference | Bound rule version | No fallback to an unapproved/superseded rule version. |
| `InputSnapshotBinder` | Bind exact authoritative input snapshot/reference for reproducibility. | Authoritative input references | Bound input set | Rejects unresolved or mutable/unversioned references where exact replayability is required. |
| `ExecutionBoundary` | Execute the bound computation in an isolated, bounded runtime according to D03/E04 controls. | Bound rule + inputs | Candidate result | Contains execution failure and prevents side effects outside the permitted execution contract. |
| `OutputIntegrityValidator` | Validate output structure and caller-supplied invariants without inventing business rules. | Candidate result | Validated technical result | Rejects malformed/out-of-contract result. |
| `LineageEvidenceAssembler` | Bind request, input, rule, output, predecessor/replay reference and D03-provided integrity evidence. | Completed attempt | Evidence envelope | Result cannot be represented complete when required evidence cannot be produced or handed off. |
| `ResultHandoff` | Return technical result/evidence to owning workflow. | Validated result | Domain-consumable technical outcome | Never approves, posts or publishes canonical business state. |

### 5.2 Primary Interaction Flow

```text
Owning workflow
  -> ComputationAdmissionGate
  -> RulePackageBinder
  -> InputSnapshotBinder
  -> ExecutionBoundary
  -> OutputIntegrityValidator
  -> LineageEvidenceAssembler
  -> B02/D04 technical handoff
  -> ResultHandoff
  -> Owning workflow business decision
```

### 5.3 Conceptual Contract Boundaries

A computation request must conceptually identify:

- tenant/institution scope;
- owning workflow/correlation reference;
- requested specialized engine;
- authoritative rule package/version reference;
- authoritative input snapshot/reference set;
- caller-supplied invariant/profile references where applicable;
- security/authorization context reference.

A computation result must conceptually return:

- correlation/execution-attempt reference;
- technical outcome classification;
- output value or output reference;
- exact rule version and input references used;
- evidence/lineage reference;
- predecessor/replay relationship where applicable;
- diagnostics safe for the owning workflow.

Exact transport fields, payload schemas and API shapes remain downstream contract design.

## 6. Processing Semantics

### 6.1 Determinism and Reproducibility

Where the parent behavior requires reproducibility, the engine must ensure that a completed attempt is attributable to the exact rule version, input reference set and governed execution profile used. If an approved rule depends on externally varying facts, those facts must be represented as authoritative inputs rather than read implicitly during execution.

### 6.2 Idempotency and Duplicate Submission

The architecture must support detection/correlation of repeated requests when the owning workflow supplies an idempotency/correlation identity. Duplicate technical submission must not create contradictory canonical business outcomes; the owning workflow remains responsible for whether a returned prior result is acceptable for the business operation.

### 6.3 Replay and Recalculation

A replay/recalculation is a new technical attempt linked to the predecessor attempt when lineage is required. Historical attempts are preserved where authoritative behavior/evidence requirements require them; replay does not overwrite prior evidence.

### 6.4 Side-Effect Control

The execution boundary may produce technical result/evidence only. Any downstream posting, issuance, publication, allocation or business-state mutation must occur through the canonical owning workflow after it evaluates the returned result.

## 7. Failure, Recovery & Degradation Model

| Failure Class | Example | Required Treatment |
|---|---|---|
| Admission failure | Missing context/rule/input reference | Reject; no execution attempt proceeds. |
| Authority failure | Rule/reference not approved or caller context insufficient | Fail closed; do not infer authority. |
| Deterministic rule failure | Invalid formula/rule evaluation | Return deterministic failure; do not automatic-retry as transient infrastructure. |
| Invariant failure | Caller-supplied invariant not satisfied | Return failed validation; owning workflow decides business consequence. |
| Runtime isolation failure | Execution sandbox/control cannot be established | Abort safely; no fallback to less-isolated execution. |
| Transient infrastructure failure | Temporary execution/persistence dependency unavailable | Delegate retry/work recovery to A08 where permitted. |
| Evidence failure | Required evidence cannot be persisted/handed off | Do not report successful completion. |
| Security/trust failure | Integrity or trust check fails | Fail closed and preserve technical diagnostic/evidence. |

Degraded operation must never broaden business authority, bypass required isolation or remove required evidence controls.

## 8. Security, Trust & Data Handling

- Execution operates under least privilege and tenant/institution isolation.
- Rule definitions and inputs are treated as untrusted data unless separately trusted by authoritative controls.
- Integrity checks are delegated to D03-approved capabilities; raw private key material is never handled by A07.
- Sensitive inputs/results are minimized in technical diagnostics and evidence references according to applicable privacy/classification rules.
- No engine may obtain direct persistence-table, arbitrary filesystem, unrestricted process or unrestricted network authority merely to simplify computation.
- Authorization failure is fail-closed; the engine does not downgrade to anonymous or broader execution.

## 9. Observability & Technical Evidence

Each execution attempt should make available sufficient technical evidence to reconstruct what happened without restating business policy. Evidence includes, where applicable:

- correlation/execution-attempt reference;
- engine type and technical outcome;
- authoritative rule/version reference;
- authoritative input snapshot/reference set;
- start/completion ordering evidence rather than hardcoded timing targets;
- failure category and safe diagnostic code/reference;
- predecessor/replay relation;
- integrity/evidence reference from D03/D04 handoff.

Metrics/logging/tracing implementation remains governed by the operations/observability architecture; this specification defines the evidence intent, not a telemetry product.

## 10. Capacity & Performance Principles

- Execution must be bounded by governed resource controls.
- Capacity exhaustion must result in explicit bounded failure/degradation rather than uncontrolled resource consumption.
- Long-running/asynchronous work must use A08-governed execution where the owning workflow permits asynchronous treatment.
- Numeric latency, throughput, concurrency, memory, CPU and timeout targets are not canonical until approved through the proper decision authority.

## 11. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-A07-001 | Computation engines are stateless with respect to canonical business state and return results to owning workflows. | Repository-derived architecture position |
| AD-A07-002 | Reproducibility binds exact authoritative inputs and exact rule version; historical attempts are preserved when required. | L4/ADR-derived |
| AD-A07-003 | Integrity operations are delegated to D03 and remain algorithm/product neutral here. | ADR-L5-0002 |
| AD-A07-004 | Deterministic/business failures are distinguished from transient infrastructure failures so retry cannot alter business semantics. | Repository-consistent technical derivation |
| OD-A07-001 | Concrete runtime isolation technology and numeric execution limits remain governed downstream. | Open decision |

## 12. Conformance & Validation Requirements

1. No specialized engine may create or override a business rule.
2. No engine may directly approve, post or publish canonical domain outcomes.
3. Every completed computation must bind the exact rule version and authoritative input reference used.
4. Missing/superseded rule or input references must fail closed rather than trigger substitution.
5. Required integrity evidence must be produced through D03-approved capability without locally selecting algorithms.
6. Deterministic/business failures must not be blindly retried as transient infrastructure failures.
7. Background retries/work scheduling must be delegated to A08.
8. Execution must remain isolated from direct canonical persistence and unrestricted side effects.
9. Replay/recalculation must preserve lineage where authoritative evidence requirements apply.
10. Numeric resource limits and runtime products must not become normative without approved authority.

### 12.1 Architecture Acceptance Evidence

A conforming implementation/design review must be able to demonstrate, at minimum:

- a request cannot execute without an authoritative rule/version reference;
- a missing authorization/context reference fails closed;
- a result cannot mutate canonical business state directly;
- a deterministic rule failure is distinguishable from transient infrastructure failure;
- a required evidence failure prevents successful completion;
- replay creates traceable lineage rather than overwriting evidence;
- no local product/algorithm/numeric choice has been promoted to canonical architecture without governance.

## 13. Traceability Summary

This baseline preserves the approved A07 separation: secure computation is reusable technical infrastructure; business meaning and business authority remain in the owning L1-L4 domain/workflow. D03 supplies trust/integrity controls, B02 supplies persistence mechanics, D04 supplies canonical evidence aggregation, A08 supplies background execution/retry, and specialized A07 specifications refine only the technical computation behavior permitted by this baseline.