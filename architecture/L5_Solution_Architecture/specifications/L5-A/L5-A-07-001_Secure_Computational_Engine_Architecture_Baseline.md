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
| Version | 1.0.0 |
| Status | Draft |
| Last Updated | 2026-09-15 |
| Depends On | L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5 Candidate Document Corpus Register v1.1.0; L5-D-03-001; L5-D-03-005; L5-A-08-001; L5-B-02-001; L5-E-04-001; applicable approved L4 System Behavior Specifications |
| Referenced By | L5-A-07-002; L5-A-07-003; L5-A-07-004; L5-A-07-005; L5-A-07-006; downstream computation engines |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0002; ADR-L5-0005; ADR-L5-0013 |

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the common solution-architecture baseline for executing governed computational work across SutraERP. The baseline guarantees that specialized computational engines execute versioned, approved rule definitions deterministically and securely, with strict input admission, resource bounding, cryptographic lineage tracking, and execution isolation, while maintaining absolute separation between technical computation and institutional business authority.

### In Scope

- Common computation admission boundary and prerequisite verification.
- Context binding and authoritative source input integrity.
- Rule and formula definition package isolation.
- Deterministic and reproducible execution mechanics where parent behavior requires reproducibility.
- Execution resource containment (CPU, memory, instruction depth, execution timeout).
- Generation of immutable calculation evidence, execution identity, and supersession lineage.
- Separation of technical calculation from business approval, posting, and publication authority.

### Out of Scope / Non-Goals

- Domain-specific calculation rules or business semantics (owned by specialized computation engines L5-A-07-002 through L5-A-07-006 and upstream L1–L4 domains).
- Business approval, posting, official publication, or monetary settlement authority.
- Direct database schema design, physical storage engine selection, or hardware provisioning.
- UI/UX interaction, presentation of formulas, or user override screens.
- Arbitrary code execution, unrestricted network access, or direct file-system mutation.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L4 | `L4-I-083` | Approved / Locked | `L4-I-083-BH-001` | Controlled admission of final marks into result computation | Reusable admission validation pattern |
| L4 | `L4-I-083` | Approved / Locked | `L4-I-083-BH-009` | Generation of attributable calculation evidence and provenance | Common computation evidence record |
| L4 | `L4-I-084` | Approved / Locked | `L4-I-084-BH-003` | Controlled recomputation execution from re-conducted exams | Lineage and supersession execution |
| L4 | `L4-I-086` | Approved / Locked | `L4-I-086-BH-003` | Resumed computation from preserved valid evaluation checkpoint | Checkpointed state continuation |
| L4 | `L4-I-087` | Approved / Locked | `L4-I-087-BH-002` | Recomputation against authoritative approved inputs before approval | Input immutability enforcement |
| L4 | `L4-I-087` | Approved / Locked | `L4-I-087-BH-009` | Provenance preservation across pre-approval calculation cycles | Calculation attempt lineage tracking |
| L4 | `L4-I-093` | Approved / Locked | `L4-I-093-BH-004` | Output integrity and current vs non-current result state | Technical output integrity verification |
| L4 | `L4-I-094` | Approved / Locked | `L4-I-094-BH-002` | Re-evaluation handling for inconsistent computed outcomes | Anomaly containment and error reporting |
| L4 | `L4-I-100` | Approved / Locked | `L4-I-100-BH-003` | Controlled recomputation and pre-publication replacement | Technical replacement without audit erase |
| Decision | Corpus Register | Locked | `L5-A-07-001` | Common trust, determinism, rule isolation; engines do not own rules | Canonical platform responsibility |
| Decision | Stage-4 Proceed | Locked | Stage-4 Gate | Mandates minimum dependencies: `D03 + A08 + B02 + E04` | Dependency-gate boundary enforcement |

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| C-001 | Computation engines shall never own business rules or business authority. | L5 Candidate Register §3.1 | Engines execute versioned definitions only; domain workflows retain approval authority. |
| C-002 | Technical computation shall not directly mutate canonical domain entities. | Repository Architecture Baseline | Computed outputs are returned to calling workflows via governed contracts. |
| C-003 | Historical execution evidence and calculation attempts shall not be overwritten. | ADR-L5-0012; L4-I-087-BH-009 | Recalculation creates an explicit successor lineage record referencing prior attempt hashes. |
| C-004 | Execution sandboxes shall not access network or external disk during execution. | ADR-L5-0001; L5-A-07-005 Boundary | Total isolation from I/O during pure computational evaluation. |

