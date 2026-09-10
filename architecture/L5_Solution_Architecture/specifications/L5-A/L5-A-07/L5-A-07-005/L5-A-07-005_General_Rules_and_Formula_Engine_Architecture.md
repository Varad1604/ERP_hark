# L5-A-07-005 — General Rules & Formula Engine Architecture

## 1. Specification Metadata

**Specification ID:** L5-A-07-005  
**Architecture Level:** L5 — Solution Architecture  
**Technical Architecture Family:** L5-A — Core Platform & Application Architecture  
**Major Area:** L5-A-07 — Computation Engines  
**Version:** 1.0.0  
**Status:** Draft  
**Author:** Varad  
**Canonical Responsibility:** Provide constrained declarative rule and formula evaluation capability for reusable calculations, without owning business rules, business policy, lifecycle authority, canonical entities, or business outcomes.

### Primary L5 Dependency

- L5-A-07-001 — Secure Computational Engine Architecture Baseline

### Related L5 Specifications

- L5-A-07-002 — Finance Computation Engine Architecture
- L5-A-07-003 — Examination Result Computation Engine Architecture
- L5-A-07-004 — Payroll Computation Engine Architecture
- L5-A-07-006 — Scheduling & Optimization Engine Architecture
- L5-D — Identity, Security & Trust Architecture
- L5-D-04 — Audit & Evidence Architecture
- L5-E — Reliability, Deployment & Operations Architecture

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the technical architecture for a constrained declarative rules and formula evaluation capability that can be reused by domain-specific computation architectures.

The engine provides the technical boundary:

`approved rule/formula definition + authoritative input context`
→ `constrained evaluation`
→ `validated derived value / evaluation outcome`
→ `owning domain workflow`

The engine is deliberately generic. It provides execution capability; it does not become the owner of the business meaning represented by a rule or formula.

### 2.2 In Scope

- admission of an approved rule/formula evaluation;
- binding an evaluation to an explicit scope and definition version;
- constrained declarative expression evaluation;
- typed inputs and outputs;
- dependency-aware evaluation;
- deterministic/reproducible evaluation where required;
- validation of definitions before execution;
- evaluation resource containment;
- prevention of unsafe or unrestricted execution;
- controlled handling of invalid, incomplete or conflicting inputs;
- evaluation lineage and evidence;
- reusable invocation by specialized computation engines;
- controlled failure and non-authoritative outcomes.

### 2.3 Explicit Non-Goals

This specification does not:

- define or own business rules;
- decide institutional or domain policy;
- determine business eligibility;
- own Finance, Examination, Payroll, Scheduling or other domain outcomes;
- create canonical entities or lifecycle states;
- publish canonical business events;
- execute arbitrary application code;
- execute arbitrary SQL;
- access arbitrary filesystem resources;
- make unrestricted network calls;
- define APIs or transport protocols;
- define database schemas or persistence mechanisms;
- define UI/UX;
- define infrastructure topology;
- select technology products or vendors.

### 2.4 Ownership Boundary

The owning business domain remains authoritative for the business rule, formula meaning, applicability, approved version and resulting business decision.

The General Rules & Formula Engine owns only the technical evaluation mechanism and its execution evidence.

A successful evaluation is not itself a business approval, authorization, lifecycle transition or canonical business event.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Applicability

The repository establishes this specification as the reusable constrained declarative formula/rule execution capability within L5-A-07. It is a shared technical dependency for specialized computation architectures rather than a replacement for those domain-specific architectures.

The established L5 decomposition explicitly assigns:

- Finance calculations to L5-A-07-002;
- Examination result calculations to L5-A-07-003;
- Payroll calculations to L5-A-07-004;
- Scheduling/optimization to L5-A-07-006;
- reusable constrained formula/rule execution to L5-A-07-005.

Accordingly, this specification may execute a rule or formula on behalf of a consuming domain, but the consuming domain remains responsible for the business semantics and authority of that rule or formula.

### 3.2 Upstream Constraints

1. A rule/formula definition must be supplied from an authoritative business/configuration context.
2. The engine must bind execution to the supplied definition/version.
3. Inputs must be explicitly identified and typed.
4. Evaluation must occur inside a constrained execution boundary.
5. No arbitrary code, SQL, filesystem access or unrestricted network execution is permitted.
6. Evaluation cannot invent a missing business rule or substitute an unrelated formula.
7. The engine cannot convert evaluation success into business authority.
8. Domain-specific calculation ownership remains with the consuming specialized L5 architecture.

### 3.3 Assumptions

- **AS-001:** Consuming architectures can supply an approved rule/formula definition and version.
- **AS-002:** Required input values and their authoritative context can be supplied before evaluation.
- **AS-003:** Definitions can be represented in a constrained declarative form suitable for validation and deterministic evaluation.
- **AS-004:** The consuming workflow can distinguish a technical evaluation result from a canonical business outcome.
- **AS-005:** Evaluation evidence can reference definitions and inputs without acquiring their ownership.

