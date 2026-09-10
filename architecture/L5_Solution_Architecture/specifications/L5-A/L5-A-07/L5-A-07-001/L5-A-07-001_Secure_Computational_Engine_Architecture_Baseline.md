# L5-A-07-001 — Secure Computational Engine Architecture Baseline

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-A-07-001 |
| Specification Title | Secure Computational Engine Architecture Baseline |
| L5 Family | L5-A — Core Platform & Application Architecture |
| Major Area | L5-A-07 — Computational & Rules Engine Architecture |
| Status | Draft — Authoring Authorized; Not Locked |
| Version | 1.0.0 |
| Author | Varad |

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the common L5 technical architecture baseline required for secure, controlled, reproducible and evidentiary computational execution across SutraERP where approved L4 behavior requires computational processing.

It establishes the shared execution controls that specialized computation architectures can consume without transferring ownership of business rules, authoritative business facts, lifecycle semantics, approval authority, publication authority, or canonical business events.

### 2.2 Scope

This specification covers the common technical architecture for:

- admission of a computational request into a controlled execution boundary;
- integrity of the execution context and its required inputs;
- isolation of rule packages and computation context;
- deterministic or reproducible execution where required by the parent behavior;
- resource and execution containment;
- integrity and provenance of computational outputs;
- controlled recalculation and supersession of computational results;
- prevention and handling of duplicate or conflicting computational outputs;
- failure containment and safe execution termination;
- technical evidence needed to reconstruct or verify a computation;
- separation between technical computation and authoritative business outcome ownership.

### 2.3 Non-Goals

This specification does not define:

- domain-specific business rules or formulas;
- ownership of authoritative business facts;
- business lifecycle definitions;
- approval, finalization, publication, or certification authority;
- canonical business event definitions or publishers;
- APIs, payload schemas, protocols, queues, brokers, or transport mechanisms;
- database schemas or persistence implementation;
- software classes, services, DTOs, or programming languages;
- infrastructure or deployment technology;
- user interface or UX behavior;
- technology or product selection.

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Upstream Applicability

The specification is a reusable technical baseline for L4 behaviors that require controlled computation. It is particularly applicable to examination-result computation behaviors mapped to L5-A-07-003 and provides the common computational foundation for finance, payroll, general rules/formula, and scheduling/optimization architectures.

### 3.2 Architectural Constraints

The architecture shall preserve the following boundaries:

1. Specialized computation specifications retain responsibility for domain-specific computational semantics.
2. The common computational baseline shall not become the owner of domain business rules.
3. Canonical source domains remain authoritative for source facts and business decisions.
4. Technical computation shall not create or redefine business lifecycle states.
5. Technical computation shall not publish or redefine canonical business events independently of the owning business domain.
6. A computational output shall not become an authoritative business outcome merely because it was technically computed.

### 3.3 Assumptions

- The parent L4 behaviors define the business conditions under which computation is permitted and the authoritative sources of required business facts.
- Specialized L5 computation specifications define domain-specific computation responsibilities.
- Security, audit/evidence, and reliability concerns are coordinated with their respective L5 specifications rather than duplicated here.

## 4. Technical Architecture

### 4.1 Architectural Model

The common computation baseline is organized around a controlled execution boundary:

**Request Admission → Context Validation → Rule/Execution Isolation → Controlled Computation → Output Integrity → Evidence Capture → Downstream Handoff**

The execution boundary shall be capable of rejecting a computation before execution when required context, authority, integrity, or admissibility conditions are not satisfied.

### 4.2 Computation Admission

A computational request shall be admitted only when its required execution context can be established from authoritative inputs and applicable parent-defined conditions.

Admission shall establish, at minimum:

- computation scope;
- identity of the computational subject/context;
- required source facts or references;
- applicable rule context;
- execution purpose or reason where required;
- prerequisite validation state;
- authorization to perform the technical computation where such authorization is required by the parent behavior.

An invalid, incomplete, conflicting, or unauthorized execution context shall not proceed as a successful computation.

### 4.3 Execution-Context Integrity

The execution context shall preserve the identity and integrity of the inputs used by the computation.

The architecture shall distinguish between:

- authoritative source facts;
- derived computational inputs;
- rule or configuration context;
- intermediate computational values;
- computed outputs;
- technical evidence describing the execution.

The computation mechanism shall not silently replace an authoritative source fact with a locally invented or altered value.

### 4.4 Rule-Package Isolation

Business-rule execution shall be isolated from unrelated computational contexts so that one rule package or computation context cannot silently alter another.