### 3.3 Assumptions

| ID | Assumption | Why Needed | Impact if False | Validation / Resolution |
|---|---|---|---|---|
| AS-001 | Calling domain workflows supply validated, authoritative input context. | Engine does not query domain databases directly. | Missing inputs cause admission rejection. | Calling workflows must bind input parameters before execution admission. |
| AS-002 | Deterministic calculations executed with identical inputs and rule versions yield identical output hashes. | Ensures audit reproducibility and verification. | Divergent results indicate non-deterministic rule logic. | Sandbox eliminates non-deterministic inputs (clocks, random seeds). |
| AS-003 | Concrete runtime isolation technology is provisioned via L5-E-04 packaging. | Engine logic remains technology-neutral while requiring process/memory containment. | Implementation-specific sandbox must satisfy resource bounds. | Validated through L5-E-04 deployment constraints. |

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 Owns

- Common computation admission boundary and parameter validation rules.
- Execution context and rule-package version binding.
- Technical execution sandbox lifecycle and resource containment.
- Cryptographic execution hash generation and output integrity checking.
- Calculation attempt lineage tracking and supersession evidence emission.
- Standard error classification for computation failure (overflow, timeout, division by zero, rule syntax fault).

### 4.2 Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-D-03-001` | Canonical Trust Boundary & Segmentation | Cryptographic boundary | Enforces runtime trust zones and engine isolation (ADR-L5-0001). |
| `L5-D-03-002` | Cryptography & Data Protection | Key custody & hashing | Generates tamper-evident execution lineage hashes (ADR-L5-0002). |
| `L5-A-08-001` | Background Processing & Job Execution | Asynchronous work-unit | Binds long-running computational jobs into governed execution queues (ADR-L5-0013). |
| `L5-B-02-001` | Database & Persistence Architecture Baseline | Persistence handoff | Stores calculation evidence records immutably in tenant-isolated persistence (ADR-L5-0005). |
| `L5-E-04-001` | Runtime Packaging & Deployment Baseline | Process isolation | Deploys computation workers in resource-constrained execution runtimes. |
| `L5-A-07-002..006` | Specialized Computation Engines | Platform baseline | Specialized engines inherit this admission, execution, and evidence framework. |

### 4.3 Must Not Own

- Business rules, formula definitions, tax slabs, grading regulations, or pay structures.
- Approval, posting, publication, or monetary distribution of calculation outcomes.
- Domain state transitions or aggregate lifecycles.
- Asynchronous message broker infrastructure (owned by L5-C-02).
- Audit trail event aggregation (owned by L5-D-04).

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| `ComputationAdmissionGate` | Control Point | Validates calling context, tenant identity, input completeness, and rule package availability. | `ComputationAdmissionRequest` | Stateless | Admitted execution envelope | `L5-D-01`, `L5-A-04` | Rejects missing context or invalid tenant binding. | L4-I-083-BH-001 |
| `RulePackageBinder` | Component | Resolves, verifies integrity, and loads approved, immutable rule definition bytecode/AST. | Rule package reference & version | Cached immutable rule AST | Bound execution package | `L5-A-07-005`, `L5-D-03` | Fails on package hash mismatch or missing rule version. | C-001, ADR-L5-0002 |
| `ExecutionSandbox` | Runtime Component | Executes computation within memory, instruction, and execution-time bounds. | Bound execution package & inputs | Ephemeral execution memory | Raw calculation results | `L5-E-04-001` | Aborts on timeout, memory limit, or arithmetic exception. | AD-A07-001, C-004 |
| `OutputIntegrityValidator` | Control Point | Validates output ranges, scale, precision, and structural correctness against output schema. | Raw calculation results | Stateless | Validated calculation output | Domain schema | Flags out-of-range or malformed outputs. | L4-I-093-BH-004 |
| `LineageEvidenceEmitter` | Component | Computes SHA-256 execution hash across inputs, rule version, and outputs; emits evidence record. | Inputs, rule version, output | Stateless | `ComputationEvidenceRecord` | `L5-D-03-002`, `L5-B-02` | Preserves failure records even when calculation fails. | L4-I-083-BH-009, ADR-L5-0012 |

