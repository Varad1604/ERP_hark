# L5-A-07-005 — General Rules & Formula Engine Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-A-07-005 |
| Specification Title | General Rules & Formula Engine Architecture |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-A — Core Platform & Application Architecture |
| Major Area | L5-A-07 — Secure Computational Engine Architecture |
| Canonical Technical Responsibility | Constrained declarative formula/rule execution; no arbitrary code, SQL, filesystem or unrestricted network execution. |
| Technical Architecture Owner | General Rules & Formula Engine Platform Architecture |
| Assigned Author | Varad |
| Version | 1.0.0 |
| Status | Draft |
| Last Updated | 2026-09-15 |
| Depends On | L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5-A-07-001; L5-D-03-001; L5-E-04-001; L4-F-001; L4-E-053; L4-I-083 |
| Referenced By | L5-A-07-002; L5-A-07-003; L5-A-07-004; L5-A-07-006; specialized computation workflows |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0002; ADR-L5-0014 |

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the solution architecture for the General Rules & Formula Engine within SutraERP. The engine provides a shared, highly constrained, declarative expression evaluation capability utilized by specialized computation architectures (Finance, Examination, Payroll, Scheduling). It enforces strict abstract syntax tree (AST) validation, strong type checking, and deterministic evaluation, while providing an absolute security guarantee: **no arbitrary code execution, no SQL execution, no filesystem access, and no network access**.

### In Scope

- Parsing, validating, compiling, and evaluating declarative mathematical and logical rule expressions.
- Strong static type checking of input parameters and formula return types.
- Strict abstract syntax tree (AST) whitelisting preventing dangerous operations or language constructs.
- Deterministic sandbox evaluation with hard ceilings on instruction counts, stack recursion depth, and execution time.
- Pre-compilation and immutable caching of verified formula AST packages.
- Emission of rule evaluation lineage and trace hashes.

### Out of Scope / Non-Goals

- Executing Turing-complete arbitrary programming scripts (Python, JavaScript, Groovy, etc.).
- Direct database query execution or dynamic SQL generation.
- Accessing local filesystems or external network endpoints.
- Owning domain business rules, tax tables, academic passing thresholds, or payroll allowances.
- Business rule authoring UI or visual rule builder forms.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L4 | `L4-F-001` | Approved / Locked | `L4-F-001-BH-004` | Reusable evaluation of student fee calculation expressions | Constrained financial formula evaluation |
| L4 | `L4-E-053` | Approved / Locked | `L4-E-053-BH-003` | Reusable evaluation of allowance, deduction, and tax formulas | Constrained payroll rule evaluation |
| L4 | `L4-I-083` | Approved / Locked | `L4-I-083-BH-002` | Reusable evaluation of academic mark-to-grade mapping logic | Constrained academic grading evaluation |
| Decision | Corpus Register | Locked | `L5-A-07-005` | Constrained declarative evaluation; no code, SQL, filesystem or network | Canonical technical responsibility |

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| C-001 | The engine shall never evaluate raw programming language code or unrestricted script strings. | L5 Candidate Register §3.1 | Rule expressions are restricted to a declarative mathematical/logical domain-specific language (DSL). |
| C-002 | The engine shall have zero access to SQL, filesystem, system environment, or network sockets. | ADR-L5-0001; Security Baseline | Total isolation from all ambient system resources during evaluation. |
| C-003 | Expressions must be strictly non-recursive and guaranteed to terminate. | Reliability Baseline | AST validator rejects recursive function definitions and unbounded loops. |

### 3.3 Assumptions

| ID | Assumption | Why Needed | Impact if False | Validation / Resolution |
|---|---|---|---|---|
| AS-001 | Domain formulas can be fully expressed as declarative functional expressions. | Eliminates necessity for imperatively scripted logic. | Complex workflows must be decomposed into chained declarative steps. | Validated across Finance, Examination, and Payroll formula requirements. |
| AS-002 | Calling specialized engines provide all required external variables in the evaluation context. | Engine does not perform external I/O lookups. | Missing variable causes immediate type-binding failure. | Context verified prior to expression execution. |

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 Owns