The common baseline shall support identification of the rule context used by an execution and preservation of sufficient information to establish which approved computational context produced an output.

Domain-specific rule semantics remain owned by the specialized L5 architecture and upstream business architecture.

### 4.5 Deterministic and Reproducible Execution

Where the parent L4 behavior requires deterministic or reproducible computation, equivalent authoritative inputs and equivalent applicable rule context shall produce a verifiable equivalent computational outcome.

The architecture shall preserve sufficient execution evidence to determine whether differences in outputs arise from:

- changed authoritative inputs;
- changed applicable rule context;
- changed execution conditions;
- an execution failure or integrity issue.

The exact technical mechanism for achieving reproducibility is deferred to implementation design.

### 4.6 Resource Containment

Computational execution shall operate within controlled resource boundaries appropriate to the computation.

The architecture shall provide a means to prevent an individual computation from consuming uncontrolled resources or preventing unrelated required processing from continuing.

When a computation exceeds an established execution boundary, the execution shall terminate or transition to an explicitly controlled failure state rather than producing an unqualified successful output.

### 4.7 Output Integrity

A computed output shall remain associated with the execution context that produced it.

The architecture shall preserve:

- computation identity;
- relevant input references;
- applicable rule-context reference;
- execution status;
- output status;
- evidence necessary to distinguish current, superseded, rejected, failed, or otherwise non-current outputs where applicable.

The common baseline does not determine which output is the authoritative business outcome. That decision remains with the owning business domain and applicable L4 behavior.

### 4.8 Recalculation and Supersession

Where L4 behavior requires recomputation, the architecture shall support a controlled new computation using the applicable authoritative inputs and rule context.

A newly produced computational output shall not silently overwrite the historical identity of the prior execution. The architecture shall preserve the relationship between prior and subsequent executions sufficiently to establish what changed and why.

Supersession at the technical computation layer shall not be interpreted as business approval, publication, finalization, or other business authority.

### 4.9 Duplicate and Conflicting Computations

The architecture shall detect or prevent computational outputs that cannot safely coexist within the same execution scope where the parent behavior requires uniqueness or consistency.

When multiple technically valid outputs exist but the business behavior requires a single current result, the technical architecture shall preserve enough evidence for the owning business process to select, reject, or supersede outputs according to its governed rules.

The computation baseline shall not invent selection criteria.

### 4.10 Failure Containment

A computation that cannot complete with sufficient integrity shall not be represented as a successful computational outcome.

Failure handling shall preserve:

- execution identity;
- known execution state;
- failure classification where available;
- relevant input/context references;
- evidence required for investigation or controlled continuation.

Failure handling shall avoid corrupting unrelated computational executions and shall not create synthetic business lifecycle transitions.

### 4.11 Business Authority Boundary

The computation engine is a technical execution capability, not a business authority.

It may calculate, validate computational consistency, preserve execution evidence, and report technical execution status. It shall not independently:

- declare a student result official;
- approve a financial obligation;
- approve payroll;
- establish an academic policy;
- publish an examination result;
- establish eligibility;
- award a qualification;
- change a business lifecycle state;
- create a canonical business event.

Those responsibilities remain with their canonical owning domains and approved business workflows.

## 5. Dependencies / Integration

### 5.1 Specialized Computation Architectures

This common baseline is consumed by specialized computation architectures where applicable:

- **L5-A-07-002 — Finance Computation Engine Architecture**
- **L5-A-07-003 — Examination Result Computation Engine Architecture**
- **L5-A-07-004 — Payroll Computation Engine Architecture**
- **L5-A-07-005 — General Rules & Formula Engine Architecture**
- **L5-A-07-006 — Scheduling & Optimization Engine Architecture**

These specifications retain their respective computational responsibilities. This specification supplies the common secure execution baseline and does not absorb their domain semantics.

### 5.2 Security and Trust Dependency

The computation boundary depends on the applicable security and trust architecture for protection of execution context, sensitive inputs, credentials, integrity controls, and trust boundaries.

### 5.3 Audit and Evidence Dependency

The computation architecture produces technical evidence that must integrate with the governed audit and evidence architecture. It does not redefine the enterprise audit model or business meaning of audit events.

### 5.4 Reliability and Operations Dependency

Execution containment, failure handling, availability, and recovery shall align with the applicable reliability and operations architecture. This specification does not define deployment or operational technology.

## 6. Technical Behavior / Failure Concerns

### 6.1 Required Technical Behaviors

