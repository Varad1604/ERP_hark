# L5-A-07-004 — Payroll Computation Engine Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-A-07-004 |
| Specification Title | Payroll Computation Engine Architecture |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-A — Core Platform & Application Architecture |
| Major Area | L5-A-07 — Secure Computational Engine Architecture |
| Canonical Technical Responsibility | Executes approved payroll calculations while Payroll retains business ownership and Finance owns downstream monetary consequences. |
| Technical Architecture Owner | Payroll Computation Engine Technical Architecture |
| Assigned Author | Varad |
| Version | 1.1.0 |
| Status | Review |
| Last Updated | 2026-09-15 |
| Depends On | L5-A-07-001; L5-A-07-005; L5-A-08-001; L5-B-02-001; L5-D-03-001; L4-E-053; L4-E-055; L4-E-056; L5 governance/template/validation standards |
| Referenced By | Payroll and downstream Finance workflows |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0005; ADR-L5-0013; ADR-L5-0014 |

## 2. Purpose & Boundary

Executes payroll calculations from authoritative payroll inputs and approved/versioned payroll rules. It does not create statutory thresholds, deduction caps, wage-floor rules, benefit policy, final-settlement policy, approval rules or Finance posting consequences.

Any statutory or institutional constraint is an authoritative business-rule input. A07-004 evaluates that input and reports the result; it does not infer a legal requirement from generic industry practice.

## 3. Architectural Invariants

1. Payroll policies and statutory interpretations are authoritative inputs, not engine-owned rules.
2. Earnings, deductions, contributions and net calculations are attributable to exact authoritative input/rule versions.
3. Payroll computation is technically separate from payroll approval, payment execution and accounting posting.
4. Missing statutory/policy inputs fail closed when required for the requested calculation.
5. Constraint violations are surfaced explicitly; the engine does not clamp, waive or substitute values silently.
6. Recalculation or rerun creates traceable technical lineage when historical evidence is required.
7. Retry cannot change authoritative payroll inputs or select a different rule version implicitly.

## 4. Technical Realization

| Element | Responsibility | Input | Output | Failure Boundary |
|---|---|---|---|---|
| `PayrollCalculationAdmission` | Validate employee/pay-period context, correlation identity, authoritative earnings/deduction inputs and exact payroll-rule package/version. | Authorized payroll calculation request | Admitted payroll request | Rejects missing or ambiguous payroll context. |
| `PayrollRuleBinder` | Bind approved payroll rule set and applicable institutional/statutory constraint references supplied by Payroll. | Rule/policy references | Bound payroll policy | No fallback to alternate local policy. |
| `PayrollEvaluationPipeline` | Execute earnings, deduction, contribution and net calculations exactly as defined by bound rules. | Bound rules + authoritative inputs | Candidate payroll result | Deterministic evaluation failure is returned without local policy invention. |
| `PayrollInvariantEvaluator` | Evaluate authoritative caller/rule-defined invariants. | Candidate result + invariant references | Validated payroll result | Cannot invent statutory or institutional thresholds. |
| `PayrollEvidenceAssembler` | Preserve exact rule/input/output lineage and predecessor rerun reference where applicable. | Completed attempt | Payroll technical evidence | Evidence failure prevents successful completion. |
| `PayrollResultHandoff` | Return computation result to Payroll for business review/approval. | Validated result | Payroll-owned result package | Cannot approve payroll, initiate payment or post Finance state. |

## 5. Conceptual Contract

A request conceptually identifies tenant/institution, employee/pay-period context, owning-workflow reference, correlation identity, authoritative compensation/attendance/benefit/deduction input references as applicable, exact payroll rule/version reference, and authoritative constraint references.

A result conceptually returns the technical attempt reference, calculated components/totals or references, evaluated invariant outcomes, exact input/rule references, predecessor/rerun reference where applicable, evidence reference, and safe failure classification.

Exact field names and schemas remain downstream design.

## 6. Payroll Evaluation Semantics

- The engine evaluates only supplied/authoritative payroll rules and facts.
- Gross-to-net sequencing must follow the authoritative payroll rule definition; no generic sequence is canonical here.
- Statutory deductions, employer contributions, taxable bases, benefit effects and final-settlement treatment remain business-rule inputs.
- Missing authoritative facts needed by a rule produce an explicit unresolved/input failure rather than a guessed value.
- Recalculation with changed authoritative inputs/rules is a new technical attempt and must remain distinguishable from the prior calculation.

## 7. Ownership

### Owns

Computation mechanics, technical invariant evaluation, lineage, execution evidence and result handoff.

### Does Not Own

Employee/payroll policy, statutory interpretation, voluntary deduction policy, final-settlement entitlement, payroll approval, payment execution, bank integration, ledger posting or accounting recognition.

## 8. Failure & Recovery

| Failure | Treatment |
|---|---|
| Missing payroll rule/version or required input | Reject admission. |
| Required statutory/institutional constraint unavailable | Fail unresolved; do not infer generic policy. |
| Authoritative constraint violation | Return violation to Payroll; no silent correction. |
| Deterministic formula failure | Return technical failure; no blind retry. |
| Evidence/persistence handoff failure | Do not report successful completion. |
| Transient execution failure | Delegate retry/work recovery to A08 where permitted. |

## 9. Security, Privacy & Evidence

- Payroll inputs/results are sensitive workforce/financial data and remain tenant/institution scoped.
- Least-privilege access and D03 trust controls apply to execution and evidence handling.
- Technical diagnostics should minimize disclosure of compensation or sensitive deduction details.
- A07-004 shall not directly mutate payroll approval, disbursement or Finance records.
- Audit/evidence aggregation remains D04-owned through the A07 baseline.

## 10. Observability & Operability

Technical evidence should support correlation of a payroll run with employee/pay-period context, rule/version, authoritative input references, constraint evaluations, calculation outcome, rerun predecessor where applicable, failure class and evidence handoff state. Performance/resource targets remain governed downstream.

## 11. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-A07-040 | Payroll policies and statutory constraints are authoritative inputs, not local engine rules. | Repository-corrected boundary |
| AD-A07-041 | Payroll computation is separated from approval, payment and accounting posting. | Canonical ownership boundary |
| AD-A07-042 | Missing required authoritative policy/facts fail unresolved rather than being replaced with generic industry assumptions. | Repository-consistent technical derivation |
| OD-A07-040 | Concrete monetary representation, payroll formula technology and numeric performance/resource limits remain unresolved until governed. | Open decision |

## 12. Conformance Requirements

1. A07-004 shall not hardcode wage floors, deduction caps, statutory rates or final-settlement rules unless cited to authoritative repository policy.
2. Every payroll calculation shall bind exact authoritative inputs and rule version.
3. The engine shall not approve payroll, initiate payment or post Finance state.
4. Violated business constraints shall be reported to Payroll rather than silently repaired.
5. Missing required statutory/policy facts shall fail unresolved rather than use generic defaults.
6. Reruns/recalculations shall preserve predecessor lineage where required.
7. Background work/retry remains A08-owned.

## 13. Architecture Acceptance Evidence

A conforming design must demonstrate that payroll rules are externally authoritative, statutory/institutional constraints are not invented locally, technical results cannot directly trigger payment/posting, and reruns remain traceable to the exact rule/input set used.

## 14. Traceability Summary

Referenced L4-E payroll behaviors remain the authority for exact payroll semantics. This L5 specification provides execution infrastructure only and deliberately excludes unsupported statutory/business-policy assertions.