### 5.2 Primary Technical Flow

```text
Domain Workflow Trigger
         ↓
ComputationAdmissionGate (verify Tenant, Scope, Input Completeness)
         ↓
RulePackageBinder (verify & load immutable versioned Rule AST)
         ↓
ExecutionSandbox (allocate ephemeral memory, set CPU timeout & instruction quota)
         ↓
[Deterministic Evaluation Execution]
         ↓
OutputIntegrityValidator (verify numerical scale, range, non-null invariants)
         ↓
LineageEvidenceEmitter (compute SHA-256 hash across Input + Rule + Output)
         ↓
Immutable Persistence Handoff (L5-B-02 / ADR-L5-0005)
         ↓
Return Result Envelope to Domain Workflow (No direct database posting)
```

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner | Basis |
|---|---|---|---|---|---|
| Admission Prerequisite Failure | `ComputationAdmissionGate` | Immediate rejection with structured diagnostic code (`ERR_ADMISSION_INCOMPLETE`). | Partial computation with inferred/default inputs. | Calling domain workflow must supply required context. | L4-I-083-BH-001 |
| Rule Package Missing or Corrupted | `RulePackageBinder` | Execution halt; diagnostic alert (`ERR_RULE_PACKAGE_UNAVAILABLE`). | Fallback to deprecated or unapproved rule version. | Rule administrator must deploy verified rule package. | C-001, ADR-L5-0002 |
| Execution Quota Exhaustion | `ExecutionSandbox` | Immediate sandbox termination (`ERR_RESOURCE_QUOTA_EXCEEDED`); record partial metrics. | Unbounded CPU looping or host memory starvation. | Specialized engine adjusts batching or flags complex entity. | AD-A07-004 |
| Arithmetic / Mathematical Fault | `ExecutionSandbox` | Trapped exception (`ERR_ARITHMETIC_FAULT`); computation marked FAILED. | Silent NaN, null injection, or arithmetic wraparound. | Domain workflow flags entity for human exception review. | L4-I-094-BH-002 |
| Output Invariant Breach | `OutputIntegrityValidator` | Output rejected (`ERR_OUTPUT_INVARIANT_VIOLATION`); quarantine result. | Emitting invalid or negative monetary/grade values. | Domain workflow halts progression of affected batch. | L4-I-085-BH-002 |
| Evidence Handoff Failure | `LineageEvidenceEmitter` | Hold computation result until evidence is safely durably queued. | Returning calculated results without audit evidence. | Retry evidence outbox handoff via L5-B-02 / L5-A-08. | ADR-L5-0012 |

### 5.4 Conceptual Contracts

#### `ComputationAdmissionRequest`
```text
ComputationAdmissionRequest {
  TenantId: UUID
  InstitutionId: UUID
  ExecutionCorrelationId: UUID
  DomainWorkflowType: String (e.g., "EXAM_RESULT", "FINANCE_FEE", "PAYROLL_RUN")
  EngineId: String ("L5-A-07-002" .. "L5-A-07-006")
  RulePackageId: String
  RulePackageVersion: String
  ExecutionMode: Enum [SYNCHRONOUS, ASYNCHRONOUS_BATCH]
  AuthoritativeInputs: Map<String, Any>
  ExecutionLimitsOverride: Optional<ExecutionLimits>
}
```

