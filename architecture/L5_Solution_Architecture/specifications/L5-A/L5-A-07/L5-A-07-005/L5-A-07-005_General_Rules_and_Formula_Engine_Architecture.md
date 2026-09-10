# L5-A-07-005 — General Rules & Formula Engine Architecture

## 1. Specification Metadata

**Specification ID:** L5-A-07-005  
**Architecture Level:** L5 — Solution Architecture  
**Technical Architecture Family:** L5-A — Core Platform & Application Architecture  
**Major Area:** L5-A-07 — Computation Engines  
**Version:** 1.0.1  
**Status:** Draft  
**Author:** Varad  
**Canonical Responsibility:** Provide constrained declarative rule and formula evaluation capability for reusable calculations, without owning business rules, business policy, lifecycle authority, canonical entities, or business outcomes.

**Primary L5 Dependency:** L5-A-07-001 — Secure Computational Engine Architecture Baseline

**Related L5 Specifications:** L5-A-07-002, L5-A-07-003, L5-A-07-004, L5-A-07-006, L5-D, L5-D-04, L5-E.

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the reusable technical architecture for constrained declarative rule and formula evaluation used by specialized computation architectures.

`approved rule/formula definition + authoritative input context`
→ `constrained evaluation`
→ `validated technical evaluation result`
→ `specialized L5 computation architecture`
→ `owning business workflow`

The engine provides execution capability only. It does not own the business meaning represented by a rule or formula.

### 2.2 In Scope

- definition/version admission and binding;
- scope and typed-input binding;
- constrained declarative evaluation;
- dependency analysis;
- deterministic/reproducible evaluation where required;
- resource containment;
- output validation;
- execution isolation;
- evaluation evidence and lineage;
- controlled failure and handoff;
- reuse by specialized computation engines.

### 2.3 Explicit Non-Goals

This specification does not own business rules, institutional policy, eligibility, domain outcomes, canonical entities, lifecycle states, canonical business events, business approval, APIs, transport protocols, database schemas, UI/UX, infrastructure topology, or technology/product selection. It does not permit arbitrary application code, SQL, filesystem access or unrestricted network execution.

### 2.4 Ownership Boundary

The owning business domain remains authoritative for rule/formula meaning, applicability, approved version and resulting business decision. L5-A-07-005 owns only the technical evaluation mechanism and execution evidence. A successful evaluation is not a business approval, authorization, lifecycle transition or canonical event.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Applicability

The repository assigns L5-A-07-005 reusable constrained declarative rule/formula execution within L5-A-07. Finance, Examination, Payroll and Scheduling retain their specialized technical calculation ownership in L5-A-07-002, L5-A-07-003, L5-A-07-004 and L5-A-07-006 respectively.

### 3.2 Upstream Constraints

1. The definition/version is supplied from an authoritative context.
2. Inputs are explicitly identified and typed.
3. Evaluation occurs inside a constrained execution boundary.
4. The engine does not invent or substitute business rules.
5. Evaluation success does not create business authority.
6. Specialized calculation ownership remains with the consuming L5 architecture.

### 3.3 Assumptions

- **AS-001:** Consuming architectures can supply an approved rule/formula definition and version.
- **AS-002:** Required authoritative inputs can be supplied before evaluation.
- **AS-003:** Definitions can be represented in a constrained declarative form.
- **AS-004:** Consumers distinguish technical evaluation from canonical business outcomes.
- **AS-005:** Evidence can reference inputs/definitions without acquiring ownership.

---

## 4. Technical Architecture

### 4.1 Controlled Pipeline

`Admission → Definition Resolution → Definition Validation → Scope/Input Binding → Dependency Analysis → Constrained Evaluation → Output Validation → Evidence/Lineage → Controlled Handoff`

No stage bypasses execution safety or business-authority boundaries.

### 4.2 Definition Admission and Validation

A definition is admitted only with identifiable definition/version identity and supported constrained declarative representation. Unsupported or unsafe constructs are rejected before a business-consumable result exists. Institutional approval is consumed from the owning context, not decided by the engine.

### 4.3 Constrained Evaluation

Only explicitly supported declarative operations and data types are available. The evaluator shall not permit arbitrary executable code, SQL/database commands, arbitrary filesystem access, unrestricted network calls, unrelated runtime-state inspection, undeclared external capabilities, or undeclared secrets/credentials.

### 4.4 Typed Inputs and Outputs

Required inputs, declared types, permitted null/missing semantics and output type shall be validated. Incompatible values shall not be silently coerced merely to complete evaluation.

### 4.5 Scope Binding

Each evaluation is bound to the consuming business context and calculation scope supplied by the caller. The engine enforces supplied scope but does not define business scope.

### 4.6 Dependency Analysis