- Declarative expression grammar, AST lexing, parsing, and syntax validation.
- Static semantic analysis and strict type checking of expressions against context schemas.
- Safe, non-Turing-complete declarative expression interpreter.
- Resource containment (instruction counter, call depth limit, execution timeout).
- Caching and versioning of pre-compiled, cryptographically signed formula AST packages.

### 4.2 Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-A-07-001` | Secure Computational Engine Baseline | Execution framework | Inherits admission gate, sandbox execution, and lineage hashing. |
| `L5-D-03-001` | Canonical Trust Boundary & Segmentation | Security boundary | Enforces complete network/filesystem isolation during evaluation. |
| `L5-E-04-001` | Runtime Packaging & Deployment Baseline | Worker execution | Deploys rule execution workers as hardened, isolated runtimes. |
| `L5-A-07-002..006` | Specialized Computation Engines | Consumer handoff | Specialized engines invoke this engine to evaluate declarative expressions. |

### 4.3 Must Not Own

- Defining business formulas, academic passing regulations, or tax policies.
- Storing domain business entities or managing student/employee records.
- Granting approval to formula execution outcomes.
- Providing arbitrary scripting runtimes for general-purpose application development.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| `ExpressionParser` | Component | Parses declarative rule text into a strongly typed Abstract Syntax Tree (AST); validates syntax. | Raw formula string | Stateless | Validated AST | Grammar spec | Rejects invalid grammar, unclosed braces, or illegal operators. | C-001 |
| `ASTSecurityValidator` | Control Point | Inspects AST nodes against strict whitelist; rejects loops, recursion, I/O calls, or reflection. | Validated AST | Stateless | Verified safe AST | Whitelist rules | Immediately aborts if forbidden functions or constructs are detected. | C-001, C-002 |
| `TypeCheckingBinder` | Component | Validates parameter names, types, and nullability against the declared input context schema. | Safe AST & Context schema | Cached schema | Typed Execution Plan | Schema catalog | Rejects type mismatches (e.g., adding String to Decimal). | C-001 |
| `ConstrainedEvaluator` | Runtime Component | Evaluates typed execution plan; enforces instruction count ceilings and timeout limits. | Typed Plan & Runtime Inputs | Ephemeral registers | Raw evaluated value | `L5-A-07-001` | Terminates on instruction limit, divide-by-zero, or timeout. | C-003, ADR-L5-0014 |
| `FormulaPackageCache` | Component | Stores pre-compiled, verified AST execution plans in immutable memory keyed by package hash. | Formula ID & Version | In-memory immutable cache | Cached Execution Plan | `L5-D-03-002` | Invalidates cache on rule package supersession. | AD-A07-051 |

### 5.2 Primary Technical Flow

```text
Specialized Engine Request (e.g., L5-A-07-002 Finance)
         ↓
FormulaPackageCache (lookup pre-compiled safe AST by RuleId + Version)
  [Cache Miss] → ExpressionParser (generate AST) 
                → ASTSecurityValidator (verify whitelist, reject I/O & loops)
                → TypeCheckingBinder (validate input/output types)
                → Cache compiled execution plan
         ↓
ConstrainedEvaluator (bind runtime inputs into ephemeral registers)
         ↓
[Execute with Instruction Counter & Timeout Clock]
         ↓
Format & Range Verification (verify result adheres to expected return type)
         ↓
Return Evaluated Result to Calling Specialized Engine (Zero database or network I/O)
```

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner | Basis |
|---|---|---|---|---|---|
| Disallowed Syntax / Construct | `ASTSecurityValidator` | Immediate rejection (`ERR_FORBIDDEN_EXPRESSION_NODE`). | Allowing dynamic function calls, reflection, or loop keywords. | Formula author must rewrite formula declaratively. | C-001, C-002 |
| Context Type Mismatch | `TypeCheckingBinder` | Evaluation halted (`ERR_TYPE_INCOMPATIBILITY`). | Implicit type coercion leading to numerical corruption. | Calling engine must supply strictly typed input context. | C-001 |
| Instruction Quota Exceeded | `ConstrainedEvaluator` | Execution aborted (`ERR_INSTRUCTION_LIMIT_EXCEEDED`). | Unbounded execution causing worker thread lockup. | Rule author must simplify nested conditional expressions. | C-003, ADR-L5-0014 |
| Division by Zero / Arithmetic Fault | `ConstrainedEvaluator` | Trapped exception (`ERR_ARITHMETIC_DIVISION_BY_ZERO`). | Silent conversion to Infinity or NaN. | Calling engine handles calculation exception per domain rules. | L4-I-094-BH-002 |