#### `ComputationEvidenceRecord`
```text
ComputationEvidenceRecord {
  EvidenceRecordId: UUID
  TenantId: UUID
  InstitutionId: UUID
  ExecutionCorrelationId: UUID
  EngineId: String
  RulePackageId: String
  RulePackageVersion: String
  InputSnapshotHash: String (SHA-256)
  OutputSnapshotHash: String (SHA-256)
  ExecutionLineageHash: String (SHA-256 of InputHash + RuleVersion + OutputHash)
  PriorAttemptHash: Optional<String> (for recalculation lineage)
  ExecutionDurationMs: Integer
  ExecutionOutcome: Enum [SUCCESS, FAILED, TERMINATED_QUOTA]
  FailureReasonCode: Optional<String>
  TimestampUtc: Timestamp
}
```

### 5.5 State, Persistence & Consistency Boundaries

The computation baseline is strictly **stateless**. It does not maintain durable domain entity state. Intermediate computational values reside exclusively in ephemeral sandbox memory and are discarded immediately upon execution completion or termination.

Evidence records generated by `LineageEvidenceEmitter` are persisted immutably through `L5-B-02` (Database Architecture) utilizing the transactional outbox pattern governed by `ADR-L5-0007`. Recalculation cycles never delete or update historical evidence records; a new attempt record is appended, explicitly citing the `PriorAttemptHash`.

### 5.6 Asynchronous & Job Realization

When invoked in `ASYNCHRONOUS_BATCH` mode (such as semester examination recalculation or monthly payroll runs), execution admission delegates to `L5-A-08-001` (Background Processing). Each batch item or chunk is modeled as an idempotent work-unit governed by `ADR-L5-0013`. Technical retry is permitted only for transient infrastructure errors; deterministic calculation failures are never retried automatically.

### 5.7 Security, Trust & Isolation

1. **Tenant Isolation:** Tenant and Institution boundaries are validated at admission. Execution context is strictly partitioned; memory spaces between tenant computations are completely isolated.
2. **Execution Sandbox Bounding:** Computation sandboxes are prohibited from executing network requests, disk file operations, or inter-process communication during calculation evaluation.
3. **No Dynamic Code Injection:** Dynamic compilation of untrusted strings or runtime code evaluation is forbidden. All rule packages must be cryptographically signed and resolved through approved catalog registries.

### 5.8 Runtime & Deployment Implications