These assumptions do not establish new business rules.

---

## 4. Technical Architecture

### 4.1 Architectural Model

The engine uses the following controlled pipeline:

1. **Evaluation Admission**
2. **Definition Resolution**
3. **Definition Validation**
4. **Scope and Input Binding**
5. **Dependency Analysis**
6. **Constrained Evaluation**
7. **Output Validation**
8. **Evidence/Lineage Capture**
9. **Controlled Handoff**

No stage may bypass the execution safety boundary or the business-authority boundary.

### 4.2 Definition Admission

A rule/formula definition shall be admitted only when it has an identifiable definition/version and is presented in the supported constrained declarative representation.

The engine shall reject definitions that contain unsupported execution constructs or cannot be statically validated within the permitted rule language.

The engine does not decide whether a definition is institutionally approved; it consumes the approval/version context established by the owner.

### 4.3 Constrained Declarative Evaluation

The execution model shall be limited to explicitly supported declarative operations and data types.

The evaluation environment shall not provide unrestricted capabilities to:

- invoke arbitrary executable code;
- execute arbitrary SQL or database commands;
- read or write arbitrary filesystem resources;
- make unrestricted network requests;
- inspect unrelated runtime state;
- invoke undeclared external capabilities.

A definition that attempts to escape the constrained evaluation model shall be rejected before producing a business-consumable result.

### 4.4 Typed Inputs and Outputs

Every evaluation shall establish the expected input and output types required by the definition.

The engine shall validate:

- required inputs are present;
- values conform to declared types;
- incompatible units or value categories are rejected where type semantics require it;
- output conforms to the declared result type;
- null/missing values are handled only where the definition explicitly permits them.

The engine shall not silently coerce incompatible values merely to complete execution.

### 4.5 Scope Binding

Each evaluation shall be bound to the consuming business context and calculation scope supplied by the caller.

Scope binding prevents accidental reuse of values belonging to another candidate, employee, financial obligation, scheduling context or other business scope.

The engine does not define those business scopes; it enforces the scope supplied for the evaluation.

### 4.6 Dependency Analysis

Definitions containing dependent expressions shall establish their dependency relationships before evaluation.

The engine shall:

- identify referenced inputs and intermediate values;
- establish a valid evaluation order;
- reject unresolved references;
- reject invalid dependency cycles where the supported evaluation model cannot resolve them;
- prevent evaluation of a dependent expression when a required predecessor is invalid.

### 4.7 Deterministic Evaluation

Where the consuming architecture requires reproducibility, equivalent definition/version, equivalent inputs and equivalent governed evaluation context shall produce an equivalent result.

Uncontrolled runtime factors shall not silently alter evaluation results.

If required deterministic context cannot be established, the engine shall fail or return a non-authoritative evaluation outcome rather than inventing missing context.

### 4.8 Rule/Formula Version Binding

Every material evaluation shall retain the exact definition/version identity used.

A new definition version shall not silently change the interpretation of a previously completed evaluation.

Re-evaluation under a changed definition shall be a distinguishable execution with its own evidence.

### 4.9 Evaluation Resource Containment

The engine shall constrain evaluation resource consumption so that a definition cannot consume unbounded computational resources.

Controls shall cover, as applicable:

- evaluation duration;
- expression depth;
- dependency expansion;
- intermediate result size;
- recursion or repeated evaluation where the supported language permits it;
- overall execution budget.

When a defined limit is exceeded, the evaluation shall terminate as a failed/non-authoritative execution.

Exact infrastructure mechanisms remain outside this specification.

### 4.10 Output Validation

Before handoff, the engine shall validate that the evaluation output:

- conforms to the declared output type;
- satisfies required structural constraints;
- was produced from the admitted definition/version;
- corresponds to the bound input context;
- completed without execution-limit or safety violations.

Output validation does not establish business validity or approval.

### 4.11 Evaluation Isolation

Each evaluation shall operate within a controlled execution context.

One evaluation shall not gain access to another evaluation's undeclared state or alter another evaluation's definition, inputs or results.

Cross-evaluation sharing shall occur only through explicitly supplied and governed inputs.

### 4.12 Failure Handling

The engine shall fail closed when it cannot safely establish the requested evaluation.

Failure conditions include:

- unsupported definition construct;
- missing definition/version;
- invalid definition;
- missing input;
- type mismatch;
- unresolved dependency;
- prohibited capability request;
- resource limit exceeded;
- execution failure;
- invalid output;
- inconsistent evaluation context.

A failed evaluation shall not be exposed as a valid business result.