The architecture shall support the following observable behaviors:

| Behavior | Required outcome |
|---|---|
| Admission validation | Invalid or insufficient execution context is rejected before successful computation. |
| Context integrity | Authoritative input identity and relevant execution context remain traceable throughout computation. |
| Rule isolation | A computation executes against its applicable rule context without unauthorized cross-context mutation. |
| Controlled execution | Resource and execution boundaries prevent uncontrolled computational impact. |
| Output integrity | Every successful output is associated with its originating execution context. |
| Recalculation control | Recalculation creates a controlled subsequent execution rather than silently destroying historical execution evidence. |
| Conflict handling | Conflicting or duplicate outputs are prevented or exposed for governed resolution. |
| Failure containment | Failed computation does not appear as successful and does not corrupt unrelated executions. |
| Evidence preservation | Sufficient technical evidence remains available to establish what was executed and against which context. |
| Authority separation | Technical computation cannot independently become the authoritative business outcome. |

### 6.2 Failure Concerns

The architecture shall account for at least:

- missing required input context;
- invalid or inconsistent input references;
- unavailable required authoritative context;
- invalid rule context;
- execution limit breach;
- computation failure;
- output integrity failure;
- duplicate execution;
- conflicting output;
- interrupted execution;
- inability to preserve required evidence.

Where a failure is recoverable, continuation shall occur only through the governing workflow and applicable L4 behavior. The computation baseline shall not invent recovery policy.

## 7. Security / Trust / Evidence

### 7.1 Security Boundary

Computational execution shall operate inside an explicit technical trust boundary appropriate to the sensitivity and authority of its inputs and outputs.

Security controls shall prevent unauthorized alteration of computation context and shall preserve integrity of computational evidence.

### 7.2 Sensitive Inputs

Where computational execution processes sensitive or protected information, the architecture shall rely on the applicable enterprise security and data-protection controls rather than introducing independent business policy.

### 7.3 Evidence and Provenance

The computation architecture shall preserve evidence sufficient, subject to applicable data-protection requirements, to establish:

- what computation was executed;
- the execution scope;
- relevant source-input references;
- applicable rule-context reference;
- execution status;
- output status;
- relationship to preceding or subsequent executions where recalculation occurred;
- technical failure or integrity information where relevant.

Evidence requirements shall be implemented consistently with the canonical audit and evidence architecture.

## 8. Conformance

An implementation conforms to this specification when it demonstrates that the common computation capability:

1. admits computation only through a controlled execution boundary;
2. preserves integrity and traceability of authoritative inputs;
3. isolates applicable rule context;
4. supports deterministic/reproducible execution where required;
5. contains computational resource and execution impact;
6. preserves computational output integrity and execution lineage;
7. handles recalculation and supersession without destroying historical execution evidence;
8. prevents or exposes duplicate/conflicting computations according to parent-governed requirements;
9. distinguishes failed execution from successful computation;
10. preserves sufficient technical evidence for verification and investigation;
11. does not assume business authority that belongs to upstream or sibling architecture.

Conformance shall be demonstrated through architecture-level evidence and later implementation-level verification without changing the business meaning established by L1–L4.

## 9. Traceability

### 9.1 Traceability Method

The atomic traceability relation is:

**(L4 Business Behavior ID, L5 Specification ID)**

The mappings below identify L4 behaviors for which this common computation baseline provides a reusable technical capability. Domain-specific computation remains assigned to the corresponding specialized L5 architecture.

### 9.2 Directly Applicable Examination Computation Behaviors

| L4 Behavior | Relationship to L5-A-07-001 |
|---|---|
| L4-I-083-BH-001 | Common controlled admission and execution baseline for computation against parent-authorized approved marks and required context. |
| L4-I-083-BH-009 | Common execution evidence/provenance baseline for the computation and its resulting technical evidence. |
| L4-I-084-BH-003 | Common controlled recomputation execution baseline while preserving authoritative source facts. |
| L4-I-086-BH-003 | Common controlled continuation/re-execution baseline from a preserved valid computation position. |
| L4-I-087-BH-002 | Common execution baseline for recomputation against authoritative approved inputs and applicable rule context. |
| L4-I-087-BH-009 | Common execution evidence and provenance baseline for recomputation. |
| L4-I-088-BH-002 | Common recomputation execution baseline after authoritative correction. |
| L4-I-089-BH-003 | Common controlled recomputation execution baseline following an authoritative moderation outcome. |
| L4-I-090-BH-004 | Common execution baseline for recomputation of dependent academic values. |
| L4-I-091-BH-003 | Common execution baseline for computation of affected academic values. |
| L4-I-093-BH-004 | Common output-integrity and current/non-current technical output handling baseline. |
| L4-I-094-BH-002 | Common re-evaluation execution baseline for inconsistent computed outcomes. |
| L4-I-095-BH-003 | Common constrained/recomputation execution baseline following authoritative malpractice consequence. |
| L4-I-096-BH-003 | Common recomputation execution baseline following authoritative exception resolution. |
| L4-I-099-BH-003 | Common controlled recomputation execution baseline after reopening. |
| L4-I-100-BH-003 | Common controlled recomputation, validation, and output-integrity baseline for pre-publication replacement. |

