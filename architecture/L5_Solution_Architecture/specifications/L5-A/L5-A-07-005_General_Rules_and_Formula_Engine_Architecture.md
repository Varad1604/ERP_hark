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
| Version | 1.0.1 |
| Status | Locked |
| Last Updated | 2026-09-15 |
| Depends On | L5-A-07-001; L5-D-03-001; L5-E-04-001; applicable approved L4 behaviors; L5 governance/template/validation standards |
| Referenced By | L5-A-07-002; L5-A-07-003; L5-A-07-004; L5-A-07-006 |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0002; ADR-L5-0014 |

## 2. Purpose & Boundary

Provides a reusable constrained declarative evaluation capability for formulas and rules supplied by authoritative owning domains. It deliberately excludes arbitrary executable code, direct SQL, filesystem access and unrestricted network access so that rule evaluation cannot become a hidden implementation bypass.

The engine owns syntax/structural validation and deterministic evaluation mechanics. It does not own the business meaning of a formula, rule precedence, policy approval or domain lifecycle.

## 3. Technical Realization

| Element | Responsibility |
|---|---|
| `RuleDefinitionAdmission` | Validate rule package identity/version, schema compatibility and owning-domain authorization/reference. |
| `DeclarativeParser` | Parse the governed declarative representation into a validated internal expression model. |
| `StaticRuleValidator` | Reject unsupported constructs, unresolved references and unsafe operations before execution. |
| `BoundedEvaluator` | Evaluate only approved declarative operations within D03/E04-governed execution bounds. |
| `RuleResultValidator` | Validate output type/structure against the caller-provided contract. |
| `RuleEvidenceAssembler` | Bind rule version, inputs, result and technical evidence to the A07 lineage model. |

## 4. Safety Boundary

The rule language capability must remain deliberately constrained. It shall not expose arbitrary host-language code execution, dynamic module loading, direct database queries, filesystem mutation, process spawning or unrestricted network calls. Exact syntax, operator catalog and implementation technology are not frozen here; they must satisfy this architectural boundary.

Resource-exhaustion controls are required, but exact instruction counts, recursion depth, timeouts, memory limits and performance targets are governed downstream. No default numeric threshold is canonical in this specification.

## 5. Ownership

Owns declarative parsing/validation/evaluation mechanics. Does not own domain policy, formula content, business rule approval, rule precedence, persistence, network access, scheduler/retry policy or cryptographic implementation.

## 6. Failure & Recovery

Unsupported syntax, unsafe operation, unresolved reference, type mismatch or deterministic evaluation error fails the rule execution and returns diagnostics. The engine never relaxes the rule language or falls back to arbitrary code. Transient runtime failures are governed by the invoking engine/A08 rather than by local retry logic.

## 7. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-A07-050 | Rule execution is declarative and capability-constrained; arbitrary code/SQL/filesystem/unrestricted network execution is forbidden. | Canonical corpus responsibility |
| AD-A07-051 | Business rule content and precedence remain owned upstream; A07-005 executes supplied definitions only. | Repository ownership rule |
| OD-A07-050 | Concrete DSL syntax, parser/runtime technology, operator catalog and numeric execution limits remain unresolved pending governed design. | Open decision |

## 8. Conformance Requirements

1. No implementation may expose arbitrary code execution as a rule feature.
2. Rule packages must be versioned and traceable to an owning-domain authorization/source.
3. Exact resource limits shall not be hardcoded into L5 without authoritative decision.
4. Evaluation must be isolated from direct persistence/filesystem/network side effects.
5. Rule results must be returned to the invoking owner; this engine shall not mutate business state directly.