The engine identifies referenced inputs/intermediate values, establishes valid evaluation order, rejects unresolved references and unsupported dependency cycles, and prevents dependent evaluation when required predecessors are invalid.

### 4.7 Determinism and Version Binding

Where required, equivalent definition/version, inputs and governed evaluation context shall produce equivalent results. Every material evaluation retains the exact definition/version used. Changed-definition re-evaluation is distinguishable and does not silently rewrite prior evidence.

### 4.8 Resource Containment

Evaluation shall be bounded for duration, expression depth, dependency expansion, intermediate result size, recursion/repeated evaluation where applicable and overall execution budget. Exceeding a bound produces a failed/non-authoritative technical outcome.

### 4.9 Output Validation and Isolation

Before handoff, output type, structure, definition/version, input context and execution integrity are validated. Each evaluation operates in a controlled context and cannot access or mutate another evaluation's undeclared state.

### 4.10 Failure Handling

The engine fails closed for unsupported/invalid definitions, missing definition/version, missing inputs, type mismatch, unresolved dependencies, prohibited capabilities, resource-limit violations, execution failure, invalid output and inconsistent context. Failed evaluation is never exposed as a valid business result.

### 4.11 Evidence and Lineage

Material evaluations retain, as applicable, evaluation identity, definition/version, scope, material input references, status, output/result reference, validation outcome, failure reason and predecessor/superseding evaluation relationship. Enterprise audit storage remains governed by L5-D-04.

### 4.12 Controlled Handoff

The consuming specialized L5 architecture receives the technical result with sufficient definition/version, scope and validation context. The consumer interprets it within its own business boundary.

### 4.13 Specialized Engine Boundary

`Specialized L5 computation → L5-A-07-005 → technical evaluated value → specialized computation/business workflow`

Invocation never transfers specialized calculation ownership to L5-A-07-005.

### 4.14 Common Computational Baseline

L5-A-07-001 provides common secure computational controls; L5-A-07-005 specializes those controls for declarative rule/formula evaluation.

---

## 5. Dependencies / Integration

### 5.1 L5-A-07-001
Common secure admission, execution containment, determinism/reproducibility, isolation and evidence controls.

### 5.2 Specialized Computation Engines
L5-A-07-002, L5-A-07-003, L5-A-07-004 and L5-A-07-006 may consume this capability while retaining their own technical calculation responsibilities.

### 5.3 Domain Business Workflows
Supply authoritative rule/formula context and interpret the technical evaluation.

### 5.4 L5-D / L5-D-04 / L5-E
Provide applicable security/trust, enterprise evidence, reliability, isolation, availability and recovery architecture. This specification does not redefine those responsibilities.

---

## 6. Technical Behavior / Failure Concerns

| Behavior | Requirement |
|---|---|
| Definition admission | Admit only identifiable supported definitions. |
| Definition validation | Reject unsafe/invalid definitions before execution. |
| Constrained execution | Permit only supported declarative operations. |
| Arbitrary execution prevention | No arbitrary code, SQL, filesystem or unrestricted network execution. |
| Scope binding | Bind evaluation to supplied consuming context. |
| Type integrity | Reject incompatible inputs/outputs. |
| Dependency integrity | Resolve dependencies before evaluation. |
| Version binding | Preserve exact definition/version identity. |
| Determinism | Provide reproducibility where required. |
| Resource containment | Enforce bounded evaluation. |
| Output validation | Reject invalid outputs. |
| Isolation | Prevent undeclared cross-evaluation state access. |
| Evidence | Preserve technical lineage. |
| Authority separation | Evaluation never independently becomes business authority. |

Failure categories: definition, input, dependency, safety, resource, execution, output and context failure.

A failed evaluation shall not produce a valid business-consumable result. Re-evaluation remains distinguishable and prior evidence remains traceable.

---

## 7. Security / Trust / Evidence

The engine is a restricted computational trust boundary. Rule definitions cannot execute arbitrary code/SQL, access arbitrary filesystems, make unrestricted network calls, access undeclared secrets, or inspect unrelated execution state.

Definition/version identity shall be protected against silent substitution. Evidence shall identify definition/version, evaluation identity, inputs, scope, output/result reference, validation, status, failure category and re-evaluation relationship where applicable.

Enterprise audit storage and retention remain owned by L5-D-04.

---

## 8. Conformance

A conforming implementation shall demonstrate that:

