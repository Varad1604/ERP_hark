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
| Version | 1.1.0 |
| Status | Review |
| Last Updated | 2026-09-15 |
| Depends On | L5-A-07-001; L5-D-03-001; L5-E-04-001; applicable approved L4 behaviors; L5 governance/template/validation standards |
| Referenced By | L5-A-07-002; L5-A-07-003; L5-A-07-004; L5-A-07-006 |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0002; ADR-L5-0014 |

## 2. Purpose & Boundary

Provides a reusable constrained declarative evaluation capability for formulas and rules supplied by authoritative owning domains. It deliberately excludes arbitrary executable code, direct SQL, filesystem access and unrestricted network access so that rule evaluation cannot become a hidden implementation bypass.

The engine owns syntax/structural validation and deterministic evaluation mechanics. It does not own the business meaning of a formula, rule precedence, policy approval or domain lifecycle.

## 3. Architectural Invariants

1. Rule content, meaning, ownership, approval and precedence remain upstream.
2. The execution model is declarative and capability-constrained; host-language arbitrary code execution is outside the architecture boundary.
3. Rule evaluation cannot directly query canonical persistence, mutate files, spawn processes or make unrestricted network calls.
4. Rule/version identity is immutable for a completed evaluation attempt.
5. Unsupported syntax or unsafe capability requests fail closed rather than falling back to a more permissive execution mode.
6. Resource exhaustion is bounded by governed controls, but exact numeric limits are not invented at L5.
7. Rule evaluation returns a technical result to the invoking owner and never mutates canonical business state directly.

## 4. Technical Realization

| Element | Responsibility | Input | Output | Failure Boundary |
|---|---|---|---|---|
| `RuleDefinitionAdmission` | Validate rule package identity/version, schema compatibility and owning-domain authorization/reference. | Authorized rule package | Admitted rule definition | Rejects missing owner/version or incompatible contract. |
| `DeclarativeParser` | Parse the governed declarative representation into a validated internal expression model. | Rule source/representation | Parsed expression model | Syntax errors fail before execution. |
| `StaticRuleValidator` | Reject unsupported constructs, unresolved references, unsafe operations and incompatible types before execution. | Parsed model | Validated executable model | Cannot downgrade validation or enable unsafe operations. |
| `ReferenceBinder` | Bind declared external facts/parameters to caller-supplied authoritative references rather than implicit database/network lookup. | Validated model + authoritative inputs | Bound evaluation context | Missing/unresolved fact references fail admission. |
| `BoundedEvaluator` | Evaluate only approved declarative operations within D03/E04-governed execution bounds. | Bound model | Candidate evaluation result | Resource/safety violation terminates evaluation safely. |
| `RuleResultValidator` | Validate output type/structure against caller-provided contract. | Candidate result | Validated result | Type/structure mismatch fails closed. |
| `RuleEvidenceAssembler` | Bind rule version, inputs, result and technical evidence to the A07 lineage model. | Evaluation attempt | Technical evidence | Evidence failure prevents successful completion where evidence is required. |

## 5. Rule Capability Model

The architecture distinguishes four capability classes:

- **Pure expression capabilities:** arithmetic, boolean, comparison and other approved deterministic operators.
- **Structured domain-neutral helpers:** bounded transformations or aggregations that do not encode business policy.
- **Authoritative fact inputs:** values supplied or referenced by the owning workflow rather than fetched implicitly.
- **Prohibited side-effect capabilities:** arbitrary code, dynamic module loading, direct persistence access, unrestricted filesystem mutation, process spawning or unrestricted network access.

The exact operator catalog and syntax are downstream governed design choices. Introducing a new side-effecting capability requires architecture impact review, not merely a parser enhancement.

## 6. Conceptual Contract

A rule package conceptually identifies owner, rule/package identity, immutable version, compatibility/profile reference, declared inputs, declared output type/profile and any caller-authorized helper capabilities.

An evaluation request conceptually identifies correlation identity, rule/version reference, authoritative input bindings and invoking context. A result conceptually returns technical outcome, value/reference, validation diagnostics and evidence/lineage reference.

Exact transport schemas remain downstream design.

## 7. Determinism, Repeatability & Versioning

- Where the invoking architecture requires repeatability, all externally varying facts must be bound explicitly as authoritative inputs.
- A completed attempt must identify the exact rule version and input bindings used.
- Updating a rule creates a new rule version rather than mutating the historical meaning of a completed attempt.
- Cached/compiled representations, if later chosen, are implementation details and must remain semantically equivalent to the approved rule version.

## 8. Failure & Recovery

| Failure | Treatment |
|---|---|
| Syntax/parse error | Reject before evaluation. |
| Unsafe/unsupported construct | Reject; never fall back to arbitrary code. |
| Missing authoritative input/reference | Reject/unresolved; do not query hidden persistence. |
| Type/output contract mismatch | Fail validation. |
| Deterministic evaluation error | Return technical rule failure; do not retry as transient. |
| Resource/safety limit exceeded | Terminate safely and return bounded technical failure. |
| Evidence failure | Do not report successful completion where evidence is required. |

## 9. Security & Trust

- Rule packages and input values are treated as untrusted data until validated.
- Evaluation runs under least privilege and D03/E04-governed isolation.
- Rule content cannot gain access to secrets, local files, processes, persistence or network merely by expression construction.
- Any future helper capability must be explicitly allow-listed through governed design and remain domain-neutral unless owned by the invoking business domain.

## 10. Observability & Evidence

Technical evidence should identify correlation identity, rule/package/version reference, validation outcome, authoritative input binding references, evaluation outcome, failure class and lineage/evidence handoff. Logs should avoid embedding unnecessary sensitive input values.

## 11. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-A07-050 | Rule execution is declarative and capability-constrained; arbitrary code/SQL/filesystem/unrestricted network execution is forbidden. | Canonical corpus responsibility |
| AD-A07-051 | Business rule content and precedence remain owned upstream; A07-005 executes supplied definitions only. | Repository ownership rule |
| AD-A07-052 | External facts are explicit authoritative inputs, not hidden direct database/network reads by the rule engine. | Repository-consistent technical derivation |
| OD-A07-050 | Concrete DSL syntax, parser/runtime technology, operator catalog and numeric execution limits remain unresolved pending governed design. | Open decision |

## 12. Conformance Requirements

1. No implementation may expose arbitrary host-language code execution as a rule feature.
2. Rule packages must be versioned and traceable to an owning-domain authorization/source.
3. Exact resource limits shall not be hardcoded into L5 without authoritative decision.
4. Evaluation must be isolated from direct persistence/filesystem/network side effects.
5. External facts must be supplied/bound authoritatively rather than fetched through hidden direct access.
6. Rule results must be returned to the invoking owner; this engine shall not mutate business state directly.
7. Unsafe or unsupported constructs must fail closed rather than trigger a permissive fallback.

## 13. Architecture Acceptance Evidence

A conforming design must demonstrate that arbitrary code cannot be executed, direct persistence/network/filesystem side effects are unavailable, rule/version and input bindings are traceable, unsupported constructs fail closed, and adding new capabilities requires explicit governed authorization.

## 14. Traceability Summary

A07-005 is a shared technical evaluation capability beneath specialized engines. It preserves upstream ownership of rule meaning and policy while supplying a safe, versioned, auditable and constrained execution boundary consistent with A07-001 and D03/E04 trust controls.