### 9.3 Specialized Computation Routing

| L4 Area | Specialized L5 Architecture | Role of L5-A-07-001 |
|---|---|---|
| Finance computation | L5-A-07-002 | Common secure execution dependency; not owner of finance rules or monetary authority. |
| Examination result computation | L5-A-07-003 | Common secure execution baseline; specialized specification owns examination computation semantics. |
| Payroll computation | L5-A-07-004 | Common secure execution dependency; not owner of payroll rules or payroll authority. |
| General rules/formulas | L5-A-07-005 | Common execution baseline; specialized specification owns reusable rule/formula semantics. |
| Scheduling/optimization | L5-A-07-006 | Common execution baseline; specialized specification owns scheduling/optimization semantics. |

### 9.4 Explicit Non-Applicability Boundaries

The following are not direct ownership of this common baseline:

- **L4-F-001** — finance-specific monetary calculation remains within the specialized finance computation architecture.
- **L4-F-005** — installment modeling is not itself a common computation-engine ownership boundary.
- **L4-E-053, L4-E-055, L4-E-056** — payroll-specific computation remains within the specialized payroll computation architecture.
- **L4-G-137** — library fine calculation remains within the canonical Library Management business ownership boundary unless a governed reuse relationship is established.
- **L4-I-101** — official examination publication remains with its canonical publishing owner; its statement concerning academic computation ownership is a boundary condition, not a standalone computation-engine behavior.

### 9.5 Upstream Traceability

This specification traces through the approved architecture chain:

**L5-A-07-001 → exact L4 BH → exact L3 scenario → L2 workflow → L1 canonical domain ownership**

The L4 Business Behavior IDs listed above are the atomic traceability anchors. No document-level phrase such as “based on L4” is treated as sufficient traceability.

## 10. Open Issues / Deferred Design

The following are intentionally deferred to later architecture or implementation decisions:

1. Exact technical isolation mechanism for computational execution.
2. Exact mechanism for deterministic/reproducible execution evidence.
3. Exact resource quota and execution-boundary implementation.
4. Exact persistence mechanism for execution lineage and evidence.
5. Exact integration mechanism between common and specialized computation capabilities.
6. Exact technical trust-boundary implementation.
7. Detailed ADRs required when implementation choices are subsequently made.

These deferred items shall not be interpreted as permission to redefine L1–L4 business behavior.

## Appendix A — Responsibility Boundary Summary

| Responsibility | L5-A-07-001 | Other Owner |
|---|---|---|
| Common secure computation execution | **Owns** | — |
| Computation admission controls | **Owns common baseline** | Parent/specialized L5 defines domain conditions |
| Rule-package isolation | **Owns common baseline** | Specialized L5 owns rule semantics |
| Deterministic/reproducible execution capability | **Owns common baseline** | Parent L4 determines when required |
| Resource containment | **Owns common baseline** | L5-E reliability/operations governs broader operational behavior |
| Finance rules/calculation semantics | Does not own | L5-A-07-002 |
| Examination result computation semantics | Does not own | L5-A-07-003 |
| Payroll rules/calculation semantics | Does not own | L5-A-07-004 |
| General rule/formula semantics | Does not own | L5-A-07-005 |
| Scheduling/optimization semantics | Does not own | L5-A-07-006 |
| Business authority | Does not own | Canonical business domain/workflow |
| Audit model | Does not own | L5-D-04 |
| Security/trust architecture | Does not own | L5-D-03 |
| Reliability/operations architecture | Does not own | L5-E |

## Appendix B — Authoring Status

This document is an **L5 authoring draft**. Broad L5 authoring is authorized, but this specification is not represented as approved or locked. Any later approval or lock shall occur through the repository's governed L5 review and corpus-lock process.