1. Definitions are admitted only in the supported constrained representation.
2. Definitions are validated before execution.
3. Arbitrary code, SQL, filesystem and unrestricted network execution are unavailable.
4. Inputs and outputs are type-validated.
5. Evaluation scope is explicitly bound.
6. Definition/version identity is preserved.
7. Dependencies are resolved and controlled.
8. Resource bounds are enforced.
9. Invalid evaluations cannot be handed off as valid business results.
10. Re-evaluations remain distinguishable and traceable.
11. Technical evidence is available for material evaluations.
12. Domain-specific business-rule ownership remains outside this engine.
13. Evaluation completion cannot independently approve, authorize, finalize or publish a business outcome.

---

## 9. Traceability

### 9.1 Atomic Traceability Method

The atomic relation is:

`(L4 Business Behavior ID, L5 Specification ID)`

For this reusable capability, the relation identifies exact L4 behaviors where constrained rule/formula evaluation is a technical contribution. The specialized L5 specification remains primary owner of the domain calculation.

### 9.2 Atomic Applicability Mapping

| Exact L4 Business Behavior | L5-A-07-005 Contribution | Specialized L5 Owner |
|---|---|---|
| L4-F-001-BH-004 | Reusable constrained evaluation of the applicable financial calculation. | L5-A-07-002 |
| L4-E-053-BH-003 | Reusable constrained evaluation within Payroll calculation. | L5-A-07-004 |
| L4-E-055-BH-002 | Reusable constrained evaluation for retrospective/retro-pay calculation. | L5-A-07-004 |
| L4-E-056-BH-003 | Reusable constrained evaluation for final-settlement Payroll calculation. | L5-A-07-004 |
| L4-I-083-BH-002 | Reusable constrained evaluation of approved academic calculation rules/formulas. | L5-A-07-003 |
| L4-I-084-BH-003 | Reusable constrained evaluation of the replacement academic result. | L5-A-07-003 |
| L4-I-087-BH-002 | Reusable constrained evaluation for pre-approval recomputation. | L5-A-07-003 |
| L4-I-088-BH-002 | Reusable constrained evaluation of dependent academic values after correction. | L5-A-07-003 |
| L4-I-089-BH-003 | Reusable constrained evaluation of affected academic values after moderation. | L5-A-07-003 |
| L4-I-090-BH-003 | Reusable constrained evaluation of affected academic values after approved benefit handling. | L5-A-07-003 |
| L4-I-091-BH-003 | Reusable constrained evaluation of affected academic values. | L5-A-07-003 |
| L4-I-093-BH-003 | Reusable constrained evaluation supporting the selected valid computation. | L5-A-07-003 |
| L4-I-094-BH-002 | Reusable constrained re-evaluation of inconsistent computed outcomes. | L5-A-07-003 |
| L4-I-095-BH-003 | Reusable constrained evaluation of affected results after authoritative consequence handling. | L5-A-07-003 |
| L4-I-096-BH-003 | Reusable constrained evaluation of dependent academic values. | L5-A-07-003 |
| L4-I-099-BH-003 | Reusable constrained re-evaluation following authorized reopening. | L5-A-07-003 |
| L4-I-100-BH-003 | Reusable constrained re-evaluation for approved pre-publication replacement. | L5-A-07-003 |

These are technical reuse relationships only. They do not transfer L4 business-rule ownership, lifecycle authority, canonical event ownership or business outcome ownership to L5-A-07-005.

### 9.3 Specialized L5 Ownership Boundary

`L4 business behavior → specialized L5 computation architecture → L5-A-07-005 reusable rule/formula evaluation where applicable → specialized computation result → owning business workflow`

L5-A-07-005 does not replace L5-A-07-002, L5-A-07-003, L5-A-07-004 or L5-A-07-006.

### 9.4 Upstream Traceability Chain

`L1 domain architecture → L2 domain workflow → L3 locked business scenario → L4 exact Business Behavior → specialized L5 computation architecture → L5-A-07-005 reusable rule/formula evaluation`

The atomic mappings above are the current applicability baseline; additional mappings require supporting approved L4 traceability evidence.

---

## 10. Open Issues / Deferred Design

- exact declarative expression representation;
- exact supported type system;
- exact evaluation resource limits;
- exact isolation mechanism;
- exact evidence storage mechanism;
- exact interface/transport mechanism;
- exact concurrency/re-evaluation mechanism.

Any future expansion to arbitrary executable code, unrestricted SQL, unrestricted filesystem or unrestricted network capability requires explicit architecture governance.

Technology/product selection is intentionally outside the current authoring scope.

---

## Canonical Architecture Boundary Note

L5-A-07-005 is the constrained reusable rule/formula evaluation architecture.

`authoritative business context + approved rule/formula definition → L5-A-07-005 constrained evaluation → validated technical evaluation result → specialized L5 computation architecture → owning business workflow`

The engine does not own business rules, institutional policy, canonical entities, lifecycle states, business approval, canonical business events or domain outcomes.

**Status:** Draft — not approved or locked.