Computation workers run as isolated worker runtimes governed by `L5-E-04` (Packaging and Runtime Isolation). Workers are stateless, scalable horizontally, and decoupled from primary web API request threads to prevent compute-heavy calculations from impacting user-facing interactive latency.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| `AD-A07-001` | Stateless Execution Sandbox | Pure in-memory stateless sandbox; intermediate state destroyed after execution. | Repository isolation baseline; C-002 | Prevents cross-request data leaks, concurrency conflicts, and state contamination between tenants. | None (Local L5 Architecture Decision) |
| `AD-A07-002` | Cryptographic Lineage Handoff | Every execution emits a SHA-256 lineage hash binding inputs, rule version, and outputs. | L4-I-083-BH-009; ADR-L5-0012 | Guarantees non-repudiation and allows independent post-calculation audit verification without re-running. | Governed by ADR-L5-0012 |
| `AD-A07-003` | Hard Isolation Boundary | Absolute ban on network, filesystem, and external process I/O during calculation phase. | ADR-L5-0001; C-004 | Eliminates side-channel leaks, non-deterministic latency, and remote code execution vulnerabilities. | Governed by ADR-L5-0001 |
| `AD-A07-004` | Deterministic Resource Quotas | Strict per-execution memory ceilings, maximum instruction cycle counts, and wall-clock execution timeouts. | L5-E-01-002 (Failure Isolation) | Prevents denial-of-service, infinite loops in complex rule evaluation, and resource starvation across engines. | Governed by ADR-L5-0014 |

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Tenant Isolation | Context binding rejects mismatched Tenant/Institution IDs; no shared memory. | Multi-tenant security baseline | `L5-A-04-001`, `L5-D-03-005` |
| Tamper Evidence | Cryptographic SHA-256 lineage hashes bound to all execution evidence records. | Legal auditability requirement | `L5-D-03-002`, `L5-D-04-003` |
| Execution Determinism | Sandbox isolates clocks and random number generators; pure input mapping. | Result reproducibility | `L5-A-07-001` |
| Resource Protection | Hard quota limits prevent computation from exhausting host worker memory. | High availability & resilience | `L5-E-01-001`, `L5-E-02-001` |

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| `CR-001` | An engine implementation shall admit computation only after verifying TenantId, InstitutionId, and required input completeness. | §5.1, `AD-A07-001` | `L4-I-083-BH-001` |
| `CR-002` | The engine shall bind an approved, immutable rule definition package and verify its version before execution begins. | §5.1, `AD-A07-002` | C-001 |
| `CR-003` | The execution environment shall enforce zero network and disk file I/O during the calculation lifecycle. | §5.7, `AD-A07-003` | C-004, ADR-L5-0001 |
| `CR-004` | Computational sandboxes shall enforce hard execution bounds on memory allocation and maximum execution time. | §5.1, `AD-A07-004` | ADR-L5-0014 |
| `CR-005` | The implementation shall compute a cryptographic SHA-256 lineage hash covering inputs, rule version, and outputs for every execution attempt. | §5.1, §5.4 | `L4-I-083-BH-009` |
| `CR-006` | In recalculation scenarios, historical calculation evidence shall be preserved immutably and linked via `PriorAttemptHash`. | §5.5, `AD-A07-002` | `L4-I-087-BH-009`, ADR-L5-0012 |
| `CR-007` | Arithmetic faults (overflow, divide-by-zero) shall result in explicit structured error codes and never return partial or default values. | §5.3 | `L4-I-094-BH-002` |
| `CR-008` | Computational outputs shall be validated against domain output schemas before release to the calling workflow. | §5.1 | `L4-I-093-BH-004` |
| `CR-009` | The engine shall not autonomously update domain database entities, publish business events, or grant business approval. | §4.3 | C-001, C-002 |
| `CR-010` | Long-running asynchronous batch computations shall integrate with the work-unit retry and progress model defined in `L5-A-08-001`. | §5.6 | ADR-L5-0013 |
| `CR-011` | Failed execution attempts shall generate an attributable evidence record documenting the failure cause. | §5.3, §5.4 | `L4-I-083-BH-009` |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| `OD-001` | Concrete Sandbox Runtime Technology | ADR-Required | WASM vs isolated container process vs hardened runtime VM | Lead Architect / L5-E-04 | No | Evaluate runtime efficiency and memory overhead in Stage 6 consolidation. |
| `OD-002` | Atomic L4 Version Binding Validation | Traceability | Exact version tagging across L4 repository files | Architecture Governance | No | Bind exact semantic versions upon final repository-wide release freeze. |
| `OD-003` | Unified Error Code Taxonomy | L5 Specification | Cross-engine diagnostic code standardization | L5-A-07 Family Owner | No | Reconcile error string enums across domain computation engines. |

---

## 10. Traceability Summary

- `L4-I-083-BH-001` — Realized by `ComputationAdmissionGate` for controlled marks admission (§5.1, `CR-001`).
- `L4-I-083-BH-009` — Realized by `LineageEvidenceEmitter` for attributable evidence emission (§5.1, `CR-005`).
- `L4-I-084-BH-003` — Realized by recomputation supersession execution and attempt tracking (§5.1, §5.5).
- `L4-I-086-BH-003` — Realized by state continuation and checkpointed input binding (§5.1).
- `L4-I-087-BH-002` — Realized by input immutability verification prior to recalculation (§5.1, `CR-001`).
- `L4-I-087-BH-009` — Realized by `PriorAttemptHash` chaining in evidence records (§5.4, `CR-006`).
- `L4-I-093-BH-004` — Realized by `OutputIntegrityValidator` preventing corrupted output publication (§5.1, `CR-008`).
- `L4-I-094-BH-002` — Realized by structured arithmetic error handling and anomaly isolation (§5.3, `CR-007`).
- `L4-I-100-BH-003` — Realized by pre-publication replacement without destroying prior evidence (§5.5, `CR-006`).