### 5.4 Conceptual Contracts

#### `FormulaEvaluationRequest`
```text
FormulaEvaluationRequest {
  TenantId: UUID
  EvaluationCorrelationId: UUID
  FormulaPackageId: String
  FormulaPackageVersion: String
  ExpectedReturnType: Enum [DECIMAL, BOOLEAN, INTEGER, STRING]
  InputContext: Map<String, TypedValue>
  MaxInstructionCount: Integer (Default: 50,000)
}
```

#### `FormulaEvaluationResult`
```text
FormulaEvaluationResult {
  EvaluationCorrelationId: UUID
  FormulaPackageId: String
  FormulaPackageVersion: String
  EvaluatedValue: Any
  ReturnType: Enum [DECIMAL, BOOLEAN, INTEGER, STRING]
  InstructionsConsumed: Integer
  ExecutionDurationMicroseconds: Integer
  EvaluationStatus: Enum [SUCCESS, FAILED_SYNTAX, FAILED_TYPE, FAILED_TIMEOUT]
  ErrorMessage: Optional<String>
}
```

### 5.5 State, Persistence & Consistency Boundaries

The General Rules & Formula Engine is strictly **stateless and in-memory**. It maintains zero persistent database state. Verified formula ASTs are stored in an in-memory, thread-safe, immutable cache. The cache is invalidated explicitly upon rule package version supersession.

### 5.6 Asynchronous & Job Realization

Because formula evaluations are pure in-memory computations taking microseconds, they execute synchronously within the calling specialized engine's worker thread. When invoked inside a batch job (e.g., evaluating tax for 10,000 employees), the caller manages batch iteration via `L5-A-08-001`.

### 5.7 Security, Trust & Isolation

1. **Zero External I/O:** The evaluator contains no dependencies on filesystem, network, process, or database libraries.
2. **Grammar Whitelisting:** The grammar supports only pure functional expressions:
   - Arithmetic: `+`, `-`, `*`, `/`, `%`, `^`
   - Comparison: `==`, `!=`, `<`, `<=`, `>`, `>=`
   - Logical: `AND`, `OR`, `NOT`
   - Built-in Pure Math Functions: `MIN`, `MAX`, `ROUND`, `CEIL`, `FLOOR`, `ABS`
   - Conditional: `IF(condition, true_expr, false_expr)`, `CASE`
3. **No Side Effects:** Variable reassignment, global state mutation, and mutable collections are strictly prohibited.

### 5.8 Runtime & Deployment Implications