### 4.13 Evidence and Lineage

Material evaluations shall preserve sufficient evidence to identify:

- evaluation identity;
- definition/version;
- consuming scope;
- material input references;
- evaluation status;
- output identity/value reference where appropriate;
- validation outcome;
- failure reason where applicable;
- predecessor or superseding evaluation where applicable.

Evidence references business inputs without becoming their canonical owner.

### 4.14 Controlled Handoff

The engine shall return the technical evaluation result to the consuming L5 architecture with sufficient context to identify the definition/version, scope, result and validation outcome.

The consuming architecture remains responsible for interpreting the result within its business boundary.

### 4.15 Specialized Engine Boundary

Specialized engines may invoke this capability for reusable rule/formula evaluation.

For example:

`L5-A-07-002 Finance Computation`
→ `L5-A-07-005 General Rules & Formula Engine`
→ `evaluated value`
→ `Finance business workflow`

The same pattern may apply to Examination or Payroll.

Invocation does not transfer specialized calculation ownership to this general engine.

### 4.16 Common Computational Baseline

L5-A-07-001 supplies common secure computational controls. L5-A-07-005 specializes those controls for constrained declarative rule/formula evaluation.

---

## 5. Dependencies / Integration

### 5.1 L5-A-07-001 — Secure Computational Engine Architecture Baseline

Provides common secure admission, execution containment, determinism/reproducibility, isolation and evidence controls.

### 5.2 Specialized Computation Engines

The engine may be consumed by:

- L5-A-07-002 Finance Computation Engine;
- L5-A-07-003 Examination Result Computation Engine;
- L5-A-07-004 Payroll Computation Engine;
- L5-A-07-006 Scheduling & Optimization Engine.

These specialized architectures retain ownership of their domain-specific calculation semantics.

### 5.3 Domain Business Workflows

Business workflows supply the authoritative rule/formula context and interpret the resulting evaluation according to their business architecture.

The general engine does not determine business applicability or authority.

### 5.4 L5-D Security / Trust

Provides enterprise security and trust controls for protected definitions, inputs, outputs and evidence.

### 5.5 L5-D-04 Audit & Evidence

Provides enterprise evidence architecture. This specification defines calculation/evaluation evidence that must be made available to that architecture.

### 5.6 L5-E Reliability / Operations

Provides platform-level availability, isolation and recovery mechanisms. This specification defines evaluation-level failure boundaries.

---

## 6. Technical Behavior / Failure Concerns

### 6.1 Required Technical Behaviors

| Behavior | Requirement |
|---|---|
| Definition admission | Admit only supported, identifiable rule/formula definitions. |
| Definition validation | Reject unsafe or structurally invalid definitions before execution. |
| Constrained execution | Permit only explicitly supported declarative operations. |
| Arbitrary execution prevention | No arbitrary code, SQL, filesystem or unrestricted network execution. |
| Scope binding | Bind evaluation to the supplied consuming context. |
| Type integrity | Reject incompatible input/output types. |
| Dependency integrity | Resolve dependencies before evaluation and reject unresolved/invalid dependencies. |
| Version binding | Preserve exact definition/version identity. |
| Determinism | Provide reproducibility where required by the consuming architecture. |
| Resource containment | Enforce bounded evaluation resource use. |
| Output validation | Reject malformed or unsafe outputs. |
| Isolation | Prevent undeclared state sharing across evaluations. |
| Evidence | Preserve evaluation lineage and technical evidence. |
| Authority separation | Evaluation success never equals business approval or authority. |

### 6.2 Failure Categories

**Definition failure:** definition is missing, unsupported or invalid.

**Input failure:** required input is missing or incompatible.

**Dependency failure:** required dependency cannot be resolved.

**Safety failure:** definition attempts prohibited execution capability.

**Resource failure:** evaluation exceeds an established execution bound.

**Execution failure:** evaluation cannot complete successfully.

**Output failure:** result does not conform to the declared output contract.

**Context failure:** evaluation scope/version/context is inconsistent.

### 6.3 Failure Containment

A failed evaluation shall not produce a valid business-consumable result.

Failure evidence shall identify the evaluation context and failure category without exposing protected implementation details beyond the authorized evidence boundary.

### 6.4 Re-evaluation

A re-evaluation under changed inputs, definition version or governed context shall be a distinguishable execution.

The prior evaluation shall remain traceable; it shall not be silently rewritten.

---

## 7. Security / Trust / Evidence

### 7.1 Trust Boundary

The General Rules & Formula Engine is a restricted execution boundary. Its principal security property is capability limitation: a rule/formula can evaluate only against explicitly supplied inputs and explicitly supported declarative operations.

### 7.2 Prohibited Capabilities

The engine shall not provide rule definitions with unrestricted ability to:

- execute arbitrary application code;
- execute arbitrary SQL;
- read/write arbitrary filesystem content;
- make unrestricted network calls;
- access undeclared secrets or credentials;
- access unrelated execution state.

Any future capability that expands this boundary requires explicit architecture governance and shall not be introduced implicitly through a rule definition.

### 7.3 Definition Integrity

The definition/version used for an evaluation shall be identifiable and protected against silent substitution during execution.

### 7.4 Evidence

Evidence should identify:

- definition/version;
- evaluation identity;
- input references;
- scope;
- output/result reference;
- validation outcome;
- execution status;
- failure category where applicable;
- re-evaluation/supersession relationship where applicable.

Enterprise audit storage and retention remain owned by L5-D-04 and related architecture.

---

## 8. Conformance

A conforming implementation shall demonstrate that:

1. Rule/formula definitions are admitted only in the supported constrained representation.
2. Definitions are validated before execution.
3. Arbitrary code execution is unavailable through the rule/formula mechanism.
4. Arbitrary SQL execution is unavailable through the rule/formula mechanism.
5. Arbitrary filesystem access is unavailable through the rule/formula mechanism.
6. Unrestricted network access is unavailable through the rule/formula mechanism.
7. Inputs and outputs are type-validated.
8. Evaluation scope is explicitly bound.
9. Definition/version identity is preserved.
10. Dependencies are resolved and controlled.
11. Resource bounds are enforced.
12. Invalid evaluations cannot be handed off as valid business results.
13. Re-evaluations remain distinguishable and traceable.
14. Technical evidence is available for material evaluations.
15. Domain-specific business rule ownership remains outside this engine.
16. Evaluation completion cannot independently approve, authorize, finalize or publish a business outcome.

---

## 9. Traceability

### 9.1 Atomic Traceability Method

The atomic relation is:

`(L4 Business Behavior ID, L5 Specification ID)`

L5-A-07-005 is a reusable technical capability. Its traceability therefore includes exact L4 behaviors that require formula/rule evaluation as a technical mechanism, while preserving the specialized L5 owner for the domain calculation.

### 9.2 Supporting Mappings

The currently established L4-to-L5 mapping foundation identifies rule/formula evaluation as a shared technical dependency for specialized computation behaviors, including:

- examination-result calculations under L4-I-083 through L4-I-100 where approved academic rules/formulas are executed;
- Finance calculation under L4-F-001-BH-004 where the applicable financial calculation is executed;
- Payroll calculation under L4-E-053-BH-003, L4-E-055-BH-002 and L4-E-056-BH-003 where governed Payroll calculations are executed.

These are **supporting technical relationships**, not transfers of L4 business-rule ownership to L5-A-07-005.

### 9.3 Specialized L5 Ownership Boundary

The architectural relation is:

`L4 business behavior`
→ `specialized L5 computation architecture`
→ `L5-A-07-005 reusable rule/formula evaluation where applicable`
→ `specialized computation result`
→ `owning business workflow`

L5-A-07-005 must not replace L5-A-07-002, L5-A-07-003, L5-A-07-004 or L5-A-07-006 as the owner of their respective technical calculation responsibilities.

### 9.4 Upstream Traceability Chain

`L1 domain architecture`
→ `L2 domain workflow`
→ `L3 locked business scenario`
→ `L4 exact Business Behavior`
→ `specialized L5 computation architecture`
→ `L5-A-07-005 reusable rule/formula evaluation where required`

The exact BH-level relation shall be recorded and validated when a consuming specialized L5 specification establishes its final traceability matrix.

---

## 10. Open Issues / Deferred Design

### 10.1 Open Technical Decisions

The following remain implementation-level decisions:

- exact declarative expression representation;
- exact supported type system;
- exact evaluation resource limits;
- exact isolation mechanism;
- exact evidence storage mechanism;
- exact interface/transport mechanism;
- exact concurrency/re-evaluation mechanism.

### 10.2 Capability Governance

Any future proposal to expand the evaluator with arbitrary executable code, unrestricted SQL, unrestricted filesystem access or unrestricted network capability is outside the current architecture boundary and requires explicit architecture governance before adoption.

### 10.3 Technology Selection

Technology/product selection is intentionally outside the current authoring scope.

---

## Canonical Architecture Boundary Note

L5-A-07-005 is the **constrained reusable rule/formula evaluation architecture**.

Its governing relationship is:

`authoritative business context + approved rule/formula definition`
→ `L5-A-07-005 constrained evaluation`
→ `validated technical evaluation result`
→ `specialized L5 computation architecture`
→ `owning business workflow`

The engine does not own business rules, institutional policy, canonical entities, lifecycle states, business approval, canonical business events or domain outcomes.

**Status:** Draft — not approved or locked.