Compiles as a lightweight, embeddable, pure computational library linked directly into the secure computational worker pool (`L5-E-04-001`).

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| `AD-A07-050` | Declarative Pure-Functional DSL | Restricted to pure mathematical and logical expressions without loops or state mutation. | Repository security baseline; C-001 | Eliminates remote code execution (RCE) vulnerabilities and ensures complete determinism. | None (Local L5 Architecture Decision) |
| `AD-A07-051` | Pre-Compiled Immutable AST Caching | Formulas are parsed, security-validated, and compiled into immutable AST objects once and cached. | Performance & security requirements | Maximizes evaluation throughput while ensuring security checks cannot be bypassed at runtime. | None (Local L5 Architecture Decision) |
| `AD-A07-052` | Total Ambient Resource Isolation | Complete absence of I/O, network, database, and reflection capabilities in the evaluator runtime. | ADR-L5-0001; C-002 | Enforces the locked computational engine security boundary. | Governed by ADR-L5-0001 |
| `AD-A07-053` | Bounded Instruction Execution Budget | Hard instruction-counter ceiling (default 50,000 operations) enforced per evaluation call. | L5-E-01-002; C-003 | Protects workers against denial-of-service through deeply nested expressions. | Governed by ADR-L5-0014 |

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Remote Code Execution Immunity | Zero scripting/eval capabilities; strict AST node whitelisting guarantees zero RCE vulnerability. | Threat modeling baseline | `L5-A-07-005`, `L5-D-03-001` |
| Evaluation Throughput | In-memory compiled AST evaluation executes in sub-millisecond durations per record. | System performance target | `L5-E-02-001` |
| Determinism Guarantee | Identical input variables and rule version strictly produce identical return values. | Mathematical integrity | `L5-A-07-001` |

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| `CR-001` | The engine shall accept only declarative expressions conforming to the approved mathematical/logical DSL grammar. | §5.1, `AD-A07-050` | C-001 |
| `CR-002` | The implementation shall enforce strict AST whitelisting and immediately reject loops, recursion, and assignment operations. | §5.1, `AD-A07-050` | C-001, C-003 |
| `CR-003` | The engine shall possess zero access to filesystem, network, environment variables, or database connections. | §5.7, `AD-A07-052` | C-002, ADR-L5-0001 |
| `CR-004` | Static type checking shall verify input parameter types and ensure the return value matches the declared output schema. | §5.1 | C-001 |
| `CR-005` | The evaluation runtime shall enforce an instruction-count ceiling and terminate evaluations exceeding the budget. | §5.1, `AD-A07-053` | C-003, ADR-L5-0014 |
| `CR-006` | Division by zero shall result in a structured arithmetic fault code and never return Infinity or NaN. | §5.3 | `L4-I-094-BH-002` |
| `CR-007` | Verified compiled ASTs shall be cached immutably and invalidated only upon explicit version supersession. | §5.1, `AD-A07-051` | `L5-A-07-001` |
| `CR-008` | The engine shall be purely stateless and maintain no persistent domain state. | §5.5 | `L5-A-07-001` |
| `CR-009` | Expression evaluation results shall be emitted with execution metrics (instructions consumed, duration). | §5.4 | `L5-A-07-001` |
| `CR-010` | The engine shall not possess authority or capability to write to domain database tables or approve business outcomes. | §4.3 | C-001 |
| `CR-011` | All variable references within an expression must be bound from the supplied input context; undeclared variables cause admission abort. | §5.1 | C-001 |
| `CR-012` | The engine shall provide deterministic execution, ensuring identical inputs and AST yield identical results. | §5.7 | `L5-A-07-001` |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| `OD-001` | Concrete Expression Parser Implementation | Technology / Library | ANTLR4 vs custom Pratt parser vs PEG parser | Lead Architect / L5-A-07 | No | Implement lightweight custom recursive-descent/Pratt parser during L6 technical design. |
| `OD-002` | Multi-Tenant Expression AST Memory Quota | Operations / Capacity | Maximum memory allocation for tenant AST caches | L5-E-02 Owner | No | Bound memory cache to 100MB per tenant in production configuration. |
| `OD-003` | User-Defined Custom Function Registry Extension | Extensibility | Extensible pure-function library registration | Platform Governance | No | Pure custom functions must be cryptographically signed and registered in L5-A-06. |

---

## 10. Traceability Summary

- `L4-F-001-BH-004` — Realized by declarative fee formula evaluation supporting Finance Engine (§5.1, `CR-001`).
- `L4-E-053-BH-003` — Realized by declarative allowance and tax formula evaluation supporting Payroll Engine (§5.1, `CR-001`).
- `L4-I-083-BH-002` — Realized by declarative grading scale and GPA expression evaluation supporting Examination Engine (§5.1, `CR-001`).
- `L4-I-094-BH-002` — Realized by structured arithmetic error trapping and fault containment (§5.3, `CR-006